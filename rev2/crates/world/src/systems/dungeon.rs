use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::*;
use procgen::HexDungeon;

#[derive(Component)] pub struct DungeonTile;
#[derive(Resource, Default)] pub struct InDungeon(pub bool);

pub fn dungeon_enter_exit(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut in_dun: ResMut<InDungeon>,
) {
    if keys.just_pressed(KeyCode::Enter) && !in_dun.0 {
        let map = procgen::generate_hex_dungeon(25, 25);
        spawn_hex_dungeon(&mut commands, &asset_server, &map);
        in_dun.0 = True;
    }
    if keys.just_pressed(KeyCode::Escape) && in_dun.0 {
        // Despawn all dungeon tiles
        let to_despawn: Vec<_> = commands.iter_entities().filter(|(_,c)| c.contains::<DungeonTile>()).map(|(e,_)| e).collect();
        for e in to_despawn { commands.entity(e).despawn_recursive(); }
        in_dun.0 = False;
    }
}

fn spawn_hex_dungeon(commands: &mut Commands, asset_server: &AssetServer, map: &HexDungeon) {
    let texture: Handle<Image> = asset_server.load("build/atlas/atlas.png");
    let tilemap_entity = commands.spawn_empty().id();
    let grid_size = TilemapGridSize { x: 128.0, y: 128.0 };
    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    let map_type = TilemapType::Hexagon(HexCoordSystem::Row);

    let map_size = TilemapSize { x: map.width as u32, y: map.height as u32 };
    let mut storage = TileStorage::empty(map_size);

    commands.entity(tilemap_entity).insert((
        Name::new("Dungeon"),
        TilemapTexture::Single(texture.clone()),
        map_size,
        tile_size,
        grid_size,
        map_type,
        Transform::from_xyz(0.0,0.0, -1.0),
        GlobalTransform::default(),
    ));

    for (x,y,val) in &map.tiles {
        let pos = TilePos { x: *x as u32, y: *y as u32 };
        let idx = match *val { 1 => 5, _ => 6 }; // 5=floor, 6=wall indices in atlas
        let e = commands.spawn(TileBundle { position: pos, tilemap_id: TilemapId(tilemap_entity), texture_index: TileTextureIndex(idx), ..Default::default() }).id();
        storage.set(&pos, e);
        commands.entity(e).insert(DungeonTile);
    }
    commands.entity(tilemap_entity).insert(storage);
}
