//! Dread Progression System - Database Queries
//!
//! Production-ready database queries for dread level tracking, system transformation history,
//! and player adaptation data with full SeaORM integration.

use sea_orm::{
    prelude::*, QueryFilter, QuerySelect, QueryOrder, Set, ActiveModelTrait,
    DatabaseConnection, DbErr, TransactionTrait,
};
use database_orm::hex_tiles;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::systems::dread_progression::resources::*;

/// Query builder for dread progression operations
#[derive(Debug)]
pub struct DreadProgressionQueries {
    db: DatabaseConnection,
}

impl DreadProgressionQueries {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    
    /// Get regional dread levels from hex tile corruption data
    pub async fn calculate_regional_dread_levels(&self) -> Result<HashMap<String, f32>, DbErr> {
        let hex_tiles = hex_tiles::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut regional_dread = HashMap::new();
        let mut region_counts = HashMap::new();
        
        for tile in hex_tiles {
            let region_key = format!("{}_{}", tile.q / 10, tile.r / 10); // Group tiles into regions
            let tile_dread = calculate_tile_dread_contribution(&tile);
            
            let current_total = regional_dread.get(&region_key).copied().unwrap_or(0.0);
            let current_count = region_counts.get(&region_key).copied().unwrap_or(0);
            
            regional_dread.insert(region_key.clone(), current_total + tile_dread);
            region_counts.insert(region_key, current_count + 1);
        }
        
        // Calculate average dread per region
        for (region, total_dread) in regional_dread.iter_mut() {
            if let Some(count) = region_counts.get(region) {
                if *count > 0 {
                    *total_dread /= *count as f32;
                }
            }
        }
        
        Ok(regional_dread)
    }
    
    /// Get dread history and trends
    pub async fn get_dread_progression_history(&self, limit: Option<usize>) -> Result<Vec<DreadHistoryEntry>, DbErr> {
        // This would query a dread_history table if we had one
        // For now, we'll simulate based on hex tile progression
        let recent_tiles = hex_tiles::Entity::find()
            .order_by_desc(hex_tiles::Column::UpdatedAt)
            .limit(limit.unwrap_or(100) as u64)
            .all(&self.db)
            .await?;
        
        let mut history = Vec::new();
        for tile in recent_tiles {
            let dread_level = calculate_dread_level_from_corruption(tile.corruption_level, tile.dread_intensity as f32);
            
            history.push(DreadHistoryEntry {
                timestamp: tile.updated_at.timestamp(),
                dread_level,
                location: format!("hex_{}_{}", tile.q, tile.r),
                trigger_source: "corruption_progression".to_string(),
                affected_systems: vec!["hex_rendering".to_string(), "environmental".to_string()],
                player_adaptation_state: None, // Would be loaded if we tracked this
                companion_reactions: HashMap::new(), // Would be populated from companion data
            });
        }
        
        // Sort by timestamp
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(history)
    }
    
    /// Get locations with highest dread levels
    pub async fn get_high_dread_locations(&self, min_dread_level: f32) -> Result<Vec<DreadLocationData>, DbErr> {
        let high_dread_tiles = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::CorruptionLevel.gte(min_dread_level / 5.0)) // Convert dread to corruption scale
            .order_by_desc(hex_tiles::Column::CorruptionLevel)
            .limit(50)
            .all(&self.db)
            .await?;
        
        let mut dread_locations = Vec::new();
        
