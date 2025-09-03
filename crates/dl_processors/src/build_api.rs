//! Build API for dl_processors crate
//! 
//! Provides comprehensive interface for apps/game to obtain processed game resources

use anyhow::Result;
use std::path::Path;
use minijinja::Environment;

/// Complete processed game data bundle ready for ECS integration
#[derive(Debug, Clone)]
pub struct ProcessedGameData {
    pub world_resources: WorldResourceData,
    pub dialogue_system: DialogueSystemData,
    pub generation_stats: ProcessingStats,
}

/// World resource data for ECS components
#[derive(Debug, Clone)]
pub struct WorldResourceData {
    pub hex_tiles: Vec<HexTileData>,
    pub region_modules: Vec<RegionModuleData>,
    pub dungeon_areas: Vec<DungeonAreaData>,
    pub total_hexes: usize,
}

/// Dialogue system data for NPC interactions
#[derive(Debug, Clone)]
pub struct DialogueSystemData {
    pub dialogue_modules: Vec<DialogueModuleData>,
    pub npc_dialogues: Vec<NpcDialogueData>,
    pub quest_definitions: Vec<QuestDefinitionData>,
    pub total_npcs: usize,
}

/// Individual hex tile data
#[derive(Debug, Clone)]
pub struct HexTileData {
    pub q: i32,
    pub r: i32,
    pub biome_type: String,
    pub corruption_level: f32,
    pub features: Vec<String>,
}

/// Region module data for world organization
#[derive(Debug, Clone)]
pub struct RegionModuleData {
    pub region_id: String,
    pub center_q: i32,
    pub center_r: i32,
    pub radius: u32,
    pub biome: String,
    pub settlements: Vec<String>,
    pub dungeons: Vec<String>,
}

/// Dungeon area data for exploration
#[derive(Debug, Clone)]
pub struct DungeonAreaData {
    pub dungeon_id: String,
    pub location_q: i32,
    pub location_r: i32,
    pub difficulty_rating: u8,
    pub loot_tables: Vec<String>,
    pub encounter_types: Vec<String>,
}

/// Dialogue module data for conversation trees
#[derive(Debug, Clone)]
pub struct DialogueModuleData {
    pub module_id: String,
    pub region_id: String,
    pub act: u8,
    pub dialogue_nodes: Vec<String>,
    pub condition_checks: Vec<String>,
}

/// Individual NPC dialogue data
#[derive(Debug, Clone)]
pub struct NpcDialogueData {
    pub npc_id: String,
    pub archetype: String,
    pub personality_traits: Vec<String>,
    pub dialogue_trees: Vec<String>,
    pub quest_hooks: Vec<String>,
}

/// Quest definition data for game systems
#[derive(Debug, Clone)]
pub struct QuestDefinitionData {
    pub quest_id: String,
    pub pattern_type: String,
    pub beats: Vec<String>,
    pub requirements: Vec<String>,
    pub rewards: Vec<String>,
}

/// Processing statistics
#[derive(Debug, Clone)]
pub struct ProcessingStats {
    pub hbf_entities_processed: usize,
    pub seeds_books_analyzed: usize,
    pub hex_tiles_generated: usize,
    pub dialogue_modules_created: usize,
    pub quests_generated: usize,
}

impl ProcessedGameData {
    /// Perform complete processing using analysis data from dl_analysis
    pub fn perform_complete_processing(
        hbf_path: &Path,
        seeds_cache_dir: &Path,
        output_dir: &Path,
    ) -> Result<Self> {
        // Get analyzed data from dl_analysis
        let analysis_data = dl_analysis::build_api::provide_analysis_data_for_processing(
            hbf_path,
            seeds_cache_dir,
            output_dir,
        )?;
        
        // Process world resources
        let world_resources = process_world_resources(&analysis_data, output_dir)?;
        
        // Process dialogue system
        let dialogue_system = process_dialogue_system(&analysis_data, output_dir)?;
        
        let generation_stats = ProcessingStats {
            hbf_entities_processed: analysis_data.combined_stats.hbf_entities_processed,
            seeds_books_analyzed: analysis_data.combined_stats.books_analyzed,
            hex_tiles_generated: world_resources.total_hexes,
            dialogue_modules_created: dialogue_system.dialogue_modules.len(),
            quests_generated: dialogue_system.quest_definitions.len(),
        };

        Ok(Self {
            world_resources,
            dialogue_system,
            generation_stats,
        })
    }

