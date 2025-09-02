use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Settlement {
    pub name: String,
    pub scale: SettlementScale,
    pub economic_activity: u32,
    pub corruption_resistance: u32,
    pub service_types: Vec<ServiceType>,
    pub npc_count: u32,
    pub establishment_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettlementScale {
    Village,
    Town,
    City,
    Metropolis,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    Commerce,
    Lodging,
    Crafting,
    Medical,
    Religious,
    Defense,
    Government,
    Learning,
}

impl Settlement {
pub fn village_of_ashamar() -> Self {
        Settlement {
            name: "Village of Ashamar".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 37,
            corruption_resistance: 7,
            service_types: vec![ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 11,
            establishment_count: 7,
        }
    }
pub fn village_of_balaal() -> Self {
        Settlement {
            name: "Village of Balaal".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 99,
            corruption_resistance: 7,
            service_types: vec![ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 15,
            establishment_count: 5,
        }
    }
pub fn town_of_devilville() -> Self {
        Settlement {
            name: "Town of Devilville".to_string(),
            scale: SettlementScale::Town,
            economic_activity: 116,
            corruption_resistance: 8,
            service_types: vec![ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 32,
            establishment_count: 7,
        }
    }
pub fn village_of_dokar() -> Self {
        Settlement {
            name: "Village of Dokar".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 64,
            corruption_resistance: 6,
            service_types: vec![ServiceType::Religious, ServiceType::Defense, ServiceType::Crafting, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 15,
            establishment_count: 3,
        }
    }
pub fn village_of_dorith() -> Self {
        Settlement {
            name: "Village of Dorith".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 92,
            corruption_resistance: 6,
            service_types: vec![ServiceType::Defense, ServiceType::Crafting, ServiceType::Learning, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 14,
            establishment_count: 6,
        }
    }
pub fn village_of_harad() -> Self {
        Settlement {
            name: "Village of Harad".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 52,
            corruption_resistance: 8,
            service_types: vec![ServiceType::Religious, ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Learning, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 17,
            establishment_count: 5,
        }
    }
pub fn village_of_headbone() -> Self {
        Settlement {
            name: "Village of Headbone".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 71,
            corruption_resistance: 8,
            service_types: vec![ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Learning, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 17,
            establishment_count: 7,
        }
    }
pub fn city_of_headsmen() -> Self {
        Settlement {
            name: "City of Headsmen".to_string(),
            scale: SettlementScale::City,
            economic_activity: 502,
            corruption_resistance: 9,
            service_types: vec![ServiceType::Religious, ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Learning, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 73,
            establishment_count: 24,
        }
    }
pub fn village_of_kothian() -> Self {
        Settlement {
            name: "Village of Kothian".to_string(),
            scale: SettlementScale::Village,
            economic_activity: 32,
            corruption_resistance: 6,
            service_types: vec![ServiceType::Defense, ServiceType::Crafting, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 7,
            establishment_count: 6,
        }
    }
pub fn city_of_palemoon() -> Self {
        Settlement {
            name: "City of Palemoon".to_string(),
            scale: SettlementScale::City,
            economic_activity: 404,
            corruption_resistance: 9,
            service_types: vec![ServiceType::Defense, ServiceType::Medical, ServiceType::Crafting, ServiceType::Learning, ServiceType::Lodging, ServiceType::Commerce],
            npc_count: 54,
            establishment_count: 25,
        }
    }
}

// Settlement-specific systems
pub fn spawn_settlements(mut commands: Commands) {
commands.spawn((
        Settlement::village_of_ashamar(),
        HexTile { 
            q: 5, 
            r: 7, 
            biome: "ashen_forest".to_string(), 
            distance_band: "peace".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_balaal(),
        HexTile { 
            q: 7, 
            r: 2, 
            biome: "ashen_forest".to_string(), 
            distance_band: "peace".to_string() 
        },
    ));
commands.spawn((
        Settlement::town_of_devilville(),
        HexTile { 
            q: 43, 
            r: 28, 
            biome: "wet_meadow".to_string(), 
            distance_band: "terror".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_dokar(),
        HexTile { 
            q: 12, 
            r: 15, 
            biome: "ashen_forest".to_string(), 
            distance_band: "unease".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_dorith(),
        HexTile { 
            q: 21, 
            r: 6, 
            biome: "ashen_forest".to_string(), 
            distance_band: "unease".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_harad(),
        HexTile { 
            q: 24, 
            r: 11, 
            biome: "wet_meadow".to_string(), 
            distance_band: "unease".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_headbone(),
        HexTile { 
            q: 18, 
            r: 1, 
            biome: "wet_meadow".to_string(), 
            distance_band: "peace".to_string() 
        },
    ));
commands.spawn((
        Settlement::city_of_headsmen(),
        HexTile { 
            q: 17, 
            r: 27, 
            biome: "wet_meadow".to_string(), 
            distance_band: "unease".to_string() 
        },
    ));
commands.spawn((
        Settlement::village_of_kothian(),
        HexTile { 
            q: 18, 
            r: 10, 
            biome: "ashen_forest".to_string(), 
            distance_band: "unease".to_string() 
        },
    ));
commands.spawn((
        Settlement::city_of_palemoon(),
        HexTile { 
            q: 23, 
            r: 45, 
            biome: "wet_meadow".to_string(), 
            distance_band: "dread".to_string() 
        },
    ));
}

pub fn update_settlement_corruption(
    mut settlements: Query<&mut Settlement>,
    player_distance: Res<PlayerDistance>,
) {
    for mut settlement in settlements.iter_mut() {
        // Apply distance-based corruption to settlements
        let base_resistance = settlement.corruption_resistance;
        let distance_corruption = player_distance.corruption_level();
        
        // Settlements resist corruption based on their characteristics
        if distance_corruption > base_resistance {
            // Settlement begins to show corruption
        }
    }
}