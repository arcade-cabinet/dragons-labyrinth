//! Main audit system for Dragon's Labyrinth
//!
//! Provides the main AuditSystem struct that orchestrates audit report generation
//! with Polars DataFrames and rotational archiving.

use anyhow::{Context, Result};
use dl_types::AuditableType;
use polars::prelude::*;
use std::path::Path;
use std::time::Instant;

use crate::archive::ArchiveManager;
use crate::dataframe::DataFrameBuilder;
use crate::reports::{AuditReportMetadata, ReportConfig, ReportSummary};

/// Main audit system for generating reports from any pipeline stage
pub struct AuditSystem {
    config: ReportConfig,
    archive_manager: ArchiveManager,
}

impl AuditSystem {
    /// Create new audit system
    /// 
    /// # Arguments
    /// * `reports_dir` - Base directory for all audit reports (required)
    /// 
    /// # Example
    /// ```
    /// use dl_audit::AuditSystem;
    /// let audit = AuditSystem::new("/tmp/reports");
    /// ```
    pub fn new<P: AsRef<Path>>(reports_dir: P) -> Self {
        let config = ReportConfig::new(&reports_dir);
        let archive_manager = ArchiveManager::new(&reports_dir);
        
        Self {
            config,
            archive_manager,
        }
    }
    
    /// Create audit system with custom configuration
    pub fn with_config(config: ReportConfig) -> Self {
        let archive_manager = ArchiveManager::new(&config.reports_dir);
        
        Self {
            config,
            archive_manager,
        }
    }
    
    /// Generate audit report for a collection of auditable types
    /// 
    /// This is the main entry point that dl_analysis, dl_processors, etc. will call.
    /// 
    /// # Arguments
    /// * `items` - Collection of items implementing AuditableType
    /// * `report_name` - Name for the report file (e.g., "narrative", "hex_tiles")
    /// 
    /// # Returns
    /// Metadata about the generated report
    /// 
    /// # Example
    /// ```ignore
    /// // From dl_analysis or dl_processors:
    /// let analyzed_tiles = vec![/* HexTile instances */];
    /// let metadata = audit_system.generate_report(&analyzed_tiles, "hex_tiles")?;
    /// ```
    pub fn generate_report<T: AuditableType>(
        &self,
        items: &[T],
        report_name: &str,
    ) -> Result<AuditReportMetadata> {
        let start_time = Instant::now();
        
        // Get report path using standardized structure
        let report_path = self.config.get_report_path::<T>(report_name);
        let reports_directory = self.config.get_reports_directory::<T>();
        
        // Step 1: Archive existing CSV files if they exist
        let archive_created = if self.config.archive_existing {
            self.archive_manager.archive_existing_reports(&reports_directory)?
        } else {
            None
        };
        
        // Step 2: Generate DataFrame using Polars lazy API
        let mut builder = DataFrameBuilder::from_auditable(items)?;
        
        // Add calculated columns if requested
        if self.config.include_statistics {
            builder = builder.with_calculated_columns(items)?;
        }
        
        // Step 3: Write CSV report
        let stats = builder.write_csv(&report_path)?;
        
        // Step 4: Get file size
        let file_size_bytes = std::fs::metadata(&report_path)
            .context("Failed to read report file metadata")?
            .len();
        
        // Step 5: Create metadata
        let generation_time = start_time.elapsed().as_millis() as u64;
        let metadata = AuditReportMetadata::new::<T>(
            report_name.to_string(),
            report_path,
            stats.row_count,
            stats.column_count,
            file_size_bytes,
            archive_created,
            generation_time,
        );
        
        // Step 6: Save metadata
        metadata.save_metadata(&self.config.reports_dir)?;
        
        Ok(metadata)
    }
    
    /// Generate multiple reports in batch
    pub fn generate_batch_reports<T: AuditableType>(
        &self,
        reports: Vec<(&[T], &str)>,
    ) -> Result<ReportSummary> {
        let mut summary = ReportSummary::new();
        
        for (items, report_name) in reports {
            let metadata = self.generate_report(items, report_name)
                .with_context(|| format!("Failed to generate report '{}'", report_name))?;
            summary.add_report(metadata);
        }
        
        Ok(summary)
    }
    
    /// Generate audit report with custom Polars operations
    /// 
    /// For advanced users who want to apply custom DataFrame operations.
    pub fn generate_custom_report<T: AuditableType, F>(
        &self,
        items: &[T],
        report_name: &str,
        custom_operations: F,
    ) -> Result<AuditReportMetadata>
    where
        F: FnOnce(DataFrameBuilder) -> Result<DataFrameBuilder>,
    {
        let start_time = Instant::now();
        
        // Get report path
        let report_path = self.config.get_report_path::<T>(report_name);
        let reports_directory = self.config.get_reports_directory::<T>();
        
        // Archive existing reports
        let archive_created = if self.config.archive_existing {
            self.archive_manager.archive_existing_reports(&reports_directory)?
        } else {
            None
        };
        
        // Build DataFrame and apply custom operations
        let builder = DataFrameBuilder::from_auditable(items)?;
        let builder = custom_operations(builder)?;
        
        // Write CSV report
        let stats = builder.write_csv(&report_path)?;
        
        // Get file size and create metadata
        let file_size_bytes = std::fs::metadata(&report_path)
            .context("Failed to read report file metadata")?
            .len();
        
        let generation_time = start_time.elapsed().as_millis() as u64;
        let metadata = AuditReportMetadata::new::<T>(
            report_name.to_string(),
            report_path,
            stats.row_count,
            stats.column_count,
            file_size_bytes,
            archive_created,
            generation_time,
        );
        
        metadata.save_metadata(&self.config.reports_dir)?;
        
        Ok(metadata)
    }
    
