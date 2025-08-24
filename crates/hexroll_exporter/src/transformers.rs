//! Data transformation from D&D-style foundation to Dragon's Labyrinth horror RPG

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::hbf_import::HbfEntity;

/// Configuration for Dragon's Labyrinth data generation
#[derive(Debug, Clone)]
pub struct DataGenerationConfig {
    pub enable_horror_transformations: bool,
    pub corruption_seed: u64,
    pub output_directory: PathBuf,
    pub generate_orm_models: bool,
    pub generate_data_files: bool,
}

/// Base fantasy data extracted from HBF (D&D-style foundation)
#[derive(Debug, Clone)]
pub struct BaseFantasyData {
    pub settlements: Vec<BaseSettlement>,
    pub hex_tiles: Vec<BaseHexTile>,
    pub npcs: Vec<BaseNpc>,
    pub dungeons: Vec<BaseDungeon>,
    pub items: Vec<BaseItem>,
    pub factions: Vec<BaseFaction>,
}

impl BaseFantasyData {
    pub fn from_hbf_entities(_entities: Vec<HbfEntity>) -> Self {
        // TODO: Implement conversion from parsed HBF entities
        Self {
            settlements: Vec::new(),
            hex_tiles: Vec::new(),
            npcs: Vec::new(),
            dungeons: Vec::new(),
            items: Vec::new(),
            factions: Vec::new(),
        }
    }
    
    pub fn total_entities(&self) -> usize {
        self.settlements.len() + 
        self.hex_tiles.len() + 
        self.npcs.len() + 
        self.dungeons.len() + 
        self.items.len() + 
        self.factions.len()
    }
}

/// Dragon's Labyrinth data after horror transformation
#[derive(Debug, Clone)]
pub struct DragonLabyrinthData {
    pub settlements: Vec<DragonSettlement>,
    pub hex_tiles: Vec<DragonHexTile>,
    pub npcs: Vec<DragonNpc>,
    pub dungeons: Vec<DragonDungeon>,
    pub items: Vec<DragonItem>,
    pub factions: Vec<DragonFaction>,
}

impl DragonLabyrinthData {
    pub fn total_entities(&self) -> usize {
        self.settlements.len() + 
        self.hex_tiles.len() + 
        self.npcs.len() + 
        self.dungeons.len() + 
        self.items.len() + 
        self.factions.len()
    }
}

/// Horror transformation engine
pub struct HorrorTransformer {
    corruption_seed: u64,
}

impl HorrorTransformer {
    pub fn new(corruption_seed: u64) -> Self {
        Self { corruption_seed }
    }
    
    pub async fn transform_settlements(&self, base: Vec<BaseSettlement>) -> Result<Vec<DragonSettlement>> {
        // Transform D&D settlements into Dragon's Labyrinth horror locations
        Ok(base.into_iter().map(|s| self.apply_settlement_horror(s)).collect())
    }
    
    pub async fn transform_hex_tiles(&self, base: Vec<BaseHexTile>) -> Result<Vec<DragonHexTile>> {
        // Transform D&D terrain into corrupted landscapes
        Ok(base.into_iter().map(|t| self.apply_terrain_corruption(t)).collect())
    }
    
    pub async fn transform_npcs(&self, base: Vec<BaseNpc>) -> Result<Vec<DragonNpc>> {
        // Transform D&D NPCs into potential companions with trauma
        Ok(base.into_iter().map(|n| self.apply_psychological_depth(n)).collect())
    }
    
    pub async fn transform_dungeons(&self, base: Vec<BaseDungeon>) -> Result<Vec<DragonDungeon>> {
        // Transform D&D dungeons into labyrinth horror chambers
        Ok(base.into_iter().map(|d| self.apply_labyrinth_horror(d)).collect())
    }
    
    pub async fn transform_items(&self, base: Vec<BaseItem>) -> Result<Vec<DragonItem>> {
        // Transform D&D items into forge system potential
        Ok(base.into_iter().map(|i| self.apply_forge_potential(i)).collect())
    }
    
