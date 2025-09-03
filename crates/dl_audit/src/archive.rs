//! Archive management for rotational audit reports
//!
//! Implements archiving of existing audit reports before creating new ones,
//! preventing overwrites and preserving historical data.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use flate2::{Compression, write::GzEncoder};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tar::Builder;
use walkdir::WalkDir;

pub struct ArchiveManager {
    base_dir: PathBuf,
}

impl ArchiveManager {
    /// Create new archive manager for the given base directory
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }
    
    /// Archive all CSV files in a directory before new report generation
    /// 
    /// Creates timestamped tar.gz archive if CSV files exist, then removes originals.
    /// Returns path to created archive or None if no files to archive.
    pub fn archive_existing_reports<P: AsRef<Path>>(&self, target_dir: P) -> Result<Option<PathBuf>> {
        let target_path = target_dir.as_ref();
        
        if !target_path.exists() {
            return Ok(None);
        }
        
        // Find all CSV files in target directory
        let csv_files: Vec<PathBuf> = WalkDir::new(target_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("csv"))
            .map(|e| e.path().to_path_buf())
            .collect();
            
        if csv_files.is_empty() {
            return Ok(None);
        }
        
        // Create archive filename with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let archive_name = format!("audit_archive_{}.tar.gz", timestamp);
        let archive_path = self.base_dir.join("archives").join(&archive_name);
        
        // Ensure archives directory exists
        if let Some(parent) = archive_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create archives directory")?;
        }
        
        // Create tar.gz archive
        let tar_gz = File::create(&archive_path)
            .context("Failed to create archive file")?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);
        
        // Add all CSV files to archive
        for csv_file in &csv_files {
            let archive_path_in_tar = csv_file.strip_prefix(target_path)
                .context("Failed to strip path prefix")?;
                
            tar.append_path_with_name(csv_file, archive_path_in_tar)
                .with_context(|| format!("Failed to add {} to archive", csv_file.display()))?;
        }
        
        // Finalize archive
        tar.finish().context("Failed to finalize tar archive")?;
        
        // Remove original CSV files
        for csv_file in csv_files {
            fs::remove_file(&csv_file)
                .with_context(|| format!("Failed to remove {}", csv_file.display()))?;
        }
        
        Ok(Some(archive_path))
    }
    
    /// Get all available archive files
    pub fn list_archives(&self) -> Result<Vec<ArchiveInfo>> {
        let archives_dir = self.base_dir.join("archives");
        
        if !archives_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut archives = Vec::new();
        
        for entry in WalkDir::new(archives_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("gz"))
        {
            let path = entry.path();
            let metadata = fs::metadata(path)
                .context("Failed to read file metadata")?;
                
            let created = metadata.created()
                .or_else(|_| metadata.modified())
                .context("Failed to get file timestamp")?;
                
            let created_utc: DateTime<Utc> = created.into();
            
            archives.push(ArchiveInfo {
                path: path.to_path_buf(),
                filename: path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                created_at: created_utc,
                size_bytes: metadata.len(),
            });
        }
        
        // Sort by creation time, newest first
        archives.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(archives)
    }
    
    /// Clean up old archives (keep only N most recent)
    pub fn cleanup_old_archives(&self, keep_count: usize) -> Result<usize> {
        let mut archives = self.list_archives()?;
        
        if archives.len() <= keep_count {
            return Ok(0);
        }
        
        // Remove oldest archives beyond keep_count
        let to_remove = &archives[keep_count..];
        let removed_count = to_remove.len();
        
        for archive in to_remove {
            fs::remove_file(&archive.path)
                .with_context(|| format!("Failed to remove archive {}", archive.filename))?;
        }
        
        Ok(removed_count)
    }
    
    /// Extract archive to specified directory (for data recovery)
    pub fn extract_archive<P: AsRef<Path>>(&self, archive_path: P, extract_to: P) -> Result<()> {
        let archive_file = File::open(archive_path.as_ref())
            .context("Failed to open archive file")?;
            
        let decoder = flate2::read::GzDecoder::new(archive_file);
        let mut archive = tar::Archive::new(decoder);
        
        archive.unpack(extract_to.as_ref())
            .context("Failed to extract archive")?;
            
        Ok(())
    }
}

/// Information about an archived audit report
#[derive(Debug, Clone)]
pub struct ArchiveInfo {
    pub path: PathBuf,
    pub filename: String,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
}

impl ArchiveInfo {
    /// Get human-readable size string
    pub fn size_human(&self) -> String {
        let bytes = self.size_bytes as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else {
            format!("{:.1} MB", bytes / (1024.0 * 1024.0))
        }
    }
    
    /// Extract timestamp from filename if possible
    pub fn parse_timestamp_from_filename(&self) -> Option<DateTime<Utc>> {
        // Extract timestamp from filename like "audit_archive_20250103_143022.tar.gz"
        let name = &self.filename;
        if let Some(captures) = regex::Regex::new(r"audit_archive_(\d{8})_(\d{6})")
            .ok()?
            .captures(name)
        {
            let date_str = captures.get(1)?.as_str();
            let time_str = captures.get(2)?.as_str();
            
            let datetime_str = format!("{}T{}", date_str, time_str);
            chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%dT%H%M%S")
                .ok()
                .map(|dt| dt.and_utc())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_archive_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let reports_dir = temp_dir.path().join("reports");
        let target_dir = reports_dir.join("analytics/seeds");
        
        fs::create_dir_all(&target_dir)?;
        
        // Create some test CSV files
        fs::write(target_dir.join("test1.csv"), "header1,header2\nval1,val2\n")?;
        fs::write(target_dir.join("test2.csv"), "header3,header4\nval3,val4\n")?;
        
        let archive_manager = ArchiveManager::new(&reports_dir);
        let archive_path = archive_manager.archive_existing_reports(&target_dir)?;
        
        assert!(archive_path.is_some());
        let archive_path = archive_path.unwrap();
        assert!(archive_path.exists());
        
        // Original CSV files should be gone
        assert!(!target_dir.join("test1.csv").exists());
        assert!(!target_dir.join("test2.csv").exists());
        
        Ok(())
    }
    
    #[test]
    fn test_no_archive_when_no_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let reports_dir = temp_dir.path().join("reports");
        let target_dir = reports_dir.join("analytics/seeds");
        
        fs::create_dir_all(&target_dir)?;
        
        let archive_manager = ArchiveManager::new(&reports_dir);
        let archive_path = archive_manager.archive_existing_reports(&target_dir)?;
        
        assert!(archive_path.is_none());
        
        Ok(())
    }
    
    #[test]
    fn test_archive_info_size_formatting() {
        let info = ArchiveInfo {
            path: PathBuf::from("test.tar.gz"),
            filename: "test.tar.gz".to_string(),
            created_at: Utc::now(),
            size_bytes: 1536,
        };
        
        assert_eq!(info.size_human(), "1.5 KB");
        
        let large_info = ArchiveInfo {
            path: PathBuf::from("large.tar.gz"),
            filename: "large.tar.gz".to_string(),
            created_at: Utc::now(),
            size_bytes: 2_097_152, // 2 MB
        };
        
        assert_eq!(large_info.size_human(), "2.0 MB");
    }
}
