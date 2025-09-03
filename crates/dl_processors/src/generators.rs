//! Code generation implementations for world resources

use anyhow::Result;
use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate hex tile modules from real HBF data using templates
pub fn generate_hex_tiles_from_data(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    out_dir: &Path,
) -> Result<()> {
    let hex_template = env.get_template("hex_tile.rs.jinja2")?;
    let hex_resources_dir = out_dir.join("hex_resources");
    fs::create_dir_all(&hex_resources_dir)?;
    
    // Process each region and generate hex tiles
    for region in &results.entities.regions {
        let region_dir = hex_resources_dir.join("regions").join(&crate::utilities::sanitize_name(&region.entity_uuid));
        fs::create_dir_all(&region_dir)?;
        
        // Extract hex coordinates from actual region data
        let hex_coords = crate::utilities::extract_hex_coordinates_from_region_properly(region)?;
        
        let hex_coords_clone = hex_coords.clone();
        for coords in hex_coords {
            // Get correlated entities at this hex from analysis
            let settlements = crate::utilities::get_settlements_at_hex_from_analysis(results, coords);
            let factions = crate::utilities::get_factions_at_hex_from_analysis(results, coords);
            let npcs = crate::utilities::get_npcs_at_hex_from_analysis(results, coords);
            let dungeons = crate::utilities::get_dungeons_at_hex_from_analysis(results, coords);
            
            let hex_context = minijinja::context! {
                q => coords.0,
                r => coords.1,
                region_uuid => region.entity_uuid,
                settlements => settlements,
                factions => factions,
                npcs => npcs,
                dungeons => dungeons,
            };
            
            let hex_module = hex_template.render(&hex_context)?;
            fs::write(region_dir.join(format!("hex_{}_{}.rs", coords.0, coords.1)), hex_module)?;
        }
        
        // Generate region module
        let region_template = env.get_template("region_module.rs.jinja2")?;
        let region_context = minijinja::context! {
            region_uuid => region.entity_uuid,
            hex_coords => hex_coords_clone,
        };
        let region_module = region_template.render(&region_context)?;
        fs::write(region_dir.join("mod.rs"), region_module)?;
    }
    
    Ok(())
}

/// Generate dungeon area modules from real HBF data using templates
pub fn generate_dungeon_areas_from_data(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    out_dir: &Path,
) -> Result<()> {
    let area_template = env.get_template("dungeon_area.rs.jinja2")?;
    let dungeon_resources_dir = out_dir.join("dungeon_resources");
    fs::create_dir_all(&dungeon_resources_dir)?;
    
    // Process each dungeon and generate area modules
    for dungeon in &results.entities.dungeons {
        let dungeon_dir = dungeon_resources_dir.join("dungeons").join(&crate::utilities::sanitize_name(&dungeon.entity_uuid));
        fs::create_dir_all(&dungeon_dir)?;
        
        // Extract area data from actual dungeon data
        let areas = crate::utilities::extract_areas_from_dungeon_properly(dungeon)?;
        let areas_clone = areas.clone();
        
        for area in areas {
            let area_context = minijinja::context! {
                dungeon_uuid => dungeon.entity_uuid,
                area_uuid => area.uuid,
                area_name => area.name,
                monsters => area.monsters,
                treasures => area.treasures,
                connections => area.connections,
            };
            
            let area_module = area_template.render(&area_context)?;
            fs::write(dungeon_dir.join(format!("{}.rs", crate::utilities::sanitize_name(&area.uuid))), area_module)?;
        }
        
        // Generate dungeon module
        let dungeon_template = env.get_template("dungeon_module.rs.jinja2")?;
        let dungeon_context = minijinja::context! {
            dungeon_uuid => dungeon.entity_uuid,
            areas => areas_clone,
        };
        let dungeon_module = dungeon_template.render(&dungeon_context)?;
        fs::write(dungeon_dir.join("mod.rs"), dungeon_module)?;
    }
    
    Ok(())
}

/// Generate dialogue modules with Seeds data integration
pub fn generate_dialogue_modules_from_data(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    seeds_manager: &Option<dl_analysis::seeds::SeedsDataManager>,
    out_dir: &Path,
) -> Result<()> {
    let dialogue_resources_dir = out_dir.join("dialogue_resources");
    fs::create_dir_all(&dialogue_resources_dir)?;
    
    // If we have Seeds data, generate advanced dialogue
    if let Some(seeds) = seeds_manager {
        generate_seeds_powered_dialogue(env, results, seeds, &dialogue_resources_dir)?;
    } else {
        generate_fallback_dialogue(env, results, &dialogue_resources_dir)?;
    }
    
    Ok(())
}

