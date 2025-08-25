//! Core HBF analysis functionality - database inspection and basic structure analysis

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::Path;

use super::types::{AnalysisReport, TableInfo, ColumnInfo, ForeignKeyInfo};

/// Core database analysis functionality
pub struct CoreAnalyzer;

impl CoreAnalyzer {
    /// Open HBF database connection for read-only analysis
    pub fn open_hbf_connection<P: AsRef<Path>>(hbf_path: P) -> Result<Connection> {
        let conn = Connection::open_with_flags(hbf_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(conn)
    }
    
    /// Analyze basic table structure and record counts
    pub fn analyze_basic_structure(conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("📊 Analyzing basic HBF structure...");
        
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
        
        println!("   Tables: {}", report.table_count);
        println!("   Total Records: {}", report.total_records);
        
        Ok(())
    }
    
    /// Analyze detailed table schemas and column information
    pub fn analyze_detailed_schemas(conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("🔍 Analyzing detailed table schemas...");
        
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
    
    /// Comprehensive sampling across ALL entity types to understand full HexRoll complexity
    pub fn sample_table_content(conn: &Connection, report: &mut AnalysisReport, 
                               max_samples: usize) -> Result<()> {
        println!("📋 Comprehensive sampling across ALL entity types ({} samples per table)...", max_samples);
        
        for table_info in report.table_info.values_mut() {
            // Sample across the entire dataset to understand all content types
            let comprehensive_samples = Self::comprehensive_entity_sampling(conn, &table_info.name, &table_info.columns, max_samples)?;
            
            if !comprehensive_samples.is_empty() {
                table_info.sample_data = Some(serde_json::to_value(&comprehensive_samples)?);
                println!("   📋 Comprehensive sample from {}: {} records covering all entity types", 
                        table_info.name, comprehensive_samples.len());
                
                // Analyze content diversity
                let content_analysis = Self::analyze_content_diversity(&comprehensive_samples, &table_info.columns);
                println!("      🔍 Content diversity: {}", content_analysis);
            }
        }
        
        Ok(())
    }
    
    /// Comprehensive entity sampling to understand all HexRoll content types
    fn comprehensive_entity_sampling(
        conn: &Connection, 
        table_name: &str, 
        columns: &[ColumnInfo], 
        max_samples: usize
    ) -> Result<Vec<HashMap<String, Option<String>>>> {
        let mut all_samples = Vec::new();
        
        // Find text columns for content analysis
        let text_columns: Vec<_> = columns.iter()
            .filter(|col| col.data_type.to_uppercase().contains("TEXT"))
            .collect();
        
        if text_columns.is_empty() {
            // Regular sampling for non-text tables
            let query = format!("SELECT * FROM {} ORDER BY RANDOM() LIMIT {}", table_name, max_samples);
            let mut stmt = conn.prepare(&query)?;
            let rows = stmt.query_map([], |row| {
                let mut row_data = HashMap::new();
                for (i, column) in columns.iter().enumerate() {
                    let value: Option<String> = row.get(i).unwrap_or(None);
                    row_data.insert(column.name.clone(), value);
                }
                Ok(row_data)
            })?;
            
            for row in rows {
                all_samples.push(row?);
            }
        } else {
            // Stratified sampling: get diverse content types
            // 1. HTML-rich content (villages, cities, towns, dungeons)
            let html_query = format!(
                "SELECT * FROM {} WHERE {} LIKE '%<%' ORDER BY LENGTH({}) DESC LIMIT {}",
                table_name, text_columns[0].name, text_columns[0].name, max_samples / 3
            );
            
            // 2. Medium-length descriptive content (NPCs, factions, cults)
            let medium_query = format!(
                "SELECT * FROM {} WHERE LENGTH({}) BETWEEN 50 AND 200 AND {} NOT LIKE '%<%' ORDER BY RANDOM() LIMIT {}",
                table_name, text_columns[0].name, text_columns[0].name, max_samples / 3
            );
            
            // 3. Short/structured content (items, locations, references)
            let short_query = format!(
                "SELECT * FROM {} WHERE LENGTH({}) < 50 ORDER BY RANDOM() LIMIT {}",
                table_name, text_columns[0].name, max_samples / 3
            );
            
            // Execute all three sampling strategies
            for (query_type, query) in [("HTML-rich", html_query), ("Medium", medium_query), ("Short", short_query)] {
                let mut stmt = conn.prepare(&query)?;
                let rows = stmt.query_map([], |row| {
                    let mut row_data = HashMap::new();
                    for (i, column) in columns.iter().enumerate() {
                        let value: Option<String> = row.get(i).unwrap_or(None);
                        row_data.insert(column.name.clone(), value);
                    }
                    Ok(row_data)
                })?;
                
                let mut query_samples = Vec::new();
                for row in rows {
                    query_samples.push(row?);
                }
                
                if !query_samples.is_empty() {
                    println!("      📊 {} content: {} samples", query_type, query_samples.len());
                    all_samples.extend(query_samples);
                }
            }
            
            // If we still don't have enough samples, get random ones
            if all_samples.len() < max_samples / 2 {
                let remaining = max_samples - all_samples.len();
                let random_query = format!("SELECT * FROM {} ORDER BY RANDOM() LIMIT {}", table_name, remaining);
                let mut stmt = conn.prepare(&random_query)?;
                let rows = stmt.query_map([], |row| {
                    let mut row_data = HashMap::new();
                    for (i, column) in columns.iter().enumerate() {
                        let value: Option<String> = row.get(i).unwrap_or(None);
                        row_data.insert(column.name.clone(), value);
                    }
                    Ok(row_data)
                })?;
                
                for row in rows {
                    all_samples.push(row?);
                }
            }
        }
        
        Ok(all_samples)
    }
    
    /// Analyze content diversity to understand all entity types
    fn analyze_content_diversity(samples: &[HashMap<String, Option<String>>], columns: &[ColumnInfo]) -> String {
        let mut analysis = Vec::new();
        
        // Count different content patterns
        let mut html_count = 0;
        let mut text_count = 0;
        let mut short_count = 0;
        let mut empty_count = 0;
        
        for sample in samples {
            let mut has_html = false;
            let mut has_text = false;
            let mut total_length = 0;
            
            for column in columns {
                if let Some(Some(value)) = sample.get(&column.name) {
                    total_length += value.len();
                    if value.contains('<') && value.contains('>') {
                        has_html = true;
                    } else if value.len() > 20 {
                        has_text = true;
                    }
                }
            }
            
            if has_html {
                html_count += 1;
            } else if has_text {
                text_count += 1;
            } else if total_length > 0 {
                short_count += 1;
            } else {
                empty_count += 1;
            }
        }
        
        analysis.push(format!("HTML: {}", html_count));
        analysis.push(format!("Text: {}", text_count));
        analysis.push(format!("Short: {}", short_count));
        analysis.push(format!("Empty: {}", empty_count));
        
        analysis.join(", ")
    }
    
    /// Build SQL query to prioritize records with rich HTML/text content
    fn build_rich_content_query(table_name: &str, text_columns: &[&super::types::ColumnInfo], max_samples: usize) -> String {
        let mut conditions = Vec::new();
        
        for col in text_columns {
            // Prioritize records with HTML content
            conditions.push(format!("{} LIKE '%<%'", col.name));
            conditions.push(format!("LENGTH({}) > 100", col.name));
        }
        
        if conditions.is_empty() {
            format!("SELECT * FROM {} LIMIT {}", table_name, max_samples)
        } else {
            format!(
                "SELECT * FROM {} WHERE ({}) ORDER BY LENGTH({}) DESC LIMIT {}",
                table_name,
                conditions.join(" OR "),
                text_columns.first().unwrap().name,
                max_samples
            )
        }
    }
    
    /// Analyze formal foreign key relationships
    pub fn analyze_foreign_keys(conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("🔗 Analyzing foreign key relationships...");
        
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
        
        println!("   Found {} foreign key relationships", report.relationships.len());
        
        Ok(())
    }
    
    /// Detect HTML patterns in text content
    pub fn detect_html_patterns(text_samples: &[String]) -> Vec<String> {
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
                if sample.contains("<a href=") {
                    patterns.push("html_links".to_string());
                }
            }
        }
        
        patterns.sort();
        patterns.dedup();
        patterns
    }
    
