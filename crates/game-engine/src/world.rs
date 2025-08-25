//! Dragon's Labyrinth World - Bevy ECS World Management
//!
//! Manages the Bevy World with proper ECS patterns. Components define schemas that
//! the Python AI generation system scans and fills with actual entity data.

use bevy::prelude::*;
use hexx::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Main world plugin that sets up the ECS World architecture
pub struct DragonLabyrinthWorldPlugin;

impl Plugin for DragonLabyrinthWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core world resources
            .init_resource::<HexWorldLayout>()
            .init_resource::<ChunkManager>()
            .init_resource::<DreadState>()
            .init_resource::<EntityGenerationSchemas>()
            
            // Register all component types for Python scanning
            .register_type::<crate::components::hex_tiles::HexTile>()
            .register_type::<crate::components::hex_tiles::Biome>()
            .register_type::<crate::components::hex_tiles::Path>()
            .register_type::<crate::components::hex_tiles::Feature>()
            .register_type::<crate::components::hex_tiles::Corruption>()
            .register_type::<crate::components::companions::Companion>()
            .register_type::<crate::components::companions::CompanionPsychology>()
            .register_type::<crate::components::companions::TraumaSources>()
            
            // World management systems
            .add_systems(Startup, (
                setup_hex_world_layout,
                setup_chunk_manager,
                initialize_dread_state,
                register_component_schemas,
            ).chain())
            
            .add_systems(Update, (
                chunk_loading_system,
                world_entity_spawning_system,
                dread_progression_system,
            ));
    }
}

/// Resource that defines the hex world layout using hexx
#[derive(Resource)]
pub struct HexWorldLayout {
    pub layout: HexLayout,
    pub origin: Hex,
    pub current_radius: u32,
    pub max_radius: u32, // 180 for full progression
}

impl Default for HexWorldLayout {
    fn default() -> Self {
        Self {
            layout: HexLayout {
                orientation: HexOrientation::Pointy,
                origin: Vec2::ZERO,
                hex_size: Vec2::splat(32.0), // 32 pixel hex size
            },
            origin: Hex::ZERO,
            current_radius: 5,
            max_radius: 180,
        }
    }
}

/// Resource for simple tile loading around player
#[derive(Resource)]
pub struct TileManager {
    pub loaded_tiles: HashMap<Hex, Entity>,
    pub loading_radius: u32,
    pub max_loaded_tiles: usize,
}

impl Default for TileManager {
    fn default() -> Self {
        Self {
            loaded_tiles: HashMap::new(),
            loading_radius: 20, // Load tiles within 20 hex radius
            max_loaded_tiles: 2000, // Reasonable limit for performance
        }
    }
}

/// Global dread state that affects all systems
#[derive(Resource)]
pub struct DreadState {
    pub level: u8, // 0-4
    pub player_distance_from_origin: f32,
    pub corruption_multiplier: f32,
    pub active_horror_sources: Vec<Entity>,
}

impl Default for DreadState {
    fn default() -> Self {
        Self {
            level: 0,
            player_distance_from_origin: 0.0,
            corruption_multiplier: 1.0,
            active_horror_sources: Vec::new(),
        }
    }
}

