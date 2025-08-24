//! Weather Engine - Database-driven environmental effects and seasonal mechanics
//!
//! This system queries weather data from settlements and applies environmental
//! effects to gameplay using HBF-imported weather tables.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;
use rand::Rng;
use super::{HexPosition, WeatherCondition};

pub struct WeatherEngine {
    db: DatabaseConnection,
    seasonal_calendar: SeasonalCalendar,
    weather_effects: HashMap<String, EnvironmentalEffect>,
}

#[derive(Debug, Clone)]
pub struct SeasonalCalendar {
    pub current_day: u32,
    pub current_season: Season,
    pub season_length: u32, // days per season
}

#[derive(Debug, Clone)]
pub enum Season {
    Warm,
    Dry,
    Wet,
    Cold,
}

impl Season {
    pub fn as_str(&self) -> &'static str {
        match self {
            Season::Warm => "warm",
            Season::Dry => "dry", 
            Season::Wet => "wet",
            Season::Cold => "cold",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentalEffect {
    pub visibility_modifier: f32,
    pub movement_modifier: f32,
    pub combat_effects: Vec<String>,
    pub corruption_influence: f32, // How weather affects corruption spread
    pub companion_effects: Vec<String>, // How weather affects companion mood
}

impl WeatherEngine {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let seasonal_calendar = SeasonalCalendar {
            current_day: 1,
            current_season: Season::Warm,
            season_length: 90, // 90 days per season = 1 year cycle
        };
        
        let weather_effects = Self::create_weather_effects_map();
        
        info!("Weather engine initialized with {} weather effect types",
              weather_effects.len());
        
        Ok(Self {
            db: db.clone(),
            seasonal_calendar,
            weather_effects,
        })
    }
    
    /// Get current weather for a specific location
    pub async fn get_weather_at_position(&self, position: HexPosition) -> Result<WeatherCondition> {
        // Find nearest settlement with weather data
        let nearest_settlement = self.find_nearest_settlement_with_weather(position).await?;
        
        match nearest_settlement {
            Some(settlement_id) => self.get_settlement_weather(settlement_id).await,
            None => Ok(self.get_default_weather_for_biome(position).await?),
        }
    }
    
    /// Get weather for specific settlement using its weather tables
    pub async fn get_settlement_weather(&self, settlement_id: Uuid) -> Result<WeatherCondition> {
        let weather_entries = settlements::weather::Entity::find()
            .filter(settlements::weather::Column::SettlementId.eq(settlement_id))
            .filter(settlements::weather::Column::Season.eq(self.seasonal_calendar.current_season.as_str()))
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
        
        // Roll 2d6 for weather (most HBF weather tables use 2d6)
        let roll = rand::thread_rng().gen_range(2..=12);
        
        // Find matching weather entry
        for entry in weather_entries {
            if self.roll_matches_range(&entry.dice_roll, roll) {
                let effects = self.weather_effects
                    .get(&entry.weather_condition)
                    .cloned()
                    .unwrap_or_else(|| EnvironmentalEffect {
                        visibility_modifier: 1.0,
                        movement_modifier: 1.0,
                        combat_effects: Vec::new(),
                        corruption_influence: 0.0,
                        companion_effects: Vec::new(),
                    });
                
                return Ok(WeatherCondition {
                    condition: entry.weather_condition,
                    visibility_modifier: effects.visibility_modifier,
                    movement_modifier: effects.movement_modifier,
                    combat_effects: effects.combat_effects,
                });
            }
        }
        
        // Fallback to first entry if no match
        let first_entry = &weather_entries[0];
        Ok(WeatherCondition {
            condition: first_entry.weather_condition.clone(),
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
        })
    }
    
    /// Advance time and update seasonal calendar
    pub fn advance_time(&mut self, hours: u32) -> Vec<WeatherEvent> {
        let mut events = Vec::new();
        
        // Check if day has passed
        if hours >= 24 {
            let days_passed = hours / 24;
            self.seasonal_calendar.current_day += days_passed;
            
            // Check for season change
            if self.seasonal_calendar.current_day > self.seasonal_calendar.season_length {
                let old_season = self.seasonal_calendar.current_season.clone();
                self.seasonal_calendar.current_season = self.advance_season();
                self.seasonal_calendar.current_day = 1;
                
                events.push(WeatherEvent::SeasonChange {
                    from: old_season,
                    to: self.seasonal_calendar.current_season.clone(),
                });
                
                info!("Season changed to {:?}", self.seasonal_calendar.current_season);
            }
        }
        
        events
    }
    
    /// Apply weather effects to corruption spread
    pub async fn apply_weather_to_corruption(&self, position: HexPosition, base_corruption: f32) -> Result<f32> {
        let weather = self.get_weather_at_position(position).await?;
        
        if let Some(effects) = self.weather_effects.get(&weather.condition) {
            // Weather can accelerate or slow corruption spread
            let modified_corruption = base_corruption * (1.0 + effects.corruption_influence);
            Ok(modified_corruption.clamp(0.0, 1.0))
        } else {
            Ok(base_corruption)
        }
    }
    
    /// Get weather effects on companion morale
    pub async fn get_companion_weather_effects(&self, position: HexPosition) -> Result<Vec<String>> {
        let weather = self.get_weather_at_position(position).await?;
        
        if let Some(effects) = self.weather_effects.get(&weather.condition) {
            Ok(effects.companion_effects.clone())
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Private helper methods
    
    async fn find_nearest_settlement_with_weather(&self, position: HexPosition) -> Result<Option<Uuid>> {
        let settlements = settlements::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut nearest_settlement = None;
        let mut min_distance = f32::INFINITY;
        
        for settlement in settlements {
            if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
                let settlement_pos = HexPosition::from_hbf_coords(hbf_x, hbf_y);
                let distance = position.distance_to(&settlement_pos) as f32;
                
                if distance < min_distance {
                    // Check if this settlement has weather data
                    let weather_count = settlements::weather::Entity::find()
                        .filter(settlements::weather::Column::SettlementId.eq(settlement.id))
                        .count(&self.db)
                        .await?;
                    
                    if weather_count > 0 {
                        min_distance = distance;
                        nearest_settlement = Some(settlement.id);
                    }
                }
            }
        }
        
        Ok(nearest_settlement)
    }
    
    async fn get_default_weather_for_biome(&self, position: HexPosition) -> Result<WeatherCondition> {
        // Get tile biome to determine default weather
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let default_condition = match tile.as_ref().map(|t| t.biome_type.as_str()) {
            Some("desert") => "Clear & Hot",
            Some("tundra") => "Cold & Clear",
            Some("swamp") => "Foggy",
            Some("jungle") => "Humid",
            Some("mountain") => "Windy",
            _ => "Clear",
        };
        
        let effects = self.weather_effects
            .get(default_condition)
            .cloned()
            .unwrap_or_else(|| EnvironmentalEffect {
                visibility_modifier: 1.0,
                movement_modifier: 1.0,
                combat_effects: Vec::new(),
                corruption_influence: 0.0,
                companion_effects: Vec::new(),
            });
        
        Ok(WeatherCondition {
            condition: default_condition.to_string(),
            visibility_modifier: effects.visibility_modifier,
            movement_modifier: effects.movement_modifier,
            combat_effects: effects.combat_effects,
        })
    }
    
    fn roll_matches_range(&self, range: &str, roll: u32) -> bool {
        if range.contains('-') {
            // Handle ranges like "3-4", "5-9"
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(min), Ok(max)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return roll >= min && roll <= max;
                }
            }
        } else {
            // Handle single numbers like "2", "12"
            if let Ok(target) = range.parse::<u32>() {
                return roll == target;
            }
        }
        false
    }
    
