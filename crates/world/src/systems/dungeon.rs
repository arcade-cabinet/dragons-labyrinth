use bevy::prelude::*;
use serde::Deserialize;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Rectangle};
use crate::material::HexTileMaterial;
use crate::systems::movement::axial_to_world;

#[derive(Debug, Clone, Deserialize)] struct DungeonTile { axial: String, kind: String }
#[derive(Debug, Clone, Deserialize)] struct DungeonFile { id: String, spawn: String, tiles: Vec<DungeonTile> }
#[derive(Component)] pub struct DungeonTileTag;

pub fn dungeon_enter_exit(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HexTileMaterial>>,
    q_dungeon: Query<Entity, With<DungeonTileTag>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        let filename = "build/dungeons/dungeon_0_-1.json";
        if let Ok(text) = std::fs::read_to_string(&filename) {
            if let Ok(df) = serde_json::from_str::<DungeonFile>(&text) {
                for t in df.tiles {
                    let parts: Vec<_> = t.axial.split(',').collect();
                    let q: i32 = parts[0].trim().parse().unwrap_or(0);
                    let r: i32 = parts[1].trim().parse().unwrap_or(0);
                    let (x,y) = axial_to_world(q, r, 32.0);
                    let mesh = Mesh::from(Rectangle::new(256.0,256.0));
                    let mh = meshes.add(mesh);
                    let mat = materials.add(HexTileMaterial{ 
                        color_texture: Some(asset_server.load("textures/dungeon.png")), color: Color::WHITE,
                        region: Vec4::new(0.0,0.0,256.0,256.0), atlas_size: Vec2::new(256.0,256.0),
                        mask_bits: 0, edge_color: Color::BLACK, edge_strength: 0.0
                    });
                    commands.spawn((MaterialMesh2dBundle { mesh: Mesh2dHandle(mh), material: mat, transform: Transform::from_xyz(x, y, -1.0), ..default() }, DungeonTileTag));
                }
                info!("Dungeon loaded");
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for e in q_dungeon.iter() { commands.entity(e).despawn_recursive(); }
    }
}
