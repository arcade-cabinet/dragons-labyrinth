//! Report generation and configuration for audit system
//!
//! Handles the creation and metadata tracking of audit reports with 
//! standardized subpath structure.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use dl_types::{AuditableType, AuditMetadata};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Configuration for generating audit reports
#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub reports_dir: PathBuf,
    pub archive_existing: bool,
    pub include_statistics: bool,
    pub include_preview: bool,
    pub max_preview_rows: usize,
}

impl ReportConfig {
    /// Create new report configuration
    pub fn new<P: AsRef<Path>>(reports_dir: P) -> Self {
        Self {
            reports_dir: reports_dir.as_ref().to_path_buf(),
            archive_existing: true,
            include_statistics: true,
            include_preview: false,
            max_preview_rows: 10,
        }
    }
    
    /// Set whether to archive existing reports
    pub fn with_archiving(mut self, archive: bool) -> Self {
        self.archive_existing = archive;
        self
    }
    
    /// Set whether to include statistical summaries
    pub fn with_statistics(mut self, stats: bool) -> Self {
        self.include_statistics = stats;
        self
    }
    
    /// Set whether to include data previews
    pub fn with_preview(mut self, preview: bool, max_rows: usize) -> Self {
        self.include_preview = preview;
        self.max_preview_rows = max_rows;
        self
    }
    
    /// Generate standardized report path for a type
    /// 
    /// Creates path like: audits/analytics/seeds/narrative.csv
    pub fn get_report_path<T: AuditableType>(&self, report_name: &str) -> PathBuf {
        self.reports_dir
            .join("audits")
            .join(T::audit_category())
            .join(T::audit_subcategory())
            .join(format!("{}.csv", report_name))
    }
    
    /// Generate directory for a specific type's reports
    pub fn get_reports_directory<T: AuditableType>(&self) -> PathBuf {
        self.reports_dir
            .join("audits")
            .join(T::audit_category())
            .join(T::audit_subcategory())
    }
}

/// Metadata about a generated audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportMetadata {
    pub category: String,
    pub subcategory: String,
    pub type_name: String,
    pub report_name: String,
    pub timestamp: DateTime<Utc>,
    pub row_count: usize,
    pub column_count: usize,
    pub file_path: String,
    pub file_size_bytes: u64,
    pub archive_created: Option<String>,
    pub generation_time_ms: u64,
}

impl AuditReportMetadata {
    /// Create new report metadata
    pub fn new<T: AuditableType>(
        report_name: String,
        file_path: PathBuf,
        row_count: usize,
        column_count: usize,
        file_size_bytes: u64,
        archive_created: Option<PathBuf>,
        generation_time_ms: u64,
    ) -> Self {
        Self {
            category: T::audit_category(),
            subcategory: T::audit_subcategory(),
            type_name: std::any::type_name::<T>().to_string(),
            report_name,
            timestamp: Utc::now(),
            row_count,
            column_count,
            file_path: file_path.to_string_lossy().to_string(),
            file_size_bytes,
            archive_created: archive_created.map(|p| p.to_string_lossy().to_string()),
            generation_time_ms,
        }
    }
    
    /// Get human-readable file size
    pub fn file_size_human(&self) -> String {
        let bytes = self.file_size_bytes as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        }
    }
    
    /// Get human-readable generation time
    pub fn generation_time_human(&self) -> String {
        if self.generation_time_ms < 1000 {
            format!("{} ms", self.generation_time_ms)
        } else {
            format!("{:.1} s", self.generation_time_ms as f64 / 1000.0)
        }
    }
    
    /// Save metadata as JSON alongside the CSV report
    pub fn save_metadata(&self, reports_dir: &Path) -> Result<()> {
        let metadata_path = reports_dir
            .join("metadata")
            .join(format!("{}_{}.json", self.report_name, self.timestamp.format("%Y%m%d_%H%M%S")));
            
        // Ensure metadata directory exists
        if let Some(parent) = metadata_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create metadata directory")?;
        }
        
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize metadata")?;
            
        std::fs::write(&metadata_path, json)
            .context("Failed to write metadata file")?;
            
        Ok(())
    }
    
    /// Load metadata from JSON file
    pub fn load_metadata<P: AsRef<Path>>(path: P) -> Result<Self> {
        let json = std::fs::read_to_string(path.as_ref())
            .context("Failed to read metadata file")?;
            
        serde_json::from_str(&json)
            .context("Failed to deserialize metadata")
    }
}

/// Report generation summary
#[derive(Debug, Clone)]
pub struct ReportSummary {
    pub reports_generated: Vec<AuditReportMetadata>,
    pub total_files: usize,
    pub total_rows: usize,
    pub total_size_bytes: u64,
    pub archives_created: Vec<String>,
    pub total_generation_time_ms: u64,
}

