//! Seeds data processing module
//! 
//! Handles loading and processing of Seeds data sources including:
//! - Project Gutenberg medieval/horror literature
//! - Linguistic wordlists (Old Norse, Welsh, Arabic, Hebrew)
//! - Dialogue templates and quest patterns

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Manager for all Seeds data sources
#[derive(Debug, Clone)]
pub struct SeedsDataManager {
    pub linguistic_sources: LinguisticSources,
    pub literature_patterns: LiteraturePatterns,
    pub dialogue_templates: DialogueTemplates,
    pub quest_patterns: QuestPatterns,
}

impl SeedsDataManager {
    /// Load Seeds data from the cached directories
    pub fn load_from_cache(seeds_dir: &Path) -> Result<Self> {
        let linguistic_sources = LinguisticSources::load_from_directory(&seeds_dir.join("linguistic"))?;
        let literature_patterns = LiteraturePatterns::load_from_directory(&seeds_dir.join("literature"))?;
        
        let dialogue_dir = seeds_dir.parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid seeds directory"))?
            .join("dialogue");
        
        let dialogue_templates = DialogueTemplates::load_from_directory(&dialogue_dir)?;
        let quest_patterns = QuestPatterns::load_from_directory(&dialogue_dir)?;
        
        Ok(Self {
            linguistic_sources,
            literature_patterns,
            dialogue_templates,
            quest_patterns,
        })
    }
    
    /// Generate names based on English seeds and Act/Region context
    pub fn generate_name(&self, english_seed: &str, context: &NameGenerationContext) -> String {
        self.linguistic_sources.generate_name(english_seed, context)
    }
    
    /// Generate dialogue for NPC based on context
    pub fn generate_dialogue(&self, context: &DialogueGenerationContext) -> Result<Vec<String>> {
        self.dialogue_templates.generate_dialogue(context)
    }
    
    /// Generate quest based on patterns and regional context
    pub fn generate_quest(&self, context: &QuestGenerationContext) -> Result<GeneratedQuest> {
        self.quest_patterns.generate_quest(context, &self.literature_patterns)
    }
}

/// Linguistic sources for name generation
#[derive(Debug, Clone)]
pub struct LinguisticSources {
    pub english_fantasy_seeds: Vec<String>,
    pub act_progression_words: HashMap<String, Vec<String>>,
    pub old_norse_themes: HashMap<String, Vec<String>>,
    pub language_blend_presets: HashMap<String, LanguageBlend>,
}

impl LinguisticSources {
    pub fn load_from_directory(linguistic_dir: &Path) -> Result<Self> {
        let english_fantasy_seeds: Vec<String> = load_json_file(
            &linguistic_dir.join("../wordlists/english_fantasy_seeds.json")
        )?;
        
        let act_progression_words: HashMap<String, Vec<String>> = load_json_file(
            &linguistic_dir.join("../wordlists/act_progression_words.json")
        )?;
        
        let old_norse_themes: HashMap<String, Vec<String>> = load_json_file(
            &linguistic_dir.join("../old_norse/thematic_lists.json")
        )?;
        
        let language_blend_presets: HashMap<String, LanguageBlend> = load_json_file(
            &linguistic_dir.join("language_blend_presets.json")
        )?;
        
        Ok(Self {
            english_fantasy_seeds,
            act_progression_words,
            old_norse_themes,
            language_blend_presets,
        })
    }
    
    pub fn generate_name(&self, english_seed: &str, context: &NameGenerationContext) -> String {
        // Get the appropriate language blend for this context
        let blend_key = format!("{}_act{}", context.region_type, context.act);
        let blend = self.language_blend_presets
            .get(&blend_key)
            .or_else(|| self.language_blend_presets.get("meadows_act1"))
            .cloned()
            .unwrap_or_default();
        
        // Generate name using linguistic blending
        self.apply_linguistic_blend(english_seed, &blend, context)
    }
    
    fn apply_linguistic_blend(&self, english_seed: &str, blend: &LanguageBlend, context: &NameGenerationContext) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Create deterministic RNG based on seed and context
        let mut hasher = DefaultHasher::new();
        english_seed.hash(&mut hasher);
        context.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Select dominant language based on blend weights
        let rand_val = (hash % 100) as f32 / 100.0;
        let mut cumulative = 0.0;
        
        for (language, weight) in [
            ("old_norse", blend.old_norse),
            ("old_english", blend.old_english), 
            ("welsh", blend.welsh),
            ("arabic", blend.arabic),
            ("hebrew", blend.hebrew),
        ] {
            cumulative += weight;
            if rand_val <= cumulative {
                return self.translate_to_language(english_seed, language, hash);
            }
        }
        
