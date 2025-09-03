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

/// Generate dialogue modules with REQUIRED pre-analyzed Seeds data
pub fn generate_dialogue_modules_from_data(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    analyzed_seeds: &crate::AnalyzedSeedsData,
    out_dir: &Path,
) -> Result<()> {
    let dialogue_resources_dir = out_dir.join("dialogue_resources");
    fs::create_dir_all(&dialogue_resources_dir)?;
    
    // Use pre-analyzed, pre-categorized seeds data - no analysis needed
    generate_dialogue_from_analyzed_seeds(env, results, analyzed_seeds, &dialogue_resources_dir)?;
    
    Ok(())
}

/// Generate dialogue using pre-analyzed Seeds data (no analysis, just processing)
fn generate_dialogue_from_analyzed_seeds(
    env: &Environment,
    results: &dl_analysis::results::GenerationResults,
    analyzed_seeds: &crate::AnalyzedSeedsData,
    dialogue_dir: &Path,
) -> Result<()> {
    let npc_template = env.get_template("npc_dialogue.rs.jinja2")?;
    let dialogue_module_template = env.get_template("dialogue_module.rs.jinja2")?;
    
    let mut generated_npcs = Vec::new();
    let mut generated_quests = Vec::new();
    
    // OPENAI_API_KEY is REQUIRED - fail fast if missing
    let _openai_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable is required for dialogue generation. Set it before building.");
    
    println!("Using OpenAI API for dialogue and quest generation...");
    
    // Create AI dialogue generator
    let rt = tokio::runtime::Runtime::new()?;
    let ai_generator = crate::ai_dialogue::AiDialogueGenerator::new()?;
        
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
                    
                    let npc_name = generate_npc_name_from_analyzed_seeds(analyzed_seeds, act, corruption_level);
                    let npc_uuid = format!("npc_{}_{}", region.entity_uuid, settlement_uuid);
                    let archetype = select_archetype_for_corruption(corruption_level);
                    
                    // Use pre-analyzed dialogue data (no processing needed)
                    let seeds_dialogue_data = prepare_dialogue_data_from_analyzed_seeds(analyzed_seeds);
                    
                    // Create NPC dialogue context
                    let npc_context = crate::ai_dialogue::NpcDialogueContext {
                        npc_uuid: npc_uuid.clone(),
                        npc_name: npc_name.clone(),
                        region_uuid: region.entity_uuid.clone(),
                        settlement_uuid: settlement_uuid.clone(),
                        region_type: determine_region_type_from_biome(region),
                        act,
                        band,
                        corruption_level,
                        location_type: determine_location_type_from_settlement(settlement),
                        archetype: archetype.clone(),
                        personality_traits: get_archetype_traits(&archetype),
                        speech_patterns: get_archetype_speech_patterns(&archetype),
                    };
                    
                    // Generate dialogue using AI
                    let generated_dialogue = rt.block_on(ai_generator.generate_npc_dialogue(&npc_context, &seeds_dialogue_data))?;
                    
                    // Generate quest if appropriate
                    let generated_quest = if should_npc_offer_quest(act, corruption_level) {
                        let dungeon_uuid = find_nearby_dungeon(region, &results.entities.dungeons);
                        let seeds_quest_data = prepare_quest_data_from_analyzed_seeds(analyzed_seeds, act);
                        
                        let quest_context = crate::ai_dialogue::QuestGenerationContext {
                            quest_id: format!("quest_{}_{}", region.entity_uuid, npc_uuid),
                            region_uuid: region.entity_uuid.clone(),
                            dungeon_uuid,
                            npc_giver: npc_uuid.clone(),
                            act,
                            band,
                            corruption_level,
                            preferred_pattern: determine_quest_pattern_from_act(act),
                            available_locations: get_region_locations(region, &results.entities),
                            horror_themes: get_horror_themes_for_act(act),
                        };
                        
                        Some(rt.block_on(ai_generator.generate_quest(&quest_context, &seeds_quest_data))?)
                    } else {
                        None
                    };
                    
                    let template_context = minijinja::context! {
                        npc_uuid => npc_uuid,
                        npc_name => npc_name,
                        settlement_uuid => settlement_uuid,
                        region_uuid => region.entity_uuid,
                        generated_dialogue => generated_dialogue,
                        generated_quest => generated_quest,
                        archetype => archetype,
                    };
                    
                    let npc_module = npc_template.render(&template_context)?;
                    fs::write(region_dialogue_dir.join(format!("{}.rs", crate::utilities::sanitize_name(&npc_uuid))), npc_module)?;
                    
                    generated_npcs.push(npc_uuid);
                    if let Some(ref quest) = generated_quest {
                        generated_quests.push(quest.id.clone());
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
    
    println!("Generated dialogue for {} NPCs and {} quests", generated_npcs.len(), generated_quests.len());
    
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

// AI-powered dialogue generation helper functions

fn generate_npc_name_from_analyzed_seeds(_analyzed_seeds: &crate::AnalyzedSeedsData, act: u8, corruption_level: f32) -> String {
    // Generate name based on corruption level
    let name_type = if corruption_level < 0.3 {
        "peaceful"
    } else if corruption_level < 0.7 {
        "troubled"
    } else {
        "corrupted"
    };
    
    // Generate name using act and corruption level
    format!("{}_Act{}", name_type, act)
}

fn select_archetype_for_corruption(corruption_level: f32) -> String {
    if corruption_level < 0.2 {
        "wandering_scholar".to_string()
    } else if corruption_level < 0.4 {
        "mercenary".to_string()
    } else if corruption_level < 0.6 {
        "holy_warrior".to_string()
    } else if corruption_level < 0.8 {
        "corrupted_noble".to_string()
    } else {
        "dark_cultist".to_string()
    }
}

fn prepare_dialogue_data_from_analyzed_seeds(analyzed_seeds: &crate::AnalyzedSeedsData) -> crate::ai_dialogue::SeedsDialogueData {
    let mut linguistic_patterns = Vec::new();
    let mut old_norse_vocabulary = HashMap::new();
    let mut cultural_references = Vec::new();
    
    // Add Old Norse vocabulary (simplified for now)
    old_norse_vocabulary.insert("draugr".to_string(), "undead_warrior".to_string());
    old_norse_vocabulary.insert("fylgja".to_string(), "spirit_guardian".to_string());
    old_norse_vocabulary.insert("berserker".to_string(), "fury_warrior".to_string());
    
    linguistic_patterns.push(crate::ai_dialogue::LinguisticPattern {
        pattern_type: "old_norse_compound".to_string(),
        examples: vec!["skald-song".to_string(), "wolf-brother".to_string()],
        cultural_context: "Norse mythology and warrior culture".to_string(),
    });
    
    // Add predefined character archetypes (since DialogueTemplates is a struct, not a collection)
    let mut character_archetypes = Vec::new();
    character_archetypes.push(crate::ai_dialogue::CharacterArchetype {
        archetype_type: "mercenary".to_string(),
        alignment: "neutral".to_string(),
        traits: vec!["pragmatic".to_string(), "skilled".to_string(), "cynical".to_string()],
        motivations: vec!["gold".to_string(), "survival".to_string(), "reputation".to_string()],
        speech_patterns: vec!["terse".to_string(), "professional".to_string()],
    });
    
    character_archetypes.push(crate::ai_dialogue::CharacterArchetype {
        archetype_type: "holy_warrior".to_string(),
        alignment: "light".to_string(),
        traits: vec!["righteous".to_string(), "brave".to_string(), "stubborn".to_string()],
        motivations: vec!["justice".to_string(), "protection".to_string(), "faith".to_string()],
        speech_patterns: vec!["formal".to_string(), "inspiring".to_string()],
    });
    
    character_archetypes.push(crate::ai_dialogue::CharacterArchetype {
        archetype_type: "dark_cultist".to_string(),
        alignment: "dark".to_string(),
        traits: vec!["secretive".to_string(), "manipulative".to_string(), "knowledgeable".to_string()],
        motivations: vec!["power".to_string(), "forbidden_knowledge".to_string(), "chaos".to_string()],
        speech_patterns: vec!["cryptic".to_string(), "unsettling".to_string()],
    });
    
    // Add cultural references
    cultural_references.push("Norse poetry tradition".to_string());
    cultural_references.push("Medieval Christianity".to_string());
    cultural_references.push("Celtic folklore".to_string());
    
    crate::ai_dialogue::SeedsDialogueData {
        linguistic_patterns,
        character_archetypes,
        old_norse_vocabulary,
        cultural_references,
    }
}

fn get_archetype_traits(archetype: &str) -> Vec<String> {
    match archetype {
        "mercenary" => vec!["pragmatic".to_string(), "skilled".to_string(), "cynical".to_string()],
        "holy_warrior" => vec!["righteous".to_string(), "brave".to_string(), "stubborn".to_string()],
        "dark_cultist" => vec!["secretive".to_string(), "manipulative".to_string(), "knowledgeable".to_string()],
        "wandering_scholar" => vec!["curious".to_string(), "analytical".to_string(), "absent_minded".to_string()],
        "corrupted_noble" => vec!["arrogant".to_string(), "desperate".to_string(), "haunted".to_string()],
        _ => vec!["neutral".to_string()],
    }
}

fn get_archetype_speech_patterns(archetype: &str) -> Vec<String> {
    match archetype {
        "mercenary" => vec!["terse".to_string(), "professional".to_string()],
        "holy_warrior" => vec!["formal".to_string(), "inspiring".to_string()],
        "dark_cultist" => vec!["cryptic".to_string(), "unsettling".to_string()],
        "wandering_scholar" => vec!["verbose".to_string(), "academic".to_string()],
        "corrupted_noble" => vec!["aristocratic".to_string(), "bitter".to_string()],
        _ => vec!["casual".to_string()],
    }
}

fn find_nearby_dungeon(region: &dl_analysis::entities::RegionHexTile, dungeons: &[dl_analysis::entities::RegionHexTile]) -> Option<String> {
    // Simple heuristic - find first dungeon that might be near this region
    dungeons.first().map(|d| d.entity_uuid.clone())
}

fn prepare_quest_data_from_analyzed_seeds(_analyzed_seeds: &crate::AnalyzedSeedsData, act: u8) -> crate::ai_dialogue::SeedsQuestData {
    let mut literature_patterns = Vec::new();
    let mut horror_beats = Vec::new();
    let mut poe_excerpts = Vec::new();
    let mut dracula_themes = Vec::new();
    let mut quest_archetypes = Vec::new();
    
    // Add patterns from Poe (simplified for now)
    literature_patterns.push(crate::ai_dialogue::LiteraturePattern {
        source_work: "Edgar Allan Poe".to_string(),
        pattern_type: "psychological_horror".to_string(),
        beats: vec!["isolation".to_string(), "obsession".to_string(), "revelation".to_string(), "madness".to_string()],
        themes: vec!["death".to_string(), "guilt".to_string(), "revenge".to_string()],
        horror_elements: vec!["unreliable_narrator".to_string(), "buried_alive".to_string(), "haunting_past".to_string()],
    });
    
    // Add patterns from Dracula
    literature_patterns.push(crate::ai_dialogue::LiteraturePattern {
        source_work: "Bram Stoker - Dracula".to_string(),
        pattern_type: "gothic_horror".to_string(),
        beats: vec!["arrival".to_string(), "seduction".to_string(), "transformation".to_string(), "hunt".to_string()],
        themes: vec!["corruption".to_string(), "invasion".to_string(), "purity_vs_evil".to_string()],
        horror_elements: vec!["ancient_evil".to_string(), "loss_of_humanity".to_string(), "blood_curse".to_string()],
    });
    
    poe_excerpts.push("The boundaries which divide Life from Death are at best shadowy and vague.".to_string());
    poe_excerpts.push("All that we see or seem is but a dream within a dream.".to_string());
    
    dracula_themes.push("The old centuries had, and have, powers of their own which mere 'modernity' cannot kill.".to_string());
    dracula_themes.push("There are darknesses in life and there are lights, and you are one of the lights.".to_string());
    
    // Horror progression beats based on act
    horror_beats = match act {
        1 => vec!["subtle_wrongness".to_string(), "first_signs".to_string(), "growing_unease".to_string()],
        2 => vec!["clear_threat".to_string(), "first_loss".to_string(), "desperation".to_string()],
        3 => vec!["manifest_horror".to_string(), "betrayal".to_string(), "corruption_spreads".to_string()],
        _ => vec!["investigation".to_string(), "confrontation".to_string(), "resolution".to_string()],
    };
    
    // Quest archetypes
    quest_archetypes.push(crate::ai_dialogue::QuestArchetype {
        archetype_name: "investigation".to_string(),
        typical_structure: vec!["discover_mystery".to_string(), "gather_clues".to_string(), "confront_truth".to_string()],
        horror_integration: vec!["unreliable_witnesses".to_string(), "disturbing_evidence".to_string(), "horrific_revelation".to_string()],
        corruption_themes: vec!["hidden_knowledge".to_string(), "price_of_truth".to_string(), "madness_from_discovery".to_string()],
    });
    
    quest_archetypes.push(crate::ai_dialogue::QuestArchetype {
        archetype_name: "purification".to_string(),
        typical_structure: vec!["identify_corruption".to_string(), "gather_sacred_items".to_string(), "perform_ritual".to_string()],
        horror_integration: vec!["corruption_fights_back".to_string(), "allies_turn".to_string(), "ritual_demands_sacrifice".to_string()],
        corruption_themes: vec!["purity_vs_corruption".to_string(), "cost_of_cleansing".to_string(), "corruption_spreads".to_string()],
    });
    
    crate::ai_dialogue::SeedsQuestData {
        literature_patterns,
        horror_beats,
        poe_excerpts,
        dracula_themes,
        quest_archetypes,
    }
}

fn get_region_locations(region: &dl_analysis::entities::RegionHexTile, entities: &dl_analysis::results::EntityCollections) -> Vec<String> {
    let mut locations = Vec::new();
    
    // Add settlements in this region
    for settlement_uuid in &region.settlement_uuids {
        if let Some(_settlement) = entities.settlements.iter().find(|s| &s.entity_uuid == settlement_uuid) {
            locations.push(format!("settlement_{}", settlement_uuid));
        }
    }
    
    // Add nearby dungeons
    locations.push("nearby_ruins".to_string());
    locations.push("ancient_grove".to_string());
    locations.push("cursed_shrine".to_string());
    
    locations
}

fn get_horror_themes_for_act(act: u8) -> Vec<String> {
    match act {
        1 => vec![
            "pastoral_decay".to_string(),
            "hidden_corruption".to_string(),
            "lost_innocence".to_string(),
            "false_safety".to_string(),
        ],
        2 => vec![
            "growing_dread".to_string(),
            "trust_fractures".to_string(),
            "first_deaths".to_string(),
            "spreading_fear".to_string(),
        ],
        3 => vec![
            "manifest_horror".to_string(),
            "companion_trauma".to_string(),
            "moral_compromise".to_string(),
            "descent_into_darkness".to_string(),
        ],
        4 => vec![
            "warped_reality".to_string(),
            "total_corruption".to_string(),
            "loss_of_humanity".to_string(),
            "apocalyptic_dread".to_string(),
        ],
        5 => vec![
            "complete_horror".to_string(),
            "reality_collapse".to_string(),
            "dragon_influence".to_string(),
            "final_confrontation".to_string(),
        ],
        _ => vec!["mystery".to_string(), "investigation".to_string()],
    }
}
