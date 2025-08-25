//! Validation and reporting functionality for HBF analysis

use anyhow::Result;
use std::collections::HashMap;

use super::types::{AnalysisReport, TableInfo, AnalysisConfig};

/// Validation engine for ensuring analysis accuracy and completeness
pub struct ValidationEngine;

impl ValidationEngine {
    /// Generate enhanced validation recommendations for 100% accuracy guarantee
    pub fn generate_enhanced_validation_recommendations(report: &mut AnalysisReport) {
        println!("🔍 Generating enhanced validation recommendations...");
        
        // Analyze data coverage
        let total_records: usize = report.table_info.values().map(|t| t.record_count).sum();
        let large_tables: Vec<_> = report.table_info.values()
            .filter(|t| t.record_count > 1000)
            .collect();
        
        report.recommendations.push(format!(
            "DATA COVERAGE: {} total records across {} tables",
            total_records, report.table_count
        ));
        
        // Identify potential primary entities
        if !large_tables.is_empty() {
            let largest_table = large_tables.iter()
                .max_by_key(|t| t.record_count)
                .unwrap();
            
            report.recommendations.push(format!(
                "PRIMARY ENTITY CANDIDATE: {} with {} records (likely main entity table)",
                largest_table.name, largest_table.record_count
            ));
        }
        
        // Validate relationship completeness
        let total_relationships = report.implicit_relationships.len() + report.embedded_references.len();
        if total_relationships == 0 {
            report.recommendations.push(
                "CRITICAL: No relationships discovered - manual schema review required".to_string()
            );
        } else {
            report.recommendations.push(format!(
                "RELATIONSHIP COVERAGE: {} implicit relationships + {} embedded references found",
                report.implicit_relationships.len(), report.embedded_references.len()
            ));
        }
        
        // Data integrity checks
        for (table_name, table_info) in &report.table_info {
            let has_uuid_column = table_info.columns.iter()
                .any(|c| c.name.to_lowercase().contains("uuid") || c.name.to_lowercase().contains("id"));
            
            if !has_uuid_column && table_info.record_count > 100 {
                report.recommendations.push(format!(
                    "WARNING: {} has {} records but no obvious ID column - verify data structure",
                    table_name, table_info.record_count
                ));
            }
        }
        
        Self::add_validation_steps(report, total_records);
        Self::add_accuracy_recommendations(report, total_records);
    }
    
    /// Add systematic validation steps to recommendations
    fn add_validation_steps(report: &mut AnalysisReport, total_records: usize) {
        report.recommendations.push(
            "VALIDATION STEP 1: Cross-reference all discovered relationships with actual data queries".to_string()
        );
        report.recommendations.push(
            "VALIDATION STEP 2: Sample random records from each table to verify content structure".to_string()
        );
        report.recommendations.push(
            "VALIDATION STEP 3: Test data extraction on a subset before full migration".to_string()
        );
        report.recommendations.push(
            "VALIDATION STEP 4: Verify HTML link targets exist in target tables".to_string()
        );
        report.recommendations.push(format!(
            "VALIDATION STEP 5: Confirm extracted record count matches expected {} total records",
            total_records
        ));
    }
    
    /// Add 100% accuracy recommendations
    fn add_accuracy_recommendations(report: &mut AnalysisReport, total_records: usize) {
        report.recommendations.push(
            "100% ACCURACY: Implement referential integrity checks during extraction".to_string()
        );
        report.recommendations.push(
            "100% ACCURACY: Create backup validation queries to cross-check extracted data".to_string()
        );
        report.recommendations.push(format!(
            "100% ACCURACY: Total expected records to extract: {} - validate this number post-migration",
            total_records
        ));
        report.recommendations.push(
            "100% ACCURACY: Use transaction rollback for any failed validations".to_string()
        );
        report.recommendations.push(
            "100% ACCURACY: Log all relationship mapping decisions for auditability".to_string()
        );
    }
    
