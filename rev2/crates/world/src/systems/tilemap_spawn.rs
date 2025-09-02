use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;
use crate::resources::*;
use crate::components::*;
use crate::index::WorldIndex;
use crate::tilemap_bridge::{parse_axial};

// Constants for atlas grid tile size (match our generated assets)
const TILE_W: f32 = 128.0;
const TILE_H: f32 = 128.0;

#[derive(Resource, Default)]
pub struct TileTextures { pub handle: Handle<Image> }

pub fn spawn_from_worldbook(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wb: Res<WorldBook>,
    mut idx: ResMut<WorldIndex>,
) {
    let texture: Handle<Image> = asset_server.load("build/atlas/atlas.png");
    commands.insert_resource(TileTextures { handle: texture.clone() });

    let map_size = TilemapSize { x: 64, y: 64 };
    let tilemap_entity = commands.spawn_empty().id();

    let grid_size = TilemapGridSize { x: TILE_W, y: TILE_H };
    let tile_size = TilemapTileSize { x: TILE_W, y: TILE_H };
    let map_type = TilemapType::Hexagon(HexCoordSystem::Row);

    // Build storage
    let mut tile_storage = TileStorage::empty(map_size);
    // Default fill kind
    let mut cells: HashMap<(i32,i32), String> = HashMap::new();
    for q in -10..=10 { for r in -10..=10 { cells.insert((q,r), "hex".into()); } }
    for region in &wb.regions {
        for poi in &region.hex_points {
            let (q,r) = parse_axial(&poi.axial);
            cells.insert((q,r), poi.kind.clone());
        }
    }

    // Spawn map components
    commands.entity(tilemap_entity).insert((
        Name::new("Overworld"),
        TilemapTexture::Single(texture),
        TilemapSize { x: map_size.x, y: map_size.y },
        tile_size,
        grid_size,
        map_type,
        TilemapBundle::default(),
    ));

    // Positioning: center-ish
    commands.entity(tilemap_entity).insert(Transform::from_xyz(0.0, 0.0, 0.0));
    commands.entity(tilemap_entity).insert(GlobalTransform::default());

    // Place some tiles at positions (we'll offset by +32 to keep indices positive)
    let offset = 32i32;
    for ((q,r), kind) in cells.iter() {
        let x = (q + offset) as u32;
        let y = (r + offset) as u32;
        if x < map_size.x && y < map_size.y {
            let pos = TilePos { x, y };
            let tile_entity = commands.spawn(TileBundle {
                position: pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(match kind.as_str() {
                    "village" => 1,
                    "shrine" => 2,
                    "lair" => 3,
                    "ruin" => 4,
                    _ => 0,
                }),
                ..Default::default()
            }).id();
            tile_storage.set(&pos, tile_entity);
            idx.axial_to_entity.insert((*q,*r), tile_entity);
        }
    }
    commands.entity(tilemap_entity).insert(tile_storage);

    // Spawn an example NPC at the village
    let npc_tex = asset_server.load("sprites/npc_pawn.png");
    commands.spawn((SpriteBundle { texture: npc_tex, transform: Transform::from_xyz(0.0, 0.0, 10.0), ..Default::default() }, 
                    Npc{ id:"n_vicar".into(), name:"Under-Vicar Marn".into() }, AxialPos{ q:0, r:0 }));
}
