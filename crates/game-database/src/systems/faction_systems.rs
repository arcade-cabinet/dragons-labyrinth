//! Faction Systems - Database-driven politics, cults, militias, and guild mechanics
//!
//! This system extracts faction data from NPC relationships and settlement data
//! to create dynamic political systems using HBF-imported content.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;
use super::{HexPosition, FactionData};

pub struct FactionSystems {
    db: DatabaseConnection,
    faction_cache: HashMap<String, FactionData>,
    influence_map: HashMap<HexPosition, Vec<String>>, // hex -> factions with influence
}

impl FactionSystems {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let mut system = Self {
            db: db.clone(),
            faction_cache: HashMap::new(),
            influence_map: HashMap::new(),
        };
        
        // Extract faction data from settlements and NPCs
        system.extract_factions_from_data().await?;
        system.build_influence_map().await?;
        
        info!("Faction systems initialized with {} factions and {} influence zones",
              system.faction_cache.len(), system.influence_map.len());
        
        Ok(system)
    }
    
    /// Get all factions that have influence in an area
    pub async fn get_factions_in_area(&self, position: HexPosition, radius: u32) -> Result<Vec<FactionData>> {
        let mut area_factions = HashMap::new();
        
        // Check influence map for nearby hexes
        for q in (position.q - radius as i32)..=(position.q + radius as i32) {
            for r in (position.r - radius as i32)..=(position.r + radius as i32) {
                let check_pos = HexPosition::new(q, r);
                
                if let Some(faction_names) = self.influence_map.get(&check_pos) {
                    for faction_name in faction_names {
                        if let Some(faction_data) = self.faction_cache.get(faction_name) {
                            area_factions.insert(faction_name.clone(), faction_data.clone());
                        }
                    }
                }
            }
        }
        
        Ok(area_factions.into_values().collect())
    }
    
    /// Get faction relationships and conflicts
    pub async fn get_faction_relationships(&self, faction_name: &str) -> Result<HashMap<String, i32>> {
        if let Some(faction) = self.faction_cache.get(faction_name) {
            // Convert UUID relationships to name relationships
            let mut name_relationships = HashMap::new();
            
            for (other_faction_id, relationship) in &faction.relationships {
                // Find faction name by ID
                for (name, data) in &self.faction_cache {
                    if data.id == *other_faction_id {
                        name_relationships.insert(name.clone(), *relationship);
                        break;
                    }
                }
            }
            
            Ok(name_relationships)
        } else {
            Ok(HashMap::new())
        }
    }
    
    /// Check if player actions affect faction standing
    pub async fn update_faction_standing(&mut self, faction_name: &str, player_action: &str, impact: i32) -> Result<()> {
        debug!("Player action '{}' affects {} faction standing by {}", 
               player_action, faction_name, impact);
        
        // Update faction data based on player actions
        if let Some(faction) = self.faction_cache.get_mut(faction_name) {
            // Adjust faction resources or goals based on player impact
            faction.resources = (faction.resources + impact).max(0);
            
            // Log faction response to player actions
            info!("Faction {} now has {} resources (changed by {})", 
                  faction_name, faction.resources, impact);
        }
        
        Ok(())
    }
    
    /// Get faction quests or missions available to player
    pub async fn get_faction_missions(&self, faction_name: &str, player_reputation: i32) -> Result<Vec<FactionMission>> {
        let faction = match self.faction_cache.get(faction_name) {
            Some(f) => f,
            None => return Ok(Vec::new()),
        };
        
        let mut missions = Vec::new();
        
        // Generate missions based on faction type and goals
        match faction.faction_type.as_str() {
            "merchant_guild" => {
                if player_reputation >= 0 {
                    missions.push(FactionMission {
                        title: "Secure Trade Route".to_string(),
                        description: "Clear monsters from the trade path between settlements".to_string(),
                        reward_type: "gold".to_string(),
                        reward_amount: 100,
                        reputation_required: 0,
                        target_locations: faction.influence_areas.clone(),
                    });
                }
            }
            "temple_order" => {
                missions.push(FactionMission {
                    title: "Purify Corrupted Ground".to_string(),
                    description: "Use holy water to cleanse corrupted hexes".to_string(),
                    reward_type: "blessing".to_string(),
                    reward_amount: 1,
                    reputation_required: 5,
                    target_locations: self.get_corrupted_hexes().await?,
                });
            }
            "thieves_guild" => {
                if player_reputation >= -5 {
                    missions.push(FactionMission {
                        title: "Information Gathering".to_string(),
                        description: "Eavesdrop on conversations in rival settlements".to_string(),
                        reward_type: "information".to_string(),
                        reward_amount: 1,
                        reputation_required: -5,
                        target_locations: Vec::new(),
                    });
                }
            }
            _ => {
                // Generic faction mission
                missions.push(FactionMission {
                    title: "Aid the Cause".to_string(),
                    description: format!("Help the {} achieve their goals", faction.name),
                    reward_type: "reputation".to_string(),
                    reward_amount: 10,
                    reputation_required: 0,
                    target_locations: faction.influence_areas.clone(),
                });
            }
        }
        
        // Filter missions by reputation requirement
        missions.retain(|mission| player_reputation >= mission.reputation_required);
        
        Ok(missions)
    }
    
    /// Determine faction type from settlement and NPC data
    fn determine_faction_type(&self, settlement_type: &str, npc_roles: &[String]) -> String {
        // Analyze settlement type and NPC roles to infer faction type
        match settlement_type {
            "temple" => "temple_order",
            "shop" => "merchant_guild",
            _ => {
                // Look at NPC roles
                if npc_roles.iter().any(|role| role.contains("guard") || role.contains("soldier")) {
                    "militia"
                } else if npc_roles.iter().any(|role| role.contains("mystic") || role.contains("witch")) {
                    "cult"
                } else if npc_roles.iter().any(|role| role.contains("craftsperson") || role.contains("trader")) {
                    "merchant_guild"
                } else {
                    "local_authority"
                }
            }
        }.to_string()
    }
    
    /// Extract faction data from existing settlement and NPC data
    async fn extract_factions_from_data(&mut self) -> Result<()> {
        let settlements = settlements::Entity::find().all(&self.db).await?;
        
        for settlement in settlements {
            // Create faction for each settlement with explicit faction data
            if let Some(faction_name) = &settlement.faction {
                if !self.faction_cache.contains_key(faction_name) {
                    // Get NPCs in this settlement to determine faction details
                    let npcs = npcs::Entity::find()
                        .filter(npcs::Column::SettlementId.eq(settlement.id))
                        .all(&self.db)
                        .await?;
                    
                    let npc_roles: Vec<String> = npcs.iter().map(|npc| npc.role.clone()).collect();
                    let faction_type = self.determine_faction_type(&settlement.settlement_type, &npc_roles);
                    
                    let faction_data = FactionData {
                        id: Uuid::new_v4(),
                        name: faction_name.clone(),
                        faction_type,
                        influence_areas: vec![HexPosition::from_hbf_coords(
                            settlement.hbf_x.unwrap_or(0),
                            settlement.hbf_y.unwrap_or(0)
                        )],
                        relationships: HashMap::new(),
                        goals: self.generate_faction_goals(&settlement.settlement_type),
                        resources: settlement.prosperity_level * 10, // Convert prosperity to resources
                    };
                    
                    self.faction_cache.insert(faction_name.clone(), faction_data);
                    info!("Extracted faction: {} ({})", faction_name, settlement.settlement_type);
                }
            }
        }
        
        // Create relationships between factions
        self.generate_faction_relationships();
        
        Ok(())
    }
    
    async fn build_influence_map(&mut self) -> Result<()> {
        for (faction_name, faction_data) in &self.faction_cache {
            for position in &faction_data.influence_areas {
                self.influence_map
                    .entry(position.clone())
                    .or_insert_with(Vec::new)
                    .push(faction_name.clone());
            }
        }
        
        Ok(())
    }
    
    fn generate_faction_goals(&self, settlement_type: &str) -> Vec<String> {
        match settlement_type {
            "temple" => vec![
                "Spread the faith".to_string(),
                "Purify corrupted lands".to_string(),
                "Protect pilgrims".to_string(),
            ],
            "tavern" => vec![
                "Increase trade".to_string(),
                "Gather information".to_string(),
                "Maintain neutrality".to_string(),
            ],
            "shop" => vec![
                "Monopolize trade routes".to_string(),
                "Acquire rare goods".to_string(),
                "Expand merchant network".to_string(),
            ],
            _ => vec![
                "Protect territory".to_string(),
                "Increase influence".to_string(),
                "Maintain order".to_string(),
            ],
        }
    }
    
    fn generate_faction_relationships(&mut self) {
        let faction_names: Vec<String> = self.faction_cache.keys().cloned().collect();
        
        for (faction_name, faction_data) in &mut self.faction_cache {
            for other_faction_name in &faction_names {
                if faction_name != other_faction_name {
                    if let Some(other_faction) = self.faction_cache.get(other_faction_name) {
                        let relationship = self.calculate_faction_relationship(
                            &faction_data.faction_type,
                            &other_faction.faction_type
                        );
                        
                        faction_data.relationships.insert(other_faction.id, relationship);
                    }
                }
            }
        }
    }
    
    fn calculate_faction_relationship(&self, faction_type1: &str, faction_type2: &str) -> i32 {
        match (faction_type1, faction_type2) {
            ("temple_order", "cult") => -8, // Religious conflict
            ("merchant_guild", "thieves_guild") => -6, // Economic conflict
            ("militia", "thieves_guild") => -7, // Law vs crime
            ("temple_order", "merchant_guild") => 3, // Mutual benefit
            ("militia", "temple_order") => 5, // Law and order alliance
            ("merchant_guild", "merchant_guild") => -2, // Competition
            ("local_authority", _) => 0, // Neutral by default
            _ => 0, // Default neutral relationship
        }
    }
    
    async fn get_corrupted_hexes(&self) -> Result<Vec<HexPosition>> {
        let corrupted_tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::CorruptionLevel.gte(0.3))
            .all(&self.db)
            .await?;
        
        Ok(corrupted_tiles.into_iter()
            .map(|tile| HexPosition::new(tile.q, tile.r))
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct FactionMission {
    pub title: String,
    pub description: String,
    pub reward_type: String, // "gold", "reputation", "blessing", "information"
    pub reward_amount: i32,
    pub reputation_required: i32,
    pub target_locations: Vec<HexPosition>,
}

/// Faction influence and conflict resolution
impl FactionSystems {
    /// Check faction conflicts when player enters an area
    pub async fn check_faction_conflicts(&self, position: HexPosition) -> Result<Vec<FactionConflict>> {
        let area_factions = self.get_factions_in_area(position, 3).await?;
        let mut conflicts = Vec::new();
        
        for faction1 in &area_factions {
            for faction2 in &area_factions {
                if faction1.name != faction2.name {
                    if let Some(relationship) = faction1.relationships.get(&faction2.id) {
                        if *relationship < -5 {
                            conflicts.push(FactionConflict {
                                faction1: faction1.name.clone(),
                                faction2: faction2.name.clone(),
                                conflict_type: self.determine_conflict_type(&faction1.faction_type, &faction2.faction_type),
                                intensity: (-relationship).min(10) as u32,
                                location: position,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// Update faction control based on player actions
    pub async fn update_faction_control(&mut self, position: HexPosition, faction_name: &str, control_change: i32) -> Result<()> {
        // Update faction influence in the area
        if let Some(faction) = self.faction_cache.get_mut(faction_name) {
            if control_change > 0 {
                faction.influence_areas.push(position);
                faction.resources += control_change;
            } else {
                faction.influence_areas.retain(|pos| pos.q != position.q || pos.r != position.r);
                faction.resources = (faction.resources + control_change).max(0);
            }
        }
        
        // Rebuild influence map
        self.build_influence_map().await?;
        
        info!("Updated faction control: {} at ({}, {}) by {}", 
              faction_name, position.q, position.r, control_change);
        
        Ok(())
    }
    
    fn determine_conflict_type(&self, faction_type1: &str, faction_type2: &str) -> String {
        match (faction_type1, faction_type2) {
            ("temple_order", "cult") => "religious_war",
            ("merchant_guild", "thieves_guild") => "economic_conflict",
            ("militia", "thieves_guild") => "law_enforcement",
            ("militia", "cult") => "suppression_campaign",
            _ => "territorial_dispute",
        }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FactionConflict {
    pub faction1: String,
    pub faction2: String,
    pub conflict_type: String,
    pub intensity: u32, // 1-10 scale
    pub location: HexPosition,
}

/// NPC faction loyalty system
impl FactionSystems {
    /// Get NPC faction loyalty and how it affects interactions
    pub async fn get_npc_faction_loyalty(&self, npc_id: Uuid) -> Result<Option<FactionLoyalty>> {
        let npc = npcs::Entity::find_by_id(npc_id)
            .one(&self.db)
            .await?;
        
        if let Some(npc) = npc {
            if let Some(faction_name) = &npc.faction {
                let loyalty_level = self.calculate_npc_loyalty(&npc);
                
                return Ok(Some(FactionLoyalty {
                    faction_name: faction_name.clone(),
                    loyalty_level,
                    will_betray: loyalty_level < 30,
                    information_value: self.calculate_information_value(&npc),
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Check if NPC will provide faction information
    pub async fn attempt_extract_faction_info(&self, npc_id: Uuid, persuasion_check: i32) -> Result<Option<FactionInformation>> {
        let loyalty = self.get_npc_faction_loyalty(npc_id).await?;
        
        if let Some(loyalty) = loyalty {
            let difficulty = 10 + loyalty.loyalty_level / 5; // Higher loyalty = harder to extract info
            
            if persuasion_check >= difficulty {
                if let Some(faction) = self.faction_cache.get(&loyalty.faction_name) {
                    return Ok(Some(FactionInformation {
                        faction_name: faction.name.clone(),
                        faction_type: faction.faction_type.clone(),
                        goals: faction.goals.clone(),
                        approximate_resources: faction.resources / 10, // Rough estimate
                        known_conflicts: self.get_faction_relationships(&faction.name).await?,
                    }));
                }
            }
        }
        
        Ok(None)
    }
    
    fn calculate_npc_loyalty(&self, npc: &npcs::Model) -> i32 {
        // Base loyalty on NPC disposition and wealth
        let base_loyalty = 50;
        let disposition_modifier = npc.disposition * 3;
        let wealth_modifier = (npc.wealth_level - 5) * 2;
        let corruption_penalty = (npc.current_corruption_level * 20.0) as i32;
        
        (base_loyalty + disposition_modifier + wealth_modifier - corruption_penalty).clamp(0, 100)
    }
    
    fn calculate_information_value(&self, npc: &npcs::Model) -> i32 {
        // Higher-ranking NPCs have more valuable information
        match npc.role.as_str() {
            "leader" | "commander" => 10,
            "lieutenant" | "captain" => 8,
            "craftsperson" | "healer" => 6,
            "guard" | "soldier" => 4,
            _ => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FactionLoyalty {
    pub faction_name: String,
    pub loyalty_level: i32, // 0-100
    pub will_betray: bool,
    pub information_value: i32,
}

#[derive(Debug, Clone)]
pub struct FactionInformation {
    pub faction_name: String,
    pub faction_type: String,
    pub goals: Vec<String>,
    pub approximate_resources: i32,
    pub known_conflicts: HashMap<String, i32>,
}