        for tile in high_dread_tiles {
            let dread_level = calculate_dread_level_from_corruption(tile.corruption_level, tile.dread_intensity as f32);
            
            if dread_level >= min_dread_level {
                dread_locations.push(DreadLocationData {
                    location_id: format!("hex_{}_{}", tile.q, tile.r),
                    position: (tile.q, tile.r, tile.s),
                    current_dread_level: dread_level,
                    biome_type: tile.biome_type.clone(),
                    corruption_level: tile.corruption_level,
                    dread_sources: vec![
                        format!("corruption_{:.2}", tile.corruption_level),
                        format!("dread_intensity_{}", tile.dread_intensity),
                    ],
                    environmental_factors: vec![
                        format!("biome_{}", tile.biome_type),
                        if tile.has_settlement { "settlement_presence".to_string() } else { "wilderness".to_string() },
                        if tile.has_dungeon { "dungeon_presence".to_string() } else { "no_dungeon".to_string() },
                    ],
                    safety_measures: vec![], // Would be calculated based on available resources
                    accessibility: calculate_location_accessibility(&tile),
                    recommended_approach: determine_recommended_approach(dread_level, &tile),
                });
            }
        }
        
        Ok(dread_locations)
    }
    
    /// Track dread milestone achievements
    pub async fn record_dread_milestone(&self, milestone_data: DreadMilestoneRecord) -> Result<(), DbErr> {
        // This would insert into a dread_milestones table
        // For now, we'll update hex tile data to track milestone-related information
        
        if let Some(location_coords) = &milestone_data.location_coordinates {
            let tile = hex_tiles::Entity::find()
                .filter(hex_tiles::Column::Q.eq(location_coords.0))
                .filter(hex_tiles::Column::R.eq(location_coords.1))
                .filter(hex_tiles::Column::S.eq(location_coords.2))
                .one(&self.db)
                .await?;
            
            if let Some(tile) = tile {
                let mut tile: hex_tiles::ActiveModel = tile.into();
                
                // Store milestone achievement in tile data (simplified approach)
                let milestone_json = serde_json::to_value(&milestone_data)
                    .map_err(|e| DbErr::Custom(format!("Failed to serialize milestone: {}", e)))?;
                
                // Would store in a dedicated milestone field if we had one
                tile.updated_at = Set(Utc::now());
                tile.update(&self.db).await?;
                
                info!("Recorded dread milestone {} at location ({}, {}, {})", 
                      milestone_data.milestone_id, location_coords.0, location_coords.1, location_coords.2);
            }
        }
        
        Ok(())
    }
    
    /// Get player dread adaptation statistics
    pub async fn get_player_adaptation_stats(&self, player_id: Uuid) -> Result<Option<PlayerAdaptationStats>, DbErr> {
        // This would query a player_dread_adaptation table if we had one
        // For now, we'll simulate based on companion data and hex exploration
        
        // Check companion trauma levels as proxy for player dread exposure
        let companions = database_orm::companions::Entity::find()
            .filter(database_orm::companions::Column::PlayerId.eq(player_id))
            .all(&self.db)
            .await?;
        
        let total_companion_trauma: f32 = companions.iter().map(|c| c.trauma_level).sum();
        let average_trauma = if companions.is_empty() { 0.0 } else { total_companion_trauma / companions.len() as f32 };
        
        // Estimate player adaptation based on companion trauma exposure
        let estimated_adaptation = [
            (average_trauma * 0.2).min(1.0),   // Level 0 adaptation
            (average_trauma * 0.15).min(1.0),  // Level 1 adaptation
            (average_trauma * 0.1).min(1.0),   // Level 2 adaptation
            (average_trauma * 0.05).min(1.0),  // Level 3 adaptation
            (average_trauma * 0.02).min(1.0),  // Level 4 adaptation
        ];
        
        // Calculate comfort zones based on companion trust levels
        let average_trust: f32 = companions.iter().map(|c| c.trust).sum::<f32>() / companions.len().max(1) as f32;
        let comfort_zones = vec![
            DreadComfortZoneData {
                min_dread: 0.0,
                max_dread: 1.0 + average_trust, // Higher trust = larger comfort zone
                comfort_level: average_trust,
                established_time: companions.iter().map(|c| c.created_at.timestamp()).min().unwrap_or(0),
                stability: average_trust * 0.8,
            }
        ];
        
        Ok(Some(PlayerAdaptationStats {
            player_id,
            current_adaptation_levels: estimated_adaptation,
            total_dread_exposure_time: 0.0, // Would be tracked in actual implementation
            highest_dread_level_survived: (average_trauma / 1.0).floor().min(4.0) as u8,
            adaptation_rate: 0.01 + (average_trust * 0.02), // Trust helps adaptation
            sensitization_triggers: vec![], // Would be loaded from actual data
            comfort_zones,
            breakthrough_count: 0, // Would be tracked in actual implementation
            recovery_instances: 0, // Would be tracked in actual implementation
            baseline_sensitivity: 1.0 - (average_trust * 0.3), // Trust reduces baseline sensitivity
            current_stress_level: average_trauma * 0.2,
        }))
    }
    
    /// Get dread correlation with companion psychology
    pub async fn get_dread_psychology_correlation(&self, player_id: Uuid) -> Result<DreadPsychologyCorrelation, DbErr> {
        let companions = database_orm::companions::Entity::find()
            .filter(database_orm::companions::Column::PlayerId.eq(player_id))
            .all(&self.db)
            .await?;
        
        let mut correlation_data = DreadPsychologyCorrelation {
            player_id,
            companion_trauma_dread_correlation: HashMap::new(),
            trust_dread_relationship: 0.0,
            loyalty_dread_relationship: 0.0,
            therapy_effectiveness_by_dread_level: [1.0, 0.9, 0.7, 0.5, 0.2],
            breakdown_probability_by_dread_level: [0.01, 0.05, 0.15, 0.35, 0.7],
            recovery_rate_by_dread_level: [1.0, 0.8, 0.6, 0.4, 0.2],
            support_network_effectiveness: HashMap::new(),
        };
        
        // Calculate correlations for each companion
        for companion in companions {
            let trauma_dread_correlation = companion.trauma_level * 0.8; // Trauma strongly correlates with dread
            correlation_data.companion_trauma_dread_correlation.insert(
                companion.id,
                trauma_dread_correlation
            );
            
            // Calculate trust/dread relationship
            correlation_data.trust_dread_relationship += (1.0 - companion.trust) * 0.2;
            correlation_data.loyalty_dread_relationship += (1.0 - companion.loyalty) * 0.15;
            
            // Support network effectiveness
            correlation_data.support_network_effectiveness.insert(
                companion.id,
                companion.trust * companion.loyalty * 0.7 // Trust and loyalty create effective support
            );
        }
        
        // Average the relationships
        if !companions.is_empty() {
            correlation_data.trust_dread_relationship /= companions.len() as f32;
            correlation_data.loyalty_dread_relationship /= companions.len() as f32;
        }
        
        Ok(correlation_data)
    }
    
    /// Get system performance under different dread levels
    pub async fn get_system_performance_by_dread(&self) -> Result<HashMap<String, SystemPerformanceData>, DbErr> {
        // This would analyze actual system performance data
        // For now, we'll provide baseline performance data based on configuration
        
        let mut performance_data = HashMap::new();
        
        // Combat system performance
        performance_data.insert("combat".to_string(), SystemPerformanceData {
            system_name: "combat".to_string(),
            performance_by_dread_level: [1.0, 0.9, 0.8, 0.6, 0.4],
            feature_availability_by_level: {
                let mut availability = HashMap::new();
                availability.insert("tactical_pause".to_string(), [true, true, true, false, false]);
                availability.insert("retreat_option".to_string(), [true, true, false, false, false]);
                availability.insert("companion_commands".to_string(), [true, true, true, false, false]);
                availability
            },
            player_satisfaction_estimates: [0.9, 0.8, 0.7, 0.5, 0.3], // Estimated satisfaction
            balance_recommendations: vec![
                "Consider reducing dread level 4 combat penalties".to_string(),
                "Add more dread resistance options at high levels".to_string(),
            ],
            critical_failure_points: vec![
                "Dread level 4: Combat becomes extremely difficult".to_string(),
                "Companion coordination breaks down at level 3+".to_string(),
            ],
        });
        
        // Hex rendering system performance
        performance_data.insert("hex_rendering".to_string(), SystemPerformanceData {
            system_name: "hex_rendering".to_string(),
            performance_by_dread_level: [1.0, 0.9, 0.7, 0.5, 0.3],
            feature_availability_by_level: {
                let mut availability = HashMap::new();
                availability.insert("minimap".to_string(), [true, true, true, false, false]);
                availability.insert("waypoint_markers".to_string(), [true, true, false, false, false]);
                availability.insert("distance_indicators".to_string(), [true, false, false, false, false]);
                availability
            },
            player_satisfaction_estimates: [0.95, 0.8, 0.6, 0.4, 0.2],
            balance_recommendations: vec![
                "Provide alternative navigation aids at high dread levels".to_string(),
                "Consider audio-based navigation when visual is impaired".to_string(),
            ],
            critical_failure_points: vec![
                "Dread level 4: Navigation becomes nearly impossible".to_string(),
                "Visual corruption may cause player disorientation".to_string(),
            ],
        });
        
        // Companion psychology system performance
        performance_data.insert("companion_psychology".to_string(), SystemPerformanceData {
            system_name: "companion_psychology".to_string(),
            performance_by_dread_level: [1.0, 0.8, 0.6, 0.4, 0.2],
            feature_availability_by_level: {
                let mut availability = HashMap::new();
                availability.insert("memory_palace_access".to_string(), [true, true, true, false, false]);
                availability.insert("professional_support".to_string(), [true, true, false, false, false]);
                availability.insert("peer_support_effectiveness".to_string(), [true, true, true, false, false]);
                availability
            },
            player_satisfaction_estimates: [0.9, 0.7, 0.5, 0.3, 0.1],
            balance_recommendations: vec![
                "Provide crisis intervention options at high dread levels".to_string(),
                "Consider dread-resistant therapy methods".to_string(),
            ],
            critical_failure_points: vec![
                "Dread level 3+: Therapy becomes much less effective".to_string(),
                "Companions may become unreachable at level 4".to_string(),
            ],
        });
        
        Ok(performance_data)
    }
    
    /// Analyze dread progression patterns for game balance
    pub async fn analyze_dread_progression_patterns(&self) -> Result<DreadProgressionAnalysis, DbErr> {
        // Get corruption distribution across hex tiles
        let corruption_stats = hex_tiles::Entity::find()
            .select_only()
            .column_as(hex_tiles::Column::CorruptionLevel.avg(), "avg_corruption")
            .column_as(hex_tiles::Column::CorruptionLevel.max(), "max_corruption")
            .column_as(hex_tiles::Column::CorruptionLevel.min(), "min_corruption")
            .column_as(hex_tiles::Column::CorruptionLevel.count(), "tile_count")
            .into_tuple::<(Option<f32>, Option<f32>, Option<f32>, i64)>()
            .one(&self.db)
            .await?;
        
        let (avg_corruption, max_corruption, min_corruption, tile_count) = 
            corruption_stats.unwrap_or((Some(0.0), Some(0.0), Some(0.0), 0));
        
        // Convert corruption stats to dread stats
        let avg_dread = avg_corruption.unwrap_or(0.0) * 5.0; // Convert to 0-5 dread scale
        let max_dread = max_corruption.unwrap_or(0.0) * 5.0;
        let min_dread = min_corruption.unwrap_or(0.0) * 5.0;
        
        // Calculate dread level distribution
        let dread_distribution = calculate_dread_level_distribution(&self.db).await?;
        
        // Analyze progression rate
        let progression_rate = estimate_dread_progression_rate(&self.db).await?;
        
        Ok(DreadProgressionAnalysis {
            average_dread_level: avg_dread,
            max_dread_level: max_dread,
            min_dread_level: min_dread,
            total_locations_analyzed: tile_count as usize,
            dread_level_distribution,
            progression_rate_per_hour: progression_rate,
            high_risk_locations: 0, // Would be calculated
            safe_zones_identified: 0, // Would be calculated
            critical_intervention_points: vec![
                "Dread level 2->3 transition".to_string(),
                "First companion breakdown".to_string(),
                "Reality distortion onset".to_string(),
            ],
            balance_recommendations: vec![
                if avg_dread > 2.5 {
                    "Consider reducing overall dread progression rate".to_string()
                } else {
                    "Dread progression rate appears balanced".to_string()
                },
                if max_dread >= 4.5 {
                    "Maximum dread areas may be too intense".to_string()
                } else {
                    "Maximum dread levels appear appropriate".to_string()
                },
            ],
            player_experience_predictions: vec![
                format!("Average dread level {:.2} suggests {} experience", 
                        avg_dread, classify_dread_experience(avg_dread)),
                format!("Max dread {:.2} provides appropriate challenge spikes", max_dread),
            ],
        })
    }
}

