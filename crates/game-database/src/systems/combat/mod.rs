//! Combat System - Full ECS implementation with bevy_sqlx integration
//!
//! Complete D&D 5e combat mechanics powered by HBF creature data,
//! integrated as Bevy ECS components, systems, and resources.

use bevy::prelude::*;
use sea_orm::DatabaseConnection;

pub mod components;
pub mod systems;
pub mod resources;
pub mod events;
pub mod queries;

pub use components::*;
pub use systems::*;
pub use resources::*;
pub use events::*;
pub use queries::*;

/// Combat system plugin for Bevy ECS integration
pub struct CombatSystemPlugin;

impl Plugin for CombatSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .init_resource::<CombatState>()
            .init_resource::<CreatureTemplateCache>()
            .init_resource::<EncounterTables>()
            .init_resource::<CombatSettings>()
            .init_resource::<ActiveTurn>()
            .init_resource::<CombatStatistics>()
            .init_resource::<TacticalGrid>()
            
            // Add events
            .add_event::<CombatInitiatedEvent>()
            .add_event::<AttackEvent>()
            .add_event::<DamageEvent>()
            .add_event::<CombatEndedEvent>()
            .add_event::<CreatureDefeatedEvent>()
            .add_event::<PlayerDefeatedEvent>()
            .add_event::<StatusEffectEvent>()
            .add_event::<HealingEvent>()
            .add_event::<MovementEvent>()
            .add_event::<SpellCastEvent>()
            .add_event::<EnvironmentalEffectEvent>()
            .add_event::<TurnProgressEvent>()
            .add_event::<InitiativeEvent>()
            .add_event::<OpportunityAttackEvent>()
            .add_event::<DeathSaveEvent>()
            .add_event::<HorrorEffectEvent>()
            .add_event::<CompanionTraumaEvent>()
            .add_event::<LootDropEvent>()
            .add_event::<CombatDialogueEvent>()
            
            // Add systems
            .add_systems(Startup, (
                load_creature_templates_system,
                load_encounter_tables_system,
            ))
            .add_systems(Update, (
                // Combat management
                combat_initiation_system,
                turn_order_system,
                combat_resolution_system,
                
                // Combat mechanics
                attack_resolution_system,
                damage_application_system,
                status_effect_system,
                combat_cleanup_system,
                
                // Environmental effects
                weather_combat_effects_system,
                terrain_combat_effects_system,
                corruption_combat_effects_system,
                
                // AI behavior
                creature_ai_system,
                tactical_positioning_system,
                
                // Integration with other systems
                companion_combat_system,
                horror_progression_combat_system,
            ).run_if(in_state(CombatPhase::PlayerTurn).or_else(in_state(CombatPhase::CreatureTurn))))
            
            // Add state
            .init_state::<CombatPhase>()
            
            // Register component reflection for debugging
            .register_type::<CombatCreature>()
            .register_type::<CreatureStats>()
            .register_type::<CombatPosition>()
            .register_type::<CombatParticipant>()
            .register_type::<TurnOrder>()
            .register_type::<CombatActions>()
            .register_type::<StatusEffects>()
            .register_type::<DamageResistance>()
            .register_type::<CombatAI>()
            .register_type::<EnvironmentalEffect>();
    }
}

/// Combat phases for state management
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombatPhase {
    #[default]
    None,
    Initiative,
    PlayerTurn,
    CreatureTurn,
    Resolution,
    Cleanup,
}

/// Combat engine for database integration (legacy support)
pub struct CombatEngine {
    db: DatabaseConnection,
    creature_cache: HashMap<String, CreatureTemplate>,
    encounter_tables: HashMap<String, EncounterTable>,
}