    /// Extract text content from sampled data for pattern analysis
    pub fn extract_text_samples_from_table(table_info: &TableInfo, column_name: &str) -> Vec<String> {
        let mut samples = Vec::new();
        
        if let Some(sample_data) = &table_info.sample_data {
            if let Some(rows) = sample_data.as_array() {
                for row in rows {
                    if let Some(row_obj) = row.as_object() {
                        if let Some(value) = row_obj.get(column_name) {
                            if let Some(text_value) = value.as_str() {
                                if !text_value.is_empty() {
                                    samples.push(text_value.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        samples
    }
    
    /// Analyze HTML patterns across all text columns in the report
    pub fn analyze_html_patterns_in_report(report: &mut AnalysisReport) {
        println!("🔍 Analyzing HTML patterns across all tables...");
        
        let mut all_patterns = Vec::new();
        
        for (table_name, table_info) in &mut report.table_info {
            for column in &table_info.columns {
                if column.data_type.to_uppercase().contains("TEXT") {
                    let text_samples = Self::extract_text_samples_from_table(table_info, &column.name);
                    let patterns = Self::detect_html_patterns(&text_samples);
                    
                    if !patterns.is_empty() {
                        println!("   🔍 HTML patterns in {}.{}: {:?}", 
                                 table_name, column.name, patterns);
                        table_info.html_patterns.extend(patterns.clone());
                        all_patterns.extend(patterns);
                    }
                }
            }
        }
        
        // Deduplicate global patterns
        all_patterns.sort();
        all_patterns.dedup();
        report.html_patterns_found = all_patterns;
    }
}
