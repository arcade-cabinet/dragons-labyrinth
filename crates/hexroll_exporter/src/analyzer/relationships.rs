//! Relationship discovery logic for HBF analysis

use anyhow::Result;
use rusqlite::Connection;
use std::collections::HashMap;

use super::types::{AnalysisReport, TableInfo, ImplicitRelationship, EmbeddedReference};

/// Relationship discovery engine for identifying implicit connections
pub struct RelationshipDiscovery;

impl RelationshipDiscovery {
    /// Discover implicit relationships through UUID patterns and cross-references
    pub fn discover_implicit_relationships(conn: &Connection, report: &mut AnalysisReport) -> Result<()> {
        println!("🔍 Discovering implicit relationships through UUID patterns...");
        
        let mut implicit_relationships = Vec::new();
        let table_names: Vec<String> = report.table_info.keys().cloned().collect();
        
        // For each pair of tables, look for matching UUID patterns
        for i in 0..table_names.len() {
            for j in (i + 1)..table_names.len() {
                let table1 = &table_names[i];
                let table2 = &table_names[j];
                
                let relationships = Self::find_uuid_relationships(conn, table1, table2, report)?;
                implicit_relationships.extend(relationships);
            }
        }
        
        // Store implicit relationships in the report
        for rel in &implicit_relationships {
            println!("   🔗 Found implicit relationship: {}.{} ↔ {}.{} ({} matches)", 
                     rel.from_table, rel.from_column, rel.to_table, rel.to_column, rel.match_count);
        }
        
        report.implicit_relationships = implicit_relationships;
        
        Ok(())
    }
    
    /// Find UUID-based relationships between two tables
    fn find_uuid_relationships(conn: &Connection, table1: &str, table2: &str, 
                              report: &AnalysisReport) -> Result<Vec<ImplicitRelationship>> {
        let mut relationships = Vec::new();
        
        let table1_info = report.table_info.get(table1).unwrap();
        let table2_info = report.table_info.get(table2).unwrap();
        
        // Look for columns that might contain UUIDs or IDs
        for col1 in &table1_info.columns {
            for col2 in &table2_info.columns {
                // Check if column names suggest they might be related
                if Self::columns_might_be_related(&col1.name, &col2.name) {
                    let match_count = Self::count_matching_values(conn, table1, &col1.name, table2, &col2.name)?;
                    
                    if match_count > 0 {
                        relationships.push(ImplicitRelationship {
                            from_table: table1.to_string(),
                            from_column: col1.name.clone(),
                            to_table: table2.to_string(),
                            to_column: col2.name.clone(),
                            match_count,
                            confidence: Self::calculate_relationship_confidence(match_count, table1_info.record_count, table2_info.record_count),
                        });
                    }
                }
            }
        }
        
        Ok(relationships)
    }
    
    /// Check if two column names suggest they might be related
    fn columns_might_be_related(col1: &str, col2: &str) -> bool {
        let col1_lower = col1.to_lowercase();
        let col2_lower = col2.to_lowercase();
        
        // Direct match
        if col1_lower == col2_lower {
            return true;
        }
        
        // Common ID patterns
        let id_patterns = ["uuid", "id", "ref", "key", "entity"];
        for pattern in &id_patterns {
            if (col1_lower.contains(pattern) && col2_lower.contains(pattern)) ||
               (col1_lower == *pattern && col2_lower.contains(pattern)) ||
               (col1_lower.contains(pattern) && col2_lower == *pattern) {
                return true;
            }
        }
        
        // Check if one references the other by name
        if col1_lower.contains(&col2_lower) || col2_lower.contains(&col1_lower) {
            return true;
        }
        
        false
    }
    
    /// Count matching values between two columns in different tables
    fn count_matching_values(conn: &Connection, table1: &str, col1: &str, 
                             table2: &str, col2: &str) -> Result<usize> {
        let query = format!(
            "SELECT COUNT(*) FROM {} t1 
             INNER JOIN {} t2 ON t1.{} = t2.{}
             WHERE t1.{} IS NOT NULL AND t2.{} IS NOT NULL",
            table1, table2, col1, col2, col1, col2
        );
        
        let count: usize = conn.query_row(&query, [], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);
        
        Ok(count)
    }
    
    /// Calculate confidence level for a relationship based on match ratio
    fn calculate_relationship_confidence(matches: usize, table1_records: usize, table2_records: usize) -> f64 {
        let smaller_table = table1_records.min(table2_records);
        if smaller_table == 0 {
            return 0.0;
        }
        
        (matches as f64) / (smaller_table as f64)
    }
    
