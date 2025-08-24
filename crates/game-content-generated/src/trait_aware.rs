//! Trait-aware content generation
//! 
//! This module shows how AI generation uses static trait/philosophy rules
//! to create content that respects the player's developed identity.

use game_content_static::{
    traits::{EmergentTrait, TraitCategory, WeaponMastery},
    philosophy::PhilosophicalIdentity,
    dread::DreadInfluence,
    DreadLevel,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generate NPC reactions based on player traits
pub fn generate_npc_reaction_prompt(
    traits: &HashMap<String, EmergentTrait>,
    philosophy: &PhilosophicalIdentity,
    dread_level: DreadLevel,
) -> String {
    let mut prompt = String::new();
    
    // Identify dominant traits
    let dominant_traits: Vec<_> = traits
        .iter()
        .filter(|(_, t)| t.is_dominant())
        .map(|(name, _)| name.clone())
        .collect();
    
    // Get dread influence on traits
    let dread_influence = DreadInfluence::for_level(dread_level.0);
    
    prompt.push_str(&format!(
        "Generate NPC dialogue reacting to a player with these characteristics:\n\n"
    ));
    
    prompt.push_str(&format!("DREAD LEVEL: {} - {}\n", 
        dread_level.stage_name(), 
        dread_level.description()
    ));
    
    prompt.push_str(&format!("\nDOMINANT TRAITS:\n"));
    for trait_name in &dominant_traits {
        prompt.push_str(&format!("- {}\n", trait_name));
    }
    
    if let Some(path) = philosophy.dominant_path {
        prompt.push_str(&format!("\nPHILOSOPHY: {:?} - {}\n", 
            path, 
            path.description()
        ));
    }
    
    prompt.push_str(&format!("\nTRAIT INFLUENCES AT THIS DREAD:\n"));
    prompt.push_str(&format!("Strengthened: {:?}\n", dread_influence.strengthened_traits));
    prompt.push_str(&format!("Weakened: {:?}\n", dread_influence.weakened_traits));
    
    prompt.push_str("\nGenerate varied NPC reactions including:\n");
    prompt.push_str("- Recognition of specific traits\n");
    prompt.push_str("- Fear/respect based on reputation\n");
    prompt.push_str("- Dialogue options that only appear for certain traits\n");
    prompt.push_str("- Reactions that change with dread level\n");
    
    prompt
}

/// Generate quest content that adapts to player philosophy
pub fn generate_philosophical_quest_prompt(
    philosophy: &PhilosophicalIdentity,
    act: u8,
) -> String {
    let mut prompt = String::new();
    
    prompt.push_str("Generate a quest that tests the player's philosophical development:\n\n");
    
    prompt.push_str(&format!("ACT: {} ", act));
    match act {
        1 => prompt.push_str("(Journey TO Labyrinth - establishing identity)\n"),
        2 => prompt.push_str("(Fighting the Dragon - testing philosophy)\n"),
        3 => prompt.push_str("(Sealing the Void - consequences)\n"),
        _ => prompt.push_str("\n"),
    }
    
    prompt.push_str(&format!("\nPLAYER PHILOSOPHY:\n"));
    prompt.push_str(&format!("Strength: {:.1}%\n", philosophy.strength_score * 100.0));
    prompt.push_str(&format!("Harmony: {:.1}%\n", philosophy.harmony_score * 100.0));
    prompt.push_str(&format!("Light: {:.1}%\n", philosophy.light_score * 100.0));
    prompt.push_str(&format!("Dark: {:.1}%\n", philosophy.dark_score * 100.0));
    
    if philosophy.internal_conflict > 0.5 {
        prompt.push_str(&format!("\nHIGH INTERNAL CONFLICT: {:.1}%\n", 
            philosophy.internal_conflict * 100.0
        ));
        prompt.push_str("Create a quest that forces them to choose a path.\n");
    }
    
    prompt.push_str("\nQuest should include:\n");
    prompt.push_str("- Choices that strengthen or weaken philosophical paths\n");
    prompt.push_str("- Consequences that reflect past philosophical choices\n");
    prompt.push_str("- NPCs who react to the player's philosophy\n");
    prompt.push_str("- Rewards that align with chosen path\n");
    
    prompt
}

/// Generate dialogue that acknowledges weapon mastery
pub fn generate_weapon_master_dialogue(
    weapon_mastery: &WeaponMastery,
) -> String {
    match weapon_mastery {
        WeaponMastery::Swordsman { proficiency, style } if *proficiency > 0.7 => {
            format!(
                "Generate dialogue where NPCs recognize the player as a master swordsman.\n\
                Style: {:?}\n\
                Proficiency: {:.0}%\n\
                Include: guards showing respect, enemies hesitating, \
                special dialogue options for sword masters.",
                style, proficiency * 100.0
            )
        }
        WeaponMastery::Archer { proficiency, .. } if *proficiency > 0.5 => {
            format!(
                "Generate dialogue acknowledging skilled archer.\n\
                Proficiency: {:.0}%\n\
                Include: recognition of keen eyes, steady hands, \
                requests for hunting/scouting help.",
                proficiency * 100.0
            )
        }
        WeaponMastery::Untrained => {
            "Generate dialogue treating player as inexperienced villager.\n\
            Include: offers to teach combat, warnings about danger, \
            underestimation by enemies.".to_string()
        }
        _ => "Generate standard combat dialogue.".to_string()
    }
}
