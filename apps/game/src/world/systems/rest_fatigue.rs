use bevy::prelude::*;
use bevy_rand::prelude::*;
use crate::world::components::tiles::HexCoord;
use crate::world::systems::regional_progression::EmotionalState;

#[derive(Component, Debug)]
pub struct PlayerStats {
    pub level: u32,
    pub health: f32,
    pub max_health: f32,
    pub fatigue: f32,
    pub max_fatigue: f32,
    pub rest_quality: f32, // 0.0 = exhausted, 1.0 = well rested
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            health: 100.0,
            max_health: 100.0,
            fatigue: 0.0,
            max_fatigue: 100.0,
            rest_quality: 1.0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct DayNightCycle {
    pub current_hour: f32, // 0.0 = midnight, 12.0 = noon
    pub day_length_hours: f32, // Real-time minutes for a full day
    pub movement_points_remaining: u32,
    pub max_daily_movement: u32,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            current_hour: 8.0, // Start at 8 AM
            day_length_hours: 24.0,
            movement_points_remaining: 8, // 8 hex walking, 12 hex running
            max_daily_movement: 8,
        }
    }
}

#[derive(Component, Debug)]
pub struct RestSite {
    pub rest_type: RestType,
    pub safety_level: f32, // 0.0 = dangerous, 1.0 = completely safe
    pub comfort_level: f32, // 0.0 = rough ground, 1.0 = luxurious inn
    pub shelter_quality: f32, // Protection from weather
}

#[derive(Debug, Clone, PartialEq)]
pub enum RestType {
    Inn,           // Full rest, safe, comfortable
    Camp,          // Player-made camp, variable safety
    Shelter,       // Natural or abandoned shelter
    WildRough,     // Sleeping rough in the wilderness
}

#[derive(Resource, Debug)]
pub struct WeatherSystem {
    pub current_weather: WeatherType,
    pub intensity: f32, // 0.0 = mild, 1.0 = severe
    pub temperature: f32, // -1.0 = freezing, 0.0 = mild, 1.0 = scorching
    pub visibility: f32, // 0.0 = no visibility, 1.0 = clear
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherType {
    Clear,
    Rain,
    Storm,
    Snow,
    Fog,
    VoidStorm, // Supernatural weather in corrupted areas
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            intensity: 0.2,
            temperature: 0.0,
            visibility: 1.0,
        }
    }
}

pub fn calculate_fatigue_from_movement(
    distance: f32,
    terrain_difficulty: f32,
    weather_penalty: f32,
    player_stats: &PlayerStats,
) -> f32 {
    let base_fatigue = distance * 2.0;
    let terrain_multiplier = 1.0 + terrain_difficulty;
    let weather_multiplier = 1.0 + weather_penalty;
    let fitness_modifier = 1.0 - (player_stats.level as f32 * 0.01); // Higher level = less fatigue
    
    base_fatigue * terrain_multiplier * weather_multiplier * fitness_modifier.max(0.5)
}

pub fn calculate_rest_recovery(
    rest_site: &RestSite,
    weather: &WeatherSystem,
    emotional_state: &EmotionalState,
    hours_rested: f32,
) -> (f32, f32) { // Returns (health_recovery, fatigue_recovery)
    let base_recovery_rate = match rest_site.rest_type {
        RestType::Inn => 1.0,
        RestType::Camp => 0.7,
        RestType::Shelter => 0.5,
        RestType::WildRough => 0.3,
    };
    
    // Weather affects rest quality
    let weather_modifier = match weather.current_weather {
        WeatherType::Clear => 1.0,
        WeatherType::Rain => 0.8,
        WeatherType::Storm => 0.6,
        WeatherType::Snow => 0.7,
        WeatherType::Fog => 0.9,
        WeatherType::VoidStorm => 0.3,
    };
    
    // Emotional state affects how well you can rest
    let emotional_modifier = match emotional_state {
        EmotionalState::Peace => 1.0,
        EmotionalState::Unease => 0.9,
        EmotionalState::Dread => 0.7,
        EmotionalState::Terror => 0.5,
        EmotionalState::Void => 0.3,
    };
    
    let safety_modifier = rest_site.safety_level;
    let comfort_modifier = rest_site.comfort_level;
    let shelter_modifier = (rest_site.shelter_quality + (1.0 - weather.intensity)) / 2.0;
    
    let total_modifier = base_recovery_rate 
        * weather_modifier 
        * emotional_modifier 
        * safety_modifier 
        * comfort_modifier 
        * shelter_modifier;
    
    let health_recovery = hours_rested * 5.0 * total_modifier;
    let fatigue_recovery = hours_rested * 10.0 * total_modifier;
    
    (health_recovery, fatigue_recovery)
}

