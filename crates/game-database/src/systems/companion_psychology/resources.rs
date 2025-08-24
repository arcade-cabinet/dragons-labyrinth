//! Companion Psychology System - ECS Resources
//!
//! Production-ready resources for managing companion psychology state, therapy configurations,
//! and memory palace systems with full database integration.

use bevy::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use database_orm::{companions, psychology};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Main psychology system state with full database backing
#[derive(Resource, Debug)]
pub struct CompanionPsychologyState {
    pub db: DatabaseConnection,
    pub active_companions: HashMap<Uuid, CompanionPsychologyData>,
    pub therapy_sessions_cache: HashMap<Uuid, Vec<psychology::Model>>,
    pub memory_palace_entities: HashMap<Uuid, Entity>,
    pub professional_network: HashMap<String, ProfessionalProvider>,
    pub trauma_trigger_mapping: HashMap<String, Vec<String>>, // trigger -> affected companions
}

#[derive(Debug, Clone)]
pub struct CompanionPsychologyData {
    pub entity: Entity,
    pub db_model: companions::Model,
    pub psychology_model: Option<psychology::Model>,
    pub last_therapy_session: Option<i64>,
    pub active_memory_palace: Option<Entity>,
}

#[derive(Debug, Clone)]
pub struct ProfessionalProvider {
    pub provider_type: String, // "therapist", "psychiatrist", "support_group"
    pub quality_rating: f32,
    pub availability: f32,
    pub cost_per_session: f32,
    pub specializations: Vec<String>,
    pub location: String,
}

impl CompanionPsychologyState {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize professional support network
        let mut professional_network = HashMap::new();
        
        // Add realistic professional support options
        professional_network.insert("village_healer".to_string(), ProfessionalProvider {
            provider_type: "folk_healer".to_string(),
            quality_rating: 0.6,
            availability: 0.8,
            cost_per_session: 5.0,
            specializations: vec!["trauma_herbs".to_string(), "spiritual_guidance".to_string()],
            location: "village_center".to_string(),
        });
        
        professional_network.insert("temple_counselor".to_string(), ProfessionalProvider {
            provider_type: "religious_counselor".to_string(),
            quality_rating: 0.7,
            availability: 0.6,
            cost_per_session: 0.0, // Free but requires religious alignment
            specializations: vec!["grief_counseling".to_string(), "moral_guidance".to_string()],
            location: "temple_district".to_string(),
        });
        
        professional_network.insert("court_physician".to_string(), ProfessionalProvider {
            provider_type: "medical_professional".to_string(),
            quality_rating: 0.9,
            availability: 0.3,
            cost_per_session: 50.0,
            specializations: vec!["severe_trauma".to_string(), "medication".to_string()],
            location: "capital_city".to_string(),
        });
        
        Ok(Self {
            db,
            active_companions: HashMap::new(),
            therapy_sessions_cache: HashMap::new(),
            memory_palace_entities: HashMap::new(),
            professional_network,
            trauma_trigger_mapping: HashMap::new(),
        })
    }
    
    /// Load companion psychology data from database
    pub async fn load_companion_psychology(&mut self, companion_id: Uuid) -> Result<Option<CompanionPsychologyData>, Box<dyn std::error::Error>> {
        // Load companion model
        let companion = companions::Entity::find_by_id(companion_id)
            .one(&self.db)
            .await?;
            
        let companion = match companion {
            Some(c) => c,
            None => return Ok(None),
        };
        
        // Load associated psychology data
        let psychology = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .one(&self.db)
            .await?;
        
        // Load therapy sessions for caching
        let therapy_sessions: Vec<psychology::Model> = psychology::Entity::find()
            .filter(psychology::Column::CompanionId.eq(companion_id))
            .all(&self.db)
            .await?;
        
        self.therapy_sessions_cache.insert(companion_id, therapy_sessions);
        
        let psychology_data = CompanionPsychologyData {
            entity: Entity::PLACEHOLDER, // Will be set when spawning ECS entity
            db_model: companion,
            psychology_model: psychology,
            last_therapy_session: None,
            active_memory_palace: None,
        };
        
        self.active_companions.insert(companion_id, psychology_data.clone());
        Ok(Some(psychology_data))
    }
    
    /// Save companion psychology changes to database
    pub async fn save_companion_psychology(&self, companion_id: Uuid, psychology_data: &CompanionPsychologyData) -> Result<(), Box<dyn std::error::Error>> {
        use sea_orm::*;
        
        // Update companion model
        let mut companion: companions::ActiveModel = psychology_data.db_model.clone().into();
        companion.updated_at = Set(chrono::Utc::now());
        companion.update(&self.db).await?;
        
        // Update psychology model if exists
        if let Some(psychology_model) = &psychology_data.psychology_model {
            let mut psychology: psychology::ActiveModel = psychology_model.clone().into();
            psychology.updated_at = Set(chrono::Utc::now());
            psychology.update(&self.db).await?;
        }
        
        Ok(())
    }
    
    /// Get available professional support for location
    pub fn get_available_professionals(&self, location: &str) -> Vec<&ProfessionalProvider> {
        self.professional_network
            .values()
            .filter(|provider| provider.location == location || provider.location == "traveling")
            .collect()
    }
}