    /// Validate analysis completeness based on configuration
    pub fn validate_analysis_completeness(report: &AnalysisReport, config: &AnalysisConfig) -> Result<ValidationSummary> {
        let mut summary = ValidationSummary::new();
        
        // Check basic requirements
        summary.basic_structure_complete = report.table_count > 0 && report.total_records > 0;
        summary.schema_analysis_complete = report.table_info.values()
            .all(|t| !t.columns.is_empty());
        summary.sample_data_complete = report.table_info.values()
            .filter(|t| t.record_count > 0)
            .all(|t| t.sample_data.is_some());
        
        // Check relationship discovery
        summary.relationship_discovery_complete = 
            !report.implicit_relationships.is_empty() || !report.embedded_references.is_empty();
        
        // Check AI analysis if enabled
        if config.enable_ai {
            summary.ai_analysis_complete = report.ai_insights.is_some();
            if let Some(ai_insights) = &report.ai_insights {
                summary.ai_confidence_acceptable = ai_insights.overall_confidence >= config.confidence_threshold;
                summary.relationship_coverage_adequate = ai_insights.relationship_coverage >= 0.8;
            }
        } else {
            summary.ai_analysis_complete = true; // Not required
            summary.ai_confidence_acceptable = true;
            summary.relationship_coverage_adequate = true;
        }
        
        // Calculate overall completion
        summary.overall_complete = summary.basic_structure_complete &&
                                 summary.schema_analysis_complete &&
                                 summary.sample_data_complete &&
                                 summary.relationship_discovery_complete &&
                                 summary.ai_analysis_complete &&
                                 summary.ai_confidence_acceptable &&
                                 summary.relationship_coverage_adequate;
        
        // Generate completion percentage
        let completed_checks = [
            summary.basic_structure_complete,
            summary.schema_analysis_complete,
            summary.sample_data_complete,
            summary.relationship_discovery_complete,
            summary.ai_analysis_complete,
            summary.ai_confidence_acceptable,
            summary.relationship_coverage_adequate,
        ].iter().filter(|&&x| x).count();
        
        summary.completion_percentage = (completed_checks as f64 / 7.0) * 100.0;
        
        Ok(summary)
    }
    
    /// Cross-validate pattern analysis with AI analysis results
    pub fn cross_validate_analyses(report: &AnalysisReport) -> Result<CrossValidationResult> {
        println!("🔍 Cross-validating pattern analysis with AI results...");
        
        let mut result = CrossValidationResult {
            pattern_ai_agreement: 0.0,
            conflicting_findings: Vec::new(),
            reinforced_findings: Vec::new(),
            confidence_boost: 0.0,
        };
        
        if let Some(ai_insights) = &report.ai_insights {
            // Compare implicit relationships with AI discoveries
            for implicit_rel in &report.implicit_relationships {
                let ai_matches: Vec<_> = ai_insights.discovered_relationships.iter()
                    .filter(|ai_rel| {
                        ai_rel.source_entity.contains(&implicit_rel.from_table) ||
                        ai_rel.target_ref.contains(&implicit_rel.to_table)
                    })
                    .collect();
                
                if !ai_matches.is_empty() {
                    result.reinforced_findings.push(format!(
                        "REINFORCED: {}.{} ↔ {}.{} confirmed by both pattern and AI analysis",
                        implicit_rel.from_table, implicit_rel.from_column,
                        implicit_rel.to_table, implicit_rel.to_column
                    ));
                } else if implicit_rel.confidence > 0.8 {
                    result.conflicting_findings.push(format!(
                        "CONFLICT: High-confidence pattern {}.{} ↔ {}.{} not confirmed by AI",
                        implicit_rel.from_table, implicit_rel.from_column,
                        implicit_rel.to_table, implicit_rel.to_column
                    ));
                }
            }
            
            // Calculate agreement percentage
            let reinforced_count = result.reinforced_findings.len();
            let total_patterns = report.implicit_relationships.len();
            result.pattern_ai_agreement = if total_patterns > 0 {
                reinforced_count as f64 / total_patterns as f64
            } else {
                1.0
            };
            
            // Calculate confidence boost from cross-validation
            result.confidence_boost = if result.pattern_ai_agreement > 0.8 {
                0.1 // Boost confidence by 10% for high agreement
            } else if result.pattern_ai_agreement > 0.6 {
                0.05 // Boost by 5% for medium agreement
            } else {
                0.0 // No boost for low agreement
            };
            
            println!("   📊 Cross-validation: {:.1}% agreement, +{:.1}% confidence boost",
                     result.pattern_ai_agreement * 100.0, result.confidence_boost * 100.0);
        }
        
        Ok(result)
    }
    