    /// Get hex tile data for specific coordinates
    pub fn get_hex_tile_at(&self, q: i32, r: i32) -> Option<&HexTileData> {
        self.world_resources.hex_tiles
            .iter()
            .find(|tile| tile.q == q && tile.r == r)
    }

    /// Get region module containing specific coordinates
    pub fn get_region_at(&self, q: i32, r: i32) -> Option<&RegionModuleData> {
        self.world_resources.region_modules
            .iter()
            .find(|region| {
                let dq = (region.center_q - q).abs() as u32;
                let dr = (region.center_r - r).abs() as u32;
                dq + dr <= region.radius
            })
    }

    /// Get dialogue for specific NPC
    pub fn get_npc_dialogue(&self, npc_id: &str) -> Option<&NpcDialogueData> {
        self.dialogue_system.npc_dialogues
            .iter()
            .find(|npc| npc.npc_id == npc_id)
    }

    /// Get quests available in specific region
    pub fn get_quests_for_region(&self, region_id: &str) -> Vec<&QuestDefinitionData> {
        // For now, return all quests - would filter by region in full implementation
        self.dialogue_system.quest_definitions.iter().collect()
    }
}

/// Process world resources from analysis data
fn process_world_resources(
    analysis_data: &dl_analysis::build_api::AnalysisBuildData,
    output_dir: &Path,
) -> Result<WorldResourceData> {
    let mut hex_tiles = Vec::new();
    let mut region_modules = Vec::new();
    let mut dungeon_areas = Vec::new();

    // Generate hex tiles based on HBF analysis
    // For now, create sample hex tiles - would use actual analysis data in full implementation
    for q in -10i32..=10i32 {
        for r in -10i32..=10i32 {
            if (q + r).abs() <= 10 {  // Valid hex coordinates
                let distance = ((q * q + r * r + q * r) as f32).sqrt();
                let corruption_level = (distance / 10.0).min(1.0);
                let biome_type = determine_biome_for_hex(q, r, corruption_level);
                
                hex_tiles.push(HexTileData {
                    q,
                    r,
                    biome_type: biome_type.to_string(),
                    corruption_level,
                    features: generate_hex_features(biome_type, corruption_level),
                });
            }
        }
    }

    // Generate region modules
    let region_centers = [(-5, -5), (0, 0), (5, 5), (-5, 5), (5, -5)];
    for (i, (cq, cr)) in region_centers.iter().enumerate() {
        region_modules.push(RegionModuleData {
            region_id: format!("region_{}", i + 1),
            center_q: *cq,
            center_r: *cr,
            radius: 7,
            biome: determine_biome_for_hex(*cq, *cr, 0.0).to_string(),
            settlements: vec![format!("settlement_{}_1", i + 1)],
            dungeons: vec![format!("dungeon_{}_1", i + 1)],
        });
    }

    // Generate dungeon areas
    for (i, (cq, cr)) in region_centers.iter().enumerate() {
        dungeon_areas.push(DungeonAreaData {
            dungeon_id: format!("dungeon_{}_1", i + 1),
            location_q: cq + 2,
            location_r: cr + 2,
            difficulty_rating: ((i + 1) * 2).min(10) as u8,
            loot_tables: vec!["basic_loot".to_string(), "corruption_items".to_string()],
            encounter_types: vec!["undead".to_string(), "corrupted_animals".to_string()],
        });
    }

    // Generate Rust code using templates
    generate_hex_tile_modules(&hex_tiles, output_dir)?;
    generate_region_modules(&region_modules, output_dir)?;
    generate_dungeon_modules(&dungeon_areas, output_dir)?;

    Ok(WorldResourceData {
        total_hexes: hex_tiles.len(),
        hex_tiles,
        region_modules,
        dungeon_areas,
    })
}