    /// Clean up old archives (keep only N most recent)
    pub fn cleanup_archives(&self, keep_count: usize) -> Result<usize> {
        self.archive_manager.cleanup_old_archives(keep_count)
    }
    
    /// List all available archive files
    pub fn list_archives(&self) -> Result<Vec<crate::archive::ArchiveInfo>> {
        self.archive_manager.list_archives()
    }
    
    /// Extract an archive for data recovery
    pub fn extract_archive<P: AsRef<Path>>(&self, archive_path: P, extract_to: P) -> Result<()> {
        self.archive_manager.extract_archive(archive_path, extract_to)
    }
    
    /// Get configuration
    pub fn config(&self) -> &ReportConfig {
        &self.config
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: ReportConfig) {
        // Update archive manager if reports_dir changed
        if config.reports_dir != self.config.reports_dir {
            self.archive_manager = ArchiveManager::new(&config.reports_dir);
        }
        self.config = config;
    }
}

/// Utility functions for common audit operations
pub mod utils {
    use super::*;
    use polars::prelude::*;
    
    /// Generate summary statistics report for any auditable type
    pub fn generate_summary_stats<T: AuditableType>(
        audit_system: &AuditSystem,
        items: &[T],
        report_name: &str,
    ) -> Result<AuditReportMetadata> {
        audit_system.generate_custom_report(items, report_name, |builder| {
            // Add statistical summary operations
            let df = builder.collect()?;
            
            // Store dimensions before moving df
            let row_count = df.height() as i32;
            let col_count = df.width() as i32;
            
            // Create summary DataFrame with aggregations
            let summary_df = df.lazy()
                .select([
                    lit("summary").alias("metric"),
                    lit(row_count).alias("total_rows"),
                    lit(col_count).alias("total_columns"),
                ])
                .collect()?;
                
            Ok(DataFrameBuilder::from(summary_df))
        })
    }
    
    /// Generate top N report (most frequent values)
    pub fn generate_top_n_report<T: AuditableType>(
        audit_system: &AuditSystem,
        items: &[T],
        column_name: &str,
        n: usize,
        report_name: &str,
    ) -> Result<AuditReportMetadata> {
        let column_name = column_name.to_string();
        audit_system.generate_custom_report(items, report_name, move |builder| {
            builder
                .group_by_and_aggregate(
                    &[column_name.clone()],
                    &[len().alias("count")]
                )?
                .sort_by("count", true)?
                .with_row_numbers()
        })
    }
}

impl From<polars::prelude::DataFrame> for DataFrameBuilder {
    fn from(df: polars::prelude::DataFrame) -> Self {
        Self {
            lazy_frame: Some(df.lazy()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::collections::HashMap;
    
    #[derive(Debug)]
    struct TestAuditData {
        name: String,
        value: f64,
        category: String,
    }
    
    impl AuditableType for TestAuditData {
        fn audit_headers() -> Vec<String> {
            vec!["name".to_string(), "value".to_string(), "category".to_string()]
        }
        
        fn audit_row(&self) -> Vec<String> {
            vec![self.name.clone(), self.value.to_string(), self.category.clone()]
        }
        
        fn audit_category() -> String {
            "test".to_string()
        }
        
        fn audit_subcategory() -> String {
            "data".to_string()
        }
        
        fn extract_numeric_fields(&self) -> HashMap<String, f64> {
            let mut fields = HashMap::new();
            fields.insert("value".to_string(), self.value);
            fields
        }
    }
    
    #[test]
    fn test_audit_system_report_generation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let audit_system = AuditSystem::new(temp_dir.path());
        
        let test_data = vec![
            TestAuditData { 
                name: "item1".to_string(), 
                value: 10.0, 
                category: "A".to_string() 
            },
            TestAuditData { 
                name: "item2".to_string(), 
                value: 20.0, 
                category: "B".to_string() 
            },
        ];
        
        let metadata = audit_system.generate_report(&test_data, "test_report")?;
        
        assert_eq!(metadata.row_count, 2);
        assert_eq!(metadata.column_count, 3);
        assert!(metadata.file_size_bytes > 0);
        assert!(metadata.generation_time_ms < 10000); // Should be fast
        
        // Check that file was actually created
        let report_path = PathBuf::from(&metadata.file_path);
        assert!(report_path.exists());
        
        Ok(())
    }
    
    #[test]
    fn test_batch_report_generation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let audit_system = AuditSystem::new(temp_dir.path());
        
        let test_data1 = vec![
            TestAuditData { 
                name: "batch1".to_string(), 
                value: 5.0, 
                category: "X".to_string() 
            },
        ];
        
        let test_data2 = vec![
            TestAuditData { 
                name: "batch2".to_string(), 
                value: 15.0, 
                category: "Y".to_string() 
            },
        ];
        
        let batch_reports = vec![
            (test_data1.as_slice(), "batch_report_1"),
            (test_data2.as_slice(), "batch_report_2"),
        ];
        
        let summary = audit_system.generate_batch_reports(batch_reports)?;
        
        assert_eq!(summary.total_files, 2);
        assert_eq!(summary.total_rows, 2);
        
        Ok(())
    }
}
