//! Replit Prompt Generator Binary - 3D model generation templates
//! 
//! Creates markdown files with recommended 3D model prompts and Yarnspinner
//! dialogue templates for Replit AI to generate GLB models and narrative content.

use anyhow::Result;
use clap::{Parser, Subcommand};
use dl_seeds::{
    containers::RawEntity,
    orchestration::RawEntities,
    utilities::sanitize_name,
    books::{WorldSeed, QuestSeed, DialogueSeed},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "replit-prompter")]
#[command(about = "Generate Replit prompts for 3D model and dialogue generation")]
#[command(version = "1.0.0")]
struct Cli {
    /// Path to analyzed HBF data (from hbf_analyzer)
    #[arg(short, long)]
    input: PathBuf,
    
    /// Path to generated RON assets (from ron_generator)
    #[arg(short, long)]
    assets: PathBuf,
    
    /// Output directory for Replit prompt templates
    #[arg(short, long)]
    output: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate all prompt templates (3D models + dialogue)
    GenerateAll {
        /// Include Dragon's Labyrinth corruption themes
        #[arg(long)]
        corruption_themes: bool,
    },
    /// Generate 3D model prompts only
    Models {
        /// Specific category (units, buildings, leaders, terrain)
        #[arg(short, long)]
        category: Option<String>,
        
        /// Specific faction/cult
        #[arg(short, long)]
        faction: Option<String>,
    },
    /// Generate Yarnspinner dialogue prompts
    Dialogue {
        /// Include companion trauma from Dragon's Labyrinth
        #[arg(long)]
        companion_trauma: bool,
    },
    /// Generate upgrade progression documentation
    Progressions {
        /// Visual progression guides for model variations
        #[arg(long)]
        visual_guides: bool,
    },
}

