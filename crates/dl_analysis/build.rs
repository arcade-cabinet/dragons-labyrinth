//! Build script for dl_analysis crate
//! 
//! Implements the sophisticated HBF analysis system with:
//! - 70,801+ entity extraction from raw/game.hbf using rusqlite
//! - Intelligent clustering by entity type with spatial coordinate extraction
//! - Two-stage AI pipeline with OpenAI structured outputs
//! - UUID relationship mapping and edge typing
//! - Generated Rust models for downstream processing

use anyhow::{Result, Context};
use rusqlite::{Connection, params};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tiktoken_rs::tiktoken::get_bpe_from_model;

// Import our sophisticated analysis system
use dl_analysis::{
    orchestration::RawEntities,
    raw::EntityStats,
    analysis_dir, models_dir,
};

/// Main build function implementing sophisticated HBF analysis pipeline
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../../raw/game.hbf");
    println!("cargo:rerun-if-changed=build.rs");
    
    println!("=== STARTING SOPHISTICATED HBF ANALYSIS PIPELINE ===");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let hbf_path = Path::new("../../raw/game.hbf");
    
    // Verify HBF database exists
    if !hbf_path.exists() {
        println!("cargo:warning=HBF database not found at {:?}, skipping analysis", hbf_path);
        return Ok(());
    }
    
    // Create output directories
    let analysis_output = analysis_dir();
    let models_output = models_dir();
    
    fs::create_dir_all(&analysis_output)?;
    fs::create_dir_all(&models_output)?;
    
    // Create logger for build output
    let mut logger = BuildLogger::new();
    
    // Initialize the sophisticated orchestration system
    let mut orchestrator = RawEntities::new();
    writeln!(logger, "Initialized orchestration system with {} known regions, {} settlements, {} factions, {} dungeons",
        crate::base::KNOWN_REGIONS.len(),
        crate::base::KNOWN_SETTLEMENTS.len(), 
        crate::base::KNOWN_FACTIONS.len(),
        crate::base::KNOWN_DUNGEONS.len()
    )?;
    
    // Extract all entities from HBF database
    writeln!(logger, "Extracting entities from HBF database: {:?}", hbf_path)?;
    let conn = Connection::open(hbf_path)
        .context("Failed to open HBF database")?;
    
    extract_entities_to_orchestrator(&conn, &mut orchestrator, &mut logger)?;
    
    // Run the complete 3-phase analysis pipeline
    let summary = orchestrator.run_complete_analysis(&analysis_output, &models_output, &mut logger)?;
    
    // Display final summary
    writeln!(logger, "\n{}", summary.summary_text())?;
    
    // Create build artifacts for dl_processors
    create_build_artifacts(&out_dir, &summary)?;
    
    writeln!(logger, "=== HBF ANALYSIS PIPELINE COMPLETE ===")?;
    
    Ok(())
}

/// Extract entities from HBF database and feed them to orchestrator
fn extract_entities_to_orchestrator(
    conn: &Connection,
    orchestrator: &mut RawEntities,
    logger: &mut dyn Write,
) -> Result<()> {
    writeln!(logger, "Querying HBF database: SELECT uuid, value FROM Entities")?;
    
    let mut stmt = conn.prepare("SELECT uuid, value FROM Entities")?;
    let entity_count = conn.prepare("SELECT COUNT(*) FROM Entities")?
        .query_row(params![], |row| row.get::<_, i64>(0))?;
    
    writeln!(logger, "Found {} entities in HBF database", entity_count)?;
    
    let entity_iter = stmt.query_map(params![], |row| {
        let uuid: String = row.get(0)?;
        let value: String = row.get(1)?;
        Ok((uuid, value))
    })?;
    
    let mut processed_count = 0;
    for entity_result in entity_iter {
        let (uuid, value) = entity_result?;
        
        // Use our sophisticated orchestrator to handle entity routing
        orchestrator.add_entity(uuid, value);
        processed_count += 1;
        
        if processed_count % 10000 == 0 {
            writeln!(logger, "  Processed {} entities...", processed_count)?;
        }
    }
    
    writeln!(logger, "Completed entity extraction: {} entities processed", processed_count)?;
    Ok(())
}

/// Simple logger for build output
struct BuildLogger;

impl BuildLogger {
    fn new() -> Self {
        Self
    }
}

impl Write for BuildLogger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        print!("cargo:warning={}", s);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Create build artifacts for dl_processors to consume
fn create_build_artifacts(out_dir: &Path, summary: &dl_analysis::AnalysisSummary) -> Result<()> {
    // Create a summary file for dl_processors to read
    let summary_path = out_dir.join("analysis_summary.ron");
    let summary_content = ron::ser::to_string_pretty(summary, ron::ser::PrettyConfig::default())?;
    fs::write(&summary_path, summary_content)?;
    
    // Create a marker file indicating analysis completion
    let marker_path = out_dir.join("analysis_complete.marker");
    fs::write(&marker_path, "Analysis pipeline completed successfully")?;
    
    println!("cargo:warning=Created build artifacts at {:?}", out_dir);
    Ok(())
}
