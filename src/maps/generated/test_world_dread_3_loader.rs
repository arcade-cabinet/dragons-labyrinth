// Auto-generated world loader for test_world_dread_3
use bevy::prelude::*;
use hexx::{Hex, HexLayout};

pub fn load_test_world_dread_3_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let layout = HexLayout::flat(1.0);
    
    // Spawn corrupted tile at (0, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, 0)),
        TileType::Corrupted,
    ));
    
    // Spawn corrupted tile at (1, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(1, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(1, 0)),
        TileType::Corrupted,
    ));
    
    // Spawn corrupted tile at (0, 1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, 1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, 1)),
        TileType::Corrupted,
    ));
    
    // Spawn corrupted tile at (-1, 1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(-1, 1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(-1, 1)),
        TileType::Corrupted,
    ));
    
    // Spawn corrupted tile at (-1, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(-1, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(-1, 0)),
        TileType::Corrupted,
    ));
    
    // Spawn corrupted tile at (1, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(1, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(1, -1)),
        TileType::Corrupted,
    ));
    
    // Spawn void tile at (2, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/void_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(2, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(2, -1)),
        TileType::Void,
    ));
    
    // Spawn corrupted tile at (-2, 1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(-2, 1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(-2, 1)),
        TileType::Corrupted,
    ));
    
    // Spawn stone tile at (0, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/stone_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, -1)),
        TileType::Stone,
    ));
    
    // Spawn corrupted tile at (1, 1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/corrupted_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(1, 1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(1, 1)),
        TileType::Corrupted,
    ));
    
}