    fn advance_season(&self) -> Season {
        match self.seasonal_calendar.current_season {
            Season::Warm => Season::Dry,
            Season::Dry => Season::Wet,
            Season::Wet => Season::Cold,
            Season::Cold => Season::Warm,
        }
    }
    
    fn create_weather_effects_map() -> HashMap<String, EnvironmentalEffect> {
        let mut effects = HashMap::new();
        
        effects.insert("Clear".to_string(), EnvironmentalEffect {
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
            corruption_influence: 0.0,
            companion_effects: vec!["good_mood".to_string()],
        });
        
        effects.insert("Rainy".to_string(), EnvironmentalEffect {
            visibility_modifier: 0.7,
            movement_modifier: 0.8,
            combat_effects: vec!["slippery_ground".to_string()],
            corruption_influence: 0.1, // Rain spreads corruption
            companion_effects: vec!["dampened_spirits".to_string()],
        });
        
        effects.insert("Stormy".to_string(), EnvironmentalEffect {
            visibility_modifier: 0.5,
            movement_modifier: 0.6,
            combat_effects: vec!["lightning_risk".to_string(), "high_winds".to_string()],
            corruption_influence: 0.2,
            companion_effects: vec!["fearful".to_string(), "seek_shelter".to_string()],
        });
        
        effects.insert("Foggy".to_string(), EnvironmentalEffect {
            visibility_modifier: 0.3,
            movement_modifier: 0.9,
            combat_effects: vec!["limited_visibility".to_string()],
            corruption_influence: 0.15, // Fog hides corruption but doesn't spread it
            companion_effects: vec!["nervous".to_string(), "disoriented".to_string()],
        });
        
        effects.insert("Cloudy".to_string(), EnvironmentalEffect {
            visibility_modifier: 0.9,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
            corruption_influence: 0.0,
            companion_effects: vec!["overcast_mood".to_string()],
        });
        
        effects.insert("Snowstorm".to_string(), EnvironmentalEffect {
            visibility_modifier: 0.2,
            movement_modifier: 0.5,
            combat_effects: vec!["freezing_cold".to_string(), "difficult_terrain".to_string()],
            corruption_influence: -0.1, // Cold slows corruption spread
            companion_effects: vec!["freezing".to_string(), "exhausted".to_string()],
        });
        
        effects.insert("Breezy".to_string(), EnvironmentalEffect {
            visibility_modifier: 1.0,
            movement_modifier: 1.1, // Slight movement bonus
            combat_effects: vec!["favorable_winds".to_string()],
            corruption_influence: -0.05, // Fresh air reduces corruption
            companion_effects: vec!["refreshed".to_string()],
        });
        
        effects
    }
}

