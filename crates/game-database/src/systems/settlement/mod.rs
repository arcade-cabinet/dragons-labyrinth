//! Settlement Systems - Database-driven inn, shop, tavern, and town mechanics
//!
//! This system queries settlement and NPC data to create rich social interactions,
//! trade mechanics, and inn/tavern experiences using HBF-imported content.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, RelationTrait, QuerySelect};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;
use rand::Rng;
use super::{HexPosition, InnInterface, NpcData, TradeData, WeatherCondition};

pub struct SettlementSystems {
    db: DatabaseConnection,
    settlement_cache: HashMap<Uuid, SettlementData>,
    npc_cache: HashMap<Uuid, NpcData>,
}

#[derive(Debug, Clone)]
pub struct SettlementData {
    pub id: Uuid,
    pub name: String,
    pub settlement_type: String,
    pub population: Option<i32>,
    pub prosperity_level: i32,
    pub safety_rating: i32,
    pub corruption_influence: f32,
    pub services: Vec<String>,
    pub notable_features: Vec<String>,
    pub faction: Option<String>,
}

impl SettlementSystems {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let mut system = Self {
            db: db.clone(),
            settlement_cache: HashMap::new(),
            npc_cache: HashMap::new(),
        };
        
        // Pre-load settlement data for performance
        system.load_settlement_cache().await?;
        system.load_npc_cache().await?;
        
        info!("Settlement systems initialized with {} settlements and {} NPCs",
              system.settlement_cache.len(), system.npc_cache.len());
        