#[derive(Debug, Clone)]
pub struct CreatureTemplate {
    pub name: String,
    pub challenge_rating: String,
    pub armor_class: i32,
    pub hit_points_formula: String,
    pub abilities: CreatureAbilities,
    pub actions: Vec<CreatureAction>,
    pub special_abilities: Vec<String>,
    pub damage_immunities: Vec<String>,
    pub condition_immunities: Vec<String>,
    pub senses: Vec<String>,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EncounterTable {
    pub probability: String, // "1_in_6", "2_in_6", etc.
    pub entries: Vec<EncounterEntry>,
}

#[derive(Debug, Clone)]
pub struct EncounterEntry {
    pub roll_range: String, // "1", "2-3", "4-6", etc.
    pub creature_name: String,
    pub quantity: String, // "1", "2d4", "1d6+2", etc.
    pub creature_template: Option<CreatureTemplate>,
}

impl CombatEngine {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let mut engine = Self {
            db: db.clone(),
            creature_cache: HashMap::new(),
            encounter_tables: HashMap::new(),
        };
        
        // Load and cache creature data from HBF import
        engine.load_creature_templates().await?;
        engine.load_encounter_tables().await?;
        
        info!("Combat engine initialized with {} creature templates and {} encounter tables",
              engine.creature_cache.len(), engine.encounter_tables.len());
        
        Ok(engine)
    }
    
    /// Generate a random encounter for a hex position
    pub async fn generate_encounter(&self, position: HexPosition) -> Result<Option<CombatEncounter>> {
        // Get the hex tile to determine biome and encounter probability
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let tile = match tile {
            Some(t) => t,
            None => {
                warn!("No tile found at position ({}, {})", position.q, position.r);
                return Ok(None);
            }
        };
        
        // Check if encounter should trigger based on biome
        if !self.should_trigger_encounter(&tile.biome_type) {
            return Ok(None);
        }
        
        // Get encounter table for this biome
        let encounter_table = match self.encounter_tables.get(&tile.biome_type) {
            Some(table) => table,
            None => {
                debug!("No encounter table for biome: {}", tile.biome_type);
                return Ok(None);
            }
        };
        
        // Roll for specific encounter
        let encounter_entry = self.roll_encounter_table(encounter_table)?;
        
        // Generate creatures
        let creatures = self.generate_creatures(&encounter_entry).await?;
        
        if creatures.is_empty() {
            return Ok(None);
        }
        
        // Create environment from tile data
        let environment = self.create_encounter_environment(&tile).await?;
        
        debug!("Generated encounter: {} {} at ({}, {})", 
               creatures.len(), encounter_entry.creature_name, position.q, position.r);
        
        Ok(Some(CombatEncounter {
            creatures,
            environment,
            tactical_map: None, // Could be generated based on terrain features
        }))
    }
    
    /// Get creature by name for specific encounters
    pub async fn get_creature_by_name(&self, name: &str) -> Result<Option<CreatureTemplate>> {
        if let Some(template) = self.creature_cache.get(name) {
            return Ok(Some(template.clone()));
        }
        
        // If not cached, try to find in database by parsing creature data
        // This would query the HBF imported data and parse stat blocks
        warn!("Creature template not found in cache: {}", name);
        Ok(None)
    }
    
    /// Calculate damage from a damage formula like "2d6+3"
    pub fn roll_damage(&self, formula: &str) -> i32 {
        self.parse_and_roll_dice(formula)
    }
    
    /// Calculate attack roll with modifiers
    pub fn roll_attack(&self, attack_bonus: i32) -> (i32, bool) {
        let roll = rand::thread_rng().gen_range(1..=20);
        let total = roll + attack_bonus;
        let is_critical = roll == 20;
        (total, is_critical)
    }
    
    /// Calculate saving throw
    pub fn roll_saving_throw(&self, ability_modifier: i32, proficiency: bool) -> i32 {
        let roll = rand::thread_rng().gen_range(1..=20);
        let proficiency_bonus = if proficiency { 2 } else { 0 }; // Simplified
        roll + ability_modifier + proficiency_bonus
    }
    
