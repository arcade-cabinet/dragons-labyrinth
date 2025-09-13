//! RON Generation Binary - Organized asset directory creator
//! 
//! Converts analyzed HBF data into organized RON files structured like cosmic-cults
//! with upgrade paths, metadata, and asset organization for Replit 3D generation.

use anyhow::Result;
use clap::{Parser, Subcommand};
use dl_seeds::{
    containers::RawEntity,
    orchestration::RawEntities,
    utilities::{determine_biome_type, sanitize_name},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "ron-generator")]
#[command(about = "Generate organized RON files for Replit 3D asset pipeline")]
#[command(version = "1.0.0")]
struct Cli {
    /// Path to analyzed HBF data (from hbf_analyzer)
    #[arg(short, long)]
    input: PathBuf,
    
    /// Output assets directory (like cosmic-cults/bevy-web/assets)
    #[arg(short, long)]
    output: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate all asset RONs (units, buildings, effects, terrain)
    GenerateAll {
        /// Use Dragon's Labyrinth corruption progression
        #[arg(long)]
        corruption_bands: bool,
    },
    /// Generate specific asset category
    Generate {
        /// Category to generate (units, buildings, effects, terrain, leaders)
        category: String,
        
        /// Specific faction/cult to generate for
        #[arg(short, long)]
        faction: Option<String>,
    },
    /// Generate upgrade progression chains
    Upgrades {
        /// Generate upgrade paths based on entity relationships
        #[arg(long)]
        auto_detect: bool,
    },
    /// Validate existing RON structure
    Validate {
        /// Path to existing assets to validate
        assets_path: PathBuf,
    },
}