        // Fallback to original if no blend
        english_seed.to_string()
    }
    
    fn translate_to_language(&self, english_seed: &str, language: &str, hash: u64) -> String {
        match language {
            "old_norse" => self.translate_to_old_norse(english_seed, hash),
            "old_english" => self.translate_to_old_english(english_seed, hash),
            "welsh" => self.translate_to_welsh(english_seed, hash),
            "arabic" => self.translate_to_arabic(english_seed, hash),
            "hebrew" => self.translate_to_hebrew(english_seed, hash),
            _ => english_seed.to_string(),
        }
    }
    
    fn translate_to_old_norse(&self, english_seed: &str, hash: u64) -> String {
        // Try to find thematic match first
        for (theme, words) in &self.old_norse_themes {
            if english_seed.contains(theme) {
                let index = (hash as usize) % words.len();
                return self.latinize(&words[index]);
            }
        }
        
        // Fallback to morphological transformation
        self.apply_old_norse_morphology(english_seed, hash)
    }
    
    fn translate_to_old_english(&self, english_seed: &str, hash: u64) -> String {
        // Simple Old English transformations
        let transformations = [
            ("night", "niht"),
            ("light", "leoht"),  
            ("dark", "deorc"),
            ("wolf", "wulf"),
            ("fire", "fyr"),
            ("water", "waeter"),
            ("stone", "stan"),
            ("wood", "wudu"),
        ];
        
        for (modern, old) in transformations {
            if english_seed.contains(modern) {
                return english_seed.replace(modern, old);
            }
        }
        
        // Apply Old English morphological patterns
        self.apply_old_english_morphology(english_seed, hash)
    }
    
    fn translate_to_welsh(&self, english_seed: &str, hash: u64) -> String {
        let transformations = [
            ("forest", "coed"),
            ("valley", "cwm"),
            ("mountain", "mynydd"),
            ("river", "afon"),
            ("village", "pentref"),
            ("castle", "castell"),
            ("dragon", "draig"),
            ("wolf", "blaidd"),
        ];
        
        for (english, welsh) in transformations {
            if english_seed.contains(english) {
                return english_seed.replace(english, welsh);
            }
        }
        
        self.apply_welsh_morphology(english_seed, hash)
    }
    
    fn translate_to_arabic(&self, english_seed: &str, hash: u64) -> String {
        let transformations = [
            ("desert", "sahra"),
            ("night", "layl"),
            ("star", "najm"),
            ("wind", "rih"),
            ("fire", "nar"),
            ("water", "ma"),
            ("stone", "hajar"),
            ("city", "madina"),
        ];
        
        for (english, arabic_latin) in transformations {
            if english_seed.contains(english) {
                return english_seed.replace(english, arabic_latin);
            }
        }
        
        format!("{}in", english_seed) // Simple Arabic-style suffix
    }
    
    fn translate_to_hebrew(&self, english_seed: &str, hash: u64) -> String {
        let transformations = [
            ("light", "or"),
            ("dark", "choshech"),
            ("life", "chaim"),
            ("death", "mavet"),
            ("peace", "shalom"),
            ("war", "milchama"),
            ("holy", "kadosh"),
            ("ancient", "atik"),
        ];
        
        for (english, hebrew_latin) in transformations {
            if english_seed.contains(english) {
                return english_seed.replace(english, hebrew_latin);
            }
        }
        
        format!("{}el", english_seed) // Simple Hebrew-style suffix
    }
    
    fn apply_old_norse_morphology(&self, seed: &str, hash: u64) -> String {
        let endings = ["r", "ur", "ir", "heim", "gard", "vik"];
        let index = (hash as usize) % endings.len();
        format!("{}{}", self.latinize(seed), endings[index])
    }
    
    fn apply_old_english_morphology(&self, seed: &str, hash: u64) -> String {
        let endings = ["ing", "ton", "ham", "burgh", "ford", "ley"];
        let index = (hash as usize) % endings.len();
        format!("{}{}", seed, endings[index])
    }
    
    fn apply_welsh_morphology(&self, seed: &str, hash: u64) -> String {
        let prefixes = ["llan", "pen", "aber", "caer", "tre"];
        let index = (hash as usize) % prefixes.len();
        format!("{}{}", prefixes[index], seed)
    }
    
    fn latinize(&self, s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'á' | 'à' | 'â' | 'ä' => result.push('a'),
                'é' | 'è' | 'ê' | 'ë' => result.push('e'),
                'í' | 'ì' | 'î' | 'ï' => result.push('i'),
                'ó' | 'ò' | 'ô' | 'ö' => result.push('o'),
                'ú' | 'ù' | 'û' | 'ü' => result.push('u'),
                'ý' | 'ÿ' => result.push('y'),
                'ð' => result.push('d'),
                'þ' => result.push_str("th"),
                _ => result.push(c),
            }
        }
        result.to_lowercase()
    }
}