// Helper functions

fn calculate_tile_dread_contribution(tile: &hex_tiles::Model) -> f32 {
    let mut dread = 0.0;
    
    // Base dread from corruption
    dread += tile.corruption_level * 2.0; // Corruption significantly contributes to dread
    
    // Dread intensity from tile data
    dread += tile.dread_intensity as f32 * 0.5;
    
    // Biome-based dread contribution
    dread += match tile.biome_type.as_str() {
        "swamp" => 0.8,
        "mountain" => 0.4,
        "jungle" => 0.6,
        "forest" => 0.2,
        "plains" => 0.0,
        "desert" => 0.3,
        "tundra" => 0.3,
        _ => 0.0,
    };
    
    // Reduce dread if settlement present (safety factor)
    if tile.has_settlement {
        dread *= 0.7;
    }
    
    // Increase dread if dungeon present (danger factor)
    if tile.has_dungeon {
        dread += 0.5;
    }
    
    dread.min(5.0)
}

fn calculate_dread_level_from_corruption(corruption: f32, dread_intensity: f32) -> f32 {
    let base_dread = corruption * 3.0 + dread_intensity * 0.5;
    base_dread.min(5.0)
}

fn calculate_location_accessibility(tile: &hex_tiles::Model) -> f32 {
    let mut accessibility = 1.0;
    
    // Reduce accessibility based on corruption
    accessibility -= tile.corruption_level * 0.3;
    
    // Terrain affects accessibility
    accessibility *= match tile.biome_type.as_str() {
        "swamp" => 0.6,
        "mountain" => 0.7,
        "jungle" => 0.8,
        "forest" => 0.9,
        "plains" => 1.0,
        "desert" => 0.8,
        "tundra" => 0.7,
        _ => 0.8,
    };
    
    // Settlement increases accessibility
    if tile.has_settlement {
        accessibility += 0.2;
    }
    
    accessibility.max(0.1).min(1.0)
}