    /// Extract embedded references from HTML/text content
    pub fn extract_embedded_references(text_samples: &[String], table_name: &str, column_name: &str) -> Result<Vec<EmbeddedReference>> {
        let mut embedded_refs = Vec::new();
        
        for sample in text_samples {
            // Look for UUID patterns in HTML/JSON content
            let uuid_pattern = regex::Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap();
            let uuids: Vec<String> = uuid_pattern.find_iter(sample)
                .map(|m| m.as_str().to_string())
                .collect();
            
            if !uuids.is_empty() {
                embedded_refs.push(EmbeddedReference {
                    table_name: table_name.to_string(),
                    column_name: column_name.to_string(),
                    reference_type: "uuid".to_string(),
                    references: uuids,
                });
            }
            
            // Look for hexroll-specific patterns like "hexroll://..." or entity references
            if sample.contains("hexroll://") || sample.contains("entity:") {
                let hexroll_pattern = regex::Regex::new(r"hexroll://[^\s]+|entity:[^\s]+").unwrap();
                let refs: Vec<String> = hexroll_pattern.find_iter(sample)
                    .map(|m| m.as_str().to_string())
                    .collect();
                    
                if !refs.is_empty() {
                    embedded_refs.push(EmbeddedReference {
                        table_name: table_name.to_string(),
                        column_name: column_name.to_string(),
                        reference_type: "hexroll_ref".to_string(),
                        references: refs,
                    });
                }
            }
            
            // Look for HTML link patterns
            if sample.contains("<a href=") {
                let link_pattern = regex::Regex::new(r#"<a\s+href=["']([^"']+)["']"#).unwrap();
                let links: Vec<String> = link_pattern.captures_iter(sample)
                    .map(|cap| cap[1].to_string())
                    .collect();
                    
                if !links.is_empty() {
                    embedded_refs.push(EmbeddedReference {
                        table_name: table_name.to_string(),
                        column_name: column_name.to_string(),
                        reference_type: "html_link".to_string(),
                        references: links,
                    });
                }
            }
        }
        
        if !embedded_refs.is_empty() {
            println!("   🔍 Found {} embedded references in {}.{}", 
                     embedded_refs.len(), table_name, column_name);
        }
        
        Ok(embedded_refs)
    }
    
    /// Analyze HTML content patterns across all tables
    pub fn analyze_embedded_references_in_report(report: &mut AnalysisReport) -> Result<()> {
        println!("🔍 Analyzing embedded references across all tables...");
        
        let mut all_embedded_refs = Vec::new();
        
        for (table_name, table_info) in &report.table_info {
            for column in &table_info.columns {
                if column.data_type.to_uppercase().contains("TEXT") {
                    let text_samples = super::core::CoreAnalyzer::extract_text_samples_from_table(table_info, &column.name);
                    let refs = Self::extract_embedded_references(&text_samples, table_name, &column.name)?;
                    all_embedded_refs.extend(refs);
                }
            }
        }
        
        report.embedded_references = all_embedded_refs;
        
        Ok(())
    }
    
    /// Generate comprehensive relationship recommendations
    pub fn generate_relationship_recommendations(report: &mut AnalysisReport) {
        println!("💡 Generating relationship recommendations...");
        
        // Analyze implicit relationships for strong candidates
        for rel in &report.implicit_relationships {
            if rel.confidence > 0.8 {
                report.recommendations.push(format!(
                    "HIGH CONFIDENCE: {}.{} → {}.{} ({:.1}% match rate, {} matches)",
                    rel.from_table, rel.from_column, rel.to_table, rel.to_column,
                    rel.confidence * 100.0, rel.match_count
                ));
            } else if rel.confidence > 0.5 {
                report.recommendations.push(format!(
                    "MEDIUM CONFIDENCE: {}.{} ↔ {}.{} ({:.1}% match rate)",
                    rel.from_table, rel.from_column, rel.to_table, rel.to_column,
                    rel.confidence * 100.0
                ));
            }
        }
        
        // Analyze embedded references
        let mut ref_summary = HashMap::new();
        for embedded_ref in &report.embedded_references {
            let key = format!("{}.{}", embedded_ref.table_name, embedded_ref.column_name);
            *ref_summary.entry(key).or_insert(0) += embedded_ref.references.len();
        }
        
        for (location, count) in ref_summary {
            report.recommendations.push(format!(
                "EMBEDDED REFS: {} contains {} embedded references - check for cross-table links",
                location, count
            ));
        }
        
        // Check for orphaned data
        if report.implicit_relationships.is_empty() && report.embedded_references.is_empty() {
            report.recommendations.push(
                "WARNING: No implicit relationships found - tables may be using application-level relationships".to_string()
            );
        }
    }
}
