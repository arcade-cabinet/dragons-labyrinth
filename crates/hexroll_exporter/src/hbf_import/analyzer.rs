//! HBF Structure Analyzer - Deep analysis of HBF format and content

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// HBF file analyzer for understanding structure and content
pub struct HbfAnalyzer {
    hbf_path: PathBuf,
}

impl HbfAnalyzer {
    pub fn new<P: AsRef<Path>>(hbf_path: P) -> Result<Self> {
        let hbf_path = hbf_path.as_ref().to_path_buf();
        
        if !hbf_path.exists() {
            return Err(anyhow::anyhow!("HBF file not found: {}", hbf_path.display()));
        }
        
        Ok(Self { hbf_path })
    }
    
    /// Analyze HBF structure at different depth levels
    pub fn analyze_structure(&self, depth: u8) -> Result<AnalysisReport> {
        let conn = Connection::open_with_flags(&self.hbf_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        
        let mut report = AnalysisReport::new();
        
        // Level 1: Basic table structure
        self.analyze_basic_structure(&conn, &mut report)?;
        
        if depth >= 2 {
            // Level 2: Detailed content analysis
            self.analyze_detailed_content(&conn, &mut report)?;
        }
        
        if depth >= 3 {
            // Level 3: Complete relationship mapping and HTML pattern analysis
            self.analyze_complete_structure(&conn, &mut report)?;
        }
        
        Ok(report)
    }
    
    /// Analyze basic table structure and counts
    fn analyze_basic_structure(&self, conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        // Get all table names
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_names: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?.collect::<Result<Vec<_>, _>>()?;
        
        report.table_count = table_names.len();
        
        // Get record counts for each table
        for table_name in table_names {
            let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
            let count: usize = conn.query_row(&count_query, [], |row| {
                Ok(row.get::<_, i64>(0)? as usize)
            })?;
            
            report.table_info.insert(table_name.clone(), TableInfo {
                name: table_name,
                record_count: count,
                columns: Vec::new(),
                sample_data: None,
                html_patterns: Vec::new(),
            });
            
            report.total_records += count;
        }
        
        println!("📊 Basic Analysis:");
        println!("   Tables: {}", report.table_count);
        println!("   Total Records: {}", report.total_records);
        
        Ok(())
    }
    
    /// Analyze detailed content including column schemas and data types
    fn analyze_detailed_content(&self, conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("🔍 Analyzing detailed content...");
        
        for table_info in report.table_info.values_mut() {
            // Get column information using PRAGMA
            let pragma_query = format!("PRAGMA table_info({})", table_info.name);
            let mut stmt = conn.prepare(&pragma_query)?;
            
            let columns: Vec<ColumnInfo> = stmt.query_map([], |row| {
                Ok(ColumnInfo {
                    name: row.get::<_, String>(1)?,
                    data_type: row.get::<_, String>(2)?,
                    not_null: row.get::<_, bool>(3)?,
                    default_value: row.get::<_, Option<String>>(4)?,
                    primary_key: row.get::<_, bool>(5)?,
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            
            table_info.columns = columns;
            
            println!("   Table '{}': {} columns, {} records", 
                     table_info.name, table_info.columns.len(), table_info.record_count);
        }
        
        Ok(())
    }
    
    /// Complete analysis including HTML pattern detection and relationship mapping
    fn analyze_complete_structure(&self, conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("🔬 Performing complete structural analysis...");
        
        // Look for HTML content patterns in text columns
        for table_info in report.table_info.values_mut() {
            for column in &table_info.columns {
                if column.data_type.to_uppercase().contains("TEXT") {
                    // Sample some text content to look for HTML patterns
                    let sample_query = format!(
                        "SELECT {} FROM {} WHERE {} IS NOT NULL LIMIT 10", 
                        column.name, table_info.name, column.name
                    );
                    
                    let mut stmt = conn.prepare(&sample_query)?;
                    let text_samples: Vec<String> = stmt.query_map([], |row| {
                        Ok(row.get::<_, String>(0)?)
                    })?.collect::<Result<Vec<_>, _>>()?;
                    
                    // Analyze for HTML patterns
                    let html_patterns = self.detect_html_patterns(&text_samples);
                    if !html_patterns.is_empty() {
                        println!("   🔍 HTML patterns in {}.{}: {:?}", 
                                 table_info.name, column.name, html_patterns);
                        table_info.html_patterns.extend(html_patterns);
                    }
                }
            }
        }
        
        // Look for foreign key relationships
        self.analyze_relationships(conn, report)?;
        
        Ok(())
    }
    
    /// Detect HTML patterns in text content
    fn detect_html_patterns(&self, text_samples: &[String]) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for sample in text_samples {
            if sample.contains("<") && sample.contains(">") {
                // Look for common HTML patterns
                if sample.contains("<table") {
                    patterns.push("html_table".to_string());
                }
                if sample.contains("<div") {
                    patterns.push("html_div".to_string());
                }
                if sample.contains("<h1") || sample.contains("<h2") || sample.contains("<h3") {
                    patterns.push("html_headers".to_string());
                }
                if sample.contains("<ul") || sample.contains("<ol") {
                    patterns.push("html_lists".to_string());
                }
                if sample.contains("class=") {
                    patterns.push("html_styled".to_string());
                }
                if sample.contains("<p>") || sample.contains("<p ") {
                    patterns.push("html_paragraphs".to_string());
                }
            }
        }
        
        patterns.sort();
        patterns.dedup();
        patterns
    }
    
    /// Analyze table relationships and foreign keys
    fn analyze_relationships(&self, conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        for table_name in report.table_info.keys().cloned().collect::<Vec<_>>() {
            let pragma_query = format!("PRAGMA foreign_key_list({})", table_name);
            let mut stmt = conn.prepare(&pragma_query)?;
            
            let foreign_keys: Vec<ForeignKeyInfo> = stmt.query_map([], |row| {
                Ok(ForeignKeyInfo {
                    from_table: table_name.clone(),
                    from_column: row.get::<_, String>(3)?,
                    to_table: row.get::<_, String>(2)?,
                    to_column: row.get::<_, String>(4)?,
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            
            report.relationships.extend(foreign_keys);
        }
        
        println!("🔗 Found {} foreign key relationships", report.relationships.len());
        
        Ok(())
    }
}

/// Comprehensive analysis report of HBF structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub table_count: usize,
    pub total_records: usize,
    pub table_info: HashMap<String, TableInfo>,
    pub relationships: Vec<ForeignKeyInfo>,
    pub html_patterns_found: Vec<String>,
    pub recommendations: Vec<String>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        Self {
            table_count: 0,
            total_records: 0,
            table_info: HashMap::new(),
            relationships: Vec::new(),
            html_patterns_found: Vec::new(),
            recommendations: Vec::new(),
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