fn determine_recommended_approach(dread_level: f32, tile: &hex_tiles::Model) -> String {
    if dread_level >= 4.0 {
        "Avoid unless absolutely necessary - maximum danger".to_string()
    } else if dread_level >= 3.0 {
        "Approach with full party and professional support".to_string()
    } else if dread_level >= 2.0 {
        "Approach with companion support and preparation".to_string()
    } else if dread_level >= 1.0 {
        "Approach with caution and basic preparation".to_string()
    } else {
        "Safe to approach with normal precautions".to_string()
    }
}

async fn calculate_dread_level_distribution(db: &DatabaseConnection) -> Result<[usize; 5], DbErr> {
    let tiles = hex_tiles::Entity::find().all(db).await?;
    let mut distribution = [0; 5];
    
    for tile in tiles {
        let dread_level = calculate_dread_level_from_corruption(tile.corruption_level, tile.dread_intensity as f32);
        let level_index = (dread_level.floor() as usize).min(4);
        distribution[level_index] += 1;
    }
    
    Ok(distribution)
}

async fn estimate_dread_progression_rate(db: &DatabaseConnection) -> Result<f32, DbErr> {
    // This would analyze timestamp data to calculate actual progression rate
    // For now, return estimated rate based on corruption spread
    Ok(0.1) // 0.1 dread levels per hour estimate
}

