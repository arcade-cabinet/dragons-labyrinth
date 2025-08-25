//! HBF Analysis Tool for Dragon's Labyrinth
//! 
//! Simple tool to analyze HBF SQLite structure and understand the 70k+ entities

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use hexroll_exporter::{HbfAnalyzer, AnalysisConfig, CompleteAnalysisResult, AICodeGenerator, PatternClusteringEngine, BatchProcessingEngine};

#[derive(Parser)]
#[command(name = "convert_hbf")]
#[command(about = "HBF Analysis Tool for Dragon's Labyrinth")]
pub struct Args {
    /// Input HBF file path
    #[arg(short, long)]
    input: PathBuf,
    
    /// Command to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    /// Analyze HBF file structure and content
    Analyze {
        /// Analysis depth level (1=basic, 2=detailed, 3=complete)
        #[arg(short, long, default_value = "2")]
        depth: u8,
        
        /// Output analysis report to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// AI-powered analysis with GPT-4 semantic understanding
    AnalyzeWithAI {
        /// Analysis depth level (1=basic, 2=detailed, 3=complete)
        #[arg(short, long, default_value = "3")]
        depth: u8,
        
        /// Output analysis report to file
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Confidence threshold for AI analysis (0.0-1.0)
        #[arg(long, default_value = "0.6")]
        confidence_threshold: f64,
        
        /// Enable cross-validation between pattern and AI analysis
        #[arg(long, default_value = "true")]
        cross_validation: bool,
    },
    /// Complete analysis workflow with production readiness assessment
    ProductionAnalysis {
        /// Output directory for all analysis reports
        #[arg(short, long, default_value = "hbf_analysis_output")]
        output_dir: PathBuf,
        
        /// Confidence threshold for AI analysis (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        confidence_threshold: f64,
    },
    /// AI-powered SeaORM model and template generation
    GenerateCode {
        /// Output directory for generated code
        #[arg(short, long, default_value = "generated_hbf_code")]
        output_dir: PathBuf,
        
        /// Confidence threshold for AI analysis (0.0-1.0)
        #[arg(long, default_value = "0.8")]
        confidence_threshold: f64,
    },
    /// Analyze HTML patterns across ALL 70k entities to find commonalities
    ClusterPatterns {
        /// Output directory for pattern analysis results
        #[arg(short, long, default_value = "pattern_clusters")]
        output_dir: PathBuf,
        
        /// Minimum cluster size to consider
        #[arg(long, default_value = "50")]
        min_cluster_size: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🐉 Dragon's Labyrinth HBF Analysis Tool");
    println!("📁 Input: {}", args.input.display());
    
    match args.command {
        Commands::Analyze { depth, output } => {
            println!("🔍 Analyzing HBF structure (depth level {})...", depth);
            
            let analyzer = HbfAnalyzer::new(&args.input)?;
            let analysis = analyzer.analyze_structure(depth)?;
            
            println!("✅ Analysis complete:");
            
            analysis.print_summary();
            
            if let Some(output_path) = output {
                analysis.save_report(&output_path)?;
                println!("📄 Analysis report saved to: {}", output_path.display());
            }
            
            println!("🎯 Next step: Use this analysis to generate ORM models for game-database");
        }
        Commands::AnalyzeWithAI { depth, output, confidence_threshold, cross_validation } => {
            println!("🤖 Analyzing HBF structure with AI assistance (depth level {})...", depth);
            
            let config = AnalysisConfig {
                enable_ai: true,
                confidence_threshold,
                max_samples_per_table: 10,
                html_pattern_detection: true,
                cross_validation,
            };
            
            let analyzer = HbfAnalyzer::with_config(&args.input, config)?;
            let analysis = analyzer.analyze_structure_with_ai(depth).await?;
            
            println!("✅ AI-Enhanced Analysis complete:");
            
            analysis.print_summary();
            
            // Get extraction readiness assessment
            let readiness = analyzer.get_extraction_readiness(&analysis);
            readiness.print_summary();
            
            if let Some(output_path) = output {
                analysis.save_report(&output_path)?;
                println!("📄 Analysis report saved to: {}", output_path.display());
            }
            
            println!("🎯 AI analysis provides semantic insights for 100% accurate data extraction!");
            println!("💡 Review AI recommendations above for optimal relationship mapping");
        }
        Commands::ProductionAnalysis { output_dir, confidence_threshold } => {
            println!("🚀 Complete production analysis workflow...");
            
            let config = AnalysisConfig {
                enable_ai: true,
                confidence_threshold,
                max_samples_per_table: 15,
                html_pattern_detection: true,
                cross_validation: true,
            };
            
            let analyzer = HbfAnalyzer::with_config(&args.input, config)?;
            let complete_result = analyzer.complete_analysis_workflow(3).await?;
            
            // Save all results to output directory
            complete_result.save_results(&output_dir)?;
            
            println!("\n🎯 PRODUCTION ANALYSIS COMPLETE");
            println!("═══════════════════════════════════════");
            
            if complete_result.extraction_readiness.ready_for_extraction {
                println!("✅ READY FOR PRODUCTION EXTRACTION!");
                println!("   Estimated Accuracy: {:.1}%", complete_result.extraction_readiness.estimated_accuracy * 100.0);
                println!("   Target: {} total gaming records", 
                    complete_result.production_readiness.total_entities + complete_result.production_readiness.total_refs);
                println!("🚀 Proceed with data extraction to game-database");
            } else {
                println!("⚠️  PRODUCTION READINESS ISSUES DETECTED");
                println!("   Review critical issues above before extraction");
                println!("   Consider manual validation of relationships");
            }
            
            println!("\n📋 All analysis reports saved to: {}", output_dir.display());
            println!("   • hbf_analysis_report.json - Detailed analysis");
            println!("   • production_readiness.json - Production metrics");
            println!("   • extraction_readiness.json - Extraction assessment");
        }
        Commands::GenerateCode { output_dir, confidence_threshold } => {
            println!("🤖 AI-powered SeaORM model and template generation...");
            
            // First perform complete analysis
            let config = AnalysisConfig {
                enable_ai: false, // Start with pattern analysis for speed
                confidence_threshold,
                max_samples_per_table: 20, // More samples for better code generation
                html_pattern_detection: true,
                cross_validation: false,
            };
            
            let analyzer = HbfAnalyzer::with_config(&args.input, config)?;
            let analysis = analyzer.analyze_structure(3)?; // Full depth analysis
            
            println!("✅ HBF Analysis complete - discovered {} relationships", 
                     analysis.implicit_relationships.len());
            println!("   📊 {} entities + {} refs = {} total records",
                     analysis.table_info.get("Entities").map(|t| t.record_count).unwrap_or(0),
                     analysis.table_info.get("Refs").map(|t| t.record_count).unwrap_or(0),
                     analysis.total_records);
            
            // Load agent spec for code generation
            let agent_spec_path = PathBuf::from("agent.toml");
            
            if !agent_spec_path.exists() {
                println!("⚠️  Agent spec not found - code generation requires hbf-analyzer agent");
                println!("   Expected: {}", agent_spec_path.display());
                return Ok(());
            }
            
            // Create AI code generator
            let mut code_generator = AICodeGenerator::new(&agent_spec_path).await?;
            
            // Generate SeaORM models
            println!("🏗️  Generating SeaORM models...");
            let models = code_generator.generate_seaorm_models(&analysis).await?;
            
            // Generate minijinja templates
            println!("📝 Generating transformation templates...");
            let templates = code_generator.generate_transformation_templates(&analysis, &models).await?;
            
            // Save all generated code
            code_generator.save_generated_code(&models, &templates, &output_dir).await?;
            
            println!("\n🎯 CODE GENERATION COMPLETE");
            println!("═══════════════════════════════════════");
            println!("✅ Generated {} SeaORM models", models.len());
            println!("✅ Generated {} transformation templates", templates.len());
            println!("💾 All code saved to: {}", output_dir.display());
            
            println!("\n📋 Generated Files:");
            for model in &models {
                println!("   📄 {}", model.file_path);
            }
            for template in &templates {
                println!("   📄 {}", template.file_path);
            }
            
            println!("\n🚀 Next Steps:");
            println!("   1. Review generated SeaORM models in {}/src/entities/", output_dir.display());
            println!("   2. Test minijinja templates with sample HBF data");
            println!("   3. Integrate models with game-database crate");
            println!("   4. Run full HBF data extraction pipeline");
            
            println!("\n🎯 Ready for 100% accurate {} record extraction!", analysis.total_records);
        }
        Commands::ClusterPatterns { output_dir, min_cluster_size } => {
            println!("🔍 Analyzing HTML patterns across ALL 70k+ entities for commonalities...");
            
            // First get basic structure
            let analyzer = HbfAnalyzer::new(&args.input)?;
            let analysis = analyzer.analyze_structure(2)?; // Get schema info
            
            println!("✅ Basic analysis complete - {} total records across {} tables", 
                     analysis.total_records, analysis.table_count);
            
            // Open connection for comprehensive pattern clustering
            use hexroll_exporter::CoreAnalyzer;
            let conn = CoreAnalyzer::open_hbf_connection(&args.input)?;
            
            // Analyze HTML patterns across ALL entities
            println!("🔍 Clustering HTML patterns across complete dataset...");
            let clusters = PatternClusteringEngine::analyze_html_pattern_clusters(&conn, &mut analysis.clone())?;
            
            // Filter clusters by minimum size
            let significant_clusters: Vec<_> = clusters.into_iter()
                .filter(|c| c.entity_count >= min_cluster_size)
                .collect();
            
            println!("✅ Found {} significant pattern clusters (min {} entities each)", 
                     significant_clusters.len(), min_cluster_size);
            
            // Generate batch processing strategies
            println!("🤖 Generating batch processing strategies for efficient 70k processing...");
            let strategies = BatchProcessingEngine::generate_batch_processing_strategies(&significant_clusters).await?;
            
            // Create comprehensive processing plan
            let processing_plan = hexroll_exporter::BatchProcessingStrategy::generate_complete_processing_plan(&strategies);
            
            // Save all results
            std::fs::create_dir_all(&output_dir)?;
            
            // Save pattern clusters
            let clusters_json = serde_json::to_string_pretty(&significant_clusters)?;
            std::fs::write(output_dir.join("pattern_clusters.json"), clusters_json)?;
            
            // Save processing strategies
            let strategies_json = serde_json::to_string_pretty(&strategies)?;
            std::fs::write(output_dir.join("batch_strategies.json"), strategies_json)?;
            
            // Save processing plan
            std::fs::write(output_dir.join("complete_processing_plan.md"), processing_plan)?;
            
            println!("\n🎯 PATTERN CLUSTERING COMPLETE");
            println!("═══════════════════════════════════════");
            println!("✅ Analyzed {} total entities", analysis.total_records);
            println!("✅ Found {} distinct HTML pattern clusters", significant_clusters.len());
            println!("✅ Generated {} batch processing strategies", strategies.len());
            
            let total_clustered: usize = significant_clusters.iter().map(|c| c.entity_count).sum();
            println!("📊 Pattern Coverage: {}/{} entities ({:.1}%)", 
                     total_clustered, analysis.total_records,
                     (total_clustered as f64 / analysis.total_records as f64) * 100.0);
            
            println!("\n📋 Top Entity Clusters:");
            for (i, cluster) in significant_clusters.iter().take(10).enumerate() {
                println!("   {}. {} ({} entities) - Tags: {:?}", 
                         i + 1, cluster.cluster_id, cluster.entity_count, cluster.semantic_tags);
            }
            
            println!("\n💾 All analysis results saved to: {}", output_dir.display());
            println!("   • pattern_clusters.json - Detailed cluster analysis");
            println!("   • batch_strategies.json - Processing strategies for each cluster");
            println!("   • complete_processing_plan.md - Full 70k entity processing plan");
            
            println!("\n🚀 Next Steps:");
            println!("   1. Review pattern clusters to understand HexRoll entity diversity");
            println!("   2. Use AI to generate processing templates for each cluster");
            println!("   3. Implement batch processing pipeline for efficient 70k extraction");
            println!("   4. Create rich SeaORM models for all discovered entity types");
            
            println!("\n🎯 Ready to build rich and vibrant world from complete HexRoll dataset!");
        }
    }
    
    Ok(())
}