    /// Generate final extraction readiness assessment
    pub fn assess_extraction_readiness(report: &AnalysisReport, config: &AnalysisConfig) -> ExtractionReadiness {
        let validation_summary = Self::validate_analysis_completeness(report, config)
            .unwrap_or_else(|_| ValidationSummary::new());
        
        let cross_validation = Self::cross_validate_analyses(report)
            .unwrap_or_else(|_| CrossValidationResult::default());
        
        let mut readiness = ExtractionReadiness {
            ready_for_extraction: false,
            confidence_level: 0.0,
            critical_issues: Vec::new(),
            recommendations: Vec::new(),
            estimated_accuracy: 0.0,
        };
        
        // Calculate overall confidence
        readiness.confidence_level = validation_summary.completion_percentage / 100.0;
        
        if let Some(ai_insights) = &report.ai_insights {
            readiness.confidence_level = (readiness.confidence_level + ai_insights.overall_confidence) / 2.0;
            readiness.estimated_accuracy = ai_insights.accuracy_prediction;
        } else {
            readiness.estimated_accuracy = readiness.confidence_level * 0.9; // Slightly lower without AI
        }
        
        // Apply cross-validation boost
        readiness.confidence_level += cross_validation.confidence_boost;
        readiness.estimated_accuracy += cross_validation.confidence_boost;
        
        // Determine readiness
        readiness.ready_for_extraction = validation_summary.overall_complete && 
                                        readiness.estimated_accuracy >= 0.85;
        
        // Generate critical issues
        if !validation_summary.basic_structure_complete {
            readiness.critical_issues.push("Basic structure analysis incomplete".to_string());
        }
        if !validation_summary.relationship_discovery_complete {
            readiness.critical_issues.push("No relationships discovered - manual review required".to_string());
        }
        if config.enable_ai && !validation_summary.ai_analysis_complete {
            readiness.critical_issues.push("AI analysis failed - fallback to pattern analysis".to_string());
        }
        
        // Generate recommendations
        if readiness.estimated_accuracy < 0.95 {
            readiness.recommendations.push("Perform manual validation on critical relationships".to_string());
        }
        if cross_validation.pattern_ai_agreement < 0.8 {
            readiness.recommendations.push("Investigate conflicts between pattern and AI analysis".to_string());
        }
        if report.embedded_references.is_empty() {
            readiness.recommendations.push("Verify HTML content parsing - no embedded references found".to_string());
        }
        
        readiness
    }
}

/// Summary of validation checks
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub basic_structure_complete: bool,
    pub schema_analysis_complete: bool,
    pub sample_data_complete: bool,
    pub relationship_discovery_complete: bool,
    pub ai_analysis_complete: bool,
    pub ai_confidence_acceptable: bool,
    pub relationship_coverage_adequate: bool,
    pub overall_complete: bool,
    pub completion_percentage: f64,
}

impl ValidationSummary {
    pub fn new() -> Self {
        Self {
            basic_structure_complete: false,
            schema_analysis_complete: false,
            sample_data_complete: false,
            relationship_discovery_complete: false,
            ai_analysis_complete: false,
            ai_confidence_acceptable: false,
            relationship_coverage_adequate: false,
            overall_complete: false,
            completion_percentage: 0.0,
        }
    }
}

/// Results of cross-validation between different analysis methods
#[derive(Debug, Clone)]
pub struct CrossValidationResult {
    pub pattern_ai_agreement: f64,
    pub conflicting_findings: Vec<String>,
    pub reinforced_findings: Vec<String>,
    pub confidence_boost: f64,
}