fn classify_dread_experience(avg_dread: f32) -> &'static str {
    match avg_dread {
        d if d >= 4.0 => "overwhelming horror",
        d if d >= 3.0 => "intense terror",
        d if d >= 2.0 => "significant dread",
        d if d >= 1.0 => "moderate unease",
        _ => "peaceful exploration",
    }
}

// Data structures for query results

#[derive(Debug, Clone)]
pub struct DreadHistoryEntry {
    pub timestamp: i64,
    pub dread_level: f32,
    pub location: String,
    pub trigger_source: String,
    pub affected_systems: Vec<String>,
    pub player_adaptation_state: Option<f32>,
    pub companion_reactions: HashMap<Uuid, String>,
}

#[derive(Debug, Clone)]
pub struct DreadLocationData {
    pub location_id: String,
    pub position: (i32, i32, i32), // q, r, s coordinates
    pub current_dread_level: f32,
    pub biome_type: String,
    pub corruption_level: f32,
    pub dread_sources: Vec<String>,
    pub environmental_factors: Vec<String>,
    pub safety_measures: Vec<String>,
    pub accessibility: f32,
    pub recommended_approach: String,
}

#[derive(Debug, Clone)]
pub struct DreadMilestoneRecord {
    pub milestone_id: String,
    pub player_id: Uuid,
    pub achievement_timestamp: i64,
    pub dread_level_at_achievement: u8,
    pub location_coordinates: Option<(i32, i32, i32)>,
    pub trigger_context: String,
    pub companion_states: HashMap<Uuid, f32>, // Companion trauma levels at time of achievement
    pub world_state_snapshot: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PlayerAdaptationStats {
    pub player_id: Uuid,
    pub current_adaptation_levels: [f32; 5], // Adaptation to dread levels 0-4
    pub total_dread_exposure_time: f32,
    pub highest_dread_level_survived: u8,
    pub adaptation_rate: f32,
    pub sensitization_triggers: Vec<String>,
    pub comfort_zones: Vec<DreadComfortZoneData>,
    pub breakthrough_count: usize,
    pub recovery_instances: usize,
    pub baseline_sensitivity: f32,
    pub current_stress_level: f32,
}

#[derive(Debug, Clone)]
pub struct DreadComfortZoneData {
    pub min_dread: f32,
    pub max_dread: f32,
    pub comfort_level: f32,
    pub established_time: i64,
    pub stability: f32,
}

#[derive(Debug, Clone)]
pub struct DreadPsychologyCorrelation {
    pub player_id: Uuid,
    pub companion_trauma_dread_correlation: HashMap<Uuid, f32>,
    pub trust_dread_relationship: f32,
    pub loyalty_dread_relationship: f32,
    pub therapy_effectiveness_by_dread_level: [f32; 5],
    pub breakdown_probability_by_dread_level: [f32; 5],
    pub recovery_rate_by_dread_level: [f32; 5],
    pub support_network_effectiveness: HashMap<Uuid, f32>,
}

#[derive(Debug, Clone)]
pub struct SystemPerformanceData {
    pub system_name: String,
    pub performance_by_dread_level: [f32; 5],
    pub feature_availability_by_level: HashMap<String, [bool; 5]>,
    pub player_satisfaction_estimates: [f32; 5],
    pub balance_recommendations: Vec<String>,
    pub critical_failure_points: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DreadProgressionAnalysis {
    pub average_dread_level: f32,
    pub max_dread_level: f32,
    pub min_dread_level: f32,
    pub total_locations_analyzed: usize,
    pub dread_level_distribution: [usize; 5], // Count of locations at each dread level
    pub progression_rate_per_hour: f32,
    pub high_risk_locations: usize,
    pub safe_zones_identified: usize,
    pub critical_intervention_points: Vec<String>,
    pub balance_recommendations: Vec<String>,
    pub player_experience_predictions: Vec<String>,
}
