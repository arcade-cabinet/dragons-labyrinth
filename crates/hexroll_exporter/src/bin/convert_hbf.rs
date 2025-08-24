//! HBF Analysis Tool for Dragon's Labyrinth
//! 
//! Simple tool to analyze HBF SQLite structure and understand the 70k+ entities

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use hexroll_exporter::HbfAnalyzer;

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
        Commands::AnalyzeWithAI { depth, output } => {
            println!("🤖 Analyzing HBF structure with AI assistance (depth level {})...", depth);
            
            let analyzer = HbfAnalyzer::new(&args.input)?;
            let analysis = analyzer.analyze_structure_with_ai(depth).await?;
            
            println!("✅ AI-Enhanced Analysis complete:");
            
            analysis.print_summary();
            
            if let Some(output_path) = output {
                analysis.save_report(&output_path)?;
                println!("📄 Analysis report saved to: {}", output_path.display());
            }
            
            println!("🎯 AI analysis provides semantic insights for 100% accurate data extraction!");
            println!("💡 Review AI recommendations above for optimal relationship mapping");
        }
    }
    
    Ok(())
}