/// Process dialogue system from analysis data
fn process_dialogue_system(
    analysis_data: &dl_analysis::build_api::AnalysisBuildData,
    output_dir: &Path,
) -> Result<DialogueSystemData> {
    let mut dialogue_modules = Vec::new();
    let mut npc_dialogues = Vec::new();
    let mut quest_definitions = Vec::new();

    // Generate dialogue modules for each act
    for act in 1..=5u8 {
        if let Some(act_dialogue) = analysis_data.get_dialogue_for_act(act) {
            dialogue_modules.push(DialogueModuleData {
                module_id: format!("dialogue_act_{}", act),
                region_id: format!("region_{}", act),  // Map acts to regions
                act,
                dialogue_nodes: act_dialogue.patterns.clone(),
                condition_checks: vec![
                    format!("player.corruption >= {}", (act - 1) * 20),
                    format!("player.distance >= {}", (act - 1) * 50),
                ],
            });
        }
    }

    // Generate NPC dialogues using character archetypes
    let npc_archetypes = ["wandering_scholar", "holy_warrior", "mercenary", "corrupted_noble", "dark_cultist"];
    for (i, archetype) in npc_archetypes.iter().enumerate() {
        let act = ((i / 2) + 1).min(5) as u8;  // Spread NPCs across acts
        
        if let Some(act_dialogue) = analysis_data.get_dialogue_for_act(act) {
            npc_dialogues.push(NpcDialogueData {
                npc_id: format!("npc_{}_{}", archetype, i + 1),
                archetype: archetype.to_string(),
                personality_traits: vec![
                    format!("{}_personality", archetype),
                    format!("act_{}_corruption", act),
                ],
                dialogue_trees: act_dialogue.patterns.iter().take(3).cloned().collect(),
                quest_hooks: vec![format!("quest_hook_{}", i + 1)],
            });
        }
    }

    // Generate quest definitions using quest patterns
    let quest_patterns = ["investigation", "purification", "escort", "exploration", "confrontation"];
    for (i, pattern) in quest_patterns.iter().enumerate() {
        if let Some(quest_data) = analysis_data.get_quest_patterns_for_type(pattern) {
            quest_definitions.push(QuestDefinitionData {
                quest_id: format!("quest_{}_{}", pattern, i + 1),
                pattern_type: pattern.to_string(),
                beats: quest_data.beats.clone(),
                requirements: vec![
                    format!("level >= {}", i + 1),
                    format!("corruption <= {}", (i + 1) * 30),
                ],
                rewards: vec![
                    "experience".to_string(),
                    format!("{}_item", pattern),
                ],
            });
        }
    }

    // Generate dialogue Rust code using templates
    generate_dialogue_modules(&dialogue_modules, output_dir)?;
    generate_npc_dialogues(&npc_dialogues, output_dir)?;

    Ok(DialogueSystemData {
        total_npcs: npc_dialogues.len(),
        dialogue_modules,
        npc_dialogues,
        quest_definitions,
    })
}

// Helper functions for world generation

fn determine_biome_for_hex(q: i32, r: i32, corruption_level: f32) -> &'static str {
    let distance = ((q * q + r * r + q * r) as f32).sqrt();
    
    match corruption_level {
        x if x < 0.2 => "meadows",
        x if x < 0.4 => "forests", 
        x if x < 0.6 => "swamps",
        x if x < 0.8 => "mountains",
        _ => "corrupted_wasteland",
    }
}

fn generate_hex_features(biome: &str, corruption_level: f32) -> Vec<String> {
    let mut features = Vec::new();
    
    match biome {
        "meadows" => features.extend(["wildflowers".to_string(), "stream".to_string(), "peaceful_grove".to_string()]),
        "forests" => features.extend(["ancient_trees".to_string(), "woodland_path".to_string(), "hidden_shrine".to_string()]),
        "swamps" => features.extend(["murky_water".to_string(), "twisted_roots".to_string(), "will_o_wisps".to_string()]),
        "mountains" => features.extend(["rocky_outcrop".to_string(), "cave_entrance".to_string(), "mountain_pass".to_string()]),
        "corrupted_wasteland" => features.extend(["withered_trees".to_string(), "cursed_ground".to_string(), "dark_obelisk".to_string()]),
        _ => features.push("mysterious_landmark".to_string()),
    }
    
    if corruption_level > 0.5 {
        features.push("corruption_source".to_string());
    }
    
    features
}

// Template generation functions

