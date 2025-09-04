//! dl_processors build script with audit integration
//!
//! CORRECT ARCHITECTURE:
//! Build-time: Call dl_analysis to generate organized output (RON files by category)
//! Runtime: Process that organized output using AI-generated models into ECS resources
//! Now includes comprehensive audit reporting for build chain metrics

use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let build_start = Instant::now();
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../dl_analysis");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Set up audit reports directory for build chain tracking
    let audit_reports_dir = env::var("AUDIT_REPORTS_DIR")
        .unwrap_or_else(|_| "audit_reports".to_string());
    println!("cargo:rustc-env=AUDIT_REPORTS_DIR={}", audit_reports_dir);
    
    // CORRECT: dl_processors BUILD TIME calls dl_analysis to generate organized output
    println!("cargo:warning=Calling dl_analysis to generate organized output with audit tracking...");
    
    // Get dl_analysis output directories
    let analysis_output_dir = dl_analysis::analysis_dir(); 
    let models_dir = dl_analysis::models_dir();
    let templates_dir = PathBuf::from("crates/dl_analysis/templates");
    
    // Call dl_analysis orchestration to generate organized subdirectories
    let hbf_path = PathBuf::from("crates/dl_analysis/game.hbf");
    if hbf_path.exists() {
        // Run dl_analysis to generate organized output with audit reporting
        let summary = dl_analysis::orchestration::RawEntities::run_complete_analysis(
            &hbf_path,
            &analysis_output_dir, 
            &models_dir,
            &templates_dir
        )?;
        
        println!("cargo:warning=dl_analysis generated organized output: {} total entities", 
                 summary.total_entities);
        println!("cargo:warning=  Organized into regions/, dungeons/, settlements/, factions/ subdirectories");
        println!("cargo:warning=  AI-generated models available in {}", models_dir.display());
        
        // Generate build chain audit report
        generate_build_chain_audit(&summary, &analysis_output_dir, &models_dir, build_start)?;
        
    } else {
        println!("cargo:warning=HBF database not found at: {}", hbf_path.display());
        println!("cargo:warning=dl_analysis will provide empty organized output");
        
        // Generate empty organized output for development
        std::fs::create_dir_all(&analysis_output_dir)?;
        std::fs::create_dir_all(&models_dir)?;
        
        // Generate empty build audit for development builds
        generate_empty_build_audit(build_start)?;
    }
    
    // Pass output directory info to runtime
    println!("cargo:rustc-env=DL_ANALYSIS_OUTPUT_DIR={}", analysis_output_dir.display());
    println!("cargo:rustc-env=DL_MODELS_DIR={}", models_dir.display());
    
    let build_duration = build_start.elapsed();
    println!("cargo:warning=dl_processors build complete - organized output ready for runtime");
    println!("cargo:warning=Total build time: {}ms", build_duration.as_millis());
    
    Ok(())
}

/// Generate comprehensive build chain audit report
fn generate_build_chain_audit(
    summary: &dl_analysis::AnalysisSummary,
    analysis_output_dir: &PathBuf,
    models_dir: &PathBuf,
    build_start: Instant,
) -> Result<()> {
    use dl_audit::AuditSystem;
    use std::collections::HashMap;
    
    // Only generate audit if reports directory is available
    if let Ok(reports_dir) = std::env::var("AUDIT_REPORTS_DIR") {
        let audit_system = AuditSystem::new(&reports_dir);
        let build_duration = build_start.elapsed();
        
        // Create build chain audit data
        let build_audit = BuildChainAudit {
            total_entities_processed: summary.total_entities,
            analysis_output_files: count_files_in_dir(analysis_output_dir)?,
            models_generated: count_files_in_dir(models_dir)?,
            build_time_ms: build_duration.as_millis() as u64,
            analysis_output_size_mb: get_directory_size_mb(analysis_output_dir)?,
            models_output_size_mb: get_directory_size_mb(models_dir)?,
            entities_per_second: (summary.total_entities as f64 / build_duration.as_secs_f64()) as u64,
            build_stage: "processors_build".to_string(),
            success: true,
        };
        
        if let Err(e) = audit_system.generate_report(&[build_audit], "build_chain_performance") {
            eprintln!("Warning: Failed to generate build chain audit report: {}", e);
        }
    }
    
    Ok(())
}