        Ok(system)
    }
    
    /// Get all settlements within a hex radius for map display
    pub async fn get_settlements_in_area(&self, center: HexPosition, radius: u32) -> Result<Vec<SettlementData>> {
        let settlements = settlements::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut nearby_settlements = Vec::new();
        
        for settlement in settlements {
            if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
                let settlement_pos = HexPosition::from_hbf_coords(hbf_x, hbf_y);
                let distance = center.distance_to(&settlement_pos);
                
                if distance <= radius {
                    nearby_settlements.push(SettlementData {
                        id: settlement.id,
                        name: settlement.name,
                        settlement_type: settlement.settlement_type,
                        population: settlement.population,
                        prosperity_level: settlement.prosperity_level,
                        safety_rating: settlement.safety_rating,
                        corruption_influence: settlement.corruption_influence,
                        services: self.parse_services(&settlement.services),
                        notable_features: self.parse_features(&settlement.notable_features),
                        faction: settlement.faction,
                    });
                }
            }
        }
        
        debug!("Found {} settlements within {} hexes of ({}, {})", 
               nearby_settlements.len(), radius, center.q, center.r);
        
        Ok(nearby_settlements)
    }
    
    /// Enter a settlement and get interaction interface
    pub async fn enter_settlement(&self, settlement_id: Uuid) -> Result<InnInterface> {
        let settlement = settlements::Entity::find_by_id(settlement_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Settlement not found"))?;
        
        // Get NPCs in this settlement
        let npcs = npcs::Entity::find()
            .filter(npcs::Column::SettlementId.eq(settlement_id))
            .all(&self.db)
            .await?;
        
        let npc_data: Vec<NpcData> = npcs.into_iter()
            .map(|npc| self.convert_npc_to_data(&npc))
            .collect();
        
        // Get current weather for the settlement
        let current_weather = self.get_settlement_weather(&settlement).await?;
        
        // Calculate room rate based on prosperity and weather
        let room_rate = self.calculate_room_rate(&settlement, &current_weather);
        
        // Get available rumors
        let rumors = self.get_settlement_rumors(&settlement).await?;
        
        info!("Player entered {}: {} NPCs, {} services available", 
              settlement.name, npc_data.len(), 
              self.parse_services(&settlement.services).len());
        
        Ok(InnInterface {
            settlement_name: settlement.name,
            npcs: npc_data,
            services: self.parse_services(&settlement.services),
            current_weather,
            room_rate,
            rumors,
        })
    }
    
    /// Interact with specific NPC for dialogue/trade
    pub async fn interact_with_npc(&self, npc_id: Uuid, interaction_type: &str) -> Result<NpcInteractionResult> {
        let npc = npcs::Entity::find_by_id(npc_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("NPC not found"))?;
        
        match interaction_type {
            "dialogue" => self.handle_npc_dialogue(&npc).await,
            "trade" => self.handle_npc_trade(&npc).await,
            "services" => self.handle_npc_services(&npc).await,
            _ => Err(anyhow::anyhow!("Unknown interaction type: {}", interaction_type))
        }
    }
    
    /// Get available shop items and services in settlement
    pub async fn get_settlement_trade_options(&self, settlement_id: Uuid) -> Result<Vec<TradeOption>> {
        let npcs = npcs::Entity::find()
            .filter(npcs::Column::SettlementId.eq(settlement_id))
            .all(&self.db)
            .await?;
        
        let mut trade_options = Vec::new();
        
        for npc in npcs {
            if let Some(trade_goods) = &npc.trade_goods {
                if let Ok(trade_data) = serde_json::from_value::<TradeData>(trade_goods.clone()) {
                    trade_options.push(TradeOption {
                        npc_id: npc.id,
                        npc_name: npc.name,
                        buys: trade_data.buys,
                        sells: trade_data.sells,
                        price_modifiers: trade_data.price_modifiers,
                    });
                }
            }
        }
        
        Ok(trade_options)
    }
    
    /// Rest at inn and recover health/sanity
    pub async fn rest_at_inn(&self, settlement_id: Uuid, rest_type: RestType) -> Result<RestResult> {
        let settlement = settlements::Entity::find_by_id(settlement_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Settlement not found"))?;
        
        // Check if settlement offers lodging
        let services = self.parse_services(&settlement.services);
        if !services.contains(&"lodging".to_string()) && settlement.settlement_type != "inn" && settlement.settlement_type != "tavern" {
            return Err(anyhow::anyhow!("This settlement doesn't offer lodging"));
        }
        
        // Calculate rest effectiveness based on settlement quality and corruption
        let rest_effectiveness = self.calculate_rest_effectiveness(&settlement);
        
        let result = match rest_type {
            RestType::ShortRest => RestResult {
                health_recovered: (rest_effectiveness * 10.0) as i32,
                sanity_recovered: (rest_effectiveness * 5.0) as i32,
                corruption_gained: settlement.corruption_influence * 0.1,
                cost: 0, // Short rest is free
                time_passed: 1, // 1 hour
            },
            RestType::LongRest => RestResult {
                health_recovered: (rest_effectiveness * 50.0) as i32,
                sanity_recovered: (rest_effectiveness * 25.0) as i32,
                corruption_gained: settlement.corruption_influence * 0.3,
                cost: self.calculate_room_rate(&settlement, &WeatherCondition {
                    condition: "Clear".to_string(),
                    visibility_modifier: 1.0,
                    movement_modifier: 1.0,
                    combat_effects: Vec::new(),
                }),
                time_passed: 8, // 8 hours
            },
        };
        
        info!("Player rested at {}: +{} HP, +{} sanity, +{:.2} corruption", 
              settlement.name, result.health_recovered, result.sanity_recovered, result.corruption_gained);
        
        Ok(result)
    }
    
    /// Update settlement reputation based on player actions
    pub async fn update_settlement_reputation(&self, settlement_id: Uuid, reputation_change: i32) -> Result<()> {
        settlements::Entity::update_many()
            .col_expr(settlements::Column::Reputation, 
                     sea_orm::sea_query::Expr::col(settlements::Column::Reputation).add(reputation_change))
            .filter(settlements::Column::Id.eq(settlement_id))
            .exec(&self.db)
            .await?;
        
        info!("Updated settlement reputation by {}", reputation_change);
        Ok(())
    }
    
    /// Private helper methods
    
    async fn load_settlement_cache(&mut self) -> Result<()> {
        let settlements = settlements::Entity::find().all(&self.db).await?;
        
        for settlement in settlements {
            let data = SettlementData {
                id: settlement.id,
                name: settlement.name.clone(),
                settlement_type: settlement.settlement_type.clone(),
                population: settlement.population,
                prosperity_level: settlement.prosperity_level,
                safety_rating: settlement.safety_rating,
                corruption_influence: settlement.corruption_influence,
                services: self.parse_services(&settlement.services),
                notable_features: self.parse_features(&settlement.notable_features),
                faction: settlement.faction.clone(),
            };
            self.settlement_cache.insert(settlement.id, data);
        }
        
        Ok(())
    }
    
    async fn load_npc_cache(&mut self) -> Result<()> {
        let npcs = npcs::Entity::find().all(&self.db).await?;
        
        for npc in npcs {
            let data = self.convert_npc_to_data(&npc);
            self.npc_cache.insert(npc.id, data);
        }
        
        Ok(())
    }
    
    fn convert_npc_to_data(&self, npc: &npcs::Model) -> NpcData {
        let trade_goods = if let Some(trade_json) = &npc.trade_goods {
            serde_json::from_value(trade_json.clone()).unwrap_or_else(|_| TradeData {
                buys: Vec::new(),
                sells: Vec::new(),
                price_modifiers: HashMap::new(),
            })
        } else {
            TradeData {
                buys: Vec::new(),
                sells: Vec::new(),
                price_modifiers: HashMap::new(),
            }
        };
        
        NpcData {
            id: npc.id,
            name: npc.name.clone(),
            role: npc.role.clone(),
            personality: npc.personality.clone().unwrap_or_else(|| "Friendly".to_string()),
            disposition: npc.disposition,
            dialogue_options: self.parse_dialogue_options(&npc.dialogue_options),
            services_offered: self.parse_services(&npc.services_offered),
            trade_goods,
        }
    }
    
    async fn get_settlement_weather(&self, settlement: &settlements::Model) -> Result<WeatherCondition> {
        // Query weather data for this settlement
        let weather_entries = settlements::weather::Entity::find()
            .filter(settlements::weather::Column::SettlementId.eq(settlement.id))
            .all(&self.db)
            .await?;
        
        if weather_entries.is_empty() {
            return Ok(WeatherCondition {
                condition: "Clear".to_string(),
                visibility_modifier: 1.0,
                movement_modifier: 1.0,
                combat_effects: Vec::new(),
            });
        }
        
        // Roll for current weather (simplified - would use actual season/time)
        let entry = &weather_entries[rand::thread_rng().gen_range(0..weather_entries.len())];
        
        Ok(WeatherCondition {
            condition: entry.weather_condition.clone(),
            visibility_modifier: self.get_weather_visibility_modifier(&entry.weather_condition),
            movement_modifier: self.get_weather_movement_modifier(&entry.weather_condition),
            combat_effects: self.get_weather_combat_effects(&entry.weather_condition),
        })
    }
    
    fn calculate_room_rate(&self, settlement: &settlements::Model, weather: &WeatherCondition) -> i32 {
        let base_rate = match settlement.settlement_type.as_str() {
            "inn" => 5,
            "tavern" => 3,
            "temple" => 1, // Temples often offer free lodging for pilgrims
            _ => 2,
        };
        
        // Prosperity affects prices
        let prosperity_modifier = (settlement.prosperity_level as f32 / 10.0) + 0.5;
        
        // Weather affects demand
        let weather_modifier = match weather.condition.as_str() {
            "Stormy" | "Rainy" => 1.5, // Higher demand during bad weather
            "Clear" => 1.0,
            _ => 1.2,
        };
        
        ((base_rate as f32) * prosperity_modifier * weather_modifier) as i32
    }
    
    fn calculate_rest_effectiveness(&self, settlement: &settlements::Model) -> f32 {
        let base_effectiveness = match settlement.settlement_type.as_str() {
            "temple" => 1.0, // Best rest at temples
            "inn" => 0.9,
            "tavern" => 0.7, // Taverns are noisy
            "shop" => 0.5,
            _ => 0.4,
        };
        
        // Safety and prosperity improve rest quality
        let safety_modifier = (settlement.safety_rating as f32 / 10.0) * 0.3 + 0.7;
        let prosperity_modifier = (settlement.prosperity_level as f32 / 10.0) * 0.2 + 0.8;
        
        // Corruption reduces rest effectiveness
        let corruption_penalty = settlement.corruption_influence * 0.5;
        
        (base_effectiveness * safety_modifier * prosperity_modifier - corruption_penalty).max(0.1)
    }
    
    async fn get_settlement_rumors(&self, settlement: &settlements::Model) -> Result<Vec<String>> {
        // Get rumors from settlement data and NPCs
        let mut rumors = Vec::new();
        
        // Settlement-specific rumors
        if let Some(settlement_rumors) = &settlement.rumors {
            if let Ok(rumor_list) = serde_json::from_value::<Vec<String>>(settlement_rumors.clone()) {
                rumors.extend(rumor_list);
            }
        }
        
        // NPC rumors
        let npcs = npcs::Entity::find()
            .filter(npcs::Column::SettlementId.eq(settlement.id))
            .all(&self.db)
            .await?;
        
        for npc in npcs {
            if let Some(npc_rumors) = &npc.rumors_known {
                if let Ok(rumor_list) = serde_json::from_value::<Vec<String>>(npc_rumors.clone()) {
                    rumors.extend(rumor_list);
                }
            }
        }
        
        // Limit to 3-5 random rumors to avoid overwhelming player
        if rumors.len() > 5 {
            let mut selected_rumors = Vec::new();
            let num_rumors = rand::thread_rng().gen_range(3..=5);
            
            for _ in 0..num_rumors {
                if !rumors.is_empty() {
                    let index = rand::thread_rng().gen_range(0..rumors.len());
                    selected_rumors.push(rumors.remove(index));
                }
            }
            
            rumors = selected_rumors;
        }
        
        Ok(rumors)
    }
    
    async fn handle_npc_dialogue(&self, npc: &npcs::Model) -> Result<NpcInteractionResult> {
        let dialogue_options = self.parse_dialogue_options(&npc.dialogue_options);
        
        // Adjust dialogue based on disposition and corruption
        let mut available_options = dialogue_options;
        
        if npc.disposition < -5 {
            available_options = vec!["hostile_greeting".to_string()];
        } else if npc.current_corruption_level > 0.7 {
            available_options.push("strange_behavior".to_string());
        }
        
        Ok(NpcInteractionResult::Dialogue {
            npc_name: npc.name.clone(),
            greeting: self.generate_npc_greeting(npc),
            options: available_options,
            personality_note: npc.personality.clone().unwrap_or_default(),
        })
    }
    
    async fn handle_npc_trade(&self, npc: &npcs::Model) -> Result<NpcInteractionResult> {
        let trade_data = if let Some(trade_json) = &npc.trade_goods {
            serde_json::from_value(trade_json.clone()).unwrap_or_else(|_| TradeData {
                buys: Vec::new(),
                sells: Vec::new(),
                price_modifiers: HashMap::new(),
            })
        } else {
            return Err(anyhow::anyhow!("NPC doesn't offer trade services"));
        };
        
        Ok(NpcInteractionResult::Trade {
            npc_name: npc.name.clone(),
            trade_data,
            wealth_level: npc.wealth_level,
        })
    }
    
    async fn handle_npc_services(&self, npc: &npcs::Model) -> Result<NpcInteractionResult> {
        let services = self.parse_services(&npc.services_offered);
        
        if services.is_empty() {
            return Err(anyhow::anyhow!("NPC doesn't offer any services"));
        }
        
        Ok(NpcInteractionResult::Services {
            npc_name: npc.name.clone(),
            available_services: services,
            service_quality: self.calculate_service_quality(npc),
        })
    }
    
    fn generate_npc_greeting(&self, npc: &npcs::Model) -> String {
        let base_greeting = match npc.disposition {
            -10..=-6 => "What do you want?",
            -5..=-1 => "I suppose you're here for something...",
            0..=2 => "Hello there, traveler.",
            3..=5 => "Welcome! How can I help you?",
            6..=10 => "Greetings, friend! Wonderful to see you!",
            _ => "Hello.",
        };
        
        // Add corruption effects
        if npc.current_corruption_level > 0.5 {
            format!("{}... *their eyes seem distant*", base_greeting)
        } else {
            base_greeting.to_string()
        }
    }
    
    fn calculate_service_quality(&self, npc: &npcs::Model) -> f32 {
        let base_quality = match npc.role.as_str() {
            "craftsperson" => 0.9,
            "healer" => 0.8,
            "mystic" => 0.7,
            "artisan" => 0.85,
            _ => 0.6,
        };
        
        // Wealth and disposition affect quality
        let wealth_modifier = (npc.wealth_level as f32 / 10.0) * 0.2 + 0.8;
        let disposition_modifier = ((npc.disposition + 10) as f32 / 20.0) * 0.3 + 0.7;
        
        // Corruption reduces service quality
        let corruption_penalty = npc.current_corruption_level * 0.4;
        
        (base_quality * wealth_modifier * disposition_modifier - corruption_penalty).clamp(0.1, 1.0)
    }
    
    fn parse_services(&self, services_json: &Option<serde_json::Value>) -> Vec<String> {
        if let Some(services) = services_json {
            serde_json::from_value(services.clone()).unwrap_or_default()
        } else {
            Vec::new()
        }
    }
    
    fn parse_features(&self, features_json: &Option<serde_json::Value>) -> Vec<String> {
        if let Some(features) = features_json {
            serde_json::from_value(features.clone()).unwrap_or_default()
        } else {
            Vec::new()
        }
    }
    
    fn parse_dialogue_options(&self, dialogue_json: &Option<serde_json::Value>) -> Vec<String> {
        if let Some(dialogue) = dialogue_json {
            serde_json::from_value(dialogue.clone()).unwrap_or_else(|_| {
                vec!["greeting".to_string(), "ask_about_area".to_string(), "farewell".to_string()]
            })
        } else {
            vec!["greeting".to_string(), "ask_about_area".to_string(), "farewell".to_string()]
        }
    }
    
    fn get_weather_visibility_modifier(&self, condition: &str) -> f32 {
        match condition {
            "Foggy" => 0.3,
            "Stormy" => 0.5,
            "Rainy" => 0.7,
            "Cloudy" => 0.9,
            _ => 1.0,
        }
    }
    
    fn get_weather_movement_modifier(&self, condition: &str) -> f32 {
        match condition {
            "Stormy" => 0.5,
            "Rainy" => 0.8,
            "Foggy" => 0.9,
            _ => 1.0,
        }
    }
    
    fn get_weather_combat_effects(&self, condition: &str) -> Vec<String> {
        match condition {
            "Stormy" => vec!["lightning_risk".to_string(), "high_winds".to_string()],
            "Rainy" => vec!["slippery_ground".to_string()],
            "Foggy" => vec!["limited_visibility".to_string()],
            _ => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RestType {
    ShortRest,
    LongRest,
}

#[derive(Debug, Clone)]
pub struct RestResult {
    pub health_recovered: i32,
    pub sanity_recovered: i32,
    pub corruption_gained: f32,
    pub cost: i32,
    pub time_passed: i32, // in hours
}

#[derive(Debug, Clone)]
pub struct TradeOption {
    pub npc_id: Uuid,
    pub npc_name: String,
    pub buys: Vec<String>,
    pub sells: Vec<String>,
    pub price_modifiers: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub enum NpcInteractionResult {
    Dialogue {
        npc_name: String,
        greeting: String,
        options: Vec<String>,
        personality_note: String,
    },
    Trade {
        npc_name: String,
        trade_data: TradeData,
        wealth_level: i32,
    },
    Services {
        npc_name: String,
        available_services: Vec<String>,
        service_quality: f32,
    },
}
