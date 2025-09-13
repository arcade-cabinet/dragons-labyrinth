use bevy::prelude::{Component, Entity};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub sanity: f32,
    pub max_sanity: f32,
    pub inventory: Vec<Item>,
    pub mount: Option<Entity>,
}

#[derive(Component, Debug, Clone)]
pub struct Mount {
    pub mount_type: MountType,
    pub speed_multiplier: f32,
    pub terrain_bonuses: std::collections::HashMap<String, f32>,
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MountType {
    Horse {
        breed: String,
        temperament: String,
    },
    Ox {
        strength: u32,
        endurance: u32,
    },
    ExoticMount {
        species: String,
        special_abilities: Vec<String>,
    },
    CorruptedMount {
        original_type: Box<MountType>,
        corruption_level: f32,
    },
}

impl MountType {
    pub fn get_base_speed_multiplier(&self) -> f32 {
        match self {
            MountType::Horse { .. } => 2.0,
            MountType::Ox { .. } => 1.5,
            MountType::ExoticMount { .. } => 2.5,
            MountType::CorruptedMount { original_type, corruption_level } => {
                original_type.get_base_speed_multiplier() * (1.0 + corruption_level)
            }
        }
    }
    
    pub fn get_terrain_penalties(&self) -> std::collections::HashMap<String, f32> {
        let mut penalties = std::collections::HashMap::new();
        
        match self {
            MountType::Horse { .. } => {
                penalties.insert("swamp".to_string(), 0.3);
                penalties.insert("mountain".to_string(), 0.5);
                penalties.insert("water".to_string(), 0.1);
            }
            MountType::Ox { .. } => {
                penalties.insert("mountain".to_string(), 0.8); // Good at climbing
                penalties.insert("forest".to_string(), 0.4);
            }
            MountType::ExoticMount { .. } => {
                // Exotic mounts have fewer penalties but are rare
                penalties.insert("void".to_string(), 0.8);
            }
            MountType::CorruptedMount { .. } => {
                // Corrupted mounts handle corrupted terrain better
                penalties.insert("void".to_string(), 1.2);
                penalties.insert("corrupted".to_string(), 1.5);
            }
        }
        
        penalties
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub weight: f32,
    pub value: u32,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon {
        damage: u32,
        weapon_type: String,
        enchantments: Vec<String>,
    },
    Armor {
        protection: u32,
        armor_type: String,
        enchantments: Vec<String>,
    },
    Consumable {
        effect: String,
        duration: f32,
        potency: u32,
    },
    Tool {
        tool_type: String,
        durability: u32,
        max_durability: u32,
    },
    QuestItem {
        quest_id: String,
        unique: bool,
    },
    Currency {
        currency_type: String,
    },
    Material {
        material_type: String,
        rarity: String,
    },
}

#[derive(Component, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: u32,
    pub current_weight: f32,
    pub max_weight: f32,
}

impl Inventory {
    pub fn new(capacity: u32, max_weight: f32) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            current_weight: 0.0,
            max_weight,
        }
    }
    
    pub fn can_add_item(&self, item: &Item) -> bool {
        self.items.len() < self.capacity as usize 
            && (self.current_weight + item.weight * item.quantity as f32) <= self.max_weight
    }
    
    pub fn add_item(&mut self, item: Item) -> bool {
        if self.can_add_item(&item) {
            self.current_weight += item.weight * item.quantity as f32;
            self.items.push(item);
            true
        } else {
            false
        }
    }
    
    pub fn remove_item(&mut self, item_name: &str, quantity: u32) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|item| item.name == item_name) {
            let item = &mut self.items[index];
            if item.quantity >= quantity {
                item.quantity -= quantity;
                self.current_weight -= item.weight * quantity as f32;
                
                if item.quantity == 0 {
                    Some(self.items.remove(index))
                } else {
                    Some(Item {
                        quantity,
                        ..item.clone()
                    })
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
