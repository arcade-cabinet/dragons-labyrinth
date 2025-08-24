//! Encounter Spawning - Database-driven encounter probability and generation
//!
//! This system uses HBF encounter tables to generate context-appropriate
//! encounters based on biome, corruption level, and time of day.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info};
use rand::Rng;
use super::{HexPosition, CombatEncounter};

pub struct EncounterSpawning {
    db: DatabaseConnection,
    encounter_probabilities: HashMap<String, f32>, // biome -> encounter chance
    time_modifiers: HashMap<String, f32>, // time_of_day -> modifier
}

impl EncounterSpawning {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let encounter_probabilities = Self::create_encounter_probabilities();
        let time_modifiers = Self::create_time_modifiers();
        
        info!("Encounter spawning initialized with {} biome probabilities and {} time modifiers",
              encounter_probabilities.len(), time_modifiers.len());
        
        Ok(Self {
            db: db.clone(),
            encounter_probabilities,
            time_modifiers,
        })
    }
    
    /// Check if encounter should spawn at location
    pub async fn check_encounter_spawn(&self, position: HexPosition, time_of_day: &str, is_camping: bool) -> Result<bool> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let tile = match tile {
            Some(t) => t,
            None => return Ok(false),
        };
        
        // Base encounter chance from biome
        let base_chance = self.encounter_probabilities
            .get(&tile.biome_type)
            .unwrap_or(&0.1);
        
        // Time of day modifier
        let time_modifier = self.time_modifiers
            .get(time_of_day)
            .unwrap_or(&1.0);
        
        // Camping increases encounter chance
        let camping_modifier = if is_camping { 2.0 } else { 1.0 };
        
        // Corruption increases encounter chance
        let corruption_modifier = 1.0 + (tile.corruption_level * 2.0);
        
        let final_chance = base_chance * time_modifier * camping_modifier * corruption_modifier;
        
        let roll = rand::thread_rng().gen::<f32>();
        let encounter_occurs = roll < final_chance;
        
        if encounter_occurs {
            debug!("Encounter triggered at ({}, {}) - chance: {:.2}, roll: {:.2}", 
                   position.q, position.r, final_chance, roll);
        }
        
        Ok(encounter_occurs)
    }
    
    /// Generate encounter based on location and conditions
    pub async fn generate_location_encounter(&self, position: HexPosition, encounter_type: EncounterType) -> Result<Option<CombatEncounter>> {
        // This would integrate with the combat engine to generate actual encounters
        // For now, placeholder implementation
        debug!("Generating {:?} encounter at ({}, {})", encounter_type, position.q, position.r);
        Ok(None)
    }
    
    /// Get encounter probability for planning purposes
    pub async fn get_encounter_probability(&self, position: HexPosition, time_of_day: &str) -> Result<f32> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let tile = match tile {
            Some(t) => t,
            None => return Ok(0.0),
        };
        
        let base_chance = self.encounter_probabilities
            .get(&tile.biome_type)
            .unwrap_or(&0.1);
        
        let time_modifier = self.time_modifiers
            .get(time_of_day)
            .unwrap_or(&1.0);
        
        let corruption_modifier = 1.0 + (tile.corruption_level * 2.0);
        
        Ok(base_chance * time_modifier * corruption_modifier)
    }
    
    fn create_encounter_probabilities() -> HashMap<String, f32> {
        let mut probs = HashMap::new();
        probs.insert("swamp".to_string(), 0.4);
        probs.insert("jungle".to_string(), 0.3);
        probs.insert("mountain".to_string(), 0.25);
        probs.insert("forest".to_string(), 0.2);
        probs.insert("desert".to_string(), 0.2);
        probs.insert("plains".to_string(), 0.15);
        probs.insert("tundra".to_string(), 0.1);
        probs.insert("coast".to_string(), 0.15);
        probs.insert("ocean".to_string(), 0.05);
        probs
    }
    
    fn create_time_modifiers() -> HashMap<String, f32> {
        let mut modifiers = HashMap::new();
        modifiers.insert("dawn".to_string(), 1.2);
        modifiers.insert("morning".to_string(), 0.8);
        modifiers.insert("midday".to_string(), 0.6);
        modifiers.insert("afternoon".to_string(), 0.8);
        modifiers.insert("dusk".to_string(), 1.5);
        modifiers.insert("night".to_string(), 2.0);
        modifiers.insert("midnight".to_string(), 2.5);
        modifiers
    }
}

#[derive(Debug, Clone)]
pub enum EncounterType {
    Random,
    Ambush,
    Territorial,
    Hunting,
    Social,
}
