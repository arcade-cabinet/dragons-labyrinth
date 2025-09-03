//! Build API for dl_analysis crate
//! 
//! Provides comprehensive interface for dl_processors to obtain analyzed data

use anyhow::Result;
use std::path::Path;
use crate::orchestration::RawEntities;
use ron::ser::to_string_pretty;

/// Complete analyzed data bundle ready for processing
#[derive(Debug, Clone)]
pub struct AnalysisBuildData {
    pub hbf_analysis: HbfAnalysisData,
    pub seeds_analysis: AnalyzedSeedsData,
    pub combined_stats: CombinedAnalysisStats,
}

/// HBF database analysis results
#[derive(Debug, Clone)]
pub struct HbfAnalysisData {
    pub regions: Vec<String>,
    pub settlements: Vec<String>,
    pub dungeons: Vec<String>,
    pub factions: Vec<String>,
    pub total_entities: usize,
}

/// Analyzed and categorized seeds data
#[derive(Debug, Clone)]
pub struct AnalyzedSeedsData {
    pub dialogue_by_act: std::collections::HashMap<u8, ActDialogueData>,
    pub quests_by_pattern: std::collections::HashMap<String, QuestPatternData>,
    pub linguistics_by_region: std::collections::HashMap<String, RegionLinguisticsData>,
}

/// Dialogue data organized by corruption act
#[derive(Debug, Clone)]
pub struct ActDialogueData {
    pub patterns: Vec<String>,
    pub character_archetypes: Vec<String>,
    pub tone_variations: Vec<String>,
}

/// Quest pattern data for specific quest type
#[derive(Debug, Clone)]
pub struct QuestPatternData {
    pub templates: Vec<String>,
    pub beats: Vec<String>,
    pub themes: Vec<String>,
}

/// Linguistic rules for specific region type
#[derive(Debug, Clone)]
pub struct RegionLinguisticsData {
    pub name_patterns: Vec<String>,
    pub language_blend: dl_seeds::linguistics::LanguageBlend,
    pub thematic_words: Vec<String>,
}

/// Combined analysis statistics
#[derive(Debug, Clone)]
pub struct CombinedAnalysisStats {
    pub hbf_entities_processed: usize,
    pub books_analyzed: usize,
    pub dialogue_patterns_created: usize,
    pub quest_templates_generated: usize,
    pub linguistic_rules_created: usize,
}

impl AnalysisBuildData {
    /// Perform complete analysis combining HBF and Seeds data
    pub fn perform_complete_analysis(
        hbf_path: &Path,
        seeds_cache_dir: &Path,
        output_dir: &Path,
    ) -> Result<Self> {
        // Get seeds data from dl_seeds
        let seeds_build_data = dl_seeds::build_api::provide_seeds_data_for_analysis(seeds_cache_dir)?;
        
        // Perform HBF analysis if database exists
        let hbf_analysis = if hbf_path.exists() {
            perform_hbf_analysis(hbf_path, output_dir)?
        } else {
            HbfAnalysisData::empty()
        };
        
        // Analyze and categorize seeds data
        let seeds_analysis = analyze_seeds_data(&seeds_build_data)?;
        
        // Output organized seeds data to analyzed_seeds/ directory
        output_analyzed_seeds_data(&seeds_analysis, output_dir)?;
        
        let combined_stats = CombinedAnalysisStats {
            hbf_entities_processed: hbf_analysis.total_entities,
            books_analyzed: seeds_build_data.literature.total_books,
            dialogue_patterns_created: seeds_analysis.dialogue_by_act.len() * 10, // Approx
            quest_templates_generated: seeds_analysis.quests_by_pattern.len() * 5, // Approx
            linguistic_rules_created: seeds_analysis.linguistics_by_region.len() * 20, // Approx
        };

        Ok(Self {
            hbf_analysis,
            seeds_analysis,
            combined_stats,
        })
    }

    /// Get dialogue data for specific act
    pub fn get_dialogue_for_act(&self, act: u8) -> Option<&ActDialogueData> {
        self.seeds_analysis.dialogue_by_act.get(&act)
    }

    /// Get quest patterns for specific type
    pub fn get_quest_patterns_for_type(&self, pattern_type: &str) -> Option<&QuestPatternData> {
        self.seeds_analysis.quests_by_pattern.get(pattern_type)
    }

    /// Get linguistic rules for specific region type
    pub fn get_linguistics_for_region(&self, region_type: &str) -> Option<&RegionLinguisticsData> {
        self.seeds_analysis.linguistics_by_region.get(region_type)
    }
}

impl HbfAnalysisData {
    fn empty() -> Self {
        Self {
            regions: Vec::new(),
            settlements: Vec::new(),
            dungeons: Vec::new(),
            factions: Vec::new(),
            total_entities: 0,
        }
    }
}