    pub async fn transform_factions(&self, base: Vec<BaseFaction>) -> Result<Vec<DragonFaction>> {
        // Transform D&D factions into corruption-aware organizations
        Ok(base.into_iter().map(|f| self.apply_corruption_awareness(f)).collect())
    }
    
    // Private transformation methods
    fn apply_settlement_horror(&self, base: BaseSettlement) -> DragonSettlement {
        DragonSettlement {
            id: uuid::Uuid::new_v4(),
            name: base.name,
            settlement_type: base.settlement_type,
            world_x: base.x as f32,
            world_z: base.y as f32,
            description: base.description,
            population: base.population,
            prosperity_level: base.prosperity,
            corruption_influence: self.calculate_corruption_influence(base.x, base.y),
            dread_level_effects: self.calculate_dread_effects(&base.settlement_type),
            corruption_description: Some(self.generate_corruption_description(&base.name)),
            // Additional Dragon's Labyrinth fields...
            safety_rating: 5, // Default values for now
            reputation: 0,
            services: serde_json::json!([]),
            trade_goods: serde_json::json!({}),
            companion_reactions: serde_json::json!({}),
        }
    }
    
    fn apply_terrain_corruption(&self, base: BaseHexTile) -> DragonHexTile {
        DragonHexTile {
            id: uuid::Uuid::new_v4(),
            q: base.x,
            r: base.y,
            s: -base.x - base.y,
            world_x: base.x as f32,
            world_z: base.y as f32,
            biome_type: base.biome,
            corruption_level: self.calculate_corruption_influence(base.x, base.y),
            dread_intensity: self.calculate_dread_intensity(&base.features),
            horror_features: serde_json::json!(base.features),
            description: base.description,
            corrupted_description: Some(self.generate_terrain_corruption(&base.biome)),
        }
    }
    
    fn apply_psychological_depth(&self, base: BaseNpc) -> DragonNpc {
        DragonNpc {
            id: uuid::Uuid::new_v4(),
            name: base.name,
            role: base.role,
            world_x: base.x as f32,
            world_z: base.y as f32,
            personality: base.personality,
            background: base.background,
            trauma_potential: self.calculate_trauma_potential(&base.background),
            corruption_resistance: self.calculate_corruption_resistance(&base.personality),
            dread_threshold: self.calculate_dread_threshold(&base.role),
            companion_potential: self.assess_companion_potential(&base.role, &base.personality),
        }
    }
    
    fn apply_labyrinth_horror(&self, base: BaseDungeon) -> DragonDungeon {
        DragonDungeon {
            id: uuid::Uuid::new_v4(),
            name: base.name,
            world_x: base.x as f32,
            world_z: base.y as f32,
            total_rooms: base.total_rooms,
            description: base.description,
            difficulty: base.difficulty,
            corruption_level: self.calculate_dungeon_corruption(base.difficulty),
            dread_amplification: self.calculate_dread_amplification(base.total_rooms),
            horror_theme: self.assign_horror_theme(&base.name),
            traumatic_potential: self.calculate_traumatic_potential(base.total_rooms, base.difficulty),
        }
    }
    
    fn apply_forge_potential(&self, base: BaseItem) -> DragonItem {
        DragonItem {
            id: uuid::Uuid::new_v4(),
            name: base.name,
            item_type: base.item_type,
            description: base.description,
            rarity: base.rarity,
            sentimental_value: self.calculate_sentimental_value(&base.name),
            trauma_association: self.assign_trauma_association(&base.item_type),
            corruption_resistance: self.calculate_item_corruption_resistance(&base.rarity),
            forge_potential: self.assess_forge_potential(&base.item_type, &base.rarity),
        }
    }
    
    fn apply_corruption_awareness(&self, base: BaseFaction) -> DragonFaction {
        DragonFaction {
            id: uuid::Uuid::new_v4(),
            name: base.name,
            faction_type: base.faction_type,
            influence: base.influence,
            goals: base.goals,
            corruption_tolerance: self.calculate_corruption_tolerance(&base.faction_type),
            dread_response: self.assign_dread_response(&base.faction_type),
            companion_relations: serde_json::json!({}),
        }
    }
    