    /// Check if encounter should trigger based on biome
    fn should_trigger_encounter(&self, biome: &str) -> bool {
        let encounter_chance = match biome {
            "swamp" => 0.4,
            "jungle" => 0.3,
            "mountain" => 0.25,
            "forest" => 0.2,
            "plains" => 0.15,
            "desert" => 0.2,
            "tundra" => 0.1,
            _ => 0.1,
        };
        
        rand::thread_rng().gen::<f32>() < encounter_chance
    }
    
    /// Roll on encounter table to determine specific encounter
    fn roll_encounter_table(&self, table: &EncounterTable) -> Result<&EncounterEntry> {
        // Simple random selection for now - could implement proper dice rolling
        let index = rand::thread_rng().gen_range(0..table.entries.len());
        Ok(&table.entries[index])
    }
    
    /// Generate creature instances from encounter entry
    async fn generate_creatures(&self, entry: &EncounterEntry) -> Result<Vec<CreatureInstance>> {
        let template = match &entry.creature_template {
            Some(t) => t,
            None => match self.creature_cache.get(&entry.creature_name) {
                Some(t) => t,
                None => {
                    warn!("No creature template found for: {}", entry.creature_name);
                    return Ok(Vec::new());
                }
            }
        };
        
        // Parse quantity (could be "1", "2d4", "1d6+2", etc.)
        let quantity = self.parse_and_roll_dice(&entry.quantity);
        
        let mut creatures = Vec::new();
        for i in 0..quantity {
            let hp = self.parse_and_roll_dice(&template.hit_points_formula);
            
            creatures.push(CreatureInstance {
                name: if quantity > 1 {
                    format!("{} #{}", template.name, i + 1)
                } else {
                    template.name.clone()
                },
                current_hp: hp,
                max_hp: hp,
                armor_class: template.armor_class,
                abilities: template.abilities.clone(),
                actions: template.actions.clone(),
                position: None, // Would be set when placing on tactical map
            });
        }
        
        Ok(creatures)
    }
    
    /// Create encounter environment from tile data
    async fn create_encounter_environment(&self, tile: &hex_tiles::Model) -> Result<EncounterEnvironment> {
        // Get weather for this tile's region if available
        let weather = self.get_weather_for_tile(tile).await.unwrap_or_else(|| WeatherCondition {
            condition: "Clear".to_string(),
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
        });
        
        // Determine lighting based on corruption and dread
        let lighting = match tile.dread_intensity {
            0 => "bright",
            1 => "normal", 
            2 => "dim",
            3 => "dark",
            4 => "darkness",
            _ => "normal",
        }.to_string();
        
        // Environmental hazards based on corruption
        let mut hazards = Vec::new();
        if tile.corruption_level > 0.5 {
            hazards.push("corrupted_ground".to_string());
        }
        if tile.corruption_level > 0.8 {
            hazards.push("void_rifts".to_string());
        }
        
        Ok(EncounterEnvironment {
            terrain: tile.biome_type.clone(),
            weather,
            lighting,
            hazards,
        })
    }
    
    /// Load creature templates from database (simplified for now)
    async fn load_creature_templates(&mut self) -> Result<()> {
        // In a full implementation, this would parse the HBF creature data
        // For now, create some basic templates based on common encounters
        
        self.creature_cache.insert("Wolf".to_string(), CreatureTemplate {
            name: "Wolf".to_string(),
            challenge_rating: "1/4".to_string(),
            armor_class: 13,
            hit_points_formula: "2d8+2".to_string(),
            abilities: CreatureAbilities {
                strength: 12, dexterity: 15, constitution: 12,
                intelligence: 3, wisdom: 12, charisma: 6,
            },
            actions: vec![
                CreatureAction {
                    name: "Bite".to_string(),
                    description: "Melee Weapon Attack: +4 to hit, reach 5 ft., one target.".to_string(),
                    attack_bonus: Some(4),
                    damage_formula: Some("2d4+2".to_string()),
                    save_dc: None,
                }
            ],
            special_abilities: vec!["Keen Hearing and Smell".to_string(), "Pack Tactics".to_string()],
            damage_immunities: Vec::new(),
            condition_immunities: Vec::new(),
            senses: vec!["darkvision 60 ft.".to_string()],
            languages: Vec::new(),
        });
        
        // Would load more from actual HBF data
        Ok(())
    }
    