impl ReportSummary {
    /// Create new empty report summary
    pub fn new() -> Self {
        Self {
            reports_generated: Vec::new(),
            total_files: 0,
            total_rows: 0,
            total_size_bytes: 0,
            archives_created: Vec::new(),
            total_generation_time_ms: 0,
        }
    }
    
    /// Add a report to the summary
    pub fn add_report(&mut self, metadata: AuditReportMetadata) {
        self.total_files += 1;
        self.total_rows += metadata.row_count;
        self.total_size_bytes += metadata.file_size_bytes;
        self.total_generation_time_ms += metadata.generation_time_ms;
        
        if let Some(archive) = &metadata.archive_created {
            if !self.archives_created.contains(archive) {
                self.archives_created.push(archive.clone());
            }
        }
        
        self.reports_generated.push(metadata);
    }
    
    /// Get total size in human-readable format
    pub fn total_size_human(&self) -> String {
        let bytes = self.total_size_bytes as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        }
    }
    
    /// Get total generation time in human-readable format
    pub fn total_time_human(&self) -> String {
        if self.total_generation_time_ms < 1000 {
            format!("{} ms", self.total_generation_time_ms)
        } else if self.total_generation_time_ms < 60000 {
            format!("{:.1} s", self.total_generation_time_ms as f64 / 1000.0)
        } else {
            let minutes = self.total_generation_time_ms / 60000;
            let seconds = (self.total_generation_time_ms % 60000) as f64 / 1000.0;
            format!("{}m {:.1}s", minutes, seconds)
        }
    }
    
    /// Print summary to stdout
    pub fn print_summary(&self) {
        println!("📊 Audit Report Generation Summary");
        println!("═══════════════════════════════════");
        println!("📁 Files generated: {}", self.total_files);
        println!("📈 Total rows: {}", self.total_rows);
        println!("💾 Total size: {}", self.total_size_human());
        println!("⏱️  Generation time: {}", self.total_time_human());
        
        if !self.archives_created.is_empty() {
            println!("🗃️  Archives created: {}", self.archives_created.len());
            for archive in &self.archives_created {
                println!("   • {}", archive);
            }
        }
        
        println!("\n📋 Generated Reports:");
        for report in &self.reports_generated {
            println!("   • {} ({}) - {} rows, {}",
                report.report_name,
                report.generation_time_human(),
                report.row_count,
                report.file_size_human()
            );
        }
    }
}

impl Default for ReportSummary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_report_config_path_generation() {
        let config = ReportConfig::new("/tmp/reports");
        
        // Mock auditable type for testing
        struct MockType;
        impl AuditableType for MockType {
            fn audit_headers() -> Vec<String> { vec![] }
            fn audit_row(&self) -> Vec<String> { vec![] }
            fn audit_category() -> String { "analytics".to_string() }
            fn audit_subcategory() -> String { "seeds".to_string() }
        }
        
        let path = config.get_report_path::<MockType>("narrative");
        assert_eq!(path, PathBuf::from("/tmp/reports/audits/analytics/seeds/narrative.csv"));
    }
    
    #[test]
    fn test_report_summary_accumulation() {
        let mut summary = ReportSummary::new();
        
        let metadata = AuditReportMetadata {
            category: "test".to_string(),
            subcategory: "data".to_string(),
            type_name: "TestType".to_string(),
            report_name: "test_report".to_string(),
            timestamp: Utc::now(),
            row_count: 100,
            column_count: 5,
            file_path: "test.csv".to_string(),
            file_size_bytes: 2048,
            archive_created: Some("archive.tar.gz".to_string()),
            generation_time_ms: 500,
        };
        
        summary.add_report(metadata);
        
        assert_eq!(summary.total_files, 1);
        assert_eq!(summary.total_rows, 100);
        assert_eq!(summary.total_size_bytes, 2048);
        assert_eq!(summary.total_generation_time_ms, 500);
        assert_eq!(summary.archives_created.len(), 1);
    }
    
    #[test]
    fn test_size_formatting() {
        let summary = ReportSummary {
            total_size_bytes: 1536,
            ..Default::default()
        };
        
        assert_eq!(summary.total_size_human(), "1.5 KB");
    }
    
    #[test]
    fn test_time_formatting() {
        let summary = ReportSummary {
            total_generation_time_ms: 2500,
            ..Default::default()
        };
        
        assert_eq!(summary.total_time_human(), "2.5 s");
        
        let long_summary = ReportSummary {
            total_generation_time_ms: 65000, // 1m 5s
            ..Default::default()
        };
        
        assert_eq!(long_summary.total_time_human(), "1m 5.0s");
    }
}