/// Configuration loaded from game balance files
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct TherapyConfiguration {
    // Trauma system configuration
    pub trauma_categories: HashMap<String, TraumaCategory>,
    pub breaking_point_modifiers: HashMap<String, f32>, // companion_type -> modifier
    pub recovery_base_rates: HashMap<String, f32>,      // trauma_type -> base recovery rate
    
    // Therapy effectiveness by type and trauma
    pub therapy_effectiveness_matrix: HashMap<String, HashMap<String, f32>>,
    
    // Memory palace configuration
    pub memory_palace_room_types: Vec<MemoryRoomTemplate>,
    pub healing_symbol_library: Vec<HealingSymbolTemplate>,
    
    // Professional support configuration
    pub professional_support_multipliers: HashMap<String, f32>,
    
    // Dragon's Labyrinth specific settings
    pub dread_level_trauma_amplifiers: [f32; 5], // 0-4 dread levels
    pub companion_support_synergies: HashMap<String, Vec<String>>, // which companions help each other
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaCategory {
    pub base_severity: f32,
    pub recovery_difficulty: f32,
    pub requires_professional_help_threshold: f32,
    pub common_triggers: Vec<String>,
    pub therapeutic_approaches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRoomTemplate {
    pub room_type: String,
    pub emotional_themes: Vec<String>,
    pub healing_potential: f32,
    pub accessibility_requirements: Vec<String>,
    pub therapeutic_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingSymbolTemplate {
    pub symbol_type: String,
    pub healing_categories: Vec<String>, // which traumas this helps
    pub activation_requirements: Vec<String>,
    pub healing_power: f32,
    pub visual_description: String,
}

impl Default for TherapyConfiguration {
    fn default() -> Self {
        let mut trauma_categories = HashMap::new();
        
        // Dragon's Labyrinth specific trauma categories
        trauma_categories.insert("dragon_encounter".to_string(), TraumaCategory {
            base_severity: 2.5,
            recovery_difficulty: 0.8,
            requires_professional_help_threshold: 1.5,
            common_triggers: vec!["large_creatures".to_string(), "roaring_sounds".to_string(), "fire".to_string()],
            therapeutic_approaches: vec!["exposure_therapy".to_string(), "grounding_techniques".to_string()],
        });
        
        trauma_categories.insert("companion_death".to_string(), TraumaCategory {
            base_severity: 3.0,
            recovery_difficulty: 0.9,
            requires_professional_help_threshold: 1.0,
            common_triggers: vec!["combat_sounds".to_string(), "blood".to_string(), "memorial_items".to_string()],
            therapeutic_approaches: vec!["grief_counseling".to_string(), "narrative_therapy".to_string()],
        });
        
        trauma_categories.insert("betrayal".to_string(), TraumaCategory {
            base_severity: 2.0,
            recovery_difficulty: 0.7,
            requires_professional_help_threshold: 1.2,
            common_triggers: vec!["trust_situations".to_string(), "secret_keeping".to_string(), "loyalty_tests".to_string()],
            therapeutic_approaches: vec!["trust_rebuilding".to_string(), "cognitive_therapy".to_string()],
        });
        
        trauma_categories.insert("corruption_exposure".to_string(), TraumaCategory {
            base_severity: 1.8,
            recovery_difficulty: 0.6,
            requires_professional_help_threshold: 2.0,
            common_triggers: vec!["dark_magic".to_string(), "corruption_visuals".to_string(), "void_sounds".to_string()],
            therapeutic_approaches: vec!["purification_rituals".to_string(), "light_therapy".to_string()],
        });
        
        let mut breaking_point_modifiers = HashMap::new();
        breaking_point_modifiers.insert("einar".to_string(), 1.2); // More resilient warrior
        breaking_point_modifiers.insert("mira".to_string(), 0.8);  // More sensitive healer
        breaking_point_modifiers.insert("sorin".to_string(), 1.0); // Balanced rogue
        breaking_point_modifiers.insert("tamara".to_string(), 0.9); // Scholarly but fragile
        
        let memory_palace_room_types = vec![
            MemoryRoomTemplate {
                room_type: "safe_haven".to_string(),
                emotional_themes: vec!["peace".to_string(), "comfort".to_string(), "security".to_string()],
                healing_potential: 0.8,
                accessibility_requirements: vec!["basic_trust".to_string()],
                therapeutic_activities: vec!["relaxation".to_string(), "positive_memory_reinforcement".to_string()],
            },
            MemoryRoomTemplate {
                room_type: "trauma_processing".to_string(),
                emotional_themes: vec!["fear".to_string(), "pain".to_string(), "healing".to_string()],
                healing_potential: 1.0,
                accessibility_requirements: vec!["therapeutic_bond > 0.5".to_string(), "professional_support".to_string()],
                therapeutic_activities: vec!["trauma_reprocessing".to_string(), "narrative_reconstruction".to_string()],
            },
            MemoryRoomTemplate {
                room_type: "integration_space".to_string(),
                emotional_themes: vec!["growth".to_string(), "wisdom".to_string(), "strength".to_string()],
                healing_potential: 0.9,
                accessibility_requirements: vec!["significant_healing_progress".to_string()],
                therapeutic_activities: vec!["meaning_making".to_string(), "post_traumatic_growth".to_string()],
            },
        ];
        
        let healing_symbol_library = vec![
            HealingSymbolTemplate {
                symbol_type: "light_orb".to_string(),
                healing_categories: vec!["corruption_exposure".to_string(), "dark_magic_trauma".to_string()],
                activation_requirements: vec!["light_magic_affinity".to_string()],
                healing_power: 0.7,
                visual_description: "A gentle, warm orb of pure light that dispels darkness".to_string(),
            },
            HealingSymbolTemplate {
                symbol_type: "memory_tree".to_string(),
                healing_categories: vec!["companion_death".to_string(), "loss_trauma".to_string()],
                activation_requirements: vec!["positive_memories_accessible".to_string()],
                healing_power: 0.8,
                visual_description: "A tree bearing fruits that contain cherished memories".to_string(),
            },
            HealingSymbolTemplate {
                symbol_type: "trust_bridge".to_string(),
                healing_categories: vec!["betrayal".to_string(), "trust_trauma".to_string()],
                activation_requirements: vec!["therapeutic_bond > 0.7".to_string()],
                healing_power: 0.9,
                visual_description: "A bridge of golden light spanning a chasm of doubt".to_string(),
            },
        ];
        
        let mut professional_support_multipliers = HashMap::new();
        professional_support_multipliers.insert("folk_healer".to_string(), 1.5);
        professional_support_multipliers.insert("religious_counselor".to_string(), 2.0);
        professional_support_multipliers.insert("medical_professional".to_string(), 3.0);
        professional_support_multipliers.insert("specialized_trauma_therapist".to_string(), 4.0);
        
        let mut companion_support_synergies = HashMap::new();
        companion_support_synergies.insert("einar".to_string(), vec!["mira".to_string(), "sorin".to_string()]); // Warrior helps others
        companion_support_synergies.insert("mira".to_string(), vec!["einar".to_string(), "tamara".to_string()]); // Healer helps all
        companion_support_synergies.insert("sorin".to_string(), vec!["tamara".to_string()]); // Rogue protects scholar
        companion_support_synergies.insert("tamara".to_string(), vec!["mira".to_string()]); // Scholar relates to healer
        
        Self {
            trauma_categories,
            breaking_point_modifiers,
            recovery_base_rates: HashMap::new(),
            therapy_effectiveness_matrix: HashMap::new(),
            memory_palace_room_types,
            healing_symbol_library,
            professional_support_multipliers,
            dread_level_trauma_amplifiers: [1.0, 1.2, 1.5, 2.0, 3.0], // Trauma amplified by dread
            companion_support_synergies,
        }
    }
}

/// Memory palace resource for managing 3D therapeutic spaces
#[derive(Resource, Debug)]
pub struct MemoryPalaceManager {
    pub active_palaces: HashMap<Uuid, MemoryPalaceInstance>,
    pub room_templates: HashMap<String, MemoryRoomTemplate>,
    pub healing_symbols: HashMap<String, HealingSymbolTemplate>,
    pub palace_generation_seeds: HashMap<Uuid, u64>, // For deterministic generation
}

#[derive(Debug, Clone)]
pub struct MemoryPalaceInstance {
    pub companion_id: Uuid,
    pub palace_entity: Entity,
    pub rooms: HashMap<String, MemoryRoomInstance>,
    pub current_room: Option<String>,
    pub exploration_progress: f32,
    pub healing_progress: HashMap<String, f32>, // trauma_type -> healing_progress
    pub accessible_areas: Vec<String>,
    pub locked_areas: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MemoryRoomInstance {
    pub room_id: String,
    pub room_type: String,
    pub trauma_objects: Vec<TraumaObjectInstance>,
    pub healing_symbols: Vec<HealingSymbolInstance>,
    pub safe_zones: Vec<SafeZoneInstance>,
    pub accessibility_score: f32,
    pub therapeutic_value_realized: f32,
    pub visit_count: i32,
    pub last_visited: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct TraumaObjectInstance {
    pub object_id: String,
    pub trauma_source_id: String,
    pub current_emotional_charge: f32,
    pub transformation_progress: f32,
    pub interaction_history: Vec<TraumaInteraction>,
    pub can_be_transformed: bool,
}

#[derive(Debug, Clone)]
pub struct TraumaInteraction {
    pub interaction_type: String,
    pub timestamp: i64,
    pub effectiveness: f32,
    pub insights_gained: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HealingSymbolInstance {
    pub symbol_id: String,
    pub symbol_type: String,
    pub current_power: f32,
    pub activation_count: i32,
    pub last_activated: Option<i64>,
    pub available_for_activation: bool,
}

#[derive(Debug, Clone)]
pub struct SafeZoneInstance {
    pub zone_id: String,
    pub comfort_level: f32,
    pub recovery_effectiveness: f32,
    pub time_spent_here: f32, // Total time in seconds
    pub emotional_associations: Vec<String>,
}

impl MemoryPalaceManager {
    pub fn new(config: &TherapyConfiguration) -> Self {
        let mut room_templates = HashMap::new();
        for template in &config.memory_palace_room_types {
            room_templates.insert(template.room_type.clone(), template.clone());
        }
        
        let mut healing_symbols = HashMap::new();
        for symbol in &config.healing_symbol_library {
            healing_symbols.insert(symbol.symbol_type.clone(), symbol.clone());
        }
        
        Self {
            active_palaces: HashMap::new(),
            room_templates,
            healing_symbols,
            palace_generation_seeds: HashMap::new(),
        }
    }
    
    /// Generate a memory palace for a companion based on their trauma history
    pub fn generate_memory_palace(&mut self, companion_id: Uuid, psychology_data: &CompanionPsychologyData) -> MemoryPalaceInstance {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Generate deterministic seed for this companion's palace
        let mut hasher = DefaultHasher::new();
        companion_id.hash(&mut hasher);
        let seed = hasher.finish();
        self.palace_generation_seeds.insert(companion_id, seed);
        
        let mut rng = fastrand::Rng::with_seed(seed);
        
        let mut palace = MemoryPalaceInstance {
            companion_id,
            palace_entity: Entity::PLACEHOLDER,
            rooms: HashMap::new(),
            current_room: None,
            exploration_progress: 0.0,
            healing_progress: HashMap::new(),
            accessible_areas: vec!["entrance".to_string()], // Always start with entrance
            locked_areas: Vec::new(),
        };
        
        // Generate rooms based on companion's trauma sources
        if let Some(psychology_model) = &psychology_data.psychology_model {
            // Parse trauma sources from JSON (this would be actual implementation)
            self.generate_trauma_specific_rooms(&mut palace, psychology_model, &mut rng);
        }
        
        // Always generate core therapeutic rooms
        self.generate_core_rooms(&mut palace, &mut rng);
        
        palace
    }
    
    fn generate_trauma_specific_rooms(&self, palace: &mut MemoryPalaceInstance, psychology_model: &psychology::Model, rng: &mut fastrand::Rng) {
        // This would parse the actual trauma data from the psychology model
        // and generate appropriate rooms for each trauma type
        
        // For now, generate based on common trauma patterns
        let trauma_types = vec!["dragon_encounter", "companion_death", "betrayal", "corruption_exposure"];
        
        for trauma_type in trauma_types {
            let room_id = format!("{}_processing_room", trauma_type);
            
            // Generate trauma objects for this room
            let trauma_objects = vec![
                TraumaObjectInstance {
                    object_id: format!("{}_memory_fragment", trauma_type),
                    trauma_source_id: trauma_type.to_string(),
                    current_emotional_charge: rng.f32() * 0.5 + 0.5, // 0.5 to 1.0
                    transformation_progress: 0.0,
                    interaction_history: Vec::new(),
                    can_be_transformed: true,
                }
            ];
            
            // Generate appropriate healing symbols
            let healing_symbols = self.generate_healing_symbols_for_trauma(trauma_type, rng);
            
            let room = MemoryRoomInstance {
                room_id: room_id.clone(),
                room_type: "trauma_processing".to_string(),
                trauma_objects,
                healing_symbols,
                safe_zones: vec![
                    SafeZoneInstance {
                        zone_id: format!("{}_safe_corner", room_id),
                        comfort_level: 0.7,
                        recovery_effectiveness: 0.6,
                        time_spent_here: 0.0,
                        emotional_associations: vec!["safety".to_string(), "progress".to_string()],
                    }
                ],
                accessibility_score: 0.3, // Trauma rooms start less accessible
                therapeutic_value_realized: 0.0,
                visit_count: 0,
                last_visited: None,
            };
            
            palace.rooms.insert(room_id.clone(), room);
            palace.locked_areas.push(room_id); // Start locked, require preparation
        }
    }
    
    fn generate_core_rooms(&self, palace: &mut MemoryPalaceInstance, rng: &mut fastrand::Rng) {
        // Generate entrance room
        let entrance_room = MemoryRoomInstance {
            room_id: "entrance".to_string(),
            room_type: "safe_haven".to_string(),
            trauma_objects: Vec::new(),
            healing_symbols: vec![
                HealingSymbolInstance {
                    symbol_id: "welcome_light".to_string(),
                    symbol_type: "light_orb".to_string(),
                    current_power: 0.5,
                    activation_count: 0,
                    last_activated: None,
                    available_for_activation: true,
                }
            ],
            safe_zones: vec![
                SafeZoneInstance {
                    zone_id: "entrance_sanctuary".to_string(),
                    comfort_level: 0.9,
                    recovery_effectiveness: 0.4,
                    time_spent_here: 0.0,
                    emotional_associations: vec!["beginning".to_string(), "hope".to_string(), "safety".to_string()],
                }
            ],
            accessibility_score: 1.0, // Always accessible
            therapeutic_value_realized: 0.0,
            visit_count: 0,
            last_visited: None,
        };
        
        palace.rooms.insert("entrance".to_string(), entrance_room);
        palace.current_room = Some("entrance".to_string());
        
        // Generate integration space for advanced healing
        let integration_room = MemoryRoomInstance {
            room_id: "integration_chamber".to_string(),
            room_type: "integration_space".to_string(),
            trauma_objects: Vec::new(),
            healing_symbols: vec![
                HealingSymbolInstance {
                    symbol_id: "wisdom_tree".to_string(),
                    symbol_type: "memory_tree".to_string(),
                    current_power: 1.0,
                    activation_count: 0,
                    last_activated: None,
                    available_for_activation: false, // Requires significant progress
                }
            ],
            safe_zones: vec![
                SafeZoneInstance {
                    zone_id: "integration_sanctuary".to_string(),
                    comfort_level: 1.0,
                    recovery_effectiveness: 0.9,
                    time_spent_here: 0.0,
                    emotional_associations: vec!["wholeness".to_string(), "growth".to_string(), "integration".to_string()],
                }
            ],
            accessibility_score: 0.0, // Only accessible after significant healing
            therapeutic_value_realized: 0.0,
            visit_count: 0,
            last_visited: None,
        };
        
        palace.rooms.insert("integration_chamber".to_string(), integration_room);
        palace.locked_areas.push("integration_chamber".to_string());
    }
    
    fn generate_healing_symbols_for_trauma(&self, trauma_type: &str, rng: &mut fastrand::Rng) -> Vec<HealingSymbolInstance> {
        let mut symbols = Vec::new();
        
        // Select appropriate healing symbols based on trauma type
        for (symbol_type, template) in &self.healing_symbols {
            if template.healing_categories.contains(&trauma_type.to_string()) {
                symbols.push(HealingSymbolInstance {
                    symbol_id: format!("{}_{}", symbol_type, rng.u32(0..1000)),
                    symbol_type: symbol_type.clone(),
                    current_power: template.healing_power,
                    activation_count: 0,
                    last_activated: None,
                    available_for_activation: false, // Requires therapeutic progress
                });
            }
        }
        
        symbols
    }
}

/// Therapy session management resource
#[derive(Resource, Debug)]
pub struct TherapySessionManager {
    pub active_sessions: HashMap<Uuid, ActiveTherapySession>,
    pub session_templates: HashMap<String, TherapySessionTemplate>,
    pub completed_sessions_cache: HashMap<Uuid, Vec<CompletedTherapySession>>,
}

#[derive(Debug, Clone)]
pub struct ActiveTherapySession {
    pub session_id: Uuid,
    pub companion_id: Uuid,
    pub session_entity: Entity,
    pub therapy_type: String,
    pub start_time: i64,
    pub planned_duration_minutes: f32,
    pub therapeutic_objectives: Vec<String>,
    pub progress_markers: Vec<SessionProgressMarker>,
    pub breakthrough_potential: f32,
    pub emotional_state_tracking: Vec<EmotionalStatePoint>,
}

#[derive(Debug, Clone)]
pub struct SessionProgressMarker {
    pub marker_type: String,
    pub timestamp: i64,
    pub progress_value: f32,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct EmotionalStatePoint {
    pub timestamp: i64,
    pub emotional_intensity: f32,
    pub emotional_valence: f32, // -1.0 (negative) to 1.0 (positive)
    pub specific_emotions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TherapySessionTemplate {
    pub session_type: String,
    pub recommended_duration_minutes: f32,
    pub prerequisite_conditions: Vec<String>,
    pub therapeutic_activities: Vec<TherapeuticActivity>,
    pub expected_outcomes: Vec<String>,
    pub breakthrough_indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TherapeuticActivity {
    pub activity_type: String,
    pub duration_minutes: f32,
    pub effectiveness_for_trauma_types: HashMap<String, f32>,
    pub required_materials: Vec<String>,
    pub player_actions_required: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CompletedTherapySession {
    pub session_id: Uuid,
    pub completion_timestamp: i64,
    pub emotional_progress: f32,
    pub insights_gained: Vec<String>,
    pub breakthrough_achieved: bool,
    pub next_session_recommendations: Vec<String>,
}
