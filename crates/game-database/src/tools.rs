//! Database tools for AI agents to query game data

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use database_orm::{hex_tiles, companions, encounters, dialogues, items};
use crate::error::DatabaseError;

/// Tool interface for AI agents to query the game database
pub struct DatabaseTool {
    connection: DatabaseConnection,
}

impl DatabaseTool {
    /// Create a new database tool
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
    
    /// Query hex tiles based on filters
    pub async fn query_hex_tiles(&self, filters: HashMap<String, Value>) -> Result<Value, DatabaseError> {
        let mut query = hex_tiles::Entity::find();
        
        // Apply filters
        if let Some(biome) = filters.get("biome").and_then(|v| v.as_str()) {
            query = query.filter(hex_tiles::Column::BiomeType.eq(biome));
        }
        
        if let Some(corruption) = filters.get("corruption_level").and_then(|v| v.as_i64()) {
            query = query.filter(hex_tiles::Column::CorruptionLevel.eq(corruption as i32));
        }
        
        let tiles = query
            .all(&self.connection)
            .await
            .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
        
        Ok(json!({
            "hex_tiles": tiles.iter().map(|t| json!({
                "id": t.id.to_string(),
                "q": t.q,
                "r": t.r,
                "biome": t.biome_type,
                "corruption": t.corruption_level,
            })).collect::<Vec<_>>(),
            "total": tiles.len()
        }))
    }
    
    /// Query companions with their current state
    pub async fn query_companions(&self, filters: HashMap<String, Value>) -> Result<Value, DatabaseError> {
        let mut query = companions::Entity::find();
        
        // Apply filters
        if let Some(min_loyalty) = filters.get("min_loyalty").and_then(|v| v.as_f64()) {
            query = query.filter(companions::Column::Loyalty.gte(min_loyalty as f32));
        }
        
        if let Some(is_present) = filters.get("is_present").and_then(|v| v.as_bool()) {
            query = query.filter(companions::Column::IsPresent.eq(is_present));
        }
        
        let companions = query
            .all(&self.connection)
            .await
            .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
        
        Ok(json!({
            "companions": companions.iter().map(|c| json!({
                "id": c.id.to_string(),
                "name": c.name,
                "loyalty": c.loyalty,
                "trauma_level": c.trauma_level,
                "is_present": c.is_present,
                "has_betrayed": c.has_betrayed,
            })).collect::<Vec<_>>(),
            "total": companions.len()
        }))
    }
    
    /// Query encounters
    pub async fn query_encounters(&self, filters: HashMap<String, Value>) -> Result<Value, DatabaseError> {
        let mut query = encounters::Entity::find();
        
        // Apply filters
        if let Some(encounter_type) = filters.get("encounter_type").and_then(|v| v.as_str()) {
            query = query.filter(encounters::Column::EncounterType.eq(encounter_type));
        }
        
        if let Some(min_dread) = filters.get("min_dread_level").and_then(|v| v.as_i64()) {
            query = query.filter(encounters::Column::MinDreadLevel.gte(min_dread as i32));
        }
        
        let encounters = query
            .all(&self.connection)
            .await
            .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
        
        Ok(json!({
            "encounters": encounters.iter().map(|e| json!({
                "id": e.id.to_string(),
                "name": e.name,
                "type": e.encounter_type,
                "difficulty": e.difficulty,
                "min_dread_level": e.min_dread_level,
                "is_boss": e.is_boss,
            })).collect::<Vec<_>>(),
            "total": encounters.len()
        }))
    }
    
