//! HBF Analysis Binary - Interactive querying and HTML refinement tool
//! 
//! Standalone binary for analyzing HBF database files with category/subcategory
//! filtering, HTML inspection, and processing refinement capabilities.

use anyhow::Result;
use clap::{Parser, Subcommand};
use dl_seeds::{
    containers::RawEntity,
    orchestration::RawEntities,
    reporting::generate_all_reports,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hbf-analyzer")]
#[command(about = "Dragon's Labyrinth HBF Database Analysis Tool")]
#[command(version = "1.0.0")]
struct Cli {
    /// Path to HBF database file
    #[arg(short, long)]
    database: PathBuf,
    
    /// Output directory for analysis results
    #[arg(short, long, default_value = "analysis_output")]
    output: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze all entities and generate reports
    AnalyzeAll {
        /// Generate CSV reports
        #[arg(long)]
        reports: bool,
    },
    /// Query specific categories
    Query {
        /// Category to query (regions, settlements, factions, dungeons)
        #[arg(short, long)]
        category: Option<String>,
        
        /// Specific entity name to inspect
        #[arg(short, long)]
        entity: Option<String>,
        
        /// Show raw HTML content
        #[arg(long)]
        show_html: bool,
        
        /// Limit number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Inspect HTML processing for specific entities
    InspectHtml {
        /// Entity UUID to inspect
        uuid: String,
        
        /// Show processing steps
        #[arg(long)]
        verbose: bool,
    },
    /// Refine categorization rules
    RefineCategories {
        /// Test new categorization rules
        #[arg(long)]
        test_rules: bool,
        
        /// Apply refinements to database
        #[arg(long)]
        apply: bool,
    },
    /// Export entities for external processing
    Export {
        /// Export format (json, ron, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("🔍 Dragon's Labyrinth HBF Analyzer");
    println!("📁 Database: {}", cli.database.display());
    println!("📁 Output: {}", cli.output.display());
    
    // Ensure output directory exists
    std::fs::create_dir_all(&cli.output)?;
    
    match &cli.command {
        Commands::AnalyzeAll { reports } => {
            analyze_all_entities(&cli.database, &cli.output, *reports)?;
        }
        Commands::Query { category, entity, show_html, limit } => {
            query_entities(&cli.database, category.as_deref(), entity.as_deref(), *show_html, *limit)?;
        }
        Commands::InspectHtml { uuid, verbose } => {
            inspect_html_processing(&cli.database, uuid, *verbose)?;
        }
        Commands::RefineCategories { test_rules, apply } => {
            refine_categorization_rules(&cli.database, &cli.output, *test_rules, *apply)?;
        }
        Commands::Export { format, category } => {
            export_entities(&cli.database, &cli.output, format, category.as_deref())?;
        }
    }
    
    Ok(())
}

fn analyze_all_entities(database_path: &PathBuf, output_dir: &PathBuf, generate_reports: bool) -> Result<()> {
    println!("🔄 Analyzing all entities from HBF database with training data enhancement...");
    
    // Try to load training data for enhanced categorization
    let training_dir = std::env::current_dir()?.join("training_data");
    let mut use_training = false;
    
    if training_dir.exists() {
        println!("📚 Training data directory found, loading enhanced categorization...");
        use_training = true;
    } else {
        println!("⚠️ No training data found, using basic categorization");
    }
    
    // Load and process all entities with or without training
    let mut raw_entities = if use_training {
        // Use enhanced categorization with training data
        let (mut entities, training_repo) = RawEntities::new_with_training(&training_dir)?;
        println!("✅ Loaded {} training examples from TOML files", training_repo.total_examples());
        
        // Load entities from database using enhanced categorization
        entities.load_from_hbf_database_with_training(database_path, &training_repo)?;
        entities
    } else {
        // Use basic categorization
        let mut entities = RawEntities::new();
        entities.load_from_hbf_database(database_path)?;
        entities
    };
    
    // Write organized entities to disk
    raw_entities.write_all_entities(output_dir)?;
    
    // Get analysis summary
    let summary = raw_entities.get_analysis_summary();
    
    println!("📊 Analysis Complete:");
    println!("  Total entities: {}", summary.total_entities);
    println!("  Regions: {}", summary.regions_count);
    println!("  Settlements: {}", summary.settlements_count);
    println!("  Factions: {}", summary.factions_count);
    println!("  Dungeons: {}", summary.dungeons_count);
    if use_training {
        println!("  Characters: {}", summary.characters_count);
        println!("  Creatures: {}", summary.creatures_count);
        println!("  Items: {}", summary.items_count);
        println!("  Spells: {}", summary.spells_count);
        println!("  Mechanics: {}", summary.mechanics_count);
    }
    println!("  Uncategorized: {}", summary.uncategorized_count);
    
    // Calculate categorization success rate
    let total_categorized = summary.total_entities - summary.uncategorized_count;
    let success_rate = if summary.total_entities > 0 {
        (total_categorized as f32 / summary.total_entities as f32 * 100.0).round()
    } else { 0.0 };
    println!("📈 Categorization Success Rate: {}% ({} of {} entities)", 
             success_rate, total_categorized, summary.total_entities);
    
    // Generate CSV reports if requested
    if generate_reports {
        println!("📈 Generating comprehensive reports...");
        let reports_dir = output_dir.join("reports");
        generate_all_reports(
            &raw_entities.regions,
            &raw_entities.settlements,
            &raw_entities.factions,
            &raw_entities.dungeons,
            &raw_entities.uncategorized,
            &reports_dir,
        )?;
        println!("✅ Reports generated in: {}", reports_dir.display());
    }
    
    Ok(())
}

fn query_entities(
    database_path: &PathBuf,
    category: Option<&str>,
    entity_name: Option<&str>,
    show_html: bool,
    limit: usize,
) -> Result<()> {
    println!("🔍 Querying entities...");
    
    // Load training data if available for consistent results
    let training_dir = std::env::current_dir()?.join("training_data");
    let raw_entities = if training_dir.exists() {
        println!("📚 Loading with training data for consistent categorization...");
        let (mut entities, training_repo) = RawEntities::new_with_training(&training_dir)?;
        entities.load_from_hbf_database_with_training(database_path, &training_repo)?;
        entities
    } else {
        let mut entities = RawEntities::new();
        entities.load_from_hbf_database(database_path)?;
        entities
    };
    
    match category {
        Some("regions") => query_category(&raw_entities.regions, entity_name, show_html, limit)?,
        Some("settlements") => query_category(&raw_entities.settlements, entity_name, show_html, limit)?,
        Some("factions") => query_category(&raw_entities.factions, entity_name, show_html, limit)?,
        Some("dungeons") => query_category(&raw_entities.dungeons, entity_name, show_html, limit)?,
        Some("characters") => query_category(&raw_entities.characters, entity_name, show_html, limit)?,
        Some("creatures") => query_category(&raw_entities.creatures, entity_name, show_html, limit)?,
        Some("items") => query_category(&raw_entities.items, entity_name, show_html, limit)?,
        Some("spells") => query_category(&raw_entities.spells, entity_name, show_html, limit)?,
        Some("mechanics") => query_category(&raw_entities.mechanics, entity_name, show_html, limit)?,
        Some(cat) => {
            println!("❌ Unknown category: {}. Use: regions, settlements, factions, dungeons, characters, creatures, items, spells, mechanics", cat);
            return Ok(());
        }
        None => {
            println!("📋 Available categories:");
            println!("  regions: {} groups", raw_entities.regions.len());
            println!("  settlements: {} groups", raw_entities.settlements.len());
            println!("  factions: {} groups", raw_entities.factions.len());
            println!("  dungeons: {} groups", raw_entities.dungeons.len());
            println!("  characters: {} groups", raw_entities.characters.len());
            println!("  creatures: {} groups", raw_entities.creatures.len());
            println!("  items: {} groups", raw_entities.items.len());
            println!("  spells: {} groups", raw_entities.spells.len());
            println!("  mechanics: {} groups", raw_entities.mechanics.len());
            println!("  uncategorized: {} entities", raw_entities.uncategorized.len());
        }
    }
    
    Ok(())
}

fn query_category(
    category_data: &std::collections::HashMap<String, Vec<RawEntity>>,
    entity_name: Option<&str>,
    show_html: bool,
    limit: usize,
) -> Result<()> {
    match entity_name {
        Some(name) => {
            if let Some(entities) = category_data.get(name) {
                println!("📝 Found {} entities for '{}':", entities.len(), name);
                for (i, entity) in entities.iter().take(limit).enumerate() {
                    println!("  {}. UUID: {}", i + 1, entity.uuid);
                    if show_html {
                        println!("     HTML: {}", 
                            if entity.raw_value.len() > 200 {
                                format!("{}...", &entity.raw_value[..200])
                            } else {
                                entity.raw_value.clone()
                            }
                        );
                    }
                }
            } else {
                println!("❌ Entity '{}' not found", name);
                println!("📋 Available entities:");
                for (i, name) in category_data.keys().take(10).enumerate() {
                    println!("  {}. {}", i + 1, name);
                }
            }
        }
        None => {
            println!("📋 Available entities in category:");
            for (i, (name, entities)) in category_data.iter().take(limit).enumerate() {
                println!("  {}. {} ({} entities)", i + 1, name, entities.len());
            }
        }
    }
    
    Ok(())
}

fn inspect_html_processing(database_path: &PathBuf, uuid: &str, verbose: bool) -> Result<()> {
    println!("🔍 Inspecting HTML processing for UUID: {}", uuid);
    
    let mut raw_entities = RawEntities::new();
    raw_entities.load_from_hbf_database(database_path)?;
    
    // Search for the specific UUID across all categories
    let mut found = false;
    
    for (category_name, category_data) in [
        ("regions", &raw_entities.regions),
        ("settlements", &raw_entities.settlements), 
        ("factions", &raw_entities.factions),
        ("dungeons", &raw_entities.dungeons),
    ] {
        for (entity_name, entities) in category_data {
            for entity in entities {
                if entity.uuid == uuid {
                    found = true;
                    println!("✅ Found entity in {}/{}", category_name, entity_name);
                    println!("📝 Category: {}", entity.category);
                    println!("📝 Entity Name: {}", entity.entity_name);
                    
                    if verbose {
                        println!("📄 Raw HTML Content:");
                        println!("{}", entity.raw_value);
                        
                        println!("\n🔬 Processing Analysis:");
                        analyze_html_content(&entity.raw_value);
                    } else {
                        println!("📄 HTML Preview (first 300 chars):");
                        println!("{}", 
                            if entity.raw_value.len() > 300 {
                                format!("{}...", &entity.raw_value[..300])
                            } else {
                                entity.raw_value.clone()
                            }
                        );
                    }
                    return Ok(());
                }
            }
        }
    }
    
    // Check uncategorized
    for entity in &raw_entities.uncategorized {
        if entity.uuid == uuid {
            found = true;
            println!("⚠️ Found entity in uncategorized");
            println!("📝 Category: {}", entity.category);
            println!("📝 Entity Name: {}", entity.entity_name);
            
            if verbose {
                println!("📄 Raw HTML Content:");
                println!("{}", entity.raw_value);
            }
            break;
        }
    }
    
    if !found {
        println!("❌ Entity with UUID '{}' not found", uuid);
    }
    
    Ok(())
}

fn analyze_html_content(html: &str) {
    println!("  Length: {} characters", html.len());
    println!("  Contains tables: {}", html.contains("<table"));
    println!("  Contains lists: {}", html.contains("<ul") || html.contains("<ol"));
    println!("  Contains spoilers: {}", html.contains("spoiler"));
    println!("  Contains coordinates: {}", html.contains("coordinate") || html.contains("location"));
    println!("  Contains dice notation: {}", html.contains("d6") || html.contains("d20"));
    println!("  Contains stat blocks: {}", html.contains("AC") || html.contains("HP"));
}

fn refine_categorization_rules(
    database_path: &PathBuf,
    output_dir: &PathBuf,
    test_rules: bool,
    apply: bool,
) -> Result<()> {
    println!("🔧 Refining categorization rules...");
    
    if test_rules {
        println!("🧪 Testing new categorization rules (dry run)");
        // Load entities and test new rules without applying
        let mut raw_entities = RawEntities::new();
        raw_entities.load_from_hbf_database(database_path)?;
        
        // Show current categorization stats
        let summary = raw_entities.get_analysis_summary();
        println!("📊 Current Stats:");
        println!("  Regions: {}", summary.regions_count);
        println!("  Settlements: {}", summary.settlements_count);
        println!("  Factions: {}", summary.factions_count);
        println!("  Dungeons: {}", summary.dungeons_count);
        println!("  Uncategorized: {}", summary.uncategorized_count);
        
        // Analyze uncategorized entities for patterns
        if summary.uncategorized_count > 0 {
            println!("\n🔍 Analyzing uncategorized entities for patterns:");
            analyze_uncategorized_patterns(&raw_entities.uncategorized);
        }
    }
    
    if apply {
        println!("⚠️ Apply functionality not yet implemented");
        println!("   This would update categorization rules and re-process database");
    }
    
    Ok(())
}

fn analyze_uncategorized_patterns(uncategorized: &[RawEntity]) {
    let mut content_patterns = std::collections::HashMap::new();
    
    for entity in uncategorized.iter().take(20) {
        let content = entity.raw_value.to_lowercase();
        
        // Look for common patterns
        if content.contains("village") || content.contains("town") {
            *content_patterns.entry("settlements").or_insert(0) += 1;
        }
        if content.contains("guild") || content.contains("organization") {
            *content_patterns.entry("factions").or_insert(0) += 1;
        }
        if content.contains("cave") || content.contains("lair") || content.contains("crypt") {
            *content_patterns.entry("dungeons").or_insert(0) += 1;
        }
        if content.contains("forest") || content.contains("mountain") || content.contains("biome") {
            *content_patterns.entry("regions").or_insert(0) += 1;
        }
    }
    
    println!("  Potential patterns found:");
    for (pattern, count) in content_patterns {
        println!("    {}: {} entities", pattern, count);
    }
}

fn export_entities(
    database_path: &PathBuf,
    output_dir: &PathBuf,
    format: &str,
    category_filter: Option<&str>,
) -> Result<()> {
    println!("📤 Exporting entities in {} format...", format);
    
    let mut raw_entities = RawEntities::new();
    raw_entities.load_from_hbf_database(database_path)?;
    
    match format {
        "json" => {
            export_as_json(&raw_entities, output_dir, category_filter)?;
        }
        "ron" => {
            export_as_ron(&raw_entities, output_dir, category_filter)?;
        }
        "csv" => {
            generate_all_reports(
                &raw_entities.regions,
                &raw_entities.settlements,
                &raw_entities.factions,
                &raw_entities.dungeons,
                &raw_entities.uncategorized,
                &output_dir.join("csv_export"),
            )?;
        }
        _ => {
            println!("❌ Unknown format: {}. Use: json, ron, csv", format);
        }
    }
    
    Ok(())
}

fn export_as_json(
    raw_entities: &RawEntities,
    output_dir: &PathBuf,
    category_filter: Option<&str>,
) -> Result<()> {
    let export_dir = output_dir.join("json_export");
    std::fs::create_dir_all(&export_dir)?;
    
    match category_filter {
        Some("regions") => {
            let json = serde_json::to_string_pretty(&raw_entities.regions)?;
            std::fs::write(export_dir.join("regions.json"), json)?;
        }
        Some("settlements") => {
            let json = serde_json::to_string_pretty(&raw_entities.settlements)?;
            std::fs::write(export_dir.join("settlements.json"), json)?;
        }
        Some("factions") => {
            let json = serde_json::to_string_pretty(&raw_entities.factions)?;
            std::fs::write(export_dir.join("factions.json"), json)?;
        }
        Some("dungeons") => {
            let json = serde_json::to_string_pretty(&raw_entities.dungeons)?;
            std::fs::write(export_dir.join("dungeons.json"), json)?;
        }
        _ => {
            // Export all categories
            raw_entities.write_all_entities(&export_dir)?;
        }
    }
    
    println!("✅ JSON export complete: {}", export_dir.display());
    Ok(())
}

fn export_as_ron(
    raw_entities: &RawEntities,
    output_dir: &PathBuf,
    category_filter: Option<&str>,
) -> Result<()> {
    let export_dir = output_dir.join("ron_export");
    std::fs::create_dir_all(&export_dir)?;
    
    match category_filter {
        Some("regions") => {
            let ron = ron::to_string(&raw_entities.regions)?;
            std::fs::write(export_dir.join("regions.ron"), ron)?;
        }
        Some("settlements") => {
            let ron = ron::to_string(&raw_entities.settlements)?;
            std::fs::write(export_dir.join("settlements.ron"), ron)?;
        }
        Some("factions") => {
            let ron = ron::to_string(&raw_entities.factions)?;
            std::fs::write(export_dir.join("factions.ron"), ron)?;
        }
        Some("dungeons") => {
            let ron = ron::to_string(&raw_entities.dungeons)?;
            std::fs::write(export_dir.join("dungeons.ron"), ron)?;
        }
        _ => {
            // Export all categories
            let regions_ron = ron::to_string(&raw_entities.regions)?;
            std::fs::write(export_dir.join("regions.ron"), regions_ron)?;
            
            let settlements_ron = ron::to_string(&raw_entities.settlements)?;
            std::fs::write(export_dir.join("settlements.ron"), settlements_ron)?;
            
            let factions_ron = ron::to_string(&raw_entities.factions)?;
            std::fs::write(export_dir.join("factions.ron"), factions_ron)?;
            
            let dungeons_ron = ron::to_string(&raw_entities.dungeons)?;
            std::fs::write(export_dir.join("dungeons.ron"), dungeons_ron)?;
        }
    }
    
    println!("✅ RON export complete: {}", export_dir.display());
    Ok(())
}
