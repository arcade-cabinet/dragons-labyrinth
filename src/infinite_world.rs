//! The ENTIRE world system - One infinite hex map to rule them all

use bevy::prelude::*;
use hexx::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

/// The entire world - generates infinitely based on progression
#[derive(Resource)]
pub struct InfiniteWorld {
    /// Perlin noise for terrain generation
    terrain_noise: Perlin,
    /// Currently loaded chunks (3x3 around player)
    loaded_chunks: HashMap<IVec2, WorldChunk>,
    /// Player's current progression (1-180)
    progression: u32,
}

/// A chunk of the world (32x32 hexes)
pub struct WorldChunk {
    pub hexes: HashMap<Hex, HexTile>,
    pub generated_at_progression: u32,
}

/// A single hex tile
#[derive(Component, Clone)]
pub struct HexTile {
    pub terrain: TerrainType,
    pub corruption: f32,
    pub encounter_seed: u64,
    pub has_landmark: bool,
}

#[derive(Clone, Debug)]
pub enum TerrainType {
    // Early game (1-60)
    Grass,
    Forest,
    Hills,
    River,
    Village,
    
    // Mid game (60-120)
    CorruptedGrass,
    DeadForest,
    VoidCracks,
    AbandonedVillage,
    
    // Late game (120-180)
    VoidWaste,
    RealityFragments,
    NothingnessPools,
    FinalApproach,
}

impl InfiniteWorld {
    pub fn new() -> Self {
        Self {
            terrain_noise: Perlin::new(42),
            loaded_chunks: HashMap::new(),
            progression: 1,
        }
    }
    
    /// Get or generate a hex tile
    pub fn get_hex(&mut self, hex: Hex) -> HexTile {
        let chunk_coord = hex_to_chunk(hex);
        
        // Generate chunk if needed
        if !self.loaded_chunks.contains_key(&chunk_coord) {
            self.generate_chunk(chunk_coord);
        }
        
        self.loaded_chunks
            .get(&chunk_coord)
            .and_then(|chunk| chunk.hexes.get(&hex))
            .cloned()
            .unwrap_or_else(|| self.generate_hex(hex))
    }
    
    /// Generate a single hex based on progression and position
    fn generate_hex(&self, hex: Hex) -> HexTile {
        let world_pos = hex_to_world_pos(hex);
        
        // Sample noise for base terrain
        let noise_value = self.terrain_noise.get([
            world_pos.x as f64 * 0.05,
            world_pos.y as f64 * 0.05,
        ]);
        
        // Determine biome based on progression
        let biome = self.get_biome_at_progression();
        
        // Calculate corruption (spreads from dragon location)
        let dragon_hex = Hex::new(180, 180);  // Dragon at "end" of world
        let distance_from_dragon = hex.unsigned_distance_to(dragon_hex) as f32;
        let corruption = if self.progression > 60 {
            (1.0 - (distance_from_dragon / 200.0)).max(0.0).min(1.0)
        } else {
            0.0
        };
        
        // Determine terrain type
        let terrain = match biome {
            Biome::Forest => {
                if noise_value > 0.3 { TerrainType::Forest }
                else if noise_value > 0.0 { TerrainType::Hills }
                else { TerrainType::Grass }
            },
            Biome::Corrupted => {
                if corruption > 0.7 { TerrainType::VoidCracks }
                else if corruption > 0.4 { TerrainType::DeadForest }
                else { TerrainType::CorruptedGrass }
            },
            Biome::Void => {
                if noise_value > 0.2 { TerrainType::RealityFragments }
                else if noise_value > -0.2 { TerrainType::VoidWaste }
                else { TerrainType::NothingnessPools }
            },
        };
        
        // Check for landmarks (villages, dungeons)
        let has_landmark = self.should_have_landmark(hex);
        
        HexTile {
            terrain,
            corruption,
            encounter_seed: hash_hex(hex),
            has_landmark,
        }
    }
    
    fn generate_chunk(&mut self, chunk_coord: IVec2) {
        let mut hexes = HashMap::new();
        
        // Generate all hexes in chunk
        for x in 0..32 {
            for y in 0..32 {
                let local = IVec2::new(x, y);
                let hex = chunk_to_hex(chunk_coord, local);
                hexes.insert(hex, self.generate_hex(hex));
            }
        }
        
        self.loaded_chunks.insert(chunk_coord, WorldChunk {
            hexes,
            generated_at_progression: self.progression,
        });
    }
    
    fn get_biome_at_progression(&self) -> Biome {
        match self.progression {
            1..=60 => Biome::Forest,
            61..=120 => Biome::Corrupted,
            121..=180 => Biome::Void,
            _ => Biome::Forest,
        }
    }
    
