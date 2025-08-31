use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Rectangle};
use std::collections::HashMap;
use crate::components::*;
use crate::resources::*;
use crate::material::HexTileMaterial;
use crate::atlas::{AtlasIndex, AtlasRect};
use crate::systems::movement::axial_to_world;

#[derive(Component)] pub struct HexCell { pub q: i32, pub r: i32, pub kind: String }

pub fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HexTileMaterial>>,
    wb: Res<WorldBook>,
    atlas: Res<AtlasIndex>,
) {
    let radius = 5;
    let mut cells: HashMap<(i32,i32), String> = HashMap::new();
    for q in -radius..=radius {
        for r in -radius..=radius {
            if (q.abs() + r.abs() + (q+r).abs())/2 <= radius { cells.insert((q,r), "hex".into()); }
        }
    }
    for region in &wb.regions {
        for poi in &region.hex_points {
            let (q,r) = parse_axial(&poi.axial);
            cells.insert((q,r), poi.kind.clone());
        }
    }
    let atlas_size = atlas.size().unwrap_or((256.0,256.0));
    for ((q,r), kind) in &cells {
        let (x,y) = axial_to_world(*q, *r, 32.0);
        let mesh = Mesh::from(Rectangle::new(256.0, 256.0));
        let mh = meshes.add(mesh);
        let (region_rect, tex_handle) = atlas_region_for(&atlas, &asset_server, &kind);
        let mask = mask_bits_for(&cells, *q, *r, &kind);
        let mat = materials.add(HexTileMaterial {
            color_texture: Some(tex_handle.clone()),
            color: Color::WHITE,
            region: Vec4::new(region_rect.u as f32, region_rect.v as f32, region_rect.w as f32, region_rect.h as f32),
            atlas_size: Vec2::new(atlas_size.0, atlas_size.1),
            mask_bits: mask,
            edge_color: Color::rgba(0.0,0.0,0.0,0.85),
            edge_strength: 0.85,
        });
        commands.spawn((
            MaterialMesh2dBundle { mesh: Mesh2dHandle(mh), material: mat, transform: Transform::from_xyz(x,y,-2.0), ..default() },
            HexCell { q: *q, r: *r, kind: kind.clone() },
        ));
    }
    // spawn an example NPC at village
    let tex = asset_server.load("sprites/npc_pawn.png");
    commands.spawn((SpriteBundle{ texture: tex, transform: Transform::from_xyz(0.0, 0.0, 10.0), ..default() }, Npc{ id:"n_vicar".into(), name:"Under-Vicar Marn".into() }, AxialPos{ q:0, r:0 }));
}

fn parse_axial(s: &str) -> (i32, i32) {
    let mut it = s.split(',');
    let q: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
    let r: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
    (q, r)
}

fn atlas_region_for(atlas: &AtlasIndex, assets: &AssetServer, kind: &str) -> (crate::atlas::AtlasRect, Handle<Image>) {
    if let Some(meta) = &atlas.meta {
        let key = format!("kinds/{}", kind);
        if let Some(r) = meta.tiles.get(&key) { return (*r, atlas.atlas_image.clone()); }
        if let Some((_,r)) = meta.tiles.iter().next() { return (*r, atlas.atlas_image.clone()); }
    }
    (crate::atlas::AtlasRect{u:0,v:0,w:256,h:256}, assets.load(&format!("textures/{}.png", kind)))
}

fn mask_bits_for(cells: &std::collections::HashMap<(i32,i32), String>, q: i32, r: i32, kind: &str) -> u32 {
    let dirs = [(0,-1),(1,-1),(1,0),(0,1),(-1,1),(-1,0)];
    let mut bits: u32 = 0;
    for (i, (dq,dr)) in dirs.iter().enumerate() {
        let nk = cells.get(&(q+dq, r+dr)).cloned().unwrap_or_else(|| "hex".into());
        if nk != *kind { bits |= 1u32 << i; }
    }
    bits
}
