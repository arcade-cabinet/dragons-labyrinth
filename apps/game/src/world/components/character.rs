use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub character_data: CharacterData,
    pub position: Vec3,
    pub facing_direction: Vec3,
    pub movement_speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub experience: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub name: String,
    pub gender: Gender,
    pub appearance: CharacterAppearance,
    pub stats: CharacterStats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    pub hair_style: HairStyle,
    pub hair_color: Color,
    pub skin_tone: SkinTone,
    pub eye_color: Color,
    pub height: f32,        // 0.8 to 1.2 (80% to 120% of base)
    pub weight: f32,        // 0.8 to 1.2 (affects width scaling)
    pub clothing_set: ClothingSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HairStyle {
    Short,
    Medium,
    Long,
    Braided,
    Shaved,
    Curly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkinTone {
    Pale,
    Fair,
    Olive,
    Tan,
    Brown,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClothingSet {
    Peasant,
    Merchant,
    Scholar,
    Warrior,
    Ranger,
    Noble,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl Default for CharacterData {
    fn default() -> Self {
        Self {
            name: "Adventurer".to_string(),
            gender: Gender::Male,
            appearance: CharacterAppearance::default(),
            stats: CharacterStats::default(),
        }
    }
}

impl Default for CharacterAppearance {
    fn default() -> Self {
        Self {
            hair_style: HairStyle::Medium,
            hair_color: Color::srgb(0.4, 0.2, 0.1), // Brown
            skin_tone: SkinTone::Fair,
            eye_color: Color::srgb(0.0, 0.4, 0.8), // Blue
            height: 1.0,
            weight: 1.0,
            clothing_set: ClothingSet::Peasant,
        }
    }
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct NPC {
    pub npc_type: NPCType,
    pub name: String,
    pub dialogue_tree: String, // YarnSpinner dialogue file
    pub shop_inventory: Option<Vec<String>>, // Item IDs if shopkeeper
    pub quest_giver: bool,
    pub reputation: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCType {
    Villager,
    Merchant,
    Innkeeper,
    Guard,
    Blacksmith,
    Priest,
    Noble,
    Hermit,
    QuestGiver,
}

#[derive(Component, Debug, Clone)]
pub struct Monster {
    pub monster_type: MonsterType,
    pub threat_level: u32,
    pub health: f32,
    pub max_health: f32,
    pub attack_damage: f32,
    pub movement_speed: f32,
    pub ai_state: AIState,
    pub loot_table: Vec<String>, // Item IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonsterType {
    // Early game (Threat 1-3)
    Wolf,
    Bandit,
    Goblin,
    
    // Mid game (Threat 4-7)
    SkeletonWarrior,
    CorruptedBeast,
    DarkWizard,
    
    // Late game (Threat 8-10)
    VoidCreature,
    DragonSpawn,
    NightmareEntity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AIState {
    Idle,
    Patrolling,
    Chasing,
    Attacking,
    Fleeing,
    Dead,
}

impl MonsterType {
    pub fn get_base_stats(&self) -> (f32, f32, f32, u32) {
        // Returns: (health, attack_damage, speed, threat_level)
        match self {
            MonsterType::Wolf => (30.0, 8.0, 1.2, 1),
            MonsterType::Bandit => (45.0, 12.0, 1.0, 2),
            MonsterType::Goblin => (25.0, 6.0, 1.4, 1),
            MonsterType::SkeletonWarrior => (60.0, 15.0, 0.8, 4),
            MonsterType::CorruptedBeast => (80.0, 20.0, 1.1, 5),
            MonsterType::DarkWizard => (50.0, 25.0, 0.9, 6),
            MonsterType::VoidCreature => (120.0, 35.0, 1.3, 8),
            MonsterType::DragonSpawn => (200.0, 45.0, 0.7, 9),
            MonsterType::NightmareEntity => (150.0, 40.0, 1.5, 10),
        }
    }
    
    pub fn get_model_path(&self) -> String {
        match self {
            MonsterType::Wolf => "models/characters/monsters/wolf.glb".to_string(),
            MonsterType::Bandit => "models/characters/monsters/bandit.glb".to_string(),
            MonsterType::Goblin => "models/characters/monsters/goblin.glb".to_string(),
            MonsterType::SkeletonWarrior => "models/characters/monsters/skeleton_warrior.glb".to_string(),
            MonsterType::CorruptedBeast => "models/characters/monsters/corrupted_beast.glb".to_string(),
            MonsterType::DarkWizard => "models/characters/monsters/dark_wizard.glb".to_string(),
            MonsterType::VoidCreature => "models/characters/monsters/void_creature.glb".to_string(),
            MonsterType::DragonSpawn => "models/characters/monsters/dragon_spawn.glb".to_string(),
            MonsterType::NightmareEntity => "models/characters/monsters/nightmare_entity.glb".to_string(),
        }
    }
}

impl NPCType {
    pub fn get_model_path(&self) -> String {
        match self {
            NPCType::Villager => "models/characters/npcs/villager.glb".to_string(),
            NPCType::Merchant => "models/characters/npcs/merchant.glb".to_string(),
            NPCType::Innkeeper => "models/characters/npcs/innkeeper.glb".to_string(),
            NPCType::Guard => "models/characters/npcs/guard.glb".to_string(),
            NPCType::Blacksmith => "models/characters/npcs/blacksmith.glb".to_string(),
            NPCType::Priest => "models/characters/npcs/priest.glb".to_string(),
            NPCType::Noble => "models/characters/npcs/noble.glb".to_string(),
            NPCType::Hermit => "models/characters/npcs/hermit.glb".to_string(),
            NPCType::QuestGiver => "models/characters/npcs/quest_giver.glb".to_string(),
        }
    }
    
    pub fn get_default_dialogue(&self) -> String {
        match self {
            NPCType::Villager => "dialogue/villager_generic.yarn".to_string(),
            NPCType::Merchant => "dialogue/merchant.yarn".to_string(),
            NPCType::Innkeeper => "dialogue/innkeeper.yarn".to_string(),
            NPCType::Guard => "dialogue/guard.yarn".to_string(),
            NPCType::Blacksmith => "dialogue/blacksmith.yarn".to_string(),
            NPCType::Priest => "dialogue/priest.yarn".to_string(),
            NPCType::Noble => "dialogue/noble.yarn".to_string(),
            NPCType::Hermit => "dialogue/hermit.yarn".to_string(),
            NPCType::QuestGiver => "dialogue/quest_giver.yarn".to_string(),
        }
    }
}

#[derive(Component, Debug)]
pub struct CharacterModel {
    pub base_model: Handle<Scene>,
    pub current_animation: Option<Handle<AnimationClip>>,
    pub scale_modifier: Vec3,
}

impl CharacterAppearance {
    pub fn get_player_model_path(&self, gender: &Gender) -> String {
        match gender {
            Gender::Male => "models/characters/player/male_base.glb".to_string(),
            Gender::Female => "models/characters/player/female_base.glb".to_string(),
        }
    }
    
    pub fn apply_customization(&self, transform: &mut Transform) {
        // Apply height and weight scaling
        transform.scale = Vec3::new(
            self.weight,  // Width scaling
            self.height,  // Height scaling
            self.weight,  // Depth scaling
        );
    }
}