    fn should_have_landmark(&self, hex: Hex) -> bool {
        // Villages at specific distances
        let distance = hex.unsigned_distance_to(Hex::ZERO);
        
        // Villages every ~15 hexes traveled
        if distance % 15 == 0 && distance > 0 {
            return true;
        }
        
        // Dungeon entrances at progression milestones
        if distance % 20 == 0 && distance > 0 {
            return true;
        }
        
        false
    }
    
    /// Update loaded chunks based on player position
    pub fn update_loaded_chunks(&mut self, player_hex: Hex) {
        let player_chunk = hex_to_chunk(player_hex);
        
        // Unload distant chunks
        self.loaded_chunks.retain(|coord, _| {
            let distance = (coord.x - player_chunk.x).abs() + 
                          (coord.y - player_chunk.y).abs();
            distance <= 4  // Keep chunks within 4 distance
        });
        
        // Load nearby chunks
        for dx in -2..=2 {
            for dy in -2..=2 {
                let chunk_coord = IVec2::new(
                    player_chunk.x + dx,
                    player_chunk.y + dy,
                );
                
                if !self.loaded_chunks.contains_key(&chunk_coord) {
                    self.generate_chunk(chunk_coord);
                }
            }
        }
    }
}

enum Biome {
    Forest,
    Corrupted,
    Void,
}

/// Convert hex to chunk coordinate
fn hex_to_chunk(hex: Hex) -> IVec2 {
    IVec2::new(hex.x / 32, hex.y / 32)
}

/// Convert chunk + local to hex
fn chunk_to_hex(chunk: IVec2, local: IVec2) -> Hex {
    Hex::new(chunk.x * 32 + local.x, chunk.y * 32 + local.y)
}

/// Convert hex to world position
fn hex_to_world_pos(hex: Hex) -> Vec2 {
    let layout = HexLayout {
        hex_size: Vec2::new(1.0, 1.0),
        orientation: HexOrientation::Flat,
        origin: Vec2::ZERO,
    };
    layout.hex_to_world_pos(hex)
}

/// Simple hash for deterministic generation
fn hash_hex(hex: Hex) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&(hex.x, hex.y), &mut hasher);
    std::hash::Hasher::finish(&hasher)
}

/// System to handle player movement on infinite map
pub fn infinite_movement_system(
    mut player_query: Query<&mut Hex, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut world: ResMut<InfiniteWorld>,
    mut progression: ResMut<Progression>,
) {
    if let Ok(mut player_hex) = player_query.get_single_mut() {
        // Get movement input
        let mut movement = Hex::ZERO;
        if input.pressed(KeyCode::KeyQ) { movement += Hex::new(-1, 0); }
        if input.pressed(KeyCode::KeyW) { movement += Hex::new(0, -1); }
        if input.pressed(KeyCode::KeyE) { movement += Hex::new(1, -1); }
        if input.pressed(KeyCode::KeyA) { movement += Hex::new(-1, 1); }
        if input.pressed(KeyCode::KeyS) { movement += Hex::new(0, 1); }
        if input.pressed(KeyCode::KeyD) { movement += Hex::new(1, 0); }
        
        if movement != Hex::ZERO {
            let new_hex = *player_hex + movement;
            
            // Get tile info
            let tile = world.get_hex(new_hex);
            
            // Check if we can move there
            if can_move_to(&tile) {
                *player_hex = new_hex;
                
                // Update progression based on distance
                let distance = new_hex.unsigned_distance_to(Hex::ZERO);
                progression.current = (distance as u32).min(180);
                world.progression = progression.current;
                
                // Update loaded chunks
                world.update_loaded_chunks(new_hex);
                
                // Check for triggers
                check_progression_triggers(progression.current, &tile);
            }
        }
    }
}

fn can_move_to(tile: &HexTile) -> bool {
    match tile.terrain {
        TerrainType::NothingnessPools => false,  // Can't walk on void
        _ => true,
    }
}

fn check_progression_triggers(progression: u32, tile: &HexTile) {
    // Check for landmark triggers
    if tile.has_landmark {
        match progression {
            p if p % 20 == 0 => {
                println!("DUNGEON ENTRANCE! Entering 3D labyrinth...");
                // Trigger labyrinth generation
            },
            p if p % 15 == 3 => {
                println!("VILLAGE! Rest and resupply...");
                // Spawn village UI
            },
            _ => {},
        }
    }
    
    // Random encounters based on tile
    let encounter_roll = rand::random::<f32>();
    let encounter_chance = 0.1 + (progression as f32 * 0.002) + tile.corruption * 0.1;
    
    if encounter_roll < encounter_chance {
        println!("ENCOUNTER! Combat begins...");
        // Spawn encounter based on progression
    }
}

#[derive(Resource)]
pub struct Progression {
    pub current: u32,
}

#[derive(Component)]
pub struct Player;

/// Plugin to run the infinite world
pub struct InfiniteWorldPlugin;

impl Plugin for InfiniteWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InfiniteWorld::new())
            .insert_resource(Progression { current: 1 })
            .add_systems(Update, infinite_movement_system);
    }
}