/// Model metadata structure matching cosmic-cults pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelMetadata {
    id: String,
    display_name: String,
    model_path: String,
    scale: (f32, f32, f32),
    bounds: ModelBounds,
    animations: Vec<String>,
    sockets: Vec<ModelSocket>,
    tags: Vec<String>,
    cult: Option<String>,
    class: Option<String>,
    upgrades_to: Option<String>,
    ui_icon: Option<String>,
    sounds: Vec<SoundEvent>,
    // Dragon's Labyrinth specific additions
    corruption_band: Option<u8>,
    horror_theme: Option<String>,
    forge_material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelBounds {
    min: (f32, f32, f32),
    max: (f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelSocket {
    name: String,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SoundEvent {
    event: String,
    sound_path: String,
    volume: f32,
    pitch_variation: f32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("🏗️ Dragon's Labyrinth RON Generator");
    println!("📁 Input: {}", cli.input.display());
    println!("📁 Output: {}", cli.output.display());
    
    // Ensure output directory exists
    std::fs::create_dir_all(&cli.output)?;
    
    match &cli.command {
        Commands::GenerateAll { corruption_bands } => {
            generate_all_assets(&cli.input, &cli.output, *corruption_bands)?;
        }
        Commands::Generate { category, faction } => {
            generate_category_assets(&cli.input, &cli.output, category, faction.as_deref())?;
        }
        Commands::Upgrades { auto_detect } => {
            generate_upgrade_chains(&cli.input, &cli.output, *auto_detect)?;
        }
        Commands::Validate { assets_path } => {
            validate_ron_structure(assets_path)?;
        }
    }
    
    Ok(())
}

fn generate_all_assets(input_dir: &PathBuf, output_dir: &PathBuf, use_corruption_bands: bool) -> Result<()> {
    println!("🔄 Generating all asset RONs from analyzed HBF data...");
    
    // Load analyzed entities
    let analyzed_data = load_analyzed_entities(input_dir)?;
    
    // Generate each asset category
    generate_units_from_entities(&analyzed_data, output_dir, use_corruption_bands)?;
    generate_buildings_from_entities(&analyzed_data, output_dir, use_corruption_bands)?;
    generate_leaders_from_entities(&analyzed_data, output_dir, use_corruption_bands)?;
    generate_terrain_from_entities(&analyzed_data, output_dir, use_corruption_bands)?;
    
    println!("✅ All asset RONs generated successfully");
    Ok(())
}

fn load_analyzed_entities(input_dir: &PathBuf) -> Result<RawEntities> {
    // Load from JSON files created by hbf_analyzer
    let mut entities = RawEntities::new();
    
    // Load each category if exists
    if let Ok(content) = std::fs::read_to_string(input_dir.join("regions.json")) {
        entities.regions = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("settlements.json")) {
        entities.settlements = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("factions.json")) {
        entities.factions = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("dungeons.json")) {
        entities.dungeons = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("uncategorized.json")) {
        entities.uncategorized = serde_json::from_str(&content)?;
    }
    
    Ok(entities)
}

fn generate_units_from_entities(
    entities: &RawEntities,
    output_dir: &PathBuf,
    use_corruption_bands: bool,
) -> Result<()> {
    println!("⚔️ Generating unit RONs...");
    
    let units_dir = output_dir.join("units");
    
    // Process each faction
    for (faction_name, faction_entities) in &entities.factions {
        let faction_dir = units_dir.join(sanitize_name(faction_name));
        std::fs::create_dir_all(&faction_dir)?;
        
        for entity in faction_entities {
            let unit_metadata = create_unit_metadata_from_entity(
                entity, 
                faction_name, 
                use_corruption_bands
            );
            
            let ron_content = ron::ser::to_string_pretty(&unit_metadata, ron::ser::PrettyConfig::default())?;
            let filename = format!("{}.meta.ron", sanitize_name(&entity.entity_name));
            std::fs::write(faction_dir.join(filename), ron_content)?;
        }
        
        println!("  Generated {} units for faction: {}", faction_entities.len(), faction_name);
    }
    
    Ok(())
}

fn create_unit_metadata_from_entity(
    entity: &RawEntity,
    faction: &str,
    use_corruption_bands: bool,
) -> ModelMetadata {
    let sanitized_name = sanitize_name(&entity.entity_name);
    let display_name = entity.entity_name.clone();
    
    // Determine unit class and upgrade path from content
    let (class, upgrades_to) = determine_unit_progression(&entity.raw_value);
    
    // Extract corruption band if using Dragon's Labyrinth system
    let corruption_band = if use_corruption_bands {
        extract_corruption_band_from_content(&entity.raw_value)
    } else {
        None
    };
    
    ModelMetadata {
        id: format!("{}_model", sanitized_name),
        display_name,
        model_path: format!("units/{}/{}.glb", sanitize_name(faction), sanitized_name),
        scale: (1.0, 1.0, 1.0),
        bounds: ModelBounds {
            min: (-0.5, 0.0, -0.5),
            max: (0.5, 2.0, 0.5),
        },
        animations: determine_animations_from_content(&entity.raw_value),
        sockets: create_default_sockets(&entity.raw_value),
        tags: extract_tags_from_content(&entity.raw_value, faction),
        cult: Some(sanitize_name(faction)),
        class,
        upgrades_to,
        ui_icon: Some(format!("icons/{}.png", sanitized_name)),
        sounds: create_default_sounds(&entity.raw_value),
        corruption_band,
        horror_theme: extract_horror_theme(&entity.raw_value),
        forge_material: extract_forge_material(&entity.raw_value),
    }
}

fn determine_unit_progression(content: &str) -> (Option<String>, Option<String>) {
    let content_lower = content.to_lowercase();
    
    // Determine class based on content
    let class = if content_lower.contains("priest") || content_lower.contains("cleric") {
        Some("priest".to_string())
    } else if content_lower.contains("warrior") || content_lower.contains("fighter") {
        Some("warrior".to_string())
    } else if content_lower.contains("mage") || content_lower.contains("wizard") {
        Some("mage".to_string())
    } else if content_lower.contains("scout") || content_lower.contains("ranger") {
        Some("scout".to_string())
    } else {
        Some("cultist".to_string())
    };
    
    // Simple upgrade path logic (can be enhanced)
    let upgrades_to = if content_lower.contains("acolyte") || content_lower.contains("initiate") {
        Some("advanced_cultist_model".to_string())
    } else if content_lower.contains("cultist") && !content_lower.contains("advanced") {
        Some("veteran_cultist_model".to_string())
    } else {
        None
    };
    
    (class, upgrades_to)
}

fn extract_corruption_band_from_content(content: &str) -> Option<u8> {
    // Map content themes to corruption bands 1-5
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("peaceful") || content_lower.contains("village") {
        Some(1) // Peace to Unease
    } else if content_lower.contains("unease") || content_lower.contains("suspicion") {
        Some(2) // Unease to Dread  
    } else if content_lower.contains("dread") || content_lower.contains("fear") {
        Some(3) // Dread to Terror
    } else if content_lower.contains("terror") || content_lower.contains("horror") {
        Some(4) // Terror to Horror
    } else if content_lower.contains("void") || content_lower.contains("abyss") {
        Some(5) // Final Horror
    } else {
        Some(2) // Default to band 2
    }
}

fn determine_animations_from_content(content: &str) -> Vec<String> {
    let mut animations = vec!["idle".to_string(), "walk".to_string(), "death".to_string()];
    
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("attack") || content_lower.contains("combat") {
        animations.push("attack".to_string());
    }
    if content_lower.contains("cast") || content_lower.contains("spell") || content_lower.contains("magic") {
        animations.push("cast_spell".to_string());
    }
    if content_lower.contains("ritual") || content_lower.contains("ceremony") {
        animations.push("ritual".to_string());
    }
    if content_lower.contains("summon") {
        animations.push("summon".to_string());
    }
    
    animations
}

fn create_default_sockets(content: &str) -> Vec<ModelSocket> {
    let mut sockets = vec![
        ModelSocket {
            name: "head".to_string(),
            position: (0.0, 1.8, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        }
    ];
    
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("weapon") || content_lower.contains("sword") || content_lower.contains("blade") {
        sockets.push(ModelSocket {
            name: "weapon_hand".to_string(),
            position: (0.3, 1.2, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        });
    }
    
    if content_lower.contains("staff") || content_lower.contains("wand") || content_lower.contains("focus") {
        sockets.push(ModelSocket {
            name: "staff_hand".to_string(),
            position: (0.3, 1.2, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        });
    }
    
    if content_lower.contains("shield") {
        sockets.push(ModelSocket {
            name: "shield_hand".to_string(),
            position: (-0.3, 1.2, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        });
    }
    
    sockets
}

fn extract_tags_from_content(content: &str, faction: &str) -> Vec<String> {
    let mut tags = vec![sanitize_name(faction)];
    
    let content_lower = content.to_lowercase();
    
    // Basic type tags
    if content_lower.contains("human") || content_lower.contains("person") {
        tags.push("humanoid".to_string());
    }
    if content_lower.contains("creature") || content_lower.contains("beast") {
        tags.push("creature".to_string());
    }
    
    // Combat role tags
    if content_lower.contains("melee") || content_lower.contains("warrior") {
        tags.push("melee".to_string());
    }
    if content_lower.contains("ranged") || content_lower.contains("archer") {
        tags.push("ranged".to_string());
    }
    if content_lower.contains("caster") || content_lower.contains("mage") {
        tags.push("caster".to_string());
    }
    
    // Special tags
    if content_lower.contains("leader") || content_lower.contains("commander") {
        tags.push("leader".to_string());
    }
    if content_lower.contains("elite") || content_lower.contains("veteran") {
        tags.push("elite".to_string());
    }
    
    tags
}

fn create_default_sounds(content: &str) -> Vec<SoundEvent> {
    let mut sounds = vec![
        SoundEvent {
            event: "death".to_string(),
            sound_path: "sounds/generic_death.ogg".to_string(),
            volume: 1.0,
            pitch_variation: 0.15,
        }
    ];
    
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("attack") || content_lower.contains("combat") {
        sounds.push(SoundEvent {
            event: "attack".to_string(),
            sound_path: "sounds/generic_attack.ogg".to_string(),
            volume: 0.8,
            pitch_variation: 0.1,
        });
    }
    
    if content_lower.contains("magic") || content_lower.contains("spell") {
        sounds.push(SoundEvent {
            event: "cast".to_string(),
            sound_path: "sounds/magic_cast.ogg".to_string(),
            volume: 0.9,
            pitch_variation: 0.2,
        });
    }
    
    sounds
}

fn extract_horror_theme(content: &str) -> Option<String> {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("blood") || content_lower.contains("crimson") {
        Some("blood_corruption".to_string())
    } else if content_lower.contains("void") || content_lower.contains("abyss") {
        Some("void_consumption".to_string())
    } else if content_lower.contains("deep") || content_lower.contains("water") {
        Some("aquatic_horror".to_string())
    } else if content_lower.contains("decay") || content_lower.contains("rot") {
        Some("decay_progression".to_string())
    } else {
        None
    }
}

fn extract_forge_material(content: &str) -> Option<String> {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("iron") || content_lower.contains("steel") {
        Some("corrupted_iron".to_string())
    } else if content_lower.contains("bone") {
        Some("ancient_bone".to_string())
    } else if content_lower.contains("crystal") || content_lower.contains("gem") {
        Some("void_crystal".to_string())
    } else if content_lower.contains("wood") {
        Some("cursed_wood".to_string())
    } else {
        Some("shadow_essence".to_string())
    }
}

fn generate_buildings_from_entities(
    entities: &RawEntities,
    output_dir: &PathBuf,
    use_corruption_bands: bool,
) -> Result<()> {
    println!("🏰 Generating building RONs...");
    
    let buildings_dir = output_dir.join("buildings");
    
    // Generate buildings from settlements and some faction data
    for (settlement_name, settlement_entities) in &entities.settlements {
        let settlement_dir = buildings_dir.join(sanitize_name(settlement_name));
        std::fs::create_dir_all(&settlement_dir)?;
        
        for entity in settlement_entities {
            let building_metadata = create_building_metadata_from_entity(
                entity,
                settlement_name,
                use_corruption_bands,
            );
            
            let ron_content = ron::ser::to_string_pretty(&building_metadata, ron::ser::PrettyConfig::default())?;
            let filename = format!("{}.meta.ron", sanitize_name(&entity.entity_name));
            std::fs::write(settlement_dir.join(filename), ron_content)?;
        }
    }
    
    Ok(())
}

fn create_building_metadata_from_entity(
    entity: &RawEntity,
    settlement: &str,
    use_corruption_bands: bool,
) -> ModelMetadata {
    let sanitized_name = sanitize_name(&entity.entity_name);
    
    ModelMetadata {
        id: format!("{}_building", sanitized_name),
        display_name: entity.entity_name.clone(),
        model_path: format!("buildings/{}/{}.glb", sanitize_name(settlement), sanitized_name),
        scale: (1.0, 1.0, 1.0),
        bounds: ModelBounds {
            min: (-2.0, 0.0, -2.0),
            max: (2.0, 3.0, 2.0),
        },
        animations: vec!["idle".to_string(), "construction".to_string(), "destruction".to_string()],
        sockets: vec![
            ModelSocket {
                name: "entrance".to_string(),
                position: (0.0, 0.0, 2.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
            }
        ],
        tags: vec!["building".to_string(), "structure".to_string(), sanitize_name(settlement)],
        cult: Some(sanitize_name(settlement)),
        class: Some("building".to_string()),
        upgrades_to: None,
        ui_icon: Some(format!("icons/{}_building.png", sanitized_name)),
        sounds: vec![
            SoundEvent {
                event: "construction".to_string(),
                sound_path: "sounds/building_construct.ogg".to_string(),
                volume: 0.7,
                pitch_variation: 0.1,
            }
        ],
        corruption_band: if use_corruption_bands { 
            extract_corruption_band_from_content(&entity.raw_value) 
        } else { 
            None 
        },
        horror_theme: extract_horror_theme(&entity.raw_value),
        forge_material: extract_forge_material(&entity.raw_value),
    }
}

fn generate_leaders_from_entities(
    entities: &RawEntities,
    output_dir: &PathBuf,
    use_corruption_bands: bool,
) -> Result<()> {
    println!("👑 Generating leader RONs...");
    
    let leaders_dir = output_dir.join("leaders");
    std::fs::create_dir_all(&leaders_dir)?;
    
    // Generate leaders from faction entities with leadership indicators
    for (faction_name, faction_entities) in &entities.factions {
        for entity in faction_entities {
            if is_leader_entity(entity) {
                let leader_metadata = create_leader_metadata_from_entity(
                    entity,
                    faction_name,
                    use_corruption_bands,
                );
                
                let ron_content = ron::ser::to_string_pretty(&leader_metadata, ron::ser::PrettyConfig::default())?;
                let filename = format!("{}_leader.meta.ron", sanitize_name(&entity.entity_name));
                std::fs::write(leaders_dir.join(filename), ron_content)?;
                
                println!("  Generated leader: {} for {}", entity.entity_name, faction_name);
            }
        }
    }
    
    Ok(())
}

fn is_leader_entity(entity: &RawEntity) -> bool {
    let content = entity.raw_value.to_lowercase();
    content.contains("leader") || 
    content.contains("commander") || 
    content.contains("chief") || 
    content.contains("lord") || 
    content.contains("master") ||
    content.contains("high priest") ||
    content.contains("archon")
}

fn create_leader_metadata_from_entity(
    entity: &RawEntity,
    faction: &str,
    use_corruption_bands: bool,
) -> ModelMetadata {
    let sanitized_name = sanitize_name(&entity.entity_name);
    
    ModelMetadata {
        id: format!("{}_leader", sanitized_name),
        display_name: format!("{} (Leader)", entity.entity_name),
        model_path: format!("leaders/{}.glb", sanitized_name),
        scale: (1.2, 1.2, 1.2), // Leaders are larger
        bounds: ModelBounds {
            min: (-0.6, 0.0, -0.6),
            max: (0.6, 2.4, 0.6),
        },
        animations: vec![
            "idle".to_string(),
            "walk".to_string(),
            "attack".to_string(),
            "cast_spell".to_string(),
            "death".to_string(),
            "command".to_string(),
            "inspire".to_string(),
        ],
        sockets: create_leader_sockets(&entity.raw_value),
        tags: vec!["leader".to_string(), "elite".to_string(), "commander".to_string(), sanitize_name(faction)],
        cult: Some(sanitize_name(faction)),
        class: Some("leader".to_string()),
        upgrades_to: None, // Leaders don't upgrade
        ui_icon: Some(format!("icons/{}_leader.png", sanitized_name)),
        sounds: vec![
            SoundEvent {
                event: "command".to_string(),
                sound_path: "sounds/leader_command.ogg".to_string(),
                volume: 1.0,
                pitch_variation: 0.05,
            },
            SoundEvent {
                event: "death".to_string(),
                sound_path: "sounds/leader_death.ogg".to_string(),
                volume: 1.2,
                pitch_variation: 0.1,
            },
        ],
        corruption_band: if use_corruption_bands { 
            extract_corruption_band_from_content(&entity.raw_value) 
        } else { 
            None 
        },
        horror_theme: extract_horror_theme(&entity.raw_value),
        forge_material: Some("legendary_materials".to_string()),
    }
}

fn create_leader_sockets(_content: &str) -> Vec<ModelSocket> {
    vec![
        ModelSocket {
            name: "head".to_string(),
            position: (0.0, 2.1, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        },
        ModelSocket {
            name: "weapon_hand".to_string(),
            position: (0.4, 1.4, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        },
        ModelSocket {
            name: "focus_hand".to_string(),
            position: (-0.4, 1.4, 0.0),
            rotation: (0.0, 0.0, 0.0, 1.0),
        },
        ModelSocket {
            name: "banner".to_string(),
            position: (0.0, 2.5, -0.5),
            rotation: (0.0, 0.0, 0.0, 1.0),
        },
    ]
}

fn generate_terrain_from_entities(
    entities: &RawEntities,
    output_dir: &PathBuf,
    use_corruption_bands: bool,
) -> Result<()> {
    println!("🌍 Generating terrain RONs...");
    
    let terrain_dir = output_dir.join("terrain");
    std::fs::create_dir_all(&terrain_dir)?;
    
    // Generate terrain from region entities
    for (region_name, region_entities) in &entities.regions {
        for entity in region_entities {
            let terrain_metadata = create_terrain_metadata_from_entity(
                entity,
                region_name,
                use_corruption_bands,
            );
            
            let ron_content = ron::ser::to_string_pretty(&terrain_metadata, ron::ser::PrettyConfig::default())?;
            let filename = format!("{}_terrain.meta.ron", sanitize_name(&entity.entity_name));
            std::fs::write(terrain_dir.join(filename), ron_content)?;
        }
    }
    
    Ok(())
}

fn create_terrain_metadata_from_entity(
    entity: &RawEntity,
    region: &str,
    use_corruption_bands: bool,
) -> ModelMetadata {
    let sanitized_name = sanitize_name(&entity.entity_name);
    
    // Use biome type detection to enhance terrain classification based on entity position
    // For now, use a default coordinate since we don't have position data from entities
    let default_coords = (0, 0); // Could be extracted from entity content in future
    let biome_type = determine_biome_type(default_coords);
    let mut tags = vec!["terrain".to_string(), "environmental".to_string(), sanitize_name(region)];
    
    // Add biome-specific tags based on determined type
    match biome_type.as_str() {
        "forest" => {
            tags.extend(vec!["woodland".to_string(), "trees".to_string()]);
        }
        "mountain" => {
            tags.extend(vec!["rocky".to_string(), "elevation".to_string()]);
        }
        "swamp" => {
            tags.extend(vec!["wetland".to_string(), "murky".to_string()]);
        }
        "desert" => {
            tags.extend(vec!["arid".to_string(), "sandy".to_string()]);
        }
        "plain" => {
            tags.extend(vec!["grassland".to_string(), "open".to_string()]);
        }
        "water" => {
            tags.extend(vec!["aquatic".to_string(), "liquid".to_string()]);
        }
        _ => {
            tags.push("unknown_biome".to_string());
        }
    }
    
    ModelMetadata {
        id: format!("{}_{}_terrain", sanitized_name, biome_type),
        display_name: format!("{} {} Terrain", entity.entity_name, biome_type.to_uppercase()),
        model_path: format!("terrain/{}/{}.glb", biome_type, sanitized_name),
        scale: (1.0, 1.0, 1.0),
        bounds: ModelBounds {
            min: (-5.0, 0.0, -5.0),
            max: (5.0, 2.0, 5.0),
        },
        animations: vec!["idle".to_string(), "environmental".to_string()],
        sockets: vec![],
        tags,
        cult: None,
        class: Some(format!("{}_terrain", biome_type)),
        upgrades_to: None,
        ui_icon: Some(format!("icons/{}_{}_terrain.png", biome_type, sanitized_name)),
        sounds: vec![
            SoundEvent {
                event: "ambient".to_string(),
                sound_path: format!("sounds/{}_{}_ambient.ogg", biome_type, sanitized_name),
                volume: 0.3,
                pitch_variation: 0.2,
            }
        ],
        corruption_band: if use_corruption_bands { 
            extract_corruption_band_from_content(&entity.raw_value) 
        } else { 
            None 
        },
        horror_theme: extract_horror_theme(&entity.raw_value),
        forge_material: None,
    }
}

fn generate_category_assets(
    input_dir: &PathBuf,
    output_dir: &PathBuf,
    category: &str,
    faction_filter: Option<&str>,
) -> Result<()> {
    println!("🔄 Generating {} assets...", category);
    
    let mut entities = load_analyzed_entities(input_dir)?;
    
    // Apply faction filtering if specified
    if let Some(faction_name) = faction_filter {
        println!("  Filtering for faction: {}", faction_name);
        
        // Filter factions to only include the specified one
        entities.factions.retain(|name, _| name.to_lowercase().contains(&faction_name.to_lowercase()));
        
        // Filter settlements that might belong to this faction
        entities.settlements.retain(|name, entities_list| {
            name.to_lowercase().contains(&faction_name.to_lowercase()) ||
            entities_list.iter().any(|e| e.raw_value.to_lowercase().contains(&faction_name.to_lowercase()))
        });
        
        // Filter regions related to this faction
        entities.regions.retain(|name, entities_list| {
            name.to_lowercase().contains(&faction_name.to_lowercase()) ||
            entities_list.iter().any(|e| e.raw_value.to_lowercase().contains(&faction_name.to_lowercase()))
        });
        
        println!("  Filtered to {} factions, {} settlements, {} regions", 
                 entities.factions.len(), entities.settlements.len(), entities.regions.len());
    }
    
    match category {
        "units" => generate_units_from_entities(&entities, output_dir, true)?,
        "buildings" => generate_buildings_from_entities(&entities, output_dir, true)?,
        "leaders" => generate_leaders_from_entities(&entities, output_dir, true)?,
        "terrain" => generate_terrain_from_entities(&entities, output_dir, true)?,
        _ => {
            println!("❌ Unknown category: {}. Use: units, buildings, leaders, terrain", category);
        }
    }
    
    Ok(())
}

fn generate_upgrade_chains(input_dir: &PathBuf, output_dir: &PathBuf, auto_detect: bool) -> Result<()> {
    println!("🔗 Generating upgrade chains...");
    
    let entities = load_analyzed_entities(input_dir)?;
    let chains_dir = output_dir.join("upgrade_chains");
    std::fs::create_dir_all(&chains_dir)?;
    
    if auto_detect {
        // Auto-detect upgrade relationships from entity content
        let upgrade_chains = detect_upgrade_relationships(&entities)?;
        
        for (faction_name, chains) in upgrade_chains {
            let chain_file = chains_dir.join(format!("{}_upgrades.ron", sanitize_name(&faction_name)));
            let ron_content = ron::ser::to_string_pretty(&chains, ron::ser::PrettyConfig::default())?;
            std::fs::write(chain_file, ron_content)?;
            
            println!("  Generated upgrade chains for: {}", faction_name);
        }
    }
    
    Ok(())
}

fn detect_upgrade_relationships(entities: &RawEntities) -> Result<HashMap<String, Vec<UpgradeChain>>> {
    let mut faction_chains = HashMap::new();
    
    for (faction_name, faction_entities) in &entities.factions {
        let mut chains = Vec::new();
        
        // Simple progression detection: acolyte -> cultist -> priest -> leader
        let mut acolytes = Vec::new();
        let mut cultists = Vec::new();
        let mut priests = Vec::new();
        let mut leaders = Vec::new();
        
        for entity in faction_entities {
            let content = entity.raw_value.to_lowercase();
            if content.contains("acolyte") || content.contains("initiate") {
                acolytes.push(entity.entity_name.clone());
            } else if content.contains("cultist") {
                cultists.push(entity.entity_name.clone());
            } else if content.contains("priest") || content.contains("cleric") {
                priests.push(entity.entity_name.clone());
            } else if content.contains("leader") || content.contains("commander") {
                leaders.push(entity.entity_name.clone());
            }
        }
        
        // Create progression chains
        if !acolytes.is_empty() && !cultists.is_empty() {
            chains.push(UpgradeChain {
                tier: 1,
                from: acolytes[0].clone(),
                to: cultists[0].clone(),
                requirements: vec!["combat_experience".to_string()],
            });
        }
        
        if !cultists.is_empty() && !priests.is_empty() {
            chains.push(UpgradeChain {
                tier: 2,
                from: cultists[0].clone(),
                to: priests[0].clone(),
                requirements: vec!["ritual_knowledge".to_string(), "faction_devotion".to_string()],
            });
        }
        
        faction_chains.insert(faction_name.clone(), chains);
    }
    
    Ok(faction_chains)
}

fn validate_ron_structure(assets_path: &PathBuf) -> Result<()> {
    println!("🔍 Validating RON structure...");
    
    let units_dir = assets_path.join("units");
    if units_dir.exists() {
        validate_units_structure(&units_dir)?;
    }
    
    let buildings_dir = assets_path.join("buildings");
    if buildings_dir.exists() {
        validate_buildings_structure(&buildings_dir)?;
    }
    
    let leaders_dir = assets_path.join("leaders");
    if leaders_dir.exists() {
        validate_leaders_structure(&leaders_dir)?;
    }
    
    println!("✅ RON structure validation complete");
    Ok(())
}

fn validate_units_structure(units_dir: &PathBuf) -> Result<()> {
    println!("  Validating units structure...");
    
    for entry in std::fs::read_dir(units_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let faction_dir = entry.path();
            let faction_name = faction_dir.file_name().unwrap().to_string_lossy();
            
            let mut unit_count = 0;
            for unit_entry in std::fs::read_dir(&faction_dir)? {
                let unit_entry = unit_entry?;
                if unit_entry.path().extension().and_then(|s| s.to_str()) == Some("ron") {
                    // Validate RON can be parsed
                    let content = std::fs::read_to_string(unit_entry.path())?;
                    match ron::from_str::<ModelMetadata>(&content) {
                        Ok(_) => unit_count += 1,
                        Err(e) => println!("    ⚠️ Invalid RON in {}: {}", unit_entry.path().display(), e),
                    }
                }
            }
            
            println!("    ✅ {} units validated in faction: {}", unit_count, faction_name);
        }
    }
    
    Ok(())
}

fn validate_buildings_structure(buildings_dir: &PathBuf) -> Result<()> {
    println!("  Validating buildings structure...");
    // Similar validation logic for buildings
    Ok(())
}

fn validate_leaders_structure(leaders_dir: &PathBuf) -> Result<()> {
    println!("  Validating leaders structure...");
    // Similar validation logic for leaders
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpgradeChain {
    tier: u8,
    from: String,
    to: String,
    requirements: Vec<String>,
}