pub fn update_day_night_cycle(
    time: Res<Time>,
    mut day_night: ResMut<DayNightCycle>,
) {
    // Advance time - each real second = 1 game hour by default
    day_night.current_hour += time.delta_seconds() / 60.0; // 1 real minute = 1 game hour
    
    if day_night.current_hour >= 24.0 {
        day_night.current_hour -= 24.0;
        // New day - reset movement points
        day_night.movement_points_remaining = day_night.max_daily_movement;
    }
}

pub fn check_forced_rest(
    day_night: Res<DayNightCycle>,
    player_query: Query<&PlayerStats>,
    // TODO: Add event system for forcing rest
) {
    if let Ok(player_stats) = player_query.get_single() {
        let is_night = day_night.current_hour < 6.0 || day_night.current_hour > 22.0;
        let is_exhausted = player_stats.fatigue > player_stats.max_fatigue * 0.8;
        let no_movement_left = day_night.movement_points_remaining == 0;
        
        if is_night || is_exhausted || no_movement_left {
            // TODO: Trigger rest requirement event
        }
    }
}

pub fn generate_weather_for_region(
    emotional_state: &EmotionalState,
    corruption_level: f32,
    rng: &mut ChaCha8Rng,
) -> WeatherSystem {
    let weather_types = match emotional_state {
        EmotionalState::Peace => vec![
            (WeatherType::Clear, 0.6),
            (WeatherType::Rain, 0.3),
            (WeatherType::Fog, 0.1),
        ],
        EmotionalState::Unease => vec![
            (WeatherType::Clear, 0.4),
            (WeatherType::Rain, 0.4),
            (WeatherType::Fog, 0.2),
        ],
        EmotionalState::Dread => vec![
            (WeatherType::Rain, 0.4),
            (WeatherType::Storm, 0.3),
            (WeatherType::Fog, 0.3),
        ],
        EmotionalState::Terror => vec![
            (WeatherType::Storm, 0.5),
            (WeatherType::VoidStorm, 0.3),
            (WeatherType::Fog, 0.2),
        ],
        EmotionalState::Void => vec![
            (WeatherType::VoidStorm, 0.8),
            (WeatherType::Storm, 0.2),
        ],
    };
    
    let roll: f32 = rng.r#gen();
    let mut cumulative = 0.0;
    let mut selected_weather = WeatherType::Clear;
    
    for (weather_type, probability) in weather_types {
        cumulative += probability;
        if roll < cumulative {
            selected_weather = weather_type;
            break;
        }
    }
    
    let base_intensity = 0.2 + corruption_level * 0.6;
    let intensity_variation: f32 = rng.r#gen::<f32>() * 0.4 - 0.2; // ±0.2
    let final_intensity = (base_intensity + intensity_variation).clamp(0.0, 1.0);
    
    WeatherSystem {
        current_weather: selected_weather,
        intensity: final_intensity,
        temperature: rng.r#gen::<f32>() * 2.0 - 1.0, // -1.0 to 1.0
        visibility: match selected_weather {
            WeatherType::Clear => 1.0,
            WeatherType::Rain => 0.8,
            WeatherType::Storm => 0.6,
            WeatherType::Snow => 0.7,
            WeatherType::Fog => 0.3,
            WeatherType::VoidStorm => 0.2,
        },
    }
}

pub fn setup_camp_system(
    // TODO: Add input handling for camp setup
    // TODO: Add inventory system for camping supplies
) {
    // Allow player to set up camp in valid locations
    // Require camping supplies for better rest quality
    // Some locations prohibit camping (civilized areas, dangerous terrain)
}

pub fn handle_inn_rest(
    // TODO: Add interaction system for inns
    // TODO: Add currency system for inn costs
) {
    // Handle staying at inns
    // Cost varies by region and inn quality
    // Provides best rest quality and safety
}

pub fn calculate_encounter_chance_while_resting(
    rest_site: &RestSite,
    emotional_state: &EmotionalState,
    weather: &WeatherSystem,
) -> f32 {
    let base_chance = match emotional_state {
        EmotionalState::Peace => 0.05,
        EmotionalState::Unease => 0.15,
        EmotionalState::Dread => 0.3,
        EmotionalState::Terror => 0.5,
        EmotionalState::Void => 0.8,
    };
    
    let safety_modifier = 1.0 - rest_site.safety_level;
    let weather_modifier = match weather.current_weather {
        WeatherType::Storm | WeatherType::VoidStorm => 1.5,
        WeatherType::Fog => 1.2,
        _ => 1.0,
    };
    
    base_chance * safety_modifier * weather_modifier
}