/// Literature patterns extracted from public domain sources
#[derive(Debug, Clone)]
pub struct LiteraturePatterns {
    pub gothic_patterns: Vec<String>,
    pub mystery_patterns: Vec<String>,
    pub folklore_patterns: Vec<String>,
}

impl LiteraturePatterns {
    pub fn load_from_directory(literature_dir: &Path) -> Result<Self> {
        // In a full implementation, this would analyze downloaded literature
        // For now, provide curated patterns
        Ok(Self {
            gothic_patterns: vec![
                "ancient_curse".to_string(),
                "haunted_manor".to_string(), 
                "family_secret".to_string(),
                "forbidden_knowledge".to_string(),
            ],
            mystery_patterns: vec![
                "disappearance".to_string(),
                "locked_room".to_string(),
                "false_identity".to_string(),
                "hidden_motive".to_string(),
            ],
            folklore_patterns: vec![
                "broken_taboo".to_string(),
                "magical_bargain".to_string(),
                "test_of_character".to_string(),
                "transformation".to_string(),
            ],
        })
    }
}

/// Dialogue templates loaded from JSON
#[derive(Debug, Clone)]
pub struct DialogueTemplates {
    pub greeting_templates: Vec<DialogueTemplate>,
    pub quest_hint_templates: Vec<DialogueTemplate>,
    pub farewell_templates: Vec<DialogueTemplate>,
    pub tone_presets: HashMap<String, TonePreset>,
}

impl DialogueTemplates {
    pub fn load_from_directory(dialogue_dir: &Path) -> Result<Self> {
        let dialogue_templates: serde_json::Value = load_json_file(
            &dialogue_dir.join("dialogue_templates.json")
        )?;
        
        let tone_presets: HashMap<String, TonePreset> = load_json_file(
            &dialogue_dir.join("../linguistic/dialogue_tone_presets.json")
        )?;
        
        let greeting_templates = parse_dialogue_templates(&dialogue_templates["greeting"])?;
        let quest_hint_templates = parse_dialogue_templates(&dialogue_templates["quest_hint"])?;
        let farewell_templates = parse_dialogue_templates(&dialogue_templates["farewell"])?;
        
        Ok(Self {
            greeting_templates,
            quest_hint_templates,
            farewell_templates,
            tone_presets,
        })
    }
    
    pub fn generate_dialogue(&self, context: &DialogueGenerationContext) -> Result<Vec<String>> {
        let mut dialogue = Vec::new();
        
        // Generate greeting
        if let Some(greeting) = self.generate_template_line(&self.greeting_templates, context)? {
            dialogue.push(greeting);
        }
        
        // Generate quest hint if appropriate
        if context.should_offer_quest {
            if let Some(quest_hint) = self.generate_template_line(&self.quest_hint_templates, context)? {
                dialogue.push(quest_hint);
            }
        }
        
        // Generate farewell
        if let Some(farewell) = self.generate_template_line(&self.farewell_templates, context)? {
            dialogue.push(farewell);
        }
        
        Ok(dialogue)
    }
    
    fn generate_template_line(&self, templates: &[DialogueTemplate], context: &DialogueGenerationContext) -> Result<Option<String>> {
        // Filter templates by tone compatibility
        let compatible_templates: Vec<_> = templates
            .iter()
            .filter(|t| context.npc_tone.is_empty() || t.tone == context.npc_tone)
            .collect();
        
        if compatible_templates.is_empty() {
            return Ok(None);
        }
        
        // Weighted selection
        let total_weight: f32 = compatible_templates.iter().map(|t| t.weight).sum();
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        let hash = hasher.finish();
        
        let mut target = ((hash % 1000) as f32 / 1000.0) * total_weight;
        
        for template in &compatible_templates {
            target -= template.weight;
            if target <= 0.0 {
                return Ok(Some(self.fill_template(template, context)?));
            }
        }
        
        // Fallback to first template
        if let Some(template) = compatible_templates.first() {
            Ok(Some(self.fill_template(template, context)?))
        } else {
            Ok(None)
        }
    }
    
    fn fill_template(&self, template: &DialogueTemplate, context: &DialogueGenerationContext) -> Result<String> {
        let mut result = template.template.clone();
        
        // Fill in template variables
        result = result.replace("{player_name}", &context.player_name);
        result = result.replace("{title}", &context.player_title);
        result = result.replace("{time_of_day}", &context.time_of_day);
        result = result.replace("{location_type}", &context.location_type);
        
        // Apply tone modifications
        if let Some(tone_preset) = self.tone_presets.get(&template.tone) {
            result = self.apply_tone_preset(&result, tone_preset, context)?;
        }
        
        Ok(result)
    }
    