    // Helper calculation methods for Dragon's Labyrinth transformations
    fn calculate_corruption_influence(&self, x: i32, y: i32) -> f32 {
        // Use position and seed to create consistent corruption patterns
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        (x, y, self.corruption_seed).hash(&mut hasher);
        let hash = hasher.finish();
        
        (hash % 100) as f32 / 100.0  // 0.0 to 1.0
    }
    
    fn calculate_dread_effects(&self, settlement_type: &str) -> i32 {
        match settlement_type {
            "tavern" | "inn" => 1,  // Social places amplify dread slightly
            "temple" => 0,          // Sacred places resist dread
            "village" => 2,         // Larger communities more affected
            "outpost" => 3,         // Isolated places most vulnerable
            _ => 1,
        }
    }
    
    fn generate_corruption_description(&self, name: &str) -> String {
        format!("When corruption spreads, {} becomes a twisted shadow of its former self...", name)
    }
    
    fn calculate_dread_intensity(&self, features: &[String]) -> i32 {
        features.len().min(4) as i32  // More features = more dread potential
    }
    
    fn generate_terrain_corruption(&self, biome: &str) -> String {
        match biome {
            "forest" => "The trees writhe with unnatural movement, their branches reaching like grasping fingers...".to_string(),
            "grassland" => "The grass grows in disturbing patterns, forming symbols that hurt to look at...".to_string(),
            "mountain" => "The rocks pulse with a sickening rhythm, as if the mountain itself has a heartbeat...".to_string(),
            _ => format!("The {} transforms into something that defies natural law...", biome),
        }
    }
    
    fn calculate_trauma_potential(&self, background: &str) -> f32 {
        match background {
            "soldier" | "guard" => 0.8,  // High trauma potential
            "merchant" | "trader" => 0.4, // Medium trauma potential
            "scholar" | "priest" => 0.3,  // Lower trauma potential
            _ => 0.5,  // Default
        }
    }
    
    fn calculate_corruption_resistance(&self, personality: &str) -> f32 {
        if personality.contains("brave") || personality.contains("strong") {
            0.8
        } else if personality.contains("fearful") || personality.contains("weak") {
            0.2
        } else {
            0.5
        }
    }
    
    fn calculate_dread_threshold(&self, role: &str) -> i32 {
        match role {
            "warrior" | "paladin" => 3,  // Can handle high dread
            "civilian" | "child" => 1,   // Low dread tolerance
            "priest" | "healer" => 2,    // Medium dread tolerance
            _ => 2,
        }
    }
    
    fn assess_companion_potential(&self, role: &str, personality: &str) -> bool {
        // Most NPCs can potentially become companions in Dragon's Labyrinth
        !role.contains("enemy") && !personality.contains("hostile")
    }
    
    fn calculate_dungeon_corruption(&self, difficulty: i32) -> f32 {
        (difficulty as f32 / 10.0).clamp(0.0, 1.0)
    }
    
    fn calculate_dread_amplification(&self, room_count: i32) -> f32 {
        (room_count as f32 / 50.0).clamp(0.1, 2.0)  // More rooms = more amplification
    }
    
    fn assign_horror_theme(&self, name: &str) -> String {
        if name.contains("crypt") || name.contains("tomb") {
            "undead_haunting".to_string()
        } else if name.contains("mine") || name.contains("cave") {
            "claustrophobic_darkness".to_string()
        } else if name.contains("tower") || name.contains("castle") {
            "abandonment_decay".to_string()
        } else {
            "creeping_dread".to_string()
        }
    }
    
    fn calculate_traumatic_potential(&self, rooms: i32, difficulty: i32) -> f32 {
        ((rooms + difficulty) as f32 / 20.0).clamp(0.0, 1.0)
    }
    
    fn calculate_sentimental_value(&self, name: &str) -> f32 {
        if name.contains("ring") || name.contains("locket") || name.contains("letter") {
            0.9  // High sentimental value
        } else if name.contains("weapon") || name.contains("armor") {
            0.6  // Medium sentimental value
        } else {
            0.3  // Low sentimental value
        }
    }
    