    /// Query dialogues
    pub async fn query_dialogues(&self, filters: HashMap<String, Value>) -> Result<Value, DatabaseError> {
        let mut query = dialogues::Entity::find();
        
        // Apply filters
        if let Some(character) = filters.get("character").and_then(|v| v.as_str()) {
            query = query.filter(dialogues::Column::CharacterName.eq(character));
        }
        
        if let Some(dread) = filters.get("dread_level").and_then(|v| v.as_i64()) {
            query = query.filter(dialogues::Column::DreadLevel.eq(dread as i32));
        }
        
        let dialogues = query
            .all(&self.connection)
            .await
            .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
        
        Ok(json!({
            "dialogues": dialogues.iter().map(|d| json!({
                "id": d.id.to_string(),
                "character": d.character_name,
                "dread_level": d.dread_level,
                "trauma_variant": d.trauma_variant,
                "is_branching": d.is_branching,
            })).collect::<Vec<_>>(),
            "total": dialogues.len()
        }))
    }
    
    /// Query items
    pub async fn query_items(&self, filters: HashMap<String, Value>) -> Result<Value, DatabaseError> {
        let mut query = items::Entity::find();
        
        // Apply filters
        if let Some(item_type) = filters.get("item_type").and_then(|v| v.as_str()) {
            query = query.filter(items::Column::ItemType.eq(item_type));
        }
        
        if let Some(rarity) = filters.get("rarity").and_then(|v| v.as_str()) {
            query = query.filter(items::Column::Rarity.eq(rarity));
        }
        
        let items = query
            .all(&self.connection)
            .await
            .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
        
        Ok(json!({
            "items": items.iter().map(|i| json!({
                "id": i.id.to_string(),
                "name": i.name,
                "type": i.item_type,
                "rarity": i.rarity,
                "corruption_level": i.corruption_level,
                "is_cursed": i.is_cursed,
            })).collect::<Vec<_>>(),
            "total": items.len()
        }))
    }
    
    /// Get horror progression information
    pub async fn get_horror_progression(&self, dread_level: u8) -> Result<Value, DatabaseError> {
        // This would query various tables to get comprehensive horror state
        let progression_data = match dread_level {
            0 => json!({
                "stage": "Peace",
                "world_corruption": 0.0,
                "companion_trauma_average": 0.0,
                "available_encounters": ["tutorial", "peaceful_quest"],
                "dialogue_tone": "cheerful",
                "item_corruption": 0.0,
            }),
            1 => json!({
                "stage": "Unease",
                "world_corruption": 0.15,
                "companion_trauma_average": 0.1,
                "available_encounters": ["first_boss", "strange_event"],
                "dialogue_tone": "nervous",
                "item_corruption": 0.05,
            }),
            2 => json!({
                "stage": "Dread",
                "world_corruption": 0.40,
                "companion_trauma_average": 0.3,
                "available_encounters": ["moral_choice", "corruption_spread"],
                "dialogue_tone": "frightened",
                "item_corruption": 0.20,
            }),
            3 => json!({
                "stage": "Terror",
                "world_corruption": 0.70,
                "companion_trauma_average": 0.6,
                "available_encounters": ["companion_betrayal", "reality_break"],
                "dialogue_tone": "desperate",
                "item_corruption": 0.50,
            }),
            4 => json!({
                "stage": "Horror",
                "world_corruption": 1.0,
                "companion_trauma_average": 0.9,
                "available_encounters": ["dragon_encounter", "labyrinth"],
                "dialogue_tone": "broken",
                "item_corruption": 0.90,
            }),
            _ => json!({ "error": "Invalid dread level" })
        };
        
        Ok(progression_data)
    }
}

/// Parameters for database queries
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseQueryParams {
    pub query_type: String,
    pub filters: Option<HashMap<String, Value>>,
}

/// Execute a database query based on type
pub async fn execute_database_query(
    tool: &DatabaseTool,
    params: DatabaseQueryParams,
) -> Result<Value, DatabaseError> {
    let filters = params.filters.unwrap_or_default();
    
    match params.query_type.as_str() {
        "hex_tiles" => tool.query_hex_tiles(filters).await,
        "companions" => tool.query_companions(filters).await,
        "encounters" => tool.query_encounters(filters).await,
        "dialogues" => tool.query_dialogues(filters).await,
        "items" => tool.query_items(filters).await,
        _ => Err(DatabaseError::InvalidQuery(format!("Unknown query type: {}", params.query_type))),
    }
}