/// Prompt template for 3D model generation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelPromptTemplate {
    model_id: String,
    display_name: String,
    category: String,
    faction: String,
    primary_prompt: String,
    style_prompt: String,
    technical_specs: TechnicalSpecs,
    corruption_progression: Option<CorruptionProgression>,
    reference_images: Vec<String>,
    animation_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TechnicalSpecs {
    target_poly_count: String,
    texture_resolution: String,
    required_animations: Vec<String>,
    bone_structure: String,
    material_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CorruptionProgression {
    band: u8,
    theme: String,
    visual_evolution: String,
    material_changes: String,
}

/// Dialogue prompt template for Yarnspinner
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DialoguePromptTemplate {
    character_id: String,
    character_name: String,
    role: String,
    personality_prompt: String,
    trauma_indicators: Vec<String>,
    speech_patterns: Vec<String>,
    sample_interactions: Vec<String>,
    corruption_evolution: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("📝 Dragon's Labyrinth Replit Prompt Generator");
    println!("📁 Input: {}", cli.input.display());
    println!("📁 Assets: {}", cli.assets.display());
    println!("📁 Output: {}", cli.output.display());
    
    // Ensure output directory exists
    std::fs::create_dir_all(&cli.output)?;
    
    match &cli.command {
        Commands::GenerateAll { corruption_themes } => {
            generate_all_prompts(&cli.input, &cli.assets, &cli.output, *corruption_themes)?;
        }
        Commands::Models { category, faction } => {
            generate_model_prompts(&cli.input, &cli.assets, &cli.output, category.as_deref(), faction.as_deref())?;
        }
        Commands::Dialogue { companion_trauma } => {
            generate_dialogue_prompts(&cli.input, &cli.output, *companion_trauma)?;
        }
        Commands::Progressions { visual_guides } => {
            generate_progression_docs(&cli.assets, &cli.output, *visual_guides)?;
        }
    }
    
    Ok(())
}

fn generate_all_prompts(
    input_dir: &PathBuf,
    assets_dir: &PathBuf,
    output_dir: &PathBuf,
    corruption_themes: bool,
) -> Result<()> {
    println!("🔄 Generating all Replit prompt templates...");
    
    // Generate 3D model prompts
    generate_model_prompts(input_dir, assets_dir, output_dir, None, None)?;
    
    // Generate dialogue prompts
    generate_dialogue_prompts(input_dir, output_dir, corruption_themes)?;
    
    // Generate progression documentation
    generate_progression_docs(assets_dir, output_dir, true)?;
    
    // Generate master index
    generate_prompt_index(output_dir)?;
    
    println!("✅ All Replit prompt templates generated");
    Ok(())
}

fn generate_model_prompts(
    input_dir: &PathBuf,
    assets_dir: &PathBuf,
    output_dir: &PathBuf,
    category_filter: Option<&str>,
    faction_filter: Option<&str>,
) -> Result<()> {
    println!("🎨 Generating 3D model prompts...");
    
    let models_dir = output_dir.join("model_prompts");
    std::fs::create_dir_all(&models_dir)?;
    
    // Load analyzed entities
    let entities = load_analyzed_entities(input_dir)?;
    
    // Load existing RON metadata to enhance prompts with asset specifications
    let ron_metadata = load_ron_metadata_from_assets(assets_dir)?;
    println!("  Loaded {} RON metadata files for prompt enhancement", ron_metadata.len());
    
    // Generate prompts for each faction
    for (faction_name, faction_entities) in &entities.factions {
        if let Some(filter) = faction_filter {
            if faction_name != filter {
                continue;
            }
        }
        
        let faction_dir = models_dir.join(sanitize_name(faction_name));
        std::fs::create_dir_all(&faction_dir)?;
        
        for entity in faction_entities {
            // Check if we should skip this entity based on category filter
            if let Some(cat_filter) = category_filter {
                let entity_category = determine_entity_category(&entity.raw_value);
                if entity_category != cat_filter {
                    continue;
                }
            }
            
            let prompt_template = create_model_prompt_from_entity_enhanced(
                entity, 
                faction_name, 
                &ron_metadata
            );
            let markdown_content = render_model_prompt_to_markdown(&prompt_template);
            
            let filename = format!("{}_prompt.md", sanitize_name(&entity.entity_name));
            std::fs::write(faction_dir.join(filename), markdown_content)?;
        }
        
        let entity_count = if category_filter.is_some() {
            faction_entities.iter().filter(|e| {
                let entity_category = determine_entity_category(&e.raw_value);
                entity_category == category_filter.unwrap()
            }).count()
        } else {
            faction_entities.len()
        };
        
        if entity_count > 0 {
            println!("  Generated {} model prompts for: {}", entity_count, faction_name);
        }
    }
    
    Ok(())
}

fn create_model_prompt_from_entity(entity: &RawEntity, faction: &str) -> ModelPromptTemplate {
    let content = &entity.raw_value;
    let content_lower = content.to_lowercase();
    
    // Determine model category
    let category = if content_lower.contains("warrior") || content_lower.contains("fighter") {
        "warrior_unit"
    } else if content_lower.contains("priest") || content_lower.contains("cleric") {
        "priest_unit"
    } else if content_lower.contains("leader") || content_lower.contains("commander") {
        "leader_unit"
    } else if content_lower.contains("building") || content_lower.contains("structure") {
        "building"
    } else {
        "cultist_unit"
    };
    
    // Create comprehensive 3D model prompt
    let primary_prompt = format!(
        "Create a detailed 3D model of a {} from the {} faction. {}. \
        This character/structure embodies themes of cosmic horror and Lovecraftian dread. \
        Focus on asymmetrical design elements, non-Euclidean geometry hints, and materials \
        that suggest otherworldly origins. The model should convey both the faction's unique \
        aesthetic and the underlying cosmic horror that drives their actions.",
        entity.entity_name,
        faction,
        extract_description_from_content(content)
    );
    
    let style_prompt = create_faction_style_prompt(faction, &content_lower);
    
    ModelPromptTemplate {
        model_id: sanitize_name(&entity.entity_name),
        display_name: entity.entity_name.clone(),
        category: category.to_string(),
        faction: faction.to_string(),
        primary_prompt,
        style_prompt,
        technical_specs: create_technical_specs(category),
        corruption_progression: create_corruption_progression(content),
        reference_images: create_reference_suggestions(faction, category),
        animation_requirements: extract_animation_requirements(content),
    }
}

fn extract_description_from_content(content: &str) -> String {
    // Extract key descriptive elements from HBF HTML content
    let sentences: Vec<&str> = content.split('.').take(3).collect();
    let cleaned: String = sentences.join(". ")
        .replace("<p>", "")
        .replace("</p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("&nbsp;", " ");
    
    if cleaned.len() > 200 {
        format!("{}...", &cleaned[..200])
    } else {
        cleaned
    }
}

fn create_faction_style_prompt(faction: &str, content: &str) -> String {
    let base_style = match faction.to_lowercase().as_str() {
        f if f.contains("crimson") || f.contains("blood") => {
            "Crimson Covenant aesthetic: Deep reds, blood-infused materials, bone decorations, \
            ritualistic scarification, flowing robes with crimson trim, copper and bronze metals \
            with blood-red patina. Emphasize organic, flowing forms with sharp ritual implements."
        }
        f if f.contains("deep") || f.contains("water") || f.contains("ocean") => {
            "Order of the Deep aesthetic: Deep blues and greens, aquatic textures, scaled surfaces, \
            coral-like growths, barnacle encrustation, tarnished silver and copper. Emphasize \
            flowing, water-influenced forms with bioluminescent accents."
        }
        f if f.contains("void") || f.contains("shadow") || f.contains("dark") => {
            "Void Seekers aesthetic: Deep purples and blacks, crystalline void structures, \
            geometric impossibilities, reality-warping materials, obsidian and dark crystals. \
            Emphasize angular, geometric forms with void-touched corruption."
        }
        _ => {
            "Generic horror aesthetic: Muted colors progressing to darkness, weathered materials, \
            asymmetrical design elements, non-Euclidean geometry hints. Focus on unsettling \
            proportions and otherworldly details."
        }
    };
    
    // Add content-specific style elements
    let content_additions = if content.contains("priest") || content.contains("ritual") {
        " Include ceremonial elements, religious iconography, and ritual implements."
    } else if content.contains("warrior") || content.contains("combat") {
        " Include battle-worn armor, weapon integration, and combat readiness."
    } else if content.contains("building") || content.contains("structure") {
        " Include architectural elements that suggest both function and cosmic significance."
    } else {
        " Include subtle horror elements that suggest cosmic awareness."
    };
    
    format!("{}{}", base_style, content_additions)
}

fn create_technical_specs(category: &str) -> TechnicalSpecs {
    match category {
        "warrior_unit" | "priest_unit" | "cultist_unit" => TechnicalSpecs {
            target_poly_count: "2000-5000 triangles".to_string(),
            texture_resolution: "1024x1024 diffuse, normal, roughness".to_string(),
            required_animations: vec![
                "idle".to_string(),
                "walk".to_string(),
                "attack".to_string(),
                "death".to_string(),
            ],
            bone_structure: "Humanoid rig with 20-30 bones".to_string(),
            material_requirements: vec![
                "PBR materials".to_string(),
                "Faction-specific textures".to_string(),
                "Wear/corruption details".to_string(),
            ],
        },
        "leader_unit" => TechnicalSpecs {
            target_poly_count: "5000-8000 triangles".to_string(),
            texture_resolution: "2048x2048 diffuse, normal, roughness, emission".to_string(),
            required_animations: vec![
                "idle".to_string(),
                "walk".to_string(),
                "attack".to_string(),
                "cast_spell".to_string(),
                "command".to_string(),
                "death".to_string(),
            ],
            bone_structure: "Advanced humanoid rig with 35-50 bones".to_string(),
            material_requirements: vec![
                "Hero-quality PBR materials".to_string(),
                "Emissive effects".to_string(),
                "Advanced shader features".to_string(),
            ],
        },
        "building" => TechnicalSpecs {
            target_poly_count: "3000-10000 triangles".to_string(),
            texture_resolution: "2048x2048 tileable textures".to_string(),
            required_animations: vec![
                "idle".to_string(),
                "construction".to_string(),
                "destruction".to_string(),
            ],
            bone_structure: "Static mesh or simple destructible segments".to_string(),
            material_requirements: vec![
                "Architectural PBR materials".to_string(),
                "Weathering and age details".to_string(),
                "Faction-specific decorations".to_string(),
            ],
        },
        _ => TechnicalSpecs {
            target_poly_count: "1000-3000 triangles".to_string(),
            texture_resolution: "512x512 basic textures".to_string(),
            required_animations: vec!["idle".to_string()],
            bone_structure: "Simple or static mesh".to_string(),
            material_requirements: vec!["Basic PBR materials".to_string()],
        },
    }
}

fn create_corruption_progression(content: &str) -> Option<CorruptionProgression> {
    let content_lower = content.to_lowercase();
    
    let band = if content_lower.contains("peaceful") || content_lower.contains("village") {
        1
    } else if content_lower.contains("unease") || content_lower.contains("suspicion") {
        2
    } else if content_lower.contains("dread") || content_lower.contains("fear") {
        3
    } else if content_lower.contains("terror") || content_lower.contains("horror") {
        4
    } else if content_lower.contains("void") || content_lower.contains("abyss") {
        5
    } else {
        2 // Default
    };
    
    let (theme, visual_evolution, material_changes) = match band {
        1 => (
            "Peace to Unease - Subtle wrongness",
            "Clean appearance with barely perceptible asymmetries, slight discoloration in corners",
            "Fresh materials with hints of future decay, normal wear patterns with unusual stains"
        ),
        2 => (
            "Unease to Dread - Growing wrongness", 
            "Noticeable asymmetries, unnatural shadows, geometry that seems slightly off",
            "Materials showing stress, unusual corrosion patterns, colors beginning to shift"
        ),
        3 => (
            "Dread to Terror - Manifest corruption",
            "Clear distortions, non-Euclidean elements, unsettling proportions becoming obvious",
            "Heavily corrupted materials, organic growths, metals showing impossible wear"
        ),
        4 => (
            "Terror to Horror - Advanced corruption",
            "Severely distorted geometry, reality-bending elements, disturbing organic integration",
            "Materials fundamentally changed, flesh-metal fusion, impossible material properties"
        ),
        5 => (
            "Final Horror - Total transformation",
            "Completely alien geometry, physics-defying structures, pure cosmic horror manifestation", 
            "Materials transcend normal physics, void-touched substances, reality-warping properties"
        ),
        _ => ("Unknown corruption state", "Standard appearance", "Normal materials"),
    };
    
    Some(CorruptionProgression {
        band,
        theme: theme.to_string(),
        visual_evolution: visual_evolution.to_string(),
        material_changes: material_changes.to_string(),
    })
}

fn create_reference_suggestions(faction: &str, category: &str) -> Vec<String> {
    let mut references = Vec::new();
    
    // Faction-specific references
    match faction.to_lowercase().as_str() {
        f if f.contains("crimson") || f.contains("blood") => {
            references.extend(vec![
                "Bloodborne character designs".to_string(),
                "Dark Souls 3 cathedral knights".to_string(),
                "Warhammer Khorne aesthetics".to_string(),
                "Medieval inquisition imagery".to_string(),
            ]);
        }
        f if f.contains("deep") || f.contains("water") => {
            references.extend(vec![
                "Lovecraft Deep Ones concept art".to_string(),
                "Bioshock underwater aesthetics".to_string(),
                "Subnautica leviathan designs".to_string(),
                "Call of Cthulhu aquatic horrors".to_string(),
            ]);
        }
        f if f.contains("void") || f.contains("shadow") => {
            references.extend(vec![
                "Event Horizon spacecraft design".to_string(),
                "Warhammer 40k Chaos aesthetics".to_string(),
                "Dead Space necromorph designs".to_string(),
                "Lovecraft void imagery".to_string(),
            ]);
        }
        _ => {
            references.push("Generic Lovecraftian horror".to_string());
        }
    }
    
    // Category-specific references
    match category {
        "building" => {
            references.extend(vec![
                "Gothic cathedral architecture".to_string(),
                "Ancient temple ruins".to_string(),
                "Eldritch architectural impossibilities".to_string(),
            ]);
        }
        "leader_unit" => {
            references.extend(vec![
                "Dark fantasy leader designs".to_string(),
                "Cult master aesthetics".to_string(),
                "Imposing horror antagonists".to_string(),
            ]);
        }
        _ => {}
    }
    
    references
}

fn extract_animation_requirements(content: &str) -> Vec<String> {
    let mut requirements = vec![
        "Smooth idle animation with subtle breathing/sway".to_string(),
        "Natural walking cycle appropriate for character type".to_string(),
        "Death animation with appropriate dramatic effect".to_string(),
    ];
    
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("attack") || content_lower.contains("combat") {
        requirements.push("Attack animation with weapon/spell casting motions".to_string());
    }
    if content_lower.contains("ritual") || content_lower.contains("ceremony") {
        requirements.push("Ritual animation with ceremonial gestures".to_string());
    }
    if content_lower.contains("leader") || content_lower.contains("command") {
        requirements.push("Command animation with authoritative gestures".to_string());
    }
    if content_lower.contains("magic") || content_lower.contains("spell") {
        requirements.push("Spellcasting animation with mystical hand movements".to_string());
    }
    
    requirements
}

fn render_model_prompt_to_markdown(template: &ModelPromptTemplate) -> String {
    format!(r#"# 3D Model Generation Prompt: {}

## Model Overview
- **ID**: `{}`
- **Category**: {}
- **Faction**: {}

## Primary Generation Prompt
```
{}
```

## Style Guidelines
```
{}
```

## Technical Specifications
- **Target Poly Count**: {}
- **Texture Resolution**: {}
- **Bone Structure**: {}

### Required Animations
{}

### Material Requirements
{}

## Corruption Progression
{}

## Reference Images/Concepts
{}

## Animation Requirements
{}

---
*Generated by Dragon's Labyrinth Replit Prompter*
*Use this template with Replit's 3D model generation capabilities*
"#,
        template.display_name,
        template.model_id,
        template.category,
        template.faction,
        template.primary_prompt,
        template.style_prompt,
        template.technical_specs.target_poly_count,
        template.technical_specs.texture_resolution,
        template.technical_specs.bone_structure,
        template.technical_specs.required_animations.iter()
            .map(|a| format!("- {}", a))
            .collect::<Vec<_>>()
            .join("\n"),
        template.technical_specs.material_requirements.iter()
            .map(|m| format!("- {}", m))
            .collect::<Vec<_>>()
            .join("\n"),
        template.corruption_progression.as_ref()
            .map(|cp| format!("**Band {}**: {} - {}", cp.band, cp.theme, cp.visual_evolution))
            .unwrap_or_else(|| "Standard progression".to_string()),
        template.reference_images.iter()
            .map(|r| format!("- {}", r))
            .collect::<Vec<_>>()
            .join("\n"),
        template.animation_requirements.iter()
            .map(|a| format!("- {}", a))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn generate_dialogue_prompts(
    input_dir: &PathBuf,
    output_dir: &PathBuf,
    companion_trauma: bool,
) -> Result<()> {
    println!("💬 Generating dialogue prompts...");
    
    let dialogue_dir = output_dir.join("dialogue_prompts");
    std::fs::create_dir_all(&dialogue_dir)?;
    
    // Load analyzed entities
    let entities = load_analyzed_entities(input_dir)?;
    
    // Generate dialogue prompts for characters
    for (faction_name, faction_entities) in &entities.factions {
        for entity in faction_entities {
            if is_dialogue_character(entity) {
                let dialogue_template = create_dialogue_prompt_from_entity(
                    entity, 
                    faction_name, 
                    companion_trauma
                );
                let markdown_content = render_dialogue_prompt_to_markdown(&dialogue_template);
                
                let filename = format!("{}_dialogue.md", sanitize_name(&entity.entity_name));
                std::fs::write(dialogue_dir.join(filename), markdown_content)?;
            }
        }
    }
    
    Ok(())
}

fn is_dialogue_character(entity: &RawEntity) -> bool {
    let content = entity.raw_value.to_lowercase();
    // Characters that should have dialogue
    content.contains("npc") || 
    content.contains("priest") || 
    content.contains("leader") || 
    content.contains("merchant") ||
    content.contains("guard") ||
    content.len() > 500 // Rich content likely to be dialogue-worthy
}

fn create_dialogue_prompt_from_entity(
    entity: &RawEntity,
    faction: &str,
    companion_trauma: bool,
) -> DialoguePromptTemplate {
    let content = &entity.raw_value;
    let role = determine_character_role(content);
    
    let personality_prompt = format!(
        "This character is a {} from the {} faction. {}. \
        They should speak with the manner and concerns appropriate to their role, \
        while underlying cosmic horror themes subtly influence their worldview. \
        Their dialogue should reflect growing awareness of forces beyond normal comprehension.",
        role,
        faction,
        extract_personality_from_content(content)
    );
    
    let trauma_indicators = if companion_trauma {
        extract_trauma_indicators_from_content(content)
    } else {
        vec!["Standard psychological progression".to_string()]
    };
    
    DialoguePromptTemplate {
        character_id: sanitize_name(&entity.entity_name),
        character_name: entity.entity_name.clone(),
        role: role.clone(),
        personality_prompt,
        trauma_indicators,
        speech_patterns: extract_speech_patterns_from_content(content),
        sample_interactions: create_sample_interactions(faction, &role),
        corruption_evolution: if companion_trauma {
            Some(create_corruption_dialogue_evolution(faction))
        } else {
            None
        },
    }
}

fn determine_character_role(content: &str) -> String {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("priest") || content_lower.contains("cleric") {
        "Cult Priest".to_string()
    } else if content_lower.contains("leader") || content_lower.contains("commander") {
        "Faction Leader".to_string()
    } else if content_lower.contains("merchant") || content_lower.contains("trader") {
        "Merchant".to_string()
    } else if content_lower.contains("guard") || content_lower.contains("warrior") {
        "Guardian".to_string()
    } else {
        "Cultist".to_string()
    }
}

fn extract_personality_from_content(content: &str) -> String {
    // Extract personality elements from content
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("wise") || content_lower.contains("knowledgeable") {
        "They possess deep knowledge but speak carefully of forbidden truths"
    } else if content_lower.contains("aggressive") || content_lower.contains("violent") {
        "They are quick to anger and speak of conflict with unsettling enthusiasm"
    } else if content_lower.contains("mysterious") || content_lower.contains("secretive") {
        "They speak in cryptic terms and seem to know more than they reveal"
    } else {
        "They appear normal at first but their words carry undertones of cosmic awareness"
    }.to_string()
}

fn extract_trauma_indicators_from_content(content: &str) -> Vec<String> {
    let mut indicators = vec!["Cosmic awareness growing".to_string()];
    
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("fear") || content_lower.contains("terror") {
        indicators.push("Shows signs of existential dread".to_string());
    }
    if content_lower.contains("madness") || content_lower.contains("insanity") {
        indicators.push("Displays indicators of sanity erosion".to_string());
    }
    if content_lower.contains("void") || content_lower.contains("abyss") {
        indicators.push("Speaks of void experiences with distant eyes".to_string());
    }
    if content_lower.contains("ritual") || content_lower.contains("ceremony") {
        indicators.push("References ritualistic experiences with reverence and unease".to_string());
    }
    
    indicators
}

fn extract_speech_patterns_from_content(content: &str) -> Vec<String> {
    let mut patterns = vec![
        "Uses faction-specific terminology naturally".to_string(),
        "Occasionally references cosmic concepts without explanation".to_string(),
    ];
    
    let content_lower = content.to_lowercase();
    
    // Extract speech patterns based on content analysis
    if content_lower.contains("formal") || content_lower.contains("ceremony") || content_lower.contains("ritual") {
        patterns.push("Speaks in formal, ceremonial tones when discussing sacred matters".to_string());
    } else if content_lower.contains("casual") || content_lower.contains("common") {
        patterns.push("Uses colloquial language but with unsettling undertones".to_string());
    } else {
        patterns.push("Speech becomes more stilted when discussing faction mysteries".to_string());
    }
    
    if content_lower.contains("ancient") || content_lower.contains("old") || content_lower.contains("elder") {
        patterns.push("References ancient knowledge with reverent whispers".to_string());
    }
    
    if content_lower.contains("fear") || content_lower.contains("terror") || content_lower.contains("dread") {
        patterns.push("Voice occasionally trembles when discussing cosmic truths".to_string());
    }
    
    if content_lower.contains("whisper") || content_lower.contains("secret") || content_lower.contains("hidden") {
        patterns.push("Tends to lower voice when sharing forbidden knowledge".to_string());
    }
    
    // Always add cosmic metaphors as fallback
    patterns.push("Uses metaphors related to their cosmic patron".to_string());
    
    patterns
}

fn create_sample_interactions(faction: &str, role: &str) -> Vec<String> {
    vec![
        format!("Greeting: Appropriate welcome for a {} {}", faction, role),
        "Information: Provides relevant faction/location knowledge".to_string(),
        "Quest: Offers tasks related to faction goals".to_string(),
        "Warning: Cryptic advice about cosmic dangers".to_string(),
        "Departure: Dismissal with subtle unease".to_string(),
    ]
}

fn create_corruption_dialogue_evolution(faction: &str) -> String {
    format!(
        "As the story progresses, {faction} characters should show increasing signs of cosmic influence. \
        Early dialogue is relatively normal with subtle hints. Mid-game dialogue becomes more cryptic \
        and references cosmic concepts. Late-game dialogue shows clear signs of otherworldly influence \
        while maintaining character personality. Use this progression to show growing cosmic dominion.",
        faction = faction
    )
}

fn render_dialogue_prompt_to_markdown(template: &DialoguePromptTemplate) -> String {
    format!(r#"# Dialogue Generation Prompt: {}

## Character Overview
- **ID**: `{}`
- **Role**: {}

## Personality Prompt
```
{}
```

## Trauma Indicators (Dragon's Labyrinth Integration)
{}

## Speech Patterns
{}

## Sample Interaction Types
{}

## Corruption Evolution
{}

## Yarnspinner Integration Notes
- Create `.yarn` files with branching dialogue trees
- Include trauma state variables for dynamic responses
- Implement corruption level checks for dialogue variations
- Add faction-specific terminology and references

---
*Generated by Dragon's Labyrinth Replit Prompter*
*Use this template for Yarnspinner dialogue generation*
"#,
        template.character_name,
        template.character_id,
        template.role,
        template.personality_prompt,
        template.trauma_indicators.iter()
            .map(|t| format!("- {}", t))
            .collect::<Vec<_>>()
            .join("\n"),
        template.speech_patterns.iter()
            .map(|p| format!("- {}", p))
            .collect::<Vec<_>>()
            .join("\n"),
        template.sample_interactions.iter()
            .map(|i| format!("- {}", i))
            .collect::<Vec<_>>()
            .join("\n"),
        template.corruption_evolution.as_ref()
            .unwrap_or(&"Standard character development".to_string())
    )
}

fn generate_progression_docs(
    assets_dir: &PathBuf,
    output_dir: &PathBuf,
    visual_guides: bool,
) -> Result<()> {
    println!("📈 Generating progression documentation...");
    
    let progression_dir = output_dir.join("progression_guides");
    std::fs::create_dir_all(&progression_dir)?;
    
    // Create upgrade progression guide enhanced with existing RON assets
    let progression_guide = if assets_dir.exists() {
        create_upgrade_progression_guide_with_assets(assets_dir)?
    } else {
        create_upgrade_progression_guide()
    };
    
    std::fs::write(
        progression_dir.join("upgrade_progressions.md"),
        progression_guide,
    )?;
    
    if visual_guides {
        // Create visual progression examples, potentially enhanced by existing assets
        let visual_guide = if assets_dir.exists() {
            create_visual_progression_guide_with_assets(assets_dir)?
        } else {
            create_visual_progression_guide()
        };
        std::fs::write(
            progression_dir.join("visual_progression_guide.md"),
            visual_guide,
        )?;
    }
    
    println!("✅ Progression documentation generated");
    Ok(())
}

fn create_upgrade_progression_guide() -> String {
    r#"# Unit Upgrade Progression Guide

## Dragon's Labyrinth + Cosmic Cults Integration

This guide shows how to create upgrade progressions that combine Dragon's Labyrinth corruption mechanics with Cosmic Dominion's faction-based upgrades.

## Standard Progression Tiers

### Tier 1: Initiate → Acolyte
- **Requirements**: Basic combat experience
- **Visual Changes**: Minor equipment upgrades, slight posture confidence
- **Corruption**: Minimal cosmic awareness signs
- **Model Variations**: Same base with improved weapons/gear

### Tier 2: Acolyte → Cultist 
- **Requirements**: Ritual knowledge, faction devotion
- **Visual Changes**: Faction-specific markings, ceremonial elements
- **Corruption**: Noticeable otherworldly influence in design
- **Model Variations**: More elaborate outfit, faction symbols

### Tier 3: Cultist → Priest/Veteran
- **Requirements**: Advanced training, cosmic insight
- **Visual Changes**: Dramatic transformation, leadership elements
- **Corruption**: Clear cosmic horror influence in proportions
- **Model Variations**: Significantly enhanced model with unique features

### Tier 4: Priest → Leader/Master
- **Requirements**: Complete faction mastery, cosmic communion
- **Visual Changes**: Heroic/terrifying presence, reality-warping elements
- **Corruption**: Advanced otherworldly transformation
- **Model Variations**: Completely unique hero-class model

## Faction-Specific Progressions

### Crimson Covenant
1. **Blood Initiate** → **Blood Acolyte** → **Hemomancer** → **Sanguine Priest**
   - Visual: Clean robes → Blood stains → Ritual scars → Organic integration

### Order of the Deep
1. **Coastal Initiate** → **Tide Touched** → **Deep Hybrid** → **Dagon Herald**
   - Visual: Human → Wet features → Scaled patches → Aquatic transformation

### Void Seekers
1. **Void Initiate** → **Stargazer** → **Reality Shaper** → **Entropy Lord**
   - Visual: Normal → Geometric tattoos → Crystalline growths → Void manifestation

## Model Generation Guidelines

### Consistency Requirements
- Maintain faction color palette throughout progression
- Preserve core character silhouette while adding elements
- Ensure upgrade feels earned and dramatic
- Include corruption progression from Dragon's Labyrinth

### Technical Considerations
- Each tier should increase polygon budget by ~50%
- Texture resolution can increase for higher tiers
- Animation complexity should grow with rank
- Socket positions must remain compatible for equipment

## Replit Integration
Use these progression concepts when generating model variations:
1. Start with base model prompt
2. Add tier-specific modifications 
3. Include corruption level visual effects
4. Ensure faction aesthetic consistency
"#.to_string()
}

fn create_visual_progression_guide() -> String {
    r#"# Visual Progression Examples for Replit

## Detailed Visual Evolution Templates

### Crimson Covenant Progression

#### Blood Acolyte → Hemomancer
**Base Changes:**
- Robes: Clean crimson → Blood-stained with ritual cuts
- Hands: Normal → Stained with permanent blood marks
- Eyes: Normal → Slight red tinge, dilated pupils
- Posture: Humble → More confident, ritual gestures

**Corruption Indicators:**
- Skin: Normal → Pale with visible veins
- Hair: Normal color → Streaked with premature gray/white
- Accessories: Simple belt → Ritual implements, bone charms

#### Hemomancer → Sanguine Priest
**Advanced Changes:**
- Robes: Stained → Elaborate ceremonial with blood-infused patterns
- Body: Pale → Ritual scarification, organic modifications
- Accessories: Basic tools → Advanced ritual implements, staff
- Aura: None → Visible blood mist effect

### Order of the Deep Progression

#### Coastal Cultist → Tide Touched
**Aquatic Evolution:**
- Skin: Normal → Slightly scaled, damp appearance
- Hair: Dry → Perpetually wet, seaweed-like texture
- Eyes: Normal → Enlarged, deep blue/black
- Clothing: Dry fabric → Water-resistant, barnacle attachments

#### Tide Touched → Deep Hybrid
**Advanced Aquatic Changes:**
- Skin: Scaled patches → Fully scaled, gill slits
- Limbs: Normal → Elongated, webbed fingers
- Clothing: Modified → Minimal, integrated with body
- Movement: Human gait → Fluid, unsettling grace

### Void Seekers Progression

#### Void Initiate → Stargazer
**Geometric Corruption:**
- Skin: Normal → Geometric tattoos, angular scars
- Eyes: Normal → Deep purple, star-like pupils
- Clothing: Fabric → Incorporating crystalline elements
- Posture: Normal → Unnaturally precise movements

#### Stargazer → Reality Shaper
**Reality Distortion:**
- Body: Mostly human → Impossible proportions, void integration
- Materials: Fabric/leather → Void crystals, reality-warping substances
- Effects: None → Visible reality distortion aura
- Presence: Human-scale → Intimidating otherworldly stature

## Replit Prompt Structure

When generating these progressions for Replit:

1. **Start with base description**
2. **Add tier-specific modifications**
3. **Include corruption level indicators**
4. **Specify technical requirements**
5. **Reference cosmic horror themes**

Example Replit prompt:
```
Create a 3D model of a Hemomancer (Tier 2 Blood Acolyte upgrade). Base human form with blood-stained robes showing ritual cuts. Skin is pale with visible dark veins. Eyes have a slight red tinge. Include ritual scarification and bone charms. The model should convey growing cosmic corruption while maintaining human recognizability. Use Bloodborne and Dark Souls 3 as visual references for the dark medieval aesthetic.
```

This creates a clear progression path that Replit can follow for consistent model generation.
"#.to_string()
}

fn generate_prompt_index(output_dir: &PathBuf) -> Result<()> {
    let index_content = r#"# Dragon's Labyrinth Replit Prompt Index

## Overview
This directory contains comprehensive prompt templates for Replit's 3D model and dialogue generation capabilities, based on analyzed data from Dragon's Labyrinth's HBF processing system.

## Directory Structure
```
replit_prompts/
├── model_prompts/           # 3D model generation prompts
│   ├── {faction_name}/      # Organized by faction
│   │   └── {entity}_prompt.md
├── dialogue_prompts/        # Yarnspinner dialogue prompts
│   └── {character}_dialogue.md
└── progression_guides/      # Upgrade progression documentation
    ├── upgrade_progressions.md
    └── visual_progression_guide.md
```

## Usage with Replit

### 3D Model Generation
1. Open the relevant `model_prompts/{faction}/{entity}_prompt.md`
2. Copy the "Primary Generation Prompt" section
3. Use with Replit's 3D model generation feature
4. Apply the technical specifications for proper GLB export
5. Follow the style guidelines for faction consistency

### Dialogue Generation
1. Open the relevant `dialogue_prompts/{character}_dialogue.md`
2. Use the personality prompt as the base for character voice
3. Create Yarnspinner `.yarn` files with the suggested interaction types
4. Include trauma indicators for dynamic dialogue progression
5. Implement corruption evolution for character development

## Integration Features

### Dragon's Labyrinth Integration
- **Corruption Bands**: 5-level progression system for visual evolution
- **Companion Trauma**: Psychological progression system for dialogue
- **Forge Materials**: Upgrade materials based on corruption level
- **Horror Themes**: Consistent atmospheric elements

### Cosmic Dominion Integration  
- **Faction Organization**: Units organized by cult affiliation
- **Upgrade Chains**: Clear progression paths with requirements
- **RON Metadata**: Compatible with existing asset pipeline
- **Technical Specs**: Optimized for web-based 3D rendering

## Best Practices

1. **Consistency**: Always follow faction aesthetic guidelines
2. **Progression**: Ensure upgrades feel earned and dramatic
3. **Performance**: Stick to polygon budgets for web performance
4. **Horror**: Maintain cosmic horror atmosphere throughout
5. **Functionality**: Consider animation and socket requirements

---
*Generated by Dragon's Labyrinth Replit Prompter*
*This toolchain transforms D&D content into production-ready 3D game assets*
"#;

    std::fs::write(output_dir.join("README.md"), index_content)?;
    Ok(())
}

fn load_ron_metadata_from_assets(assets_dir: &PathBuf) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    
    // Search for RON files in assets directory to enhance prompts
    if assets_dir.exists() {
        for entry in walkdir::WalkDir::new(assets_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("ron") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    let file_stem = entry.path().file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");
                    metadata.insert(file_stem.to_string(), content);
                }
            }
        }
    }
    
    Ok(metadata)
}

fn determine_entity_category(content: &str) -> String {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("warrior") || content_lower.contains("fighter") {
        "units"
    } else if content_lower.contains("priest") || content_lower.contains("cleric") {
        "units"
    } else if content_lower.contains("leader") || content_lower.contains("commander") {
        "leaders"
    } else if content_lower.contains("building") || content_lower.contains("structure") {
        "buildings"
    } else if content_lower.contains("terrain") || content_lower.contains("landscape") {
        "terrain"
    } else {
        "units" // Default category
    }.to_string()
}

fn create_model_prompt_from_entity_enhanced(
    entity: &RawEntity,
    faction: &str,
    ron_metadata: &HashMap<String, String>,
) -> ModelPromptTemplate {
    let mut template = create_model_prompt_from_entity(entity, faction);
    
    // Enhance template with RON metadata if available
    let entity_key = sanitize_name(&entity.entity_name);
    if let Some(ron_content) = ron_metadata.get(&entity_key) {
        // Extract additional specifications from RON metadata
        template.primary_prompt = format!(
            "{}\n\nAdditional RON specifications: Consider the technical metadata requirements \
            for animation sockets, material properties, and upgrade compatibility as defined \
            in the asset pipeline.",
            template.primary_prompt
        );
        
        // Enhance technical specs based on RON content
        if ron_content.contains("corruption_band") {
            template.technical_specs.material_requirements.push(
                "Corruption progression materials".to_string()
            );
        }
        if ron_content.contains("sockets") {
            template.technical_specs.material_requirements.push(
                "Equipment socket compatibility".to_string()
            );
        }
    }
    
    template
}

fn create_upgrade_progression_guide_with_assets(assets_dir: &PathBuf) -> Result<String> {
    let base_guide = create_upgrade_progression_guide();
    
    // Scan assets directory for actual upgrade chains
    let mut asset_examples = String::new();
    
    let upgrade_chains_dir = assets_dir.join("upgrade_chains");
    if upgrade_chains_dir.exists() {
        asset_examples.push_str("\n\n## Existing Asset Upgrade Chains\n\n");
        
        for entry in std::fs::read_dir(&upgrade_chains_dir)? {
            if let Ok(entry) = entry {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("ron") {
                    if let Some(filename) = entry.path().file_stem().and_then(|s| s.to_str()) {
                        asset_examples.push_str(&format!("- {}\n", filename.replace('_', " ")));
                    }
                }
            }
        }
    }
    
    // Scan for faction-specific assets
    let units_dir = assets_dir.join("units");
    if units_dir.exists() {
        asset_examples.push_str("\n## Available Faction Units\n\n");
        
        for entry in std::fs::read_dir(&units_dir)? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_dir() {
                    if let Some(faction_name) = entry.path().file_name().and_then(|s| s.to_str()) {
                        let mut unit_count = 0;
                        if let Ok(faction_entries) = std::fs::read_dir(entry.path()) {
                            unit_count = faction_entries.count();
                        }
                        asset_examples.push_str(&format!("- {}: {} units\n", 
                            faction_name.replace('_', " "), unit_count));
                    }
                }
            }
        }
    }
    
    Ok(format!("{}{}", base_guide, asset_examples))
}