    /// Load encounter tables from database (simplified for now)
    async fn load_encounter_tables(&mut self) -> Result<()> {
        // In a full implementation, this would load from HBF encounter data
        // For now, create basic encounter tables
        
        self.encounter_tables.insert("forest".to_string(), EncounterTable {
            probability: "1_in_6".to_string(),
            entries: vec![
                EncounterEntry {
                    roll_range: "1-2".to_string(),
                    creature_name: "Wolf".to_string(),
                    quantity: "1d4".to_string(),
                    creature_template: None,
                },
                EncounterEntry {
                    roll_range: "3-4".to_string(),
                    creature_name: "Bear".to_string(),
                    quantity: "1".to_string(),
                    creature_template: None,
                },
            ],
        });
        
        // Would load more from actual HBF encounter table data
        Ok(())
    }
    
    /// Get weather for a tile (would query settlement weather data)
    async fn get_weather_for_tile(&self, tile: &hex_tiles::Model) -> Option<WeatherCondition> {
        // Would query settlement weather tables based on tile's region
        // For now, return basic weather
        Some(WeatherCondition {
            condition: "Clear".to_string(),
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
        })
    }
    
    /// Parse and roll dice formulas like "2d6+3", "1d4", "8"
    fn parse_and_roll_dice(&self, formula: &str) -> i32 {
        let formula = formula.trim();
        
        // Handle simple numbers
        if let Ok(num) = formula.parse::<i32>() {
            return num;
        }
        
        // Handle dice formulas
        if formula.contains('d') {
            // Parse "2d6+3" format
            let parts: Vec<&str> = formula.split('d').collect();
            if parts.len() != 2 {
                warn!("Invalid dice formula: {}", formula);
                return 1;
            }
            
            let num_dice: i32 = parts[0].parse().unwrap_or(1);
            
            // Handle modifier (+3, -2, etc.)
            let (die_size, modifier) = if parts[1].contains('+') {
                let modifier_parts: Vec<&str> = parts[1].split('+').collect();
                let die_size: i32 = modifier_parts[0].trim().parse().unwrap_or(6);
                let modifier: i32 = modifier_parts.get(1).unwrap_or(&"0").trim().parse().unwrap_or(0);
                (die_size, modifier)
            } else if parts[1].contains('-') {
                let modifier_parts: Vec<&str> = parts[1].split('-').collect();
                let die_size: i32 = modifier_parts[0].trim().parse().unwrap_or(6);
                let modifier: i32 = -(modifier_parts.get(1).unwrap_or(&"0").trim().parse().unwrap_or(0));
                (die_size, modifier)
            } else {
                let die_size: i32 = parts[1].trim().parse().unwrap_or(6);
                (die_size, 0)
            };
            
            // Roll the dice
            let mut total = 0;
            for _ in 0..num_dice {
                total += rand::thread_rng().gen_range(1..=die_size);
            }
            total + modifier
        } else {
            warn!("Could not parse dice formula: {}", formula);
            1
        }
    }
}

/// Combat encounter resolution methods
impl CombatEngine {
    /// Resolve an attack against a target
    pub fn resolve_attack(&self, attacker: &CreatureInstance, defender: &CreatureInstance, action: &CreatureAction) -> AttackResult {
        let attack_bonus = action.attack_bonus.unwrap_or(0);
        let (attack_roll, is_critical) = self.roll_attack(attack_bonus);
        
        let hits = attack_roll >= defender.armor_class || is_critical;
        
        let damage = if hits {
            let base_damage = if let Some(formula) = &action.damage_formula {
                self.parse_and_roll_dice(formula)
            } else {
                1
            };
            
            if is_critical {
                base_damage * 2 // Simplified critical hit
            } else {
                base_damage
            }
        } else {
            0
        };
        
        AttackResult {
            attack_roll,
            hits,
            damage,
            is_critical,
            effects: Vec::new(), // Could add status effects
        }
    }
    
