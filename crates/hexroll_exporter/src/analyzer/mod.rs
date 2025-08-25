//! HBF Analysis Subpackage - Modular, production-ready HBF structure analysis
//!
//! This module provides comprehensive analysis of HexRoll HBF files with:
//! - Core database inspection and schema analysis
//! - AI-powered semantic relationship discovery
//! - Pattern-based implicit relationship detection
//! - Cross-validation and production readiness assessment

pub mod types;
pub mod core;
pub mod relationships;
pub mod ai_integration;
pub mod validation;
pub mod code_generation;
pub mod pattern_clustering;

// Re-export key types and functionality
pub use types::{
    AnalysisReport, TableInfo, ColumnInfo, ForeignKeyInfo, ImplicitRelationship,
    EmbeddedReference, AIInsights, DiscoveredRelationship, AIAnalysisResult,
    AnalysisConfig,
};

pub use core::CoreAnalyzer;
pub use relationships::RelationshipDiscovery;
pub use ai_integration::{AIAnalysisEngine, SemanticAnalyzer};
pub use validation::{
    ValidationEngine, ValidationSummary, CrossValidationResult, 
    ExtractionReadiness, ProductionValidator, ProductionReadinessReport,
};
pub use code_generation::{
    AICodeGenerator, GeneratedModel, GeneratedTemplate, HTMLPattern, TableRelationship,
};
pub use pattern_clustering::{
    PatternClusteringEngine, BatchProcessingEngine, HTMLPatternCluster, BatchProcessingStrategy,
};

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Main HBF analyzer orchestrating all analysis modules
pub struct HbfAnalyzer {
    hbf_path: PathBuf,
    config: AnalysisConfig,
}

impl HbfAnalyzer {
    /// Create new HBF analyzer with default configuration
    pub fn new<P: AsRef<Path>>(hbf_path: P) -> Result<Self> {
        let hbf_path = hbf_path.as_ref().to_path_buf();
        
        if !hbf_path.exists() {
            return Err(anyhow::anyhow!("HBF file not found: {}", hbf_path.display()));
        }
        
        Ok(Self { 
            hbf_path,
            config: AnalysisConfig::default(),
        })
    }
    
    /// Create HBF analyzer with custom configuration
    pub fn with_config<P: AsRef<Path>>(hbf_path: P, config: AnalysisConfig) -> Result<Self> {
        let hbf_path = hbf_path.as_ref().to_path_buf();
        
        if !hbf_path.exists() {
            return Err(anyhow::anyhow!("HBF file not found: {}", hbf_path.display()));
        }
        
        Ok(Self { hbf_path, config })
    }
    
    /// Perform standard pattern-based analysis
    pub fn analyze_structure(&self, depth: u8) -> Result<AnalysisReport> {
        println!("🔍 Starting HBF structure analysis (depth: {})...", depth);
        
        let conn = CoreAnalyzer::open_hbf_connection(&self.hbf_path)?;
        let mut report = AnalysisReport::new();
        
        // Level 1: Basic table structure
        CoreAnalyzer::analyze_basic_structure(&conn, &mut report)?;
        
        if depth >= 2 {
            // Level 2: Detailed content analysis
            CoreAnalyzer::analyze_detailed_schemas(&conn, &mut report)?;
            CoreAnalyzer::sample_table_content(&conn, &mut report, self.config.max_samples_per_table)?;
        }
        
        if depth >= 3 {
            // Level 3: Complete relationship mapping and HTML pattern analysis
            if self.config.html_pattern_detection {
                CoreAnalyzer::analyze_html_patterns_in_report(&mut report);
            }
            
            RelationshipDiscovery::discover_implicit_relationships(&conn, &mut report)?;
            RelationshipDiscovery::analyze_embedded_references_in_report(&mut report)?;
            CoreAnalyzer::analyze_foreign_keys(&conn, &mut report)?;
            RelationshipDiscovery::generate_relationship_recommendations(&mut report);
        }
        
        // Always add enhanced validation recommendations
        ValidationEngine::generate_enhanced_validation_recommendations(&mut report);
        
        Ok(report)
    }
    
    /// Perform AI-enhanced semantic analysis for maximum accuracy
    pub async fn analyze_structure_with_ai(&self, depth: u8) -> Result<AnalysisReport> {
        println!("🤖 Starting AI-Enhanced HBF Analysis...");
        
        // Start with traditional analysis
        let mut report = self.analyze_structure(depth)?;
        
        if !self.config.enable_ai {
            println!("   ⚠️  AI analysis disabled in configuration");
            return Ok(report);
        }
        
        // Load the hbf-analyzer agent specification
        let agent_spec_path = std::env::current_dir()?
            .join("crates/hexroll_exporter/agent.toml");
        
        if !agent_spec_path.exists() {
            println!("   ⚠️  HBF analyzer agent spec not found, falling back to pattern analysis");
            return Ok(report);
        }
        
        // Execute AI analysis
        match self.execute_ai_enhanced_analysis(&agent_spec_path, &mut report).await {
            Ok(_) => {
                println!("🤖 AI-Enhanced Analysis Complete!");
                
                // Perform cross-validation if enabled
                if self.config.cross_validation {
                    self.perform_cross_validation(&mut report)?;
                }
            }
            Err(e) => {
                println!("⚠️  AI analysis failed: {}, using enhanced pattern analysis", e);
                report.recommendations.push(format!("AI ERROR: {} - Used enhanced pattern analysis instead", e));
            }
        }
        
        Ok(report)
    }
    