    fn assign_trauma_association(&self, item_type: &str) -> Option<String> {
        match item_type {
            "weapon" => Some("combat_trauma".to_string()),
            "jewelry" => Some("loss_grief".to_string()),
            "book" => Some("knowledge_burden".to_string()),
            _ => None,
        }
    }
    
    fn calculate_item_corruption_resistance(&self, rarity: &str) -> f32 {
        match rarity {
            "legendary" => 0.9,
            "rare" => 0.7,
            "uncommon" => 0.5,
            "common" => 0.3,
            _ => 0.4,
        }
    }
    
    fn assess_forge_potential(&self, item_type: &str, rarity: &str) -> bool {
        matches!(item_type, "weapon" | "armor" | "jewelry") && rarity != "common"
    }
    
    fn calculate_corruption_tolerance(&self, faction_type: &str) -> f32 {
        match faction_type {
            "cult" => 0.9,        // Cults embrace corruption
            "military" => 0.4,    // Military tries to resist
            "religious" => 0.2,   // Religious strongly resists
            _ => 0.5,
        }
    }
    
    fn assign_dread_response(&self, faction_type: &str) -> String {
        match faction_type {
            "cult" => "embrace_and_amplify".to_string(),
            "military" => "organize_resistance".to_string(),
            "religious" => "seek_divine_protection".to_string(),
            "merchant" => "flee_or_profit".to_string(),
            _ => "panic_and_scatter".to_string(),
        }
    }
}

// Base data structures (D&D-style foundation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseSettlement {
    pub name: String,
    pub settlement_type: String,
    pub x: i32,
    pub y: i32,
    pub description: String,
    pub population: Option<i32>,
    pub prosperity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseHexTile {
    pub x: i32,
    pub y: i32,
    pub biome: String,
    pub features: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseNpc {
    pub name: String,
    pub role: String,
    pub x: i32,
    pub y: i32,
    pub personality: String,
    pub background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseDungeon {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub total_rooms: i32,
    pub description: String,
    pub difficulty: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
    pub name: String,
    pub item_type: String,
    pub description: String,
    pub rarity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseFaction {
    pub name: String,
    pub faction_type: String,
    pub influence: i32,
    pub goals: Vec<String>,
}

// Dragon's Labyrinth data structures (horror-transformed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonSettlement {
    pub id: uuid::Uuid,
    pub name: String,
    pub settlement_type: String,
    pub world_x: f32,
    pub world_z: f32,
    pub description: String,
    pub population: Option<i32>,
    pub prosperity_level: i32,
    pub corruption_influence: f32,
    pub dread_level_effects: i32,
    pub corruption_description: Option<String>,
    pub safety_rating: i32,
    pub reputation: i32,
    pub services: serde_json::Value,
    pub trade_goods: serde_json::Value,
    pub companion_reactions: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonHexTile {
    pub id: uuid::Uuid,
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub world_x: f32,
    pub world_z: f32,
    pub biome_type: String,
    pub corruption_level: f32,
    pub dread_intensity: i32,
    pub horror_features: serde_json::Value,
    pub description: String,
    pub corrupted_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonNpc {
    pub id: uuid::Uuid,
    pub name: String,
    pub role: String,
    pub world_x: f32,
    pub world_z: f32,
    pub personality: String,
    pub background: String,
    pub trauma_potential: f32,
    pub corruption_resistance: f32,
    pub dread_threshold: i32,
    pub companion_potential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonDungeon {
    pub id: uuid::Uuid,
    pub name: String,
    pub world_x: f32,
    pub world_z: f32,
    pub total_rooms: i32,
    pub description: String,
    pub difficulty: i32,
    pub corruption_level: f32,
    pub dread_amplification: f32,
    pub horror_theme: String,
    pub traumatic_potential: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub item_type: String,
    pub description: String,
    pub rarity: String,
    pub sentimental_value: f32,
    pub trauma_association: Option<String>,
    pub corruption_resistance: f32,
    pub forge_potential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonFaction {
    pub id: uuid::Uuid,
    pub name: String,
    pub faction_type: String,
    pub influence: i32,
    pub goals: Vec<String>,
    pub corruption_tolerance: f32,
    pub dread_response: String,
    pub companion_relations: serde_json::Value,
}