#[derive(Debug, Clone)]
pub enum WeatherEvent {
    SeasonChange { from: Season, to: Season },
    SpecialWeather { condition: String, duration_hours: u32 },
    FloodWarning { affected_hexes: Vec<HexPosition> },
}

#[derive(Debug, Clone)]
pub struct WeatherForecast {
    pub current: WeatherCondition,
    pub next_few_hours: Vec<WeatherCondition>,
    pub seasonal_trends: Vec<String>,
}

/// Weather system for Dragon's Labyrinth horror progression
impl WeatherEngine {
    /// Get detailed weather forecast for strategic planning
    pub async fn get_weather_forecast(&self, position: HexPosition, hours_ahead: u32) -> Result<WeatherForecast> {
        let current_weather = self.get_weather_at_position(position).await?;
        
        // Generate forecast based on current weather and seasonal patterns
        let mut forecast = Vec::new();
        let mut current_condition = current_weather.condition.clone();
        
        for hour in 1..=hours_ahead {
            // Simple weather progression model
            current_condition = self.evolve_weather_condition(&current_condition, hour)?;
            
            if let Some(effects) = self.weather_effects.get(&current_condition) {
                forecast.push(WeatherCondition {
                    condition: current_condition.clone(),
                    visibility_modifier: effects.visibility_modifier,
                    movement_modifier: effects.movement_modifier,
                    combat_effects: effects.combat_effects.clone(),
                });
            }
        }
        
        let seasonal_trends = self.get_seasonal_trends();
        
        Ok(WeatherForecast {
            current: current_weather,
            next_few_hours: forecast,
            seasonal_trends,
        })
    }
    
    /// Check for special weather events (floods, storms, etc.)
    pub async fn check_for_special_weather_events(&self) -> Result<Vec<WeatherEvent>> {
        let mut events = Vec::new();
        
        // Check for flood warnings in wet season
        if matches!(self.seasonal_calendar.current_season, Season::Wet) {
            let flood_risk = rand::thread_rng().gen::<f32>();
            if flood_risk < 0.02 { // 2% daily chance during wet season
                let affected_hexes = self.get_flood_prone_hexes().await?;
                events.push(WeatherEvent::FloodWarning { affected_hexes });
            }
        }
        
        // Check for extreme weather
        let extreme_weather_risk = rand::thread_rng().gen::<f32>();
        if extreme_weather_risk < 0.01 { // 1% daily chance
            let extreme_condition = match self.seasonal_calendar.current_season {
                Season::Warm => "Heatwave",
                Season::Dry => "Dust Storm", 
                Season::Wet => "Thunderstorm",
                Season::Cold => "Blizzard",
            };
            
            events.push(WeatherEvent::SpecialWeather {
                condition: extreme_condition.to_string(),
                duration_hours: rand::thread_rng().gen_range(2..=8),
            });
        }
        
        Ok(events)
    }
    
    /// Apply weather effects to hex tile corruption
    pub async fn apply_weather_corruption_effects(&self, position: HexPosition) -> Result<f32> {
        let weather = self.get_weather_at_position(position).await?;
        
        if let Some(effects) = self.weather_effects.get(&weather.condition) {
            Ok(effects.corruption_influence)
        } else {
            Ok(0.0)
        }
    }
    
    /// Get companion morale effects from current weather
    pub async fn get_companion_weather_morale(&self, position: HexPosition) -> Result<HashMap<String, f32>> {
        let weather = self.get_weather_at_position(position).await?;
        let mut morale_effects = HashMap::new();
        
        if let Some(effects) = self.weather_effects.get(&weather.condition) {
            for effect in &effects.companion_effects {
                let morale_change = match effect.as_str() {
                    "good_mood" => 5.0,
                    "refreshed" => 3.0,
                    "overcast_mood" => -1.0,
                    "dampened_spirits" => -3.0,
                    "nervous" => -5.0,
                    "fearful" => -8.0,
                    "freezing" => -10.0,
                    _ => 0.0,
                };
                
                morale_effects.insert(effect.clone(), morale_change);
            }
        }
        
        Ok(morale_effects)
    }
    