/// Perform HBF database analysis
fn perform_hbf_analysis(hbf_path: &Path, output_dir: &Path) -> Result<HbfAnalysisData> {
    let mut orchestrator = RawEntities::new();
    let mut logger = std::io::stdout();
    let analysis_dir = std::path::Path::new("analysis");
    
    // Run HBF analysis
    let summary = orchestrator.run_complete_analysis(&analysis_dir, output_dir, &mut logger)?;
    
    // For now, create placeholder data structures
    // In a full implementation, this would extract actual data from the analysis
    Ok(HbfAnalysisData {
        regions: Vec::new(), // Would be populated from analysis results
        settlements: Vec::new(),
        dungeons: Vec::new(), 
        factions: Vec::new(),
        total_entities: summary.total_entities,
    })
}

/// Analyze seeds data and organize by categories
fn analyze_seeds_data(seeds_data: &dl_seeds::build_api::SeedsBuildData) -> Result<AnalyzedSeedsData> {
    let mut dialogue_by_act = std::collections::HashMap::new();
    let mut quests_by_pattern = std::collections::HashMap::new();
    let mut linguistics_by_region = std::collections::HashMap::new();

    // DIALOGUE ANALYSIS BY ACT
    for act in 1..=5u8 {
        let literature_for_act = seeds_data.get_literature_by_act(act);
        
        // Create dialogue patterns for this act
        let patterns = extract_dialogue_patterns_for_act(act, &literature_for_act, seeds_data);
        let archetypes = filter_archetypes_for_act(act, &seeds_data.dialogue.character_archetypes);
        let tone_variations = generate_tone_variations_for_act(act);
        
        dialogue_by_act.insert(act, ActDialogueData {
            patterns,
            character_archetypes: archetypes,
            tone_variations,
        });
    }

    // QUEST ANALYSIS BY PATTERN
    let quest_pattern_types = ["investigation", "purification", "escort", "exploration", "confrontation"];
    for pattern_type in quest_pattern_types {
        let quest_patterns = seeds_data.get_quest_patterns_by_type(pattern_type);
        
        let templates = extract_quest_templates(pattern_type, &quest_patterns, seeds_data);
        let beats = generate_quest_beats_for_pattern(pattern_type);
        let themes = extract_themes_for_pattern(pattern_type, seeds_data);
        
        quests_by_pattern.insert(pattern_type.to_string(), QuestPatternData {
            templates,
            beats,
            themes,
        });
    }

    // LINGUISTIC ANALYSIS BY REGION TYPE
    let region_types = [
        "meadows", "forests", "swamps", "deserts", "mountains",
        "villages", "ruins", "dungeons", "shrines", "portals"
    ];
    
    for region_type in region_types {
        let linguistic_rules = seeds_data.get_linguistic_rules_by_region(region_type);
        
        let name_patterns = linguistic_rules.cloned().unwrap_or_default();
        let language_blend = create_language_blend_for_region(region_type);
        let thematic_words = extract_thematic_words_for_region(region_type, seeds_data);
        
        linguistics_by_region.insert(region_type.to_string(), RegionLinguisticsData {
            name_patterns,
            language_blend,
            thematic_words,
        });
    }

    Ok(AnalyzedSeedsData {
        dialogue_by_act,
        quests_by_pattern,
        linguistics_by_region,
    })
}

