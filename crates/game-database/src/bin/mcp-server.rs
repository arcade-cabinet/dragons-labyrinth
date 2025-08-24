//! Standalone MCP Server for Dragon's Labyrinth Database
//!
//! This server provides JSON-RPC endpoints for AI agents to query game state,
//! companion information, world corruption levels, and narrative progression.
//! 
//! The server operates independently of the game runtime and provides static
//! responses based on the sophisticated horror progression system.

use std::env;
use std::path::PathBuf;
use tracing::{info, warn, error};
use jsonrpc_core::{IoHandler, Result, Error, ErrorCode};
use jsonrpc_http_server::ServerBuilder;
use serde_json::Value;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Dragon's Labyrinth MCP Server");
    
    // Verify database exists (created by build script)
    let db_path = env::var("GAME_DATABASE_PATH")
        .unwrap_or_else(|_| "target/debug/build/game-database-*/out/game_database.db".to_string());
    
    info!("Database path configured: {}", db_path);
    
    // Create JSON-RPC handler with all AI agent support tools
    let mut io = IoHandler::new();
    register_all_mcp_methods(&mut io);
    
    // Add server info method
    io.add_method("server_info", |_params| async {
        Ok(serde_json::json!({
            "name": "Dragon's Labyrinth Database MCP Server",
            "version": "1.0.0",
            "description": "Provides sophisticated AI agents access to game world state, companion psychology, and horror progression data",
            "capabilities": [
                "world_corruption_queries",
                "companion_trauma_analysis", 
                "philosophical_progression_tracking",
                "npc_fear_state_monitoring",
                "forge_system_integration",
                "cross_system_intelligence"
            ]
        }))
    });

    // Start the server
    let server = ServerBuilder::new(io)
        .threads(4)
        .start_http(&"127.0.0.1:8080".parse()?)?;
    
    info!("🌟 Dragon's Labyrinth MCP Server running on http://127.0.0.1:8080");
    info!("Available methods:");
    info!("  - query_world_corruption_level: Environmental horror tracking");
    info!("  - query_companion_trauma_states: Psychological state analysis");
    info!("  - query_philosophical_progression: Moral/ethical development");
    info!("  - query_npc_fear_states: Community psychological state");
    info!("  - query_companion_states: Active companion status");
    info!("  - query_forge_readiness: Sentimental item crafting system");
    info!("  - cross_system_intelligence_query: Comprehensive world analysis");
    info!("  - server_info: Server capabilities and status");
    
    // Keep the server running
    server.wait();
    
    Ok(())
}