/// Generate dialogue using Seeds linguistic processing and templates
fn generate_seeds_powered_dialogue(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    seeds: &dl_analysis::seeds::SeedsDataManager,
    dialogue_dir: &Path,
) -> Result<()> {
    use dl_analysis::seeds::{NameGenerationContext, DialogueGenerationContext, QuestGenerationContext};
    
    let npc_template = env.get_template("npc_dialogue.rs.jinja2")?;
    let dialogue_module_template = env.get_template("dialogue_module.rs.jinja2")?;
    
    let mut generated_npcs = Vec::new();
    let mut generated_quests = Vec::new();
    
    // Generate dialogue for each region's NPCs
    for region in &results.entities.regions {
        let region_dialogue_dir = dialogue_dir.join("regions").join(&crate::utilities::sanitize_name(&region.entity_uuid));
        fs::create_dir_all(&region_dialogue_dir)?;
        
        // Determine Act/Band from region corruption or UUID pattern
        let (act, band) = determine_act_band_from_region(region);
        let corruption_level = calculate_corruption_level(act, band);
        
        // Generate NPCs for this region based on settlements
        for settlement_uuid in &region.settlement_uuids {
            if let Some(settlement) = results.entities.settlements.iter()
                .find(|s| &s.entity_uuid == settlement_uuid) {
                
                // Generate NPC name using Seeds linguistic processing
                let name_context = NameGenerationContext {
                    region_type: determine_region_type_from_biome(region),
                    act,
                    band,
                    corruption_level,
                };
                
                let npc_name = seeds.generate_name("villager", &name_context);
                let npc_uuid = format!("npc_{}_{}", region.entity_uuid, settlement_uuid);
                
                // Generate dialogue for this NPC
                let dialogue_context = DialogueGenerationContext {
                    player_name: "stranger".to_string(),
                    player_title: "traveler".to_string(),
                    time_of_day: "day".to_string(),
                    location_type: determine_location_type_from_settlement(settlement),
                    npc_tone: determine_npc_tone_from_corruption(corruption_level),
                    should_offer_quest: should_npc_offer_quest(act, corruption_level),
                    npc_uuid: npc_uuid.clone(),
                };
                
                let dialogue_lines = seeds.generate_dialogue(&dialogue_context)?;
                
                // Generate quest if appropriate
                let quest = if dialogue_context.should_offer_quest {
                    let quest_context = QuestGenerationContext {
                        region_uuid: region.entity_uuid.clone(),
                        npc_uuid: npc_uuid.clone(),
                        preferred_pattern: determine_quest_pattern_from_act(act),
                        act,
                        corruption_level,
                    };
                    Some(seeds.generate_quest(&quest_context)?)
                } else {
                    None
                };
                
                let npc_context = minijinja::context! {
                    npc_uuid => npc_uuid,
                    npc_name => npc_name,
                    settlement_uuid => settlement_uuid,
                    region_uuid => region.entity_uuid,
                    dialogue_lines => dialogue_lines,
                    quest => quest,
                };
                
                let npc_module = npc_template.render(&npc_context)?;
                fs::write(region_dialogue_dir.join(format!("{}.rs", crate::utilities::sanitize_name(&npc_uuid))), npc_module)?;
                
                generated_npcs.push(npc_uuid);
                if let Some(q) = quest {
                    generated_quests.push(q.id);
                }
            }
        }
    }
    
    // Generate main dialogue module
    let dialogue_context = minijinja::context! {
        regions => results.entities.regions,
        generated_npcs => generated_npcs,
        generated_quests => generated_quests,
        seeds_powered => true,
    };
    
    let dialogue_module = dialogue_module_template.render(&dialogue_context)?;
    fs::write(dialogue_dir.join("mod.rs"), dialogue_module)?;
    
    Ok(())
}

/// Generate basic fallback dialogue when Seeds data is not available
fn generate_fallback_dialogue(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    dialogue_dir: &Path,
) -> Result<()> {
    let dialogue_module_template = env.get_template("dialogue_module.rs.jinja2")?;
    
    // Create basic dialogue context
    let dialogue_context = minijinja::context! {
        regions => results.entities.regions,
        generated_npcs => Vec::<String>::new(),
        generated_quests => Vec::<String>::new(),
        seeds_powered => false,
    };
    
    let dialogue_module = dialogue_module_template.render(&dialogue_context)?;
    fs::write(dialogue_dir.join("mod.rs"), dialogue_module)?;
    
    Ok(())
}

// Helper functions for dialogue generation

fn determine_act_band_from_region(region: &dl_analysis::entities::RegionHexTile) -> (u8, u8) {
    // Simple heuristic based on UUID or content
    let hash = crate::utilities::simple_hash(&region.entity_uuid);
    let act = ((hash % 3) + 1) as u8;
    let band = ((hash % 60) + 1) as u8;
    (act, band)
}

fn calculate_corruption_level(act: u8, band: u8) -> f32 {
    match act {
        1 => (band as f32 / 60.0) * 0.3, // Peace to Dread (0.0 to 0.3)
        2 => 0.3 + ((band as f32 / 60.0) * 0.4), // Dread to Terror (0.3 to 0.7)
        3 => 0.7 + ((band as f32 / 60.0) * 0.3), // Terror to Horror (0.7 to 1.0)
        _ => 0.5,
    }
}

fn determine_region_type_from_biome(_region: &dl_analysis::entities::RegionHexTile) -> String {
    // Could analyze biome information from region content
    "meadows".to_string() // Fallback
}

fn determine_location_type_from_settlement(_settlement: &dl_analysis::entities::SettlementEstablishment) -> String {
    "village".to_string() // Fallback
}

fn determine_npc_tone_from_corruption(corruption_level: f32) -> String {
    if corruption_level < 0.3 {
        "friendly".to_string()
    } else if corruption_level < 0.7 {
        "formal".to_string()
    } else {
        "grumpy".to_string()
    }
}

fn should_npc_offer_quest(act: u8, corruption_level: f32) -> bool {
    // More likely to offer quests in middle acts
    match act {
        1 => corruption_level > 0.1,
        2 => true,
        3 => corruption_level < 0.9,
        _ => false,
    }
}

fn determine_quest_pattern_from_act(act: u8) -> String {
    match act {
        1 => "investigation".to_string(),
        2 => "purification".to_string(), 
        3 => "escort".to_string(),
        _ => "investigation".to_string(),
    }
}