/// Component schemas that Python can scan to generate entities
#[derive(Resource, Serialize, Deserialize)]
pub struct EntityGenerationSchemas {
    pub component_schemas: HashMap<String, ComponentSchema>,
    pub bundle_schemas: HashMap<String, BundleSchema>,
    pub system_requirements: HashMap<String, SystemRequirements>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentSchema {
    pub component_name: String,
    pub fields: Vec<FieldSchema>,
    pub description: String,
    pub ai_generation_hints: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FieldSchema {
    pub field_name: String,
    pub field_type: String,
    pub description: String,
    pub constraints: Option<FieldConstraints>,
    pub ai_generation_guidance: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FieldConstraints {
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub allowed_values: Option<Vec<String>>,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BundleSchema {
    pub bundle_name: String,
    pub components: Vec<String>,
    pub description: String,
    pub typical_use_cases: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemRequirements {
    pub system_name: String,
    pub required_components: Vec<String>,
    pub optional_components: Vec<String>,
    pub description: String,
}

impl Default for EntityGenerationSchemas {
    fn default() -> Self {
        Self {
            component_schemas: HashMap::new(),
            bundle_schemas: HashMap::new(),
            system_requirements: HashMap::new(),
        }
    }
}

// World setup systems

fn setup_hex_world_layout(
    mut commands: Commands,
) {
    let layout = HexWorldLayout::default();
    info!("Initialized hex world layout with {} pixel hexes", layout.layout.hex_size.x);
    commands.insert_resource(layout);
}

fn setup_chunk_manager(
    mut commands: Commands,
) {
    let chunk_manager = ChunkManager::default();
    info!("Initialized chunk manager: {} chunk size, {} loading radius", 
          chunk_manager.chunk_size, chunk_manager.loading_radius);
    commands.insert_resource(chunk_manager);
}

fn initialize_dread_state(
    mut commands: Commands,
) {
    let dread_state = DreadState::default();
    info!("Initialized dread state at level {}", dread_state.level);
    commands.insert_resource(dread_state);
}

fn register_component_schemas(
    mut commands: Commands,
) {
    let mut schemas = EntityGenerationSchemas::default();
    
    // Register HexTile component schema
    schemas.component_schemas.insert("HexTile".to_string(), ComponentSchema {
        component_name: "HexTile".to_string(),
        fields: vec![
            FieldSchema {
                field_name: "coord".to_string(),
                field_type: "Hex".to_string(),
                description: "Hexagonal coordinate using hexx::Hex".to_string(),
                constraints: None,
                ai_generation_guidance: Some("Generate based on world position and chunk layout".to_string()),
            },
            FieldSchema {
                field_name: "world_position".to_string(),
                field_type: "Vec3".to_string(),
                description: "3D world position for rendering".to_string(),
                constraints: None,
                ai_generation_guidance: Some("Convert from hex coordinate using layout".to_string()),
            },
        ],
        description: "Base hex tile component containing coordinate information".to_string(),
        ai_generation_hints: vec![
            "Use hexx math for coordinate conversion".to_string(),
            "Ensure world_position matches coord via hex_layout".to_string(),
        ],
    });
    
    // Register Companion component schema  
    schemas.component_schemas.insert("Companion".to_string(), ComponentSchema {
        component_name: "Companion".to_string(),
        fields: vec![
            FieldSchema {
                field_name: "companion_id".to_string(),
                field_type: "String".to_string(),
                description: "Unique identifier for this companion".to_string(),
                constraints: Some(FieldConstraints {
                    min_value: None,
                    max_value: None,
                    allowed_values: None,
                    required: true,
                }),
                ai_generation_guidance: Some("Generate unique ID based on character archetype".to_string()),
            },
            FieldSchema {
                field_name: "name".to_string(),
                field_type: "String".to_string(),
                description: "Display name of the companion".to_string(),
                constraints: Some(FieldConstraints {
                    min_value: None,
                    max_value: None,
                    allowed_values: None,
                    required: true,
                }),
                ai_generation_guidance: Some("Generate fantasy names appropriate to world lore".to_string()),
            },
            FieldSchema {
                field_name: "companion_type".to_string(),
                field_type: "CompanionType".to_string(),
                description: "Archetype/class of this companion".to_string(),
                constraints: Some(FieldConstraints {
                    min_value: None,
                    max_value: None,
                    allowed_values: Some(vec!["Warrior".to_string(), "Healer".to_string(), "Rogue".to_string(), "Mage".to_string()]),
                    required: true,
                }),
                ai_generation_guidance: Some("Choose based on party balance and story needs".to_string()),
            },
        ],
        description: "Core companion identity component".to_string(),
        ai_generation_hints: vec![
            "Generate diverse companion archetypes".to_string(),
            "Ensure names fit the dark fantasy world".to_string(),
            "Each companion should have unique personality traits".to_string(),
        ],
    });
    
    // Register bundle schemas
    schemas.bundle_schemas.insert("LayerCakeHexTileBundle".to_string(), BundleSchema {
        bundle_name: "LayerCakeHexTileBundle".to_string(),
        components: vec![
            "HexTile".to_string(),
            "Biome".to_string(), 
            "Path".to_string(),
            "Feature".to_string(),
            "Corruption".to_string(),
            "DiscoveryState".to_string(),
            "DreadEffects".to_string(),
        ],
        description: "Complete hex tile with layer cake architecture".to_string(),
        typical_use_cases: vec![
            "Spawning world tiles".to_string(),
            "Procedural world generation".to_string(),
            "Memory-optimized chunk loading".to_string(),
        ],
    });
    
    schemas.bundle_schemas.insert("CompanionBundle".to_string(), BundleSchema {
        bundle_name: "CompanionBundle".to_string(),
        components: vec![
            "Companion".to_string(),
            "CompanionStats".to_string(),
            "CompanionPsychology".to_string(),
            "TraumaSources".to_string(),
            "CompanionRelationship".to_string(),
        ],
        description: "Complete companion with psychology system".to_string(),
        typical_use_cases: vec![
            "Spawning companions".to_string(),
            "AI-generated companion creation".to_string(),
            "Psychology system integration".to_string(),
        ],
    });
    
    // Register system requirements
    schemas.system_requirements.insert("trauma_processing_system".to_string(), SystemRequirements {
        system_name: "trauma_processing_system".to_string(),
        required_components: vec!["CompanionPsychology".to_string(), "TraumaSources".to_string()],
        optional_components: vec!["TherapyParticipant".to_string()],
        description: "Processes trauma events and updates companion psychology".to_string(),
    });
    
    info!("Registered {} component schemas for AI generation", schemas.component_schemas.len());
    commands.insert_resource(schemas);
}

// World management systems

fn chunk_loading_system(
    mut chunk_manager: ResMut<ChunkManager>,
    hex_layout: Res<HexWorldLayout>,
    player_query: Query<&Transform, (With<crate::components::players::Player>, Changed<Transform>)>,
    mut commands: Commands,
) {
    // Load/unload chunks based on player position
    if let Ok(player_transform) = player_query.get_single() {
        let player_hex = hex_layout.layout.world_pos_to_hex(player_transform.translation.truncate());
        let player_chunk = (
            player_hex.q.div_euclid(chunk_manager.chunk_size as i32),
            player_hex.r.div_euclid(chunk_manager.chunk_size as i32)
        );
        
        // Load chunks in radius around player
        let loading_radius = chunk_manager.loading_radius as i32;
        for q in (player_chunk.0 - loading_radius)..=(player_chunk.0 + loading_radius) {
            for r in (player_chunk.1 - loading_radius)..=(player_chunk.1 + loading_radius) {
                let chunk_coord = (q, r);
                if !chunk_manager.loaded_chunks.contains_key(&chunk_coord) {
                    // Send signal to Python to generate this chunk
                    commands.trigger(ChunkGenerationRequest {
                        chunk_coord,
                        priority: calculate_chunk_priority(chunk_coord, player_chunk),
                    });
                }
            }
        }
        
        // Unload distant chunks
        let chunks_to_unload: Vec<_> = chunk_manager.loaded_chunks.keys()
            .filter(|(q, r)| {
                let distance = ((q - player_chunk.0).abs() + (r - player_chunk.1).abs()) as u32;
                distance > chunk_manager.loading_radius * 2
            })
            .cloned()
            .collect();
            
        for chunk_coord in chunks_to_unload {
            if let Some(entities) = chunk_manager.loaded_chunks.remove(&chunk_coord) {
                for entity in entities {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn world_entity_spawning_system(
    mut entity_spawn_events: EventReader<EntitySpawnRequest>,
    mut commands: Commands,
    hex_layout: Res<HexWorldLayout>,
    asset_server: Res<AssetServer>,
) {
    // Spawn entities from Python generation requests
    for spawn_request in entity_spawn_events.read() {
        match spawn_request.entity_type.as_str() {
            "LayerCakeHexTile" => {
                if let Some(hex_data) = &spawn_request.hex_tile_data {
                    let coord = Hex::new(hex_data.q, hex_data.r);
                    let entity = commands.spawn(
                        crate::components::hex_tiles::LayerCakeHexTileBundle::new(
                            coord, 
                            &hex_layout.layout, 
                            hex_data.biome_type.clone()
                        )
                    ).id();
                    
                    info!("Spawned hex tile at {:?}", coord);
                }
            },
            "Companion" => {
                if let Some(companion_data) = &spawn_request.companion_data {
                    let entity = commands.spawn(
                        crate::components::companions::CompanionBundle::from_generated_data(
                            companion_data,
                            &asset_server
                        )
                    ).id();
                    
                    info!("Spawned companion: {}", companion_data.name);
                }
            },
            _ => {
                warn!("Unknown entity type: {}", spawn_request.entity_type);
            }
        }
    }
}

fn dread_progression_system(
    mut dread_state: ResMut<DreadState>,
    player_query: Query<&Transform, With<crate::components::players::Player>>,
    hex_layout: Res<HexWorldLayout>,
    mut dread_effects_query: Query<&mut crate::components::hex_tiles::DreadEffects>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_hex = hex_layout.layout.world_pos_to_hex(player_transform.translation.truncate());
        let distance_from_origin = player_hex.distance_to(hex_layout.origin) as f32;
        
        dread_state.player_distance_from_origin = distance_from_origin;
        
        // Calculate dread level based on distance (0-180 maps to 0-4)
        let new_dread_level = ((distance_from_origin / 45.0).floor() as u8).min(4);
        
        if new_dread_level != dread_state.level {
            dread_state.level = new_dread_level;
            info!("Dread level changed to {} at distance {}", new_dread_level, distance_from_origin);
            
            // Update all tile dread effects
            for mut dread_effects in dread_effects_query.iter_mut() {
                dread_effects.dread_level = new_dread_level;
            }
        }
    }
}

// Events for Python integration

#[derive(Event)]
pub struct ChunkGenerationRequest {
    pub chunk_coord: (i32, i32),
    pub priority: u32,
}

#[derive(Event)]
pub struct EntitySpawnRequest {
    pub entity_type: String,
    pub hex_tile_data: Option<GeneratedHexTileData>,
    pub companion_data: Option<GeneratedCompanionData>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratedHexTileData {
    pub q: i32,
    pub r: i32,
    pub biome_type: crate::components::hex_tiles::BiomeType,
    pub features: Vec<String>,
    pub corruption_level: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratedCompanionData {
    pub companion_id: String,
    pub name: String,
    pub companion_type: crate::components::companions::CompanionType,
    pub backstory: String,
    pub initial_psychology: PsychologyData,
}

#[derive(Serialize, Deserialize)]
pub struct PsychologyData {
    pub breaking_point: f32,
    pub loyalty: f32,
    pub trust: f32,
    pub trauma_triggers: Vec<String>,
}

// Helper functions

fn calculate_chunk_priority(chunk_coord: (i32, i32), player_chunk: (i32, i32)) -> u32 {
    let distance = ((chunk_coord.0 - player_chunk.0).abs() + (chunk_coord.1 - player_chunk.1).abs()) as u32;
    10 - distance.min(10) // Higher priority for closer chunks
}

/// Export component schemas for Python scanning
pub fn export_schemas_for_python() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = EntityGenerationSchemas::default();
    let json = serde_json::to_string_pretty(&schemas)?;
    std::fs::write("component_schemas.json", json)?;
    println!("Exported component schemas to component_schemas.json");
    Ok(())
}