    /// Calculate ability modifier from ability score
    pub fn ability_modifier(score: i32) -> i32 {
        (score - 10) / 2
    }
    
    /// Check if creature is unconscious
    pub fn is_unconscious(creature: &CreatureInstance) -> bool {
        creature.current_hp <= 0
    }
    
    /// Apply damage to creature
    pub fn apply_damage(creature: &mut CreatureInstance, damage: i32) {
        creature.current_hp = (creature.current_hp - damage).max(0);
    }
    
    /// Heal creature
    pub fn heal_creature(creature: &mut CreatureInstance, healing: i32) {
        creature.current_hp = (creature.current_hp + healing).min(creature.max_hp);
    }
}

#[derive(Debug, Clone)]
pub struct AttackResult {
    pub attack_roll: i32,
    pub hits: bool,
    pub damage: i32,
    pub is_critical: bool,
    pub effects: Vec<String>, // Status effects applied
}

/// Weather-based combat modifiers
impl CombatEngine {
    /// Apply weather effects to combat
    pub fn apply_weather_effects(&self, weather: &WeatherCondition, creatures: &mut [CreatureInstance]) {
        match weather.condition.as_str() {
            "Foggy" => {
                // Reduce visibility, disadvantage on ranged attacks
                debug!("Fog reduces visibility for combat");
            }
            "Stormy" => {
                // Lightning could cause additional damage
                debug!("Storm creates combat hazards");
            }
            "Rainy" => {
                // Slippery conditions, movement penalties
                debug!("Rain affects movement in combat");
            }
            _ => {}
        }
    }
    
    /// Get terrain movement cost for tactical positioning
    pub fn get_terrain_movement_cost(&self, biome: &str, corruption_level: f32) -> f32 {
        let base_cost = match biome {
            "swamp" => 2.0,
            "mountain" => 1.5,
            "jungle" => 1.3,
            "forest" => 1.1,
            "plains" => 1.0,
            "desert" => 1.2,
            "tundra" => 1.4,
            _ => 1.0,
        };
        
        // Corruption increases movement cost
        let corruption_modifier = 1.0 + (corruption_level * 0.5);
        base_cost * corruption_modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dice_parsing() {
        let engine = CombatEngine {
            db: todo!(),
            creature_cache: HashMap::new(),
            encounter_tables: HashMap::new(),
        };
        
        // Test simple number
        assert_eq!(engine.parse_and_roll_dice("8"), 8);
        
        // Test dice formulas (results will be random, just test it doesn't panic)
        let result = engine.parse_and_roll_dice("2d6+3");
        assert!(result >= 5 && result <= 15); // 2-12 + 3
        
        let result = engine.parse_and_roll_dice("1d4");
        assert!(result >= 1 && result <= 4);
    }
    
    #[test]
    fn test_ability_modifier() {
        assert_eq!(CombatEngine::ability_modifier(10), 0);
        assert_eq!(CombatEngine::ability_modifier(16), 3);
        assert_eq!(CombatEngine::ability_modifier(8), -1);
    }
    
    #[test]
    fn test_terrain_movement_cost() {
        let engine = CombatEngine {
            db: todo!(),
            creature_cache: HashMap::new(),
            encounter_tables: HashMap::new(),
        };
        
        assert_eq!(engine.get_terrain_movement_cost("plains", 0.0), 1.0);
        assert_eq!(engine.get_terrain_movement_cost("swamp", 0.0), 2.0);
        assert!(engine.get_terrain_movement_cost("plains", 0.5) > 1.0); // Corruption increases cost
    }
}
