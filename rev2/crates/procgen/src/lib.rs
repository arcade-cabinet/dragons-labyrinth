use serde::{Serialize, Deserialize};
use mapgen::{MapBuilder, MapFilter};
use mapgen::filter::{NoiseGenerator, CellularAutomata, starting_point::{AreaStartingPosition, XStart, YStart}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexDungeon {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<(usize, usize, u8)>, // x,y,val: 1=floor, 0=wall
}

pub fn generate_hex_dungeon(w: usize, h: usize) -> HexDungeon {
    let map = MapBuilder::new(w, h)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .build();

    let mut tiles = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let floor = if map.map[idx] == 0 { 0 } else { 1 };
            tiles.push((x, y, floor));
        }
    }
    HexDungeon { width: w, height: h, tiles }
}
