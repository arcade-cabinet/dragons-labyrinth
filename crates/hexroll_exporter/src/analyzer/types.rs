//! Data types and structures for HBF analysis

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

/// Comprehensive analysis report of HBF structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub table_count: usize,
    pub total_records: usize,
    pub table_info: HashMap<String, TableInfo>,
    pub relationships: Vec<ForeignKeyInfo>,
    pub implicit_relationships: Vec<ImplicitRelationship>,
    pub embedded_references: Vec<EmbeddedReference>,
    pub html_patterns_found: Vec<String>,
    pub recommendations: Vec<String>,
    pub ai_insights: Option<AIInsights>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        Self {
            table_count: 0,
            total_records: 0,
            table_info: HashMap::new(),
            relationships: Vec::new(),
            implicit_relationships: Vec::new(),
            embedded_references: Vec::new(),
            html_patterns_found: Vec::new(),
            recommendations: Vec::new(),
            ai_insights: None,
        }
    }
    
    pub fn save_report<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }
    
    pub fn print_summary(&self) {
        println!("\n📋 HBF Analysis Summary:");
        println!("   📊 {} tables with {} total records", self.table_count, self.total_records);
        
        for (table_name, info) in &self.table_info {
            println!("   📋 {}: {} records, {} columns", 
                     table_name, info.record_count, info.columns.len());
            
            if !info.html_patterns.is_empty() {
                println!("      🔍 HTML patterns: {:?}", info.html_patterns);
            }
        }
        
        if !self.relationships.is_empty() {
            println!("   🔗 Foreign Key Relationships:");
            for rel in &self.relationships {
                println!("      {}.{} → {}.{}", 
                         rel.from_table, rel.from_column, rel.to_table, rel.to_column);
            }
        }
        
        if !self.recommendations.is_empty() {
            println!("   💡 Recommendations:");
            for rec in &self.recommendations {
                println!("      • {}", rec);
            }
        }
        
        if let Some(ai_insights) = &self.ai_insights {
            println!("   🤖 AI Analysis Results:");
            println!("      Relationship Coverage: {:.1}%", ai_insights.relationship_coverage * 100.0);
            println!("      Overall Confidence: {:.1}%", ai_insights.overall_confidence * 100.0);
            if !ai_insights.validation_warnings.is_empty() {
                println!("      Validation Warnings: {}", ai_insights.validation_warnings.len());
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub record_count: usize,
    pub columns: Vec<ColumnInfo>,
    pub sample_data: Option<serde_json::Value>,
    pub html_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyInfo {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplicitRelationship {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
    pub match_count: usize,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedReference {
    pub table_name: String,
    pub column_name: String,
    pub reference_type: String,
    pub references: Vec<String>,
}

/// AI analysis results and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInsights {
    pub relationship_coverage: f64,
    pub overall_confidence: f64,
    pub discovered_relationships: Vec<DiscoveredRelationship>,
    pub validation_warnings: Vec<String>,
    pub extraction_suggestions: Vec<String>,
    pub accuracy_prediction: f64,
}

/// AI-discovered relationship with confidence and reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredRelationship {
    pub source_entity: String,
    pub target_ref: String,
    pub relationship_type: String,
    pub confidence: f64,
    pub reasoning: String,
}

/// Result of AI analysis execution
#[derive(Debug, Clone)]
pub struct AIAnalysisResult {
    pub discovered_relationships: Vec<DiscoveredRelationship>,
    pub confidence_score: f64,
    pub validation_concerns: Vec<String>,
    pub extraction_suggestions: Vec<String>,
}

/// Analysis configuration parameters
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub enable_ai: bool,
    pub confidence_threshold: f64,
    pub max_samples_per_table: usize,
    pub html_pattern_detection: bool,
    pub cross_validation: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            enable_ai: true,
            confidence_threshold: 0.6,
            max_samples_per_table: 10,
            html_pattern_detection: true,
            cross_validation: true,
        }
    }
}