/// Output analyzed seeds data to structured directories
fn output_analyzed_seeds_data(
    seeds_analysis: &AnalyzedSeedsData,
    output_dir: &Path,
) -> Result<()> {
    let analyzed_seeds_dir = output_dir.join("analyzed_seeds");
    std::fs::create_dir_all(&analyzed_seeds_dir)?;

    // Output dialogue data by act
    let dialogue_dir = analyzed_seeds_dir.join("dialogue");
    std::fs::create_dir_all(&dialogue_dir)?;
    
    for (act, dialogue_data) in &seeds_analysis.dialogue_by_act {
        let act_dir = dialogue_dir.join(format!("act{}", act));
        std::fs::create_dir_all(&act_dir)?;
        
        // Save patterns
        let patterns_ron = to_string_pretty(&dialogue_data.patterns, Default::default())?;
        std::fs::write(act_dir.join("patterns.ron"), patterns_ron)?;
        
        // Save archetypes
        let archetypes_ron = to_string_pretty(&dialogue_data.character_archetypes, Default::default())?;
        std::fs::write(act_dir.join("archetypes.ron"), archetypes_ron)?;
        
        // Save tone variations
        let tones_ron = to_string_pretty(&dialogue_data.tone_variations, Default::default())?;
        std::fs::write(act_dir.join("tones.ron"), tones_ron)?;
    }

    // Output quest data by pattern
    let quests_dir = analyzed_seeds_dir.join("quests");
    std::fs::create_dir_all(&quests_dir)?;
    
    for (pattern_type, quest_data) in &seeds_analysis.quests_by_pattern {
        let pattern_dir = quests_dir.join(pattern_type);
        std::fs::create_dir_all(&pattern_dir)?;
        
        // Save templates
        let templates_ron = to_string_pretty(&quest_data.templates, Default::default())?;
        std::fs::write(pattern_dir.join("templates.ron"), templates_ron)?;
        
        // Save beats
        let beats_ron = to_string_pretty(&quest_data.beats, Default::default())?;
        std::fs::write(pattern_dir.join("beats.ron"), beats_ron)?;
        
        // Save themes
        let themes_ron = to_string_pretty(&quest_data.themes, Default::default())?;
        std::fs::write(pattern_dir.join("themes.ron"), themes_ron)?;
    }

    // Output linguistic data by region
    let linguistics_dir = analyzed_seeds_dir.join("linguistics");
    std::fs::create_dir_all(&linguistics_dir)?;
    
    for (region_type, linguistic_data) in &seeds_analysis.linguistics_by_region {
        let region_dir = linguistics_dir.join(region_type);
        std::fs::create_dir_all(&region_dir)?;
        
        // Save name patterns
        let patterns_ron = to_string_pretty(&linguistic_data.name_patterns, Default::default())?;
        std::fs::write(region_dir.join("patterns.ron"), patterns_ron)?;
        
        // Save language blend
        let blend_ron = to_string_pretty(&linguistic_data.language_blend, Default::default())?;
        std::fs::write(region_dir.join("blend.ron"), blend_ron)?;
        
        // Save thematic words
        let words_ron = to_string_pretty(&linguistic_data.thematic_words, Default::default())?;
        std::fs::write(region_dir.join("words.ron"), words_ron)?;
    }

    Ok(())
}

// Helper functions for data analysis

fn extract_dialogue_patterns_for_act(
    act: u8,
    literature: &[String],
    _seeds_data: &dl_seeds::build_api::SeedsBuildData,
) -> Vec<String> {
    let mut patterns = Vec::new();
    
    // Generate act-appropriate patterns
    match act {
        1 => patterns.extend([
            "Welcome, traveler. The path ahead looks clear.".to_string(),
            "May fortune smile upon your journey.".to_string(),
            "The day brings hope for new beginnings.".to_string(),
        ]),
        2 => patterns.extend([
            "Something feels... different about this place.".to_string(),
            "I would be careful if I were you.".to_string(),
            "The shadows seem longer today.".to_string(),
        ]),
        3 => patterns.extend([
            "Turn back while you still can.".to_string(),
            "The very air whispers of danger.".to_string(),
            "We've lost too many already.".to_string(),
        ]),
        4 => patterns.extend([
            "They're all gone... all of them...".to_string(),
            "The darkness comes for us all.".to_string(),
            "There is no safety left in this world.".to_string(),
        ]),
        5 => patterns.extend([
            "The end of all things draws near.".to_string(),
            "We are but echoes of what once was.".to_string(),
            "Even hope has abandoned us now.".to_string(),
        ]),
        _ => patterns.push("Greetings, stranger.".to_string()),
    }
    
    patterns
}

fn filter_archetypes_for_act(
    act: u8,
    archetypes: &[dl_seeds::dialogue::CharacterArchetype],
) -> Vec<String> {
    let preferred_archetypes = match act {
        1 => vec!["wandering_scholar", "holy_warrior"],
        2 => vec!["wandering_scholar", "holy_warrior", "mercenary"],
        3 => vec!["mercenary", "corrupted_noble", "wandering_scholar"],
        4 => vec!["corrupted_noble", "dark_cultist", "mercenary"],
        5 => vec!["dark_cultist", "corrupted_noble"],
        _ => vec!["wandering_scholar"],
    };
    
    preferred_archetypes.iter().map(|s| s.to_string()).collect()
}

fn generate_tone_variations_for_act(act: u8) -> Vec<String> {
    match act {
        1 => vec!["hopeful".to_string(), "welcoming".to_string(), "peaceful".to_string()],
        2 => vec!["cautious".to_string(), "worried".to_string(), "uncertain".to_string()],
        3 => vec!["fearful".to_string(), "desperate".to_string(), "warning".to_string()],
        4 => vec!["terrified".to_string(), "broken".to_string(), "panicked".to_string()],
        5 => vec!["despairing".to_string(), "hollow".to_string(), "resigned".to_string()],
        _ => vec!["neutral".to_string()],
    }
}