fn generate_hex_tile_modules(hex_tiles: &[HexTileData], output_dir: &Path) -> Result<()> {
    // Generate hex_tiles.rs module
    let mut content = String::from("//! Generated hex tile definitions\n\n");
    content.push_str("use bevy::prelude::*;\n");
    content.push_str("use crate::world::components::*;\n\n");
    
    content.push_str("pub fn initialize_hex_tiles(mut commands: Commands) {\n");
    for tile in hex_tiles {
        content.push_str(&format!(
            "    commands.spawn(HexTile {{ q: {}, r: {}, biome: {:?}, corruption: {:.2} }});\n",
            tile.q, tile.r, tile.biome_type, tile.corruption_level
        ));
    }
    content.push_str("}\n");
    
    std::fs::write(output_dir.join("hex_tiles.rs"), content)?;
    Ok(())
}

fn generate_region_modules(regions: &[RegionModuleData], output_dir: &Path) -> Result<()> {
    // Generate regions.rs module
    let mut content = String::from("//! Generated region definitions\n\n");
    content.push_str("use bevy::prelude::*;\n");
    content.push_str("use crate::world::components::*;\n\n");
    
    content.push_str("pub fn initialize_regions(mut commands: Commands) {\n");
    for region in regions {
        content.push_str(&format!(
            "    commands.spawn(Region {{ id: {:?}, center_q: {}, center_r: {}, radius: {} }});\n",
            region.region_id, region.center_q, region.center_r, region.radius
        ));
    }
    content.push_str("}\n");
    
    std::fs::write(output_dir.join("regions.rs"), content)?;
    Ok(())
}

fn generate_dungeon_modules(dungeons: &[DungeonAreaData], output_dir: &Path) -> Result<()> {
    // Generate dungeons.rs module
    let mut content = String::from("//! Generated dungeon definitions\n\n");
    content.push_str("use bevy::prelude::*;\n");
    content.push_str("use crate::world::components::*;\n\n");
    
    content.push_str("pub fn initialize_dungeons(mut commands: Commands) {\n");
    for dungeon in dungeons {
        content.push_str(&format!(
            "    commands.spawn(Dungeon {{ id: {:?}, q: {}, r: {}, difficulty: {} }});\n",
            dungeon.dungeon_id, dungeon.location_q, dungeon.location_r, dungeon.difficulty_rating
        ));
    }
    content.push_str("}\n");
    
    std::fs::write(output_dir.join("dungeons.rs"), content)?;
    Ok(())
}

fn generate_dialogue_modules(modules: &[DialogueModuleData], output_dir: &Path) -> Result<()> {
    // Generate dialogue.rs module
    let mut content = String::from("//! Generated dialogue system\n\n");
    content.push_str("use bevy::prelude::*;\n");
    content.push_str("use std::collections::HashMap;\n\n");
    
    content.push_str("pub fn initialize_dialogue_system(mut commands: Commands) {\n");
    content.push_str("    let mut dialogue_map = HashMap::new();\n");
    
    for module in modules {
        content.push_str(&format!(
            "    dialogue_map.insert({:?}, vec!{:?});\n",
            module.module_id, module.dialogue_nodes
        ));
    }
    
    content.push_str("    commands.insert_resource(DialogueMap(dialogue_map));\n");
    content.push_str("}\n\n");
    content.push_str("#[derive(Resource)]\npub struct DialogueMap(pub HashMap<String, Vec<String>>);\n");
    
    std::fs::write(output_dir.join("dialogue.rs"), content)?;
    Ok(())
}

fn generate_npc_dialogues(npcs: &[NpcDialogueData], output_dir: &Path) -> Result<()> {
    // Generate npcs.rs module
    let mut content = String::from("//! Generated NPC definitions\n\n");
    content.push_str("use bevy::prelude::*;\n");
    content.push_str("use crate::world::components::*;\n\n");
    
    content.push_str("pub fn spawn_npcs(mut commands: Commands) {\n");
    for (i, npc) in npcs.iter().enumerate() {
        content.push_str(&format!(
            "    commands.spawn(NPC {{ id: {:?}, archetype: {:?}, dialogue_trees: vec!{:?} }});\n",
            npc.npc_id, npc.archetype, npc.dialogue_trees
        ));
    }
    content.push_str("}\n");
    
    std::fs::write(output_dir.join("npcs.rs"), content)?;
    Ok(())
}

/// Public API function for apps/game to call during build
pub fn provide_game_resources_for_integration(
    hbf_path: &Path,
    seeds_cache_dir: &Path,
    output_dir: &Path,
) -> Result<ProcessedGameData> {
    ProcessedGameData::perform_complete_processing(hbf_path, seeds_cache_dir, output_dir)
}