    fn apply_tone_preset(&self, text: &str, tone_preset: &TonePreset, context: &DialogueGenerationContext) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        context.hash(&mut hasher);
        let hash = hasher.finish();
        
        let mut result = text.to_string();
        
        // Maybe add opener
        if (hash % 3) == 0 && !tone_preset.openers.is_empty() {
            let index = (hash as usize) % tone_preset.openers.len();
            result = format!("{} {}", tone_preset.openers[index], result);
        }
        
        // Maybe add closer
        if (hash % 4) == 0 && !tone_preset.closers.is_empty() {
            let index = ((hash / 2) as usize) % tone_preset.closers.len();
            if !tone_preset.closers[index].is_empty() {
                result = format!("{} {}", result, tone_preset.closers[index]);
            }
        }
        
        Ok(result)
    }
}

/// Quest patterns for generating quest structures
#[derive(Debug, Clone)]
pub struct QuestPatterns {
    pub investigation_pattern: QuestPattern,
    pub purification_pattern: QuestPattern,
    pub escort_pattern: QuestPattern,
}

impl QuestPatterns {
    pub fn load_from_directory(dialogue_dir: &Path) -> Result<Self> {
        let quest_patterns: serde_json::Value = load_json_file(
            &dialogue_dir.join("quest_pattern_templates.json")
        )?;
        
        let investigation_pattern = parse_quest_pattern(&quest_patterns["investigation"])?;
        let purification_pattern = parse_quest_pattern(&quest_patterns["purification"])?;
        let escort_pattern = parse_quest_pattern(&quest_patterns["escort_in_denial"])?;
        
        Ok(Self {
            investigation_pattern,
            purification_pattern,
            escort_pattern,
        })
    }
    
    pub fn generate_quest(&self, context: &QuestGenerationContext, literature: &LiteraturePatterns) -> Result<GeneratedQuest> {
        // Select pattern based on context
        let pattern = match context.preferred_pattern.as_str() {
            "investigation" => &self.investigation_pattern,
            "purification" => &self.purification_pattern,
            "escort" => &self.escort_pattern,
            _ => &self.investigation_pattern, // Default
        };
        
        Ok(GeneratedQuest {
            id: format!("quest_{}_{}", context.region_uuid, context.npc_uuid),
            pattern_type: pattern.pattern_type.clone(),
            beats: pattern.beats.clone(),
            themes: pattern.themes.clone(),
            estimated_length: pattern.beats.len(),
        })
    }
}

// Supporting types and structures

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageBlend {
    pub old_norse: f32,
    pub old_english: f32,
    pub welsh: f32,
    pub arabic: f32,
    pub hebrew: f32,
}

#[derive(Debug, Clone)]
pub struct NameGenerationContext {
    pub region_type: String,
    pub act: u8,
    pub band: u8,
    pub corruption_level: f32,
}

impl std::hash::Hash for NameGenerationContext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.region_type.hash(state);
        self.act.hash(state);
        self.band.hash(state);
        // Convert f32 to ordered representation for hashing
        self.corruption_level.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, Hash)]
pub struct DialogueGenerationContext {
    pub player_name: String,
    pub player_title: String,
    pub time_of_day: String,
    pub location_type: String,
    pub npc_tone: String,
    pub should_offer_quest: bool,
    pub npc_uuid: String,
}

#[derive(Debug, Clone)]
pub struct QuestGenerationContext {
    pub region_uuid: String,
    pub npc_uuid: String,
    pub preferred_pattern: String,
    pub act: u8,
    pub corruption_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTemplate {
    pub template: String,
    pub tone: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TonePreset {
    pub openers: Vec<String>,
    pub closers: Vec<String>,
    pub hedges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestPattern {
    pub pattern_type: String,
    pub beats: Vec<String>,
    pub themes: Vec<String>,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GeneratedQuest {
    pub id: String,
    pub pattern_type: String,
    pub beats: Vec<String>,
    pub themes: Vec<String>,
    pub estimated_length: usize,
}

// Utility functions

fn load_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))
}

fn parse_dialogue_templates(value: &serde_json::Value) -> Result<Vec<DialogueTemplate>> {
    let templates: Vec<DialogueTemplate> = serde_json::from_value(value.clone())?;
    Ok(templates)
}

fn parse_quest_pattern(value: &serde_json::Value) -> Result<QuestPattern> {
    let pattern: QuestPattern = serde_json::from_value(value.clone())?;
    Ok(pattern)
}