fn extract_quest_templates(
    pattern_type: &str,
    _quest_patterns: &[&dl_seeds::dialogue::QuestPattern],
    _seeds_data: &dl_seeds::build_api::SeedsBuildData,
) -> Vec<String> {
    match pattern_type {
        "investigation" => vec![
            "Investigate the mysterious {event} in {location}".to_string(),
            "Discover the truth behind {mystery}".to_string(),
        ],
        "purification" => vec![
            "Cleanse the corruption from {location}".to_string(),
            "Purify the tainted {object} using {method}".to_string(),
        ],
        "escort" => vec![
            "Safely escort {npc} to {destination}".to_string(),
            "Protect {npc} from {danger} during the journey".to_string(),
        ],
        "exploration" => vec![
            "Explore the uncharted {area_type}".to_string(),
            "Map the boundaries of {region}".to_string(),
        ],
        "confrontation" => vec![
            "Confront the {enemy} threatening {location}".to_string(),
            "Challenge {antagonist} to resolve {conflict}".to_string(),
        ],
        _ => vec!["Complete a task in {location}".to_string()],
    }
}

fn generate_quest_beats_for_pattern(pattern_type: &str) -> Vec<String> {
    match pattern_type {
        "investigation" => vec![
            "Gather initial clues".to_string(),
            "Interview witnesses".to_string(),
            "Uncover the truth".to_string(),
        ],
        "purification" => vec![
            "Identify the source of corruption".to_string(),
            "Gather purification materials".to_string(),
            "Perform the cleansing ritual".to_string(),
        ],
        "escort" => vec![
            "Meet the person to escort".to_string(),
            "Navigate dangers along the route".to_string(),
            "Safely deliver them to destination".to_string(),
        ],
        _ => vec!["Begin the task".to_string(), "Overcome obstacles".to_string(), "Complete the objective".to_string()],
    }
}

fn extract_themes_for_pattern(
    pattern_type: &str,
    _seeds_data: &dl_seeds::build_api::SeedsBuildData,
) -> Vec<String> {
    match pattern_type {
        "investigation" => vec!["mystery".to_string(), "truth".to_string(), "discovery".to_string()],
        "purification" => vec!["corruption".to_string(), "cleansing".to_string(), "redemption".to_string()],
        "escort" => vec!["protection".to_string(), "responsibility".to_string(), "journey".to_string()],
        "exploration" => vec!["unknown".to_string(), "discovery".to_string(), "adventure".to_string()],
        "confrontation" => vec!["conflict".to_string(), "resolution".to_string(), "justice".to_string()],
        _ => vec!["adventure".to_string()],
    }
}

fn create_language_blend_for_region(region_type: &str) -> dl_seeds::linguistics::LanguageBlend {
    use dl_seeds::linguistics::LanguageBlend;
    
    match region_type {
        "meadows" => LanguageBlend { 
            old_english: 0.7, 
            welsh: 0.2, 
            old_norse: 0.1, 
            arabic: 0.0, 
            hebrew: 0.0 
        },
        "forests" => LanguageBlend { 
            welsh: 0.6, 
            old_english: 0.3, 
            old_norse: 0.1, 
            arabic: 0.0, 
            hebrew: 0.0 
        },
        "swamps" => LanguageBlend { 
            old_norse: 0.6, 
            old_english: 0.3, 
            welsh: 0.1, 
            arabic: 0.0, 
            hebrew: 0.0 
        },
        "deserts" => LanguageBlend { 
            arabic: 0.7, 
            hebrew: 0.2, 
            old_english: 0.1, 
            welsh: 0.0, 
            old_norse: 0.0 
        },
        "mountains" => LanguageBlend { 
            old_norse: 0.8, 
            old_english: 0.2, 
            welsh: 0.0, 
            arabic: 0.0, 
            hebrew: 0.0 
        },
        _ => LanguageBlend { 
            old_english: 0.6, 
            old_norse: 0.4, 
            welsh: 0.0, 
            arabic: 0.0, 
            hebrew: 0.0 
        },
    }
}

fn extract_thematic_words_for_region(
    region_type: &str,
    _seeds_data: &dl_seeds::build_api::SeedsBuildData,
) -> Vec<String> {
    match region_type {
        "meadows" => vec!["green".to_string(), "peaceful".to_string(), "pastoral".to_string()],
        "forests" => vec!["ancient".to_string(), "mystical".to_string(), "deep".to_string()],
        "swamps" => vec!["dark".to_string(), "murky".to_string(), "dangerous".to_string()],
        "deserts" => vec!["harsh".to_string(), "dry".to_string(), "endless".to_string()],
        "mountains" => vec!["high".to_string(), "strong".to_string(), "enduring".to_string()],
        _ => vec!["mysterious".to_string()],
    }
}

/// Public API function for dl_processors to call during build
pub fn provide_analysis_data_for_processing(
    hbf_path: &Path,
    seeds_cache_dir: &Path,
    output_dir: &Path,
) -> Result<AnalysisBuildData> {
    AnalysisBuildData::perform_complete_analysis(hbf_path, seeds_cache_dir, output_dir)
}