fn create_visual_progression_guide_with_assets(assets_dir: &PathBuf) -> Result<String> {
    let base_guide = create_visual_progression_guide();
    
    // Add asset-specific examples based on existing RON files
    let mut asset_enhancements = String::new();
    
    let ron_metadata = load_ron_metadata_from_assets(assets_dir)?;
    if !ron_metadata.is_empty() {
        asset_enhancements.push_str("\n\n## Asset-Based Examples\n\n");
        asset_enhancements.push_str("Based on existing RON metadata files, the following progression examples \n");
        asset_enhancements.push_str("are available for reference:\n\n");
        
        for (asset_name, ron_content) in ron_metadata.iter().take(5) {
            if ron_content.contains("corruption_band") {
                asset_enhancements.push_str(&format!("### {} (Asset Reference)\n", asset_name));
                asset_enhancements.push_str("- Has corruption progression metadata\n");
                if ron_content.contains("upgrades_to") {
                    asset_enhancements.push_str("- Part of upgrade chain\n");
                }
                if ron_content.contains("sockets") {
                    asset_enhancements.push_str("- Has equipment sockets defined\n");
                }
                asset_enhancements.push_str("\n");
            }
        }
    }
    
    Ok(format!("{}{}", base_guide, asset_enhancements))
}

fn load_analyzed_entities(input_dir: &PathBuf) -> Result<RawEntities> {
    // Load from JSON files created by hbf_analyzer
    let mut entities = RawEntities::new();
    
    // Load each category if exists
    if let Ok(content) = std::fs::read_to_string(input_dir.join("regions.json")) {
        entities.regions = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("settlements.json")) {
        entities.settlements = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("factions.json")) {
        entities.factions = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("dungeons.json")) {
        entities.dungeons = serde_json::from_str(&content)?;
    }
    if let Ok(content) = std::fs::read_to_string(input_dir.join("uncategorized.json")) {
        entities.uncategorized = serde_json::from_str(&content)?;
    }
    
    Ok(entities)
}