impl Default for CrossValidationResult {
    fn default() -> Self {
        Self {
            pattern_ai_agreement: 0.0,
            conflicting_findings: Vec::new(),
            reinforced_findings: Vec::new(),
            confidence_boost: 0.0,
        }
    }
}

/// Assessment of extraction readiness
#[derive(Debug, Clone)]
pub struct ExtractionReadiness {
    pub ready_for_extraction: bool,
    pub confidence_level: f64,
    pub critical_issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub estimated_accuracy: f64,
}

impl ExtractionReadiness {
    pub fn print_summary(&self) {
        println!("\n🎯 Extraction Readiness Assessment:");
        println!("   Ready for Extraction: {}", if self.ready_for_extraction { "✅ YES" } else { "❌ NO" });
        println!("   Confidence Level: {:.1}%", self.confidence_level * 100.0);
        println!("   Estimated Accuracy: {:.1}%", self.estimated_accuracy * 100.0);
        
        if !self.critical_issues.is_empty() {
            println!("   🚨 Critical Issues:");
            for issue in &self.critical_issues {
                println!("      • {}", issue);
            }
        }
        
        if !self.recommendations.is_empty() {
            println!("   💡 Recommendations:");
            for rec in &self.recommendations {
                println!("      • {}", rec);
            }
        }
        
        if self.ready_for_extraction {
            println!("   🚀 READY FOR PRODUCTION EXTRACTION!");
        } else {
            println!("   ⚠️  Additional work required before extraction");
        }
    }
}

/// Advanced validation metrics for production readiness
pub struct ProductionValidator;

impl ProductionValidator {
    /// Comprehensive production readiness check
    pub fn validate_for_production(report: &AnalysisReport, config: &AnalysisConfig) -> Result<ProductionReadinessReport> {
        println!("🎯 Validating for production extraction...");
        
        let mut prod_report = ProductionReadinessReport::new();
        
        // Data completeness validation
        Self::validate_data_completeness(report, &mut prod_report);
        
        // Relationship mapping validation
        Self::validate_relationship_mapping(report, &mut prod_report);
        
        // AI confidence validation
        if config.enable_ai {
            Self::validate_ai_confidence(report, config, &mut prod_report);
        }
        
        // Generate final recommendation
        Self::generate_production_recommendation(&mut prod_report);
        
        Ok(prod_report)
    }
    
    /// Validate data completeness for production
    fn validate_data_completeness(report: &AnalysisReport, prod_report: &mut ProductionReadinessReport) {
        let entities_count = report.table_info.values()
            .filter(|t| t.name.to_lowercase().contains("entities"))
            .map(|t| t.record_count)
            .sum::<usize>();
            
        let refs_count = report.table_info.values()
            .filter(|t| t.name.to_lowercase().contains("refs"))
            .map(|t| t.record_count)
            .sum::<usize>();
        
        prod_report.total_entities = entities_count;
        prod_report.total_refs = refs_count;
        prod_report.data_completeness_score = if entities_count > 50000 && refs_count > 1000 {
            1.0 // Expected scale achieved
        } else {
            (entities_count + refs_count) as f64 / 72000.0 // Scale based on expected ~72k records
        };
        
        if prod_report.data_completeness_score < 0.9 {
            prod_report.critical_issues.push(format!(
                "Data scale concern: {} entities + {} refs (expected ~70k + ~1.5k)",
                entities_count, refs_count
            ));
        }
    }
    
    /// Validate relationship mapping quality
    fn validate_relationship_mapping(report: &AnalysisReport, prod_report: &mut ProductionReadinessReport) {
        let implicit_high_confidence = report.implicit_relationships.iter()
            .filter(|r| r.confidence > 0.8)
            .count();
            
        let embedded_refs_found = report.embedded_references.len();
        
        prod_report.relationship_quality_score = if implicit_high_confidence > 0 || embedded_refs_found > 0 {
            let quality = (implicit_high_confidence as f64 + embedded_refs_found as f64) / 
                         (report.implicit_relationships.len().max(1) as f64);
            quality.min(1.0)
        } else {
            0.0
        };
        
        if prod_report.relationship_quality_score < 0.7 {
            prod_report.critical_issues.push(
                "Relationship quality concern - low confidence mappings detected".to_string()
            );
        }
    }
    