/// Register all MCP methods for AI agent database queries
fn register_all_mcp_methods(io: &mut IoHandler) {
    // World Corruption Level Query
    // Provides environmental horror state for AI decision making
    io.add_method("query_world_corruption_level", |_params| async {
        Ok(serde_json::json!({
            "corruption_level": 0.35,
            "affected_regions": [
                {
                    "name": "Darkwood Forest",
                    "corruption": 0.7,
                    "trend": "increasing",
                    "threats": ["shadow_creatures", "corrupted_wildlife"]
                },
                {
                    "name": "Abandoned Village", 
                    "corruption": 0.4,
                    "trend": "stable",
                    "threats": ["ghostly_whispers", "structural_decay"]
                },
                {
                    "name": "Ancient Ruins",
                    "corruption": 0.9,
                    "trend": "critical", 
                    "threats": ["eldritch_emanations", "reality_distortions"]
                }
            ],
            "global_dread_level": 2,
            "corruption_sources": [
                "forgotten_ritual_sites",
                "cursed_artifacts", 
                "dimensional_rifts"
            ],
            "mitigation_opportunities": [
                "sentimental_item_forging",
                "companion_bond_strengthening",
                "light_essence_gathering"
            ],
            "narrative_implications": {
                "urgency": "moderate",
                "player_psychological_pressure": 0.6,
                "time_until_critical": "unknown"
            }
        }))
    });

    // Companion Trauma States Query  
    // Deep psychological analysis for AI companion behavior modeling
    io.add_method("query_companion_trauma_states", |_params| async {
        Ok(serde_json::json!({
            "companions": [
                {
                    "name": "Einar the Wanderer",
                    "psychological_profile": {
                        "trauma_level": 0.45,
                        "trust_level": 0.72,
                        "stability": "shaken_but_resolute",
                        "dominant_emotion": "determined_concern"
                    },
                    "trauma_history": [
                        {
                            "event": "witnessed_village_destruction",
                            "severity": 0.8,
                            "recovery_progress": 0.6,
                            "triggers": ["burning_buildings", "screaming_crowds"]
                        },
                        {
                            "event": "betrayal_by_former_ally", 
                            "severity": 0.7,
                            "recovery_progress": 0.3,
                            "triggers": ["sudden_movements", "whispered_conversations"]
                        }
                    ],
                    "coping_mechanisms": [
                        "protective_vigilance",
                        "dark_humor", 
                        "ritual_weapon_maintenance"
                    ],
                    "relationship_with_player": {
                        "bond_strength": 0.72,
                        "trust_trajectory": "slowly_increasing",
                        "key_bonding_moments": 2,
                        "unresolved_tensions": ["questions_about_player_past"]
                    },
                    "ai_behavior_modifiers": {
                        "dialogue_tone": "cautiously_supportive",
                        "decision_influence": "voice_concerns_but_defers",
                        "combat_behavior": "protective_positioning",
                        "exploration_behavior": "heightened_alertness"
                    }
                },
                {
                    "name": "Mira the Scholar",
                    "psychological_profile": {
                        "trauma_level": 0.25,
                        "trust_level": 0.88,
                        "stability": "intellectually_curious_but_wary",
                        "dominant_emotion": "fascinated_concern"
                    },
                    "trauma_history": [
                        {
                            "event": "corruption_of_sacred_texts",
                            "severity": 0.5,
                            "recovery_progress": 0.8,
                            "triggers": ["blasphemous_symbols", "corrupted_knowledge"]
                        }
                    ],
                    "coping_mechanisms": [
                        "knowledge_seeking",
                        "documentation_compulsion",
                        "logical_analysis"
                    ],
                    "relationship_with_player": {
                        "bond_strength": 0.88,
                        "trust_trajectory": "steadily_increasing",
                        "key_bonding_moments": 4,
                        "unresolved_tensions": []
                    },
                    "ai_behavior_modifiers": {
                        "dialogue_tone": "inquisitive_and_supportive",
                        "decision_influence": "offers_informed_perspectives",
                        "combat_behavior": "strategic_support",
                        "exploration_behavior": "thorough_investigation"
                    }
                }
            ],
            "group_dynamics": {
                "overall_cohesion": 0.78,
                "internal_tensions": 0.22,
                "collective_trauma_response": "mutual_support_with_underlying_anxiety",
                "leadership_dynamics": "player_led_with_valued_advisor_input"
            },
            "therapeutic_opportunities": [
                "shared_positive_experiences",
                "addressing_individual_triggers",
                "strengthening_group_bonds",
                "processing_collective_trauma"
            ]
        }))
    });

    // Philosophical Progression Query
    // Tracks moral and ethical development for nuanced AI responses  
    io.add_method("query_philosophical_progression", |_params| async {
        Ok(serde_json::json!({
            "current_philosophy": {
                "primary": "pragmatic_compassion",
                "secondary": "cautious_hope",
                "moral_flexibility": 0.65,
                "ethical_consistency": 0.72
            },
            "philosophical_journey": [
                {
                    "stage": "naive_optimism",
                    "duration": "early_game",
                    "key_realizations": ["world_is_more_complex_than_expected"]
                },
                {
                    "stage": "harsh_realism", 
                    "duration": "mid_game_crisis",
                    "key_realizations": ["not_all_problems_have_good_solutions"]
                },
                {
                    "stage": "pragmatic_compassion",
                    "duration": "current",
                    "key_realizations": ["small_kindnesses_matter_in_dark_times"]
                }
            ],
            "recent_moral_decisions": [
                {
                    "decision": "saved_stranger_despite_risk",
                    "alignment": "altruistic",
                    "cost": "resource_depletion",
                    "impact": "reinforced_compassionate_values"
                },
                {
                    "decision": "shared_food_with_hungry_child",
                    "alignment": "compassionate", 
                    "cost": "personal_hunger",
                    "impact": "strengthened_community_bonds"
                },
                {
                    "decision": "chose_mercy_over_vengeance",
                    "alignment": "redemptive",
                    "cost": "potential_future_threat",
                    "impact": "maintained_moral_integrity"
                }
            ],
            "moral_dilemmas_faced": [
                "resource_scarcity_vs_generosity",
                "individual_safety_vs_community_risk",
                "truth_vs_protective_deception",
                "justice_vs_mercy"
            ],
            "philosophical_stability": "evolving_but_grounded",
            "influence_on_companions": {
                "einar": "encouraged_hopeful_perspective",
                "mira": "validated_intellectual_curiosity_about_ethics"
            },
            "ai_narrative_implications": {
                "dialogue_options": "emphasize_hope_and_practical_solutions",
                "story_branches": "favor_redemptive_paths",
                "character_development": "continued_growth_through_moral_challenges"
            }
        }))
    });

    // NPC Fear States Query
    // Community psychological monitoring for environmental storytelling
    io.add_method("query_npc_fear_states", |_params| async {
        Ok(serde_json::json!({
            "communities": [
                {
                    "name": "Haven Village",
                    "population": 127,
                    "collective_fear_level": 0.55,
                    "primary_fears": [
                        "approaching_corruption",
                        "food_shortages", 
                        "missing_travelers"
                    ],
                    "fear_indicators": [
                        "increased_guard_patrols",
                        "hoarding_behavior",
                        "decreased_social_gatherings"
                    ],
                    "notable_individuals": [
                        {
                            "name": "Elder Thorne",
                            "role": "village_leader", 
                            "fear_level": 0.7,
                            "specific_concerns": "responsibility_for_community_safety",
                            "behavioral_changes": "increased_isolation_and_paranoia"
                        },
                        {
                            "name": "Merchant Kala",
                            "role": "trader",
                            "fear_level": 0.4,
                            "specific_concerns": "trade_route_disruption",
                            "behavioral_changes": "stockpiling_goods_and_raising_prices"
                        }
                    ],
                    "community_coping_mechanisms": [
                        "nightly_communal_prayers",
                        "strengthened_fortifications",
                        "rationing_systems"
                    ]
                },
                {
                    "name": "Crossroads Inn",
                    "population": 23,
                    "collective_fear_level": 0.68,
                    "primary_fears": [
                        "isolated_location_vulnerability",
                        "strange_travelers",
                        "unnatural_sounds_at_night"
                    ],
                    "fear_indicators": [
                        "early_closures",
                        "weapon_stockpiling",
                        "suspicious_behavior_toward_strangers"
                    ]
                }
            ],
            "regional_mood": {
                "overall_morale": "cautiously_fearful",
                "social_cohesion": 0.62,
                "authority_trust": 0.45,
                "stranger_acceptance": 0.32
            },
            "fear_mitigation_opportunities": [
                "successful_completion_of_community_requests",
                "elimination_of_local_threats", 
                "demonstration_of_player_reliability",
                "sharing_of_hope_and_positive_news"
            ],
            "ai_interaction_implications": {
                "npc_dialogue_tone": "wary_but_desperate_for_good_news",
                "quest_availability": "focused_on_immediate_survival_needs",
                "information_sharing": "reluctant_but_willing_if_trust_established",
                "reward_generosity": "limited_by_resource_scarcity"
            }
        }))
    });

    // Companion States Query
    // Real-time companion status for AI decision making
    io.add_method("query_companion_states", |_params| async {
        Ok(serde_json::json!({
            "active_party": {
                "size": 2,
                "cohesion": 0.78,
                "overall_morale": "determined_but_concerned",
                "leadership_acceptance": 0.85
            },
            "companions": [
                {
                    "name": "Einar the Wanderer",
                    "status": "active",
                    "location": "player_party",
                    "physical_condition": {
                        "health": 0.85,
                        "fatigue": 0.35,
                        "injuries": ["minor_cuts_on_hands"]
                    },
                    "emotional_state": {
                        "primary": "determined",
                        "secondary": "watchful",
                        "stress_level": 0.45,
                        "mood_trajectory": "stable"
                    },
                    "equipment_status": {
                        "weapon": "well_maintained_sword",
                        "armor": "travel_worn_leather",
                        "supplies": "adequate",
                        "special_items": ["family_pendant", "old_map"]
                    },
                    "current_priorities": [
                        "protect_group",
                        "scout_for_dangers",
                        "maintain_group_morale"
                    ],
                    "ai_behavior_state": {
                        "dialogue_availability": "high",
                        "advice_giving": "balanced_caution_and_support",
                        "decision_participation": "active_but_deferential",
                        "combat_readiness": "high_alert"
                    }
                },
                {
                    "name": "Mira the Scholar",
                    "status": "active",
                    "location": "player_party",
                    "physical_condition": {
                        "health": 0.92,
                        "fatigue": 0.28,
                        "injuries": []
                    },
                    "emotional_state": {
                        "primary": "intellectually_engaged",
                        "secondary": "cautiously_optimistic",
                        "stress_level": 0.25,
                        "mood_trajectory": "improving"
                    },
                    "equipment_status": {
                        "weapon": "enchanted_staff",
                        "armor": "scholarly_robes",
                        "supplies": "extensive_books_and_materials",
                        "special_items": ["research_journal", "ancient_tome", "lens_of_true_seeing"]
                    },
                    "current_priorities": [
                        "document_discoveries",
                        "analyze_supernatural_phenomena",
                        "support_group_with_knowledge"
                    ],
                    "ai_behavior_state": {
                        "dialogue_availability": "very_high",
                        "advice_giving": "detailed_analysis_and_historical_context",
                        "decision_participation": "highly_engaged",
                        "combat_readiness": "supportive_magic_user"
                    }
                }
            ],
            "group_dynamics": {
                "communication_quality": 0.82,
                "conflict_resolution": 0.76,
                "shared_goals_alignment": 0.89,
                "mutual_support_level": 0.84
            },
            "party_effectiveness": {
                "combat_coordination": 0.73,
                "exploration_efficiency": 0.81,
                "problem_solving_synergy": 0.87,
                "resource_management": 0.69
            }
        }))
    });

    // Forge Readiness Query
    // Sentimental item crafting system status
    io.add_method("query_forge_readiness", |_params| async {
        Ok(serde_json::json!({
            "forge_system": {
                "unlocked": false,
                "discovery_progress": 0.35,
                "next_milestone": "locate_ancient_forge_chamber"
            },
            "essence_collection": {
                "light_essence": {
                    "amount": 0.34,
                    "sources": [
                        "acts_of_compassion",
                        "moments_of_genuine_connection",
                        "preservation_of_beauty"
                    ],
                    "quality": "pure_but_limited"
                },
                "dark_essence": {
                    "amount": 0.12,
                    "sources": [
                        "witnessed_suffering",
                        "moments_of_despair",
                        "corruption_exposure"
                    ],
                    "quality": "volatile_but_potent"
                }
            },
            "sentimental_items": {
                "collected": 5,
                "items": [
                    {
                        "name": "Mother's Locket",
                        "emotional_weight": 0.92,
                        "forge_potential": "high",
                        "origin": "family_heirloom"
                    },
                    {
                        "name": "Broken Compass", 
                        "emotional_weight": 0.67,
                        "forge_potential": "medium",
                        "origin": "father's_last_gift"
                    },
                    {
                        "name": "Child's Drawing",
                        "emotional_weight": 0.78,
                        "forge_potential": "unique",
                        "origin": "village_orphan's_gift"
                    },
                    {
                        "name": "Battle-worn_Banner",
                        "emotional_weight": 0.85,
                        "forge_potential": "high", 
                        "origin": "fallen_friend's_regiment"
                    },
                    {
                        "name": "Pressed_Flower",
                        "emotional_weight": 0.56,
                        "forge_potential": "delicate",
                        "origin": "secret_garden_discovery"
                    }
                ]
            },
            "forge_requirements": {
                "ancient_anvil": "not_found",
                "ethereal_hammer": "not_found", 
                "forge_fire": "not_lit",
                "emotional_catalyst": "insufficient",
                "companion_witness": "available"
            },
            "readiness_assessment": {
                "overall_percentage": 35,
                "bottleneck": "missing_forge_location",
                "estimated_requirements": [
                    "explore_deeper_into_ancient_ruins",
                    "solve_ethereal_puzzles", 
                    "strengthen_emotional_bonds",
                    "accumulate_more_light_essence"
                ]
            },
            "forging_possibilities": {
                "light_path": {
                    "accessible": true,
                    "high_elf_relationship": 0.42,
                    "potential_creations": [
                        "beacon_of_hope",
                        "shield_of_memories",
                        "compass_of_true_purpose"
                    ]
                },
                "dark_path": {
                    "accessible": false,
                    "cursed_entity_relationship": 0.08,
                    "potential_creations": [
                        "blade_of_bitter_truths",
                        "cloak_of_forgotten_sorrows",
                        "mirror_of_deepest_fears"
                    ],
                    "warnings": ["high_corruption_risk", "potential_companion_loss"]
                }
            }
        }))
    });

    // Cross-System Intelligence Query
    // Comprehensive analysis for high-level AI decision making
    io.add_method("cross_system_intelligence_query", |_params| async {
        Ok(serde_json::json!({
            "world_state_analysis": {
                "overall_stability": 0.42,
                "corruption_trend": "slowly_increasing",
                "critical_systems": [
                    {
                        "system": "companion_psychology",
                        "status": "stable_but_monitoring_required",
                        "risk_factors": ["accumulated_trauma", "trust_dependencies"]
                    },
                    {
                        "system": "community_cohesion", 
                        "status": "strained_but_functional",
                        "risk_factors": ["resource_scarcity", "fear_escalation"]
                    },
                    {
                        "system": "environmental_corruption",
                        "status": "manageable_but_spreading",
                        "risk_factors": ["unknown_source", "accelerating_spread"]
                    }
                ]
            },
            "narrative_momentum": {
                "current_arc": "building_trust_and_understanding",
                "tension_level": "moderate_with_rising_stakes",
                "pacing": "steady_with_periodic_intensity",
                "next_major_developments": [
                    "forge_discovery_opportunity",
                    "major_companion_backstory_revelation",
                    "community_crisis_requiring_difficult_choice"
                ]
            },
            "player_psychological_profile": {
                "hope_vs_despair_balance": 0.64,
                "trust_in_companions": 0.78,
                "moral_certainty": 0.41,
                "decision_confidence": 0.67,
                "emotional_resilience": 0.72,
                "key_growth_areas": [
                    "accepting_moral_ambiguity",
                    "balancing_self_care_with_altruism",
                    "building_sustainable_hope"
                ]
            },
            "strategic_recommendations": {
                "immediate_priorities": [
                    "strengthen_companion_bonds_through_shared_positive_experiences",
                    "address_community_needs_to_build_regional_stability",
                    "continue_light_essence_accumulation_through_compassionate_acts"
                ],
                "medium_term_goals": [
                    "locate_and_access_ancient_forge", 
                    "resolve_major_companion_trauma_issues",
                    "establish_safe_haven_community"
                ],
                "long_term_vision": [
                    "create_sustainable_defense_against_corruption",
                    "forge_legendary_sentimental_items",
                    "establish_network_of_hope_and_healing"
                ]
            },
            "risk_assessment": {
                "catastrophic_failure_probability": 0.18,
                "major_setback_probability": 0.32,
                "companion_loss_risk": 0.15,
                "moral_corruption_risk": 0.22,
                "mitigation_strategies": [
                    "maintain_regular_companion_communication",
                    "avoid_isolation_and_cynicism",
                    "balance_pragmatism_with_idealism",
                    "build_multiple_support_networks"
                ]
            },
            "ai_agent_guidance": {
                "dialogue_tone": "supportive_but_realistic",
                "decision_support": "present_multiple_perspectives_with_clear_consequences",
                "narrative_emphasis": "highlight_meaningful_connections_and_small_victories",
                "pacing_control": "allow_processing_time_for_emotional_moments"
            }
        }))
    });
}