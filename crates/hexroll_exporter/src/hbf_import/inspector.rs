//! SQLite database inspector for HBF files

use anyhow::Result;
use sea_orm::{Database, DatabaseConnection, Statement, DbBackend, ConnectionTrait};
use std::path::{Path, PathBuf};

/// SQLite inspector for examining HBF database structure
pub struct SqliteInspector {
    hbf_path: PathBuf,
}

impl SqliteInspector {
    pub async fn new<P: AsRef<Path>>(hbf_path: P) -> Result<Self> {
        let hbf_path = hbf_path.as_ref().to_path_buf();
        
        if !hbf_path.exists() {
            return Err(anyhow::anyhow!("HBF file not found: {}", hbf_path.display()));
        }
        
        Ok(Self { hbf_path })
    }
    
    /// Show table schemas
    pub async fn show_table_schemas(&self) -> Result<()> {
        let database_url = format!("sqlite:{}", self.hbf_path.display());
        let conn = Database::connect(&database_url).await?;
        
        println!("📋 Table Schemas:");
        
        // Get all table names
        let stmt = Statement::from_string(
            DbBackend::Sqlite, 
            "SELECT name FROM sqlite_master WHERE type='table'".to_string()
        );
        let table_names = conn.query_all(stmt).await?;
        
        for row in table_names {
            if let Some(name_value) = row.try_get("", "name")? {
                let table_name: String = name_value.try_into()?;
                
                println!("\n  🔹 Table: {}", table_name);
                
                // Get column info
                let pragma_stmt = Statement::from_string(
                    DbBackend::Sqlite,
                    format!("PRAGMA table_info({})", table_name)
                );
                let columns = conn.query_all(pragma_stmt).await?;
                
                for column_row in columns {
                    if let (Some(col_name), Some(col_type)) = (
                        column_row.try_get("", "name").ok().flatten(),
                        column_row.try_get("", "type").ok().flatten()
                    ) {
                        let name: String = col_name.try_into().unwrap_or_default();
                        let col_type: String = col_type.try_into().unwrap_or_default();
                        let not_null: bool = column_row.try_get("", "notnull")
                            .ok().flatten().and_then(|v| v.try_into().ok()).unwrap_or(false);
                        let pk: bool = column_row.try_get("", "pk")
                            .ok().flatten().and_then(|v| v.try_into().ok()).unwrap_or(false);
                        
                        let flags = match (pk, not_null) {
                            (true, _) => " [PRIMARY KEY]",
                            (false, true) => " [NOT NULL]",
                            _ => "",
                        };
                        
                        println!("    • {}: {}{}", name, col_type, flags);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Show sample data from tables
    pub async fn show_sample_data(&self) -> Result<()> {
        let database_url = format!("sqlite:{}", self.hbf_path.display());
        let conn = Database::connect(&database_url).await?;
        
        println!("📊 Sample Data:");
        
        // Get table names
        let stmt = Statement::from_string(
            DbBackend::Sqlite, 
            "SELECT name FROM sqlite_master WHERE type='table' LIMIT 5".to_string()
        );
        let table_names = conn.query_all(stmt).await?;
        
        for row in table_names {
            if let Some(name_value) = row.try_get("", "name")? {
                let table_name: String = name_value.try_into()?;
                
                println!("\n  🔹 Sample from '{}':", table_name);
                
                // Get sample records
                let sample_stmt = Statement::from_string(
                    DbBackend::Sqlite,
                    format!("SELECT * FROM {} LIMIT 2", table_name)
                );
                let samples = conn.query_all(sample_stmt).await?;
                
                if samples.is_empty() {
                    println!("    (empty table)");
                } else {
                    println!("    Found {} sample records", samples.len());
                    // For now just show record count - detailed display would be complex
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze relationships between tables
    pub async fn analyze_relationships(&self) -> Result<()> {
        println!("🔗 Analyzing table relationships...");
        println!("   (Relationship analysis would be implemented here)");
        
        Ok(())
    }
    
    /// Export full SQLite dump
    pub async fn export_full_dump<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        println!("💾 Exporting SQLite dump...");
        println!("   (Full dump export would be implemented here)");
        println!("   Target: {}", output_path.as_ref().display());
        
        Ok(())
    }
}
