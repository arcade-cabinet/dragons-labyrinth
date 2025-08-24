// Auto-generated world loader for test_world_dread_1
use bevy::prelude::*;
use hexx::{Hex, HexLayout};

pub fn load_test_world_dread_1_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let layout = HexLayout::flat(1.0);
    
    // Spawn grass tile at (0, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, 0)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (1, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(1, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(1, -1)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (1, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(1, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(1, 0)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (0, 1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, 1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, 1)),
        TileType::Grass,
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
    
    // Spawn grass tile at (-1, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(-1, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(-1, -1)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (0, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(0, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(0, -1)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (2, -1)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(2, -1)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(2, -1)),
        TileType::Grass,
    ));
    
    // Spawn grass tile at (2, 0)
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("tiles/grass_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new(2, 0)).extend(0.0)
            ),
            ..default()
        },
        HexPosition(Hex::new(2, 0)),
        TileType::Grass,
    ));
    
}