    /// Private helper methods
    
    async fn find_nearest_settlement_with_weather(&self, position: HexPosition) -> Result<Option<Uuid>> {
        let settlements = settlements::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut nearest_settlement = None;
        let mut min_distance = f32::INFINITY;
        
        for settlement in settlements {
            if let (Some(hbf_x), Some(hbf_y)) = (settlement.hbf_x, settlement.hbf_y) {
                let settlement_pos = HexPosition::from_hbf_coords(hbf_x, hbf_y);
                let distance = position.distance_to(&settlement_pos) as f32;
                
                // Only consider settlements within reasonable range (50 hexes)
                if distance < min_distance && distance <= 50.0 {
                    min_distance = distance;
                    nearest_settlement = Some(settlement.id);
                }
            }
        }
        
        Ok(nearest_settlement)
    }
    
    async fn get_default_weather_for_biome(&self, position: HexPosition) -> Result<WeatherCondition> {
        let tile = hex_tiles::Entity::find()
            .filter(hex_tiles::Column::Q.eq(position.q))
            .filter(hex_tiles::Column::R.eq(position.r))
            .filter(hex_tiles::Column::S.eq(position.s))
            .one(&self.db)
            .await?;
        
        let condition = match tile.as_ref().map(|t| t.biome_type.as_str()) {
            Some("desert") => "Clear & Hot",
            Some("tundra") => "Cold & Clear", 
            Some("swamp") => "Foggy",
            Some("jungle") => "Humid",
            Some("mountain") => "Windy",
            _ => "Clear",
        };
        
        let effects = self.weather_effects.get(condition).cloned().unwrap_or_default();
        
        Ok(WeatherCondition {
            condition: condition.to_string(),
            visibility_modifier: effects.visibility_modifier,
            movement_modifier: effects.movement_modifier,
            combat_effects: effects.combat_effects,
        })
    }
    
    fn evolve_weather_condition(&self, current: &str, hour: u32) -> Result<String> {
        // Simple weather evolution - could be more sophisticated
        let change_chance = 0.1 + (hour as f32 * 0.05); // Higher chance of change over time
        
        if rand::thread_rng().gen::<f32>() < change_chance {
            let new_conditions = match current {
                "Clear" => vec!["Cloudy", "Partly Cloudy"],
                "Cloudy" => vec!["Clear", "Rainy", "Foggy"],
                "Rainy" => vec!["Cloudy", "Stormy", "Clear"],
                "Stormy" => vec!["Rainy", "Cloudy"],
                "Foggy" => vec!["Cloudy", "Clear"],
                _ => vec!["Clear"],
            };
            
            if !new_conditions.is_empty() {
                let index = rand::thread_rng().gen_range(0..new_conditions.len());
                Ok(new_conditions[index].to_string())
            } else {
                Ok(current.to_string())
            }
        } else {
            Ok(current.to_string())
        }
    }
    
    fn get_seasonal_trends(&self) -> Vec<String> {
        match self.seasonal_calendar.current_season {
            Season::Warm => vec![
                "Temperatures rising".to_string(),
                "Clear skies common".to_string(),
                "Good traveling weather".to_string(),
            ],
            Season::Dry => vec![
                "Low humidity".to_string(),
                "Dust storms possible".to_string(), 
                "Water sources scarce".to_string(),
            ],
            Season::Wet => vec![
                "Heavy rains expected".to_string(),
                "Flooding in low areas".to_string(),
                "Rivers running high".to_string(),
            ],
            Season::Cold => vec![
                "Temperatures dropping".to_string(),
                "Snow in mountains".to_string(),
                "Shorter daylight hours".to_string(),
            ],
        }
    }
    
    async fn get_flood_prone_hexes(&self) -> Result<Vec<HexPosition>> {
        // Find hexes with rivers or near water that could flood
        let tiles = hex_tiles::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut flood_hexes = Vec::new();
        
        for tile in tiles {
            if let Some(features) = &tile.features {
                if let Some(feature_obj) = features.as_object() {
                    if let Some(rivers) = feature_obj.get("rivers") {
                        if let Some(river_array) = rivers.as_array() {
                            if !river_array.is_empty() {
                                flood_hexes.push(HexPosition::new(tile.q, tile.r));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(flood_hexes)
    }
}

impl Default for EnvironmentalEffect {
    fn default() -> Self {
        Self {
            visibility_modifier: 1.0,
            movement_modifier: 1.0,
            combat_effects: Vec::new(),
            corruption_influence: 0.0,
            companion_effects: Vec::new(),
        }
    }
}