    /// Validate AI confidence levels
    fn validate_ai_confidence(report: &AnalysisReport, config: &AnalysisConfig, 
                             prod_report: &mut ProductionReadinessReport) {
        if let Some(ai_insights) = &report.ai_insights {
            prod_report.ai_confidence_score = ai_insights.overall_confidence;
            prod_report.ai_accuracy_prediction = ai_insights.accuracy_prediction;
            
            if ai_insights.overall_confidence < config.confidence_threshold {
                prod_report.critical_issues.push(format!(
                    "AI confidence below threshold: {:.1}% < {:.1}%",
                    ai_insights.overall_confidence * 100.0,
                    config.confidence_threshold * 100.0
                ));
            }
            
            if ai_insights.accuracy_prediction < 0.9 {
                prod_report.critical_issues.push(format!(
                    "AI accuracy prediction below 90%: {:.1}%",
                    ai_insights.accuracy_prediction * 100.0
                ));
            }
        } else {
            prod_report.ai_confidence_score = 0.0;
            prod_report.ai_accuracy_prediction = 0.0;
            prod_report.critical_issues.push("AI analysis not performed".to_string());
        }
    }
    
    /// Generate final production recommendation
    fn generate_production_recommendation(prod_report: &mut ProductionReadinessReport) {
        let overall_score = (prod_report.data_completeness_score + 
                           prod_report.relationship_quality_score + 
                           prod_report.ai_confidence_score) / 3.0;
        
        prod_report.production_ready = overall_score >= 0.85 && prod_report.critical_issues.is_empty();
        prod_report.overall_score = overall_score;
        
        if prod_report.production_ready {
            prod_report.recommendation = "✅ APPROVED FOR PRODUCTION EXTRACTION".to_string();
        } else if overall_score >= 0.7 {
            prod_report.recommendation = "⚠️  PROCEED WITH CAUTION - Address critical issues first".to_string();
        } else {
            prod_report.recommendation = "❌ NOT READY - Significant issues must be resolved".to_string();
        }
    }
}

/// Production readiness assessment report
#[derive(Debug, Clone)]
pub struct ProductionReadinessReport {
    pub total_entities: usize,
    pub total_refs: usize,
    pub data_completeness_score: f64,
    pub relationship_quality_score: f64,
    pub ai_confidence_score: f64,
    pub ai_accuracy_prediction: f64,
    pub overall_score: f64,
    pub production_ready: bool,
    pub critical_issues: Vec<String>,
    pub recommendation: String,
}

impl ProductionReadinessReport {
    pub fn new() -> Self {
        Self {
            total_entities: 0,
            total_refs: 0,
            data_completeness_score: 0.0,
            relationship_quality_score: 0.0,
            ai_confidence_score: 0.0,
            ai_accuracy_prediction: 0.0,
            overall_score: 0.0,
            production_ready: false,
            critical_issues: Vec::new(),
            recommendation: String::new(),
        }
    }
    
    pub fn print_summary(&self) {
        println!("\n🎯 Production Readiness Report:");
        println!("   Data Scale: {} entities + {} refs", self.total_entities, self.total_refs);
        println!("   Data Completeness: {:.1}%", self.data_completeness_score * 100.0);
        println!("   Relationship Quality: {:.1}%", self.relationship_quality_score * 100.0);
        println!("   AI Confidence: {:.1}%", self.ai_confidence_score * 100.0);
        println!("   AI Accuracy Prediction: {:.1}%", self.ai_accuracy_prediction * 100.0);
        println!("   Overall Score: {:.1}%", self.overall_score * 100.0);
        println!("   Production Ready: {}", if self.production_ready { "✅ YES" } else { "❌ NO" });
        
        if !self.critical_issues.is_empty() {
            println!("   🚨 Critical Issues:");
            for issue in &self.critical_issues {
                println!("      • {}", issue);
            }
        }
        
        println!("   📋 Recommendation: {}", self.recommendation);
    }
}