/// Generate empty build audit for development builds without HBF
fn generate_empty_build_audit(build_start: Instant) -> Result<()> {
    use dl_audit::AuditSystem;
    
    if let Ok(reports_dir) = std::env::var("AUDIT_REPORTS_DIR") {
        let audit_system = AuditSystem::new(&reports_dir);
        let build_duration = build_start.elapsed();
        
        let build_audit = BuildChainAudit {
            total_entities_processed: 0,
            analysis_output_files: 0,
            models_generated: 0,
            build_time_ms: build_duration.as_millis() as u64,
            analysis_output_size_mb: 0,
            models_output_size_mb: 0,
            entities_per_second: 0,
            build_stage: "processors_build_empty".to_string(),
            success: false,
        };
        
        if let Err(e) = audit_system.generate_report(&[build_audit], "build_chain_performance") {
            eprintln!("Warning: Failed to generate empty build audit report: {}", e);
        }
    }
    
    Ok(())
}

/// Count files in a directory recursively
fn count_files_in_dir(dir: &PathBuf) -> Result<usize> {
    if !dir.exists() {
        return Ok(0);
    }
    
    let mut count = 0;
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            count += 1;
        }
    }
    Ok(count)
}

/// Get directory size in MB
fn get_directory_size_mb(dir: &PathBuf) -> Result<u64> {
    if !dir.exists() {
        return Ok(0);
    }
    
    let mut total_size = 0u64;
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            total_size += entry.metadata()?.len();
        }
    }
    Ok(total_size / (1024 * 1024)) // Convert to MB
}

/// Build chain audit data type
#[derive(Debug, Clone)]
struct BuildChainAudit {
    total_entities_processed: usize,
    analysis_output_files: usize,
    models_generated: usize,
    build_time_ms: u64,
    analysis_output_size_mb: u64,
    models_output_size_mb: u64,
    entities_per_second: u64,
    build_stage: String,
    success: bool,
}

impl dl_types::AuditableType for BuildChainAudit {
    fn audit_headers() -> Vec<String> {
        vec![
            "total_entities_processed".to_string(),
            "analysis_output_files".to_string(),
            "models_generated".to_string(),
            "build_time_ms".to_string(),
            "analysis_output_size_mb".to_string(),
            "models_output_size_mb".to_string(),
            "entities_per_second".to_string(),
            "build_stage".to_string(),
            "success".to_string(),
            "throughput_files_per_second".to_string(),
        ]
    }
    
    fn audit_row(&self) -> Vec<String> {
        let throughput = if self.build_time_ms > 0 {
            ((self.analysis_output_files + self.models_generated) as f64 / (self.build_time_ms as f64 / 1000.0))
        } else {
            0.0
        };
        
        vec![
            self.total_entities_processed.to_string(),
            self.analysis_output_files.to_string(),
            self.models_generated.to_string(),
            self.build_time_ms.to_string(),
            self.analysis_output_size_mb.to_string(),
            self.models_output_size_mb.to_string(),
            self.entities_per_second.to_string(),
            self.build_stage.clone(),
            self.success.to_string(),
            format!("{:.2}", throughput),
        ]
    }
    
    fn audit_category() -> String {
        "build_chain".to_string()
    }
    
    fn audit_subcategory() -> String {
        "processors".to_string()
    }
    
    fn extract_numeric_fields(&self) -> std::collections::HashMap<String, f64> {
        let mut fields = std::collections::HashMap::new();
        fields.insert("total_entities_processed".to_string(), self.total_entities_processed as f64);
        fields.insert("analysis_output_files".to_string(), self.analysis_output_files as f64);
        fields.insert("models_generated".to_string(), self.models_generated as f64);
        fields.insert("build_time_ms".to_string(), self.build_time_ms as f64);
        fields.insert("analysis_output_size_mb".to_string(), self.analysis_output_size_mb as f64);
        fields.insert("models_output_size_mb".to_string(), self.models_output_size_mb as f64);
        fields.insert("entities_per_second".to_string(), self.entities_per_second as f64);
        fields
    }
}