    /// Execute comprehensive AI-enhanced analysis
    async fn execute_ai_enhanced_analysis(&self, agent_spec_path: &Path, 
                                         report: &mut AnalysisReport) -> Result<()> {
        // Create AI analysis engine
        let mut ai_engine = AIAnalysisEngine::new(agent_spec_path).await?;
        
        // Perform AI analysis on the HBF data
        ai_engine.analyze_hbf_with_ai(report).await?;
        
        // Perform specialized Entity-Ref semantic analysis
        SemanticAnalyzer::analyze_entity_ref_connections(&mut ai_engine, report).await?;
        
        Ok(())
    }
    
    /// Perform cross-validation between pattern and AI analysis
    fn perform_cross_validation(&self, report: &mut AnalysisReport) -> Result<()> {
        println!("🔍 Performing cross-validation analysis...");
        
        let cross_validation = ValidationEngine::cross_validate_analyses(report)?;
        
        // Add cross-validation results to recommendations
        if !cross_validation.reinforced_findings.is_empty() {
            report.recommendations.push(format!(
                "CROSS-VALIDATION: {} findings reinforced by both methods",
                cross_validation.reinforced_findings.len()
            ));
        }
        
        if !cross_validation.conflicting_findings.is_empty() {
            report.recommendations.push(format!(
                "CROSS-VALIDATION: {} conflicts detected - manual review required",
                cross_validation.conflicting_findings.len()
            ));
        }
        
        report.recommendations.push(format!(
            "CROSS-VALIDATION: {:.1}% agreement between pattern and AI analysis",
            cross_validation.pattern_ai_agreement * 100.0
        ));
        
        Ok(())
    }
    
    /// Assess production readiness for extraction
    pub fn assess_production_readiness(&self, report: &AnalysisReport) -> Result<ProductionReadinessReport> {
        ProductionValidator::validate_for_production(report, &self.config)
    }
    
    /// Get extraction readiness assessment
    pub fn get_extraction_readiness(&self, report: &AnalysisReport) -> ExtractionReadiness {
        ValidationEngine::assess_extraction_readiness(report, &self.config)
    }
    
    /// Complete analysis workflow with all validations
    pub async fn complete_analysis_workflow(&self, depth: u8) -> Result<CompleteAnalysisResult> {
        println!("🚀 Starting complete HBF analysis workflow...");
        
        // Perform analysis (with or without AI based on config)
        let report = if self.config.enable_ai {
            self.analyze_structure_with_ai(depth).await?
        } else {
            self.analyze_structure(depth)?
        };
        
        // Assess production readiness
        let production_readiness = self.assess_production_readiness(&report)?;
        let extraction_readiness = self.get_extraction_readiness(&report);
        
        // Print comprehensive summary
        report.print_summary();
        production_readiness.print_summary();
        extraction_readiness.print_summary();
        
        Ok(CompleteAnalysisResult {
            analysis_report: report,
            production_readiness,
            extraction_readiness,
        })
    }
}

/// Complete analysis result with all assessments
#[derive(Debug)]
pub struct CompleteAnalysisResult {
    pub analysis_report: AnalysisReport,
    pub production_readiness: ProductionReadinessReport,
    pub extraction_readiness: ExtractionReadiness,
}

impl CompleteAnalysisResult {
    /// Save complete analysis results to files
    pub fn save_results<P: AsRef<Path>>(&self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;
        
        // Save main analysis report
        self.analysis_report.save_report(output_dir.join("hbf_analysis_report.json"))?;
        
        // Save production readiness as JSON
        let prod_json = serde_json::to_string_pretty(&serde_json::json!({
            "total_entities": self.production_readiness.total_entities,
            "total_refs": self.production_readiness.total_refs,
            "data_completeness_score": self.production_readiness.data_completeness_score,
            "relationship_quality_score": self.production_readiness.relationship_quality_score,
            "ai_confidence_score": self.production_readiness.ai_confidence_score,
            "ai_accuracy_prediction": self.production_readiness.ai_accuracy_prediction,
            "overall_score": self.production_readiness.overall_score,
            "production_ready": self.production_readiness.production_ready,
            "critical_issues": self.production_readiness.critical_issues,
            "recommendation": self.production_readiness.recommendation
        }))?;
        std::fs::write(output_dir.join("production_readiness.json"), prod_json)?;
        
        // Save extraction readiness as JSON
        let readiness_json = serde_json::to_string_pretty(&serde_json::json!({
            "ready_for_extraction": self.extraction_readiness.ready_for_extraction,
            "confidence_level": self.extraction_readiness.confidence_level,
            "critical_issues": self.extraction_readiness.critical_issues,
            "recommendations": self.extraction_readiness.recommendations,
            "estimated_accuracy": self.extraction_readiness.estimated_accuracy
        }))?;
        std::fs::write(output_dir.join("extraction_readiness.json"), readiness_json)?;
        
        println!("💾 Complete analysis results saved to: {}", output_dir.display());
        
        Ok(())
    }
}
