use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use regex::Regex;

/// Asset category definitions for Dragon's Labyrinth
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetCategory {
    Core,       // Sacred assets that never change
    Library,    // CC0 library assets
    Generated,  // AI-generated assets
}

/// Asset type classifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Model3D,    // .glb, .gltf files
    Texture,    // .png, .jpg files
    Audio,      // .ogg, .mp3 files
    Video,      // .mp4, .webm files
    UI,         // UI elements
    Dialogue,   // .yarn files
    HexTile,    // Hex world tiles
    Companion,  // Companion assets
}

/// Dread level for horror progression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DreadLevel {
    Peace = 0,
    Unease = 1,
    Dread = 2,
    Terror = 3,
    Horror = 4,
}

/// Asset metadata extracted from files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub path: PathBuf,
    pub filename: String,
    pub category: AssetCategory,
    pub asset_type: AssetType,
    pub dread_level: Option<DreadLevel>,
    pub display_name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub file_size_mb: f32,
    pub dimensions: Option<(u32, u32)>,
    pub vertex_count: Option<u32>,
    pub mobile_compatible: bool,
    pub performance_score: f32,
    pub validation_status: ValidationStatus,
}

/// Validation status for assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Pending,
    Approved,
    Rejected,
    NeedsReview,
}

/// Asset validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRules {
    pub max_file_size_mb: f32,
    pub max_vertex_count: u32,
    pub max_texture_size: u32,
    pub required_formats: Vec<String>,
    pub forbidden_patterns: Vec<String>,
    pub mobile_requirements: MobileRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileRequirements {
    pub max_file_size_mb: f32,
    pub max_vertex_count: u32,
    pub max_texture_size: u32,
    pub required_compression: bool,
}

/// Core asset parsing rules (similar to Professor Pixels' core_rules.py)
pub struct CoreAssetRules;

impl CoreAssetRules {
    /// Parse filename to extract metadata
    pub fn parse_filename(filename: &str, category: AssetCategory) -> AssetMetadata {
        let path = PathBuf::from(filename);
        let name_without_ext = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        // Extract display name
        let display_name = name_without_ext
            .replace('_', " ")
            .replace('-', " ");
        
        // Extract dread level if present
        let dread_level = Self::extract_dread_level(name_without_ext);
        
        // Determine asset type from extension
        let asset_type = Self::determine_asset_type(&path);
        
        // Extract tags from filename
        let tags = Self::extract_tags(name_without_ext);
        
        // Build description
        let description = match category {
            AssetCategory::Core => format!("Core asset: {}", display_name),
            AssetCategory::Library => format!("CC0 library asset: {}", display_name),
            AssetCategory::Generated => format!("AI-generated asset: {}", display_name),
        };
        
        AssetMetadata {
            path: path.clone(),
            filename: filename.to_string(),
            category,
            asset_type,
            dread_level,
            display_name,
            description,
            tags,
            file_size_mb: 0.0, // Will be filled by scanner
            dimensions: None,   // Will be filled by scanner
            vertex_count: None, // Will be filled by scanner
            mobile_compatible: true,
            performance_score: 0.5,
            validation_status: ValidationStatus::Pending,
        }
    }
    
    fn extract_dread_level(name: &str) -> Option<DreadLevel> {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("dread4") || name_lower.contains("horror") {
            Some(DreadLevel::Horror)
        } else if name_lower.contains("dread3") || name_lower.contains("terror") {
            Some(DreadLevel::Terror)
        } else if name_lower.contains("dread2") || name_lower.contains("dread") {
            Some(DreadLevel::Dread)
        } else if name_lower.contains("dread1") || name_lower.contains("unease") {
            Some(DreadLevel::Unease)
        } else if name_lower.contains("dread0") || name_lower.contains("peace") {
            Some(DreadLevel::Peace)
        } else {
            None
        }
    }
    
    fn determine_asset_type(path: &Path) -> AssetType {
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "glb" | "gltf" => AssetType::Model3D,
            "png" | "jpg" | "jpeg" => AssetType::Texture,
            "ogg" | "mp3" | "wav" => AssetType::Audio,
            "mp4" | "webm" => AssetType::Video,
            "yarn" => AssetType::Dialogue,
            _ => {
                // Check filename patterns
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                
                if name.contains("hex") || name.contains("tile") {
                    AssetType::HexTile
                } else if name.contains("companion") || name.contains("einar") || 
                          name.contains("mira") || name.contains("sorin") || name.contains("tamara") {
                    AssetType::Companion
                } else if name.contains("ui") || name.contains("button") || name.contains("menu") {
                    AssetType::UI
                } else {
                    AssetType::Texture // Default
                }
            }
        }
    }
    
    fn extract_tags(name: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let name_lower = name.to_lowercase();
        
        // Horror progression tags
        if name_lower.contains("corrupt") {
            tags.push("corruption".to_string());
        }
        if name_lower.contains("dark") {
            tags.push("dark".to_string());
        }
        if name_lower.contains("nightmare") {
            tags.push("nightmare".to_string());
        }
        
        // Biome tags
        for biome in &["grassland", "forest", "swamp", "mountain", "dungeon", "village", "ruins"] {
            if name_lower.contains(biome) {
                tags.push(biome.to_string());
            }
        }
        
        // Companion tags
        for companion in &["einar", "mira", "sorin", "tamara"] {
            if name_lower.contains(companion) {
                tags.push(companion.to_string());
            }
        }
        
        // Asset quality tags
        if name_lower.contains("hd") || name_lower.contains("high") {
            tags.push("high_quality".to_string());
        }
        if name_lower.contains("mobile") || name_lower.contains("optimized") {
            tags.push("mobile_optimized".to_string());
        }
        
        tags
    }
}

/// Asset scanner for Dragon's Labyrinth
pub struct AssetScanner {
    pub base_path: PathBuf,
    pub rules: AssetRules,
    pub scanned_assets: Vec<AssetMetadata>,
    pub validation_report: ValidationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub total_scanned: usize,
    pub core_assets: usize,
    pub library_assets: usize,
    pub generated_assets: usize,
    pub approved: usize,
    pub rejected: usize,
    pub needs_review: usize,
    pub mobile_compatible: usize,
    pub performance_warnings: Vec<String>,
    pub validation_errors: Vec<String>,
}

impl AssetScanner {
    pub fn new(base_path: PathBuf) -> Self {
        let rules = AssetRules {
            max_file_size_mb: 10.0,
            max_vertex_count: 100_000,
            max_texture_size: 2048,
            required_formats: vec![
                "glb".to_string(),
                "gltf".to_string(),
                "png".to_string(),
                "ogg".to_string(),
            ],
            forbidden_patterns: vec![
                "test".to_string(),
                "temp".to_string(),
                "backup".to_string(),
            ],
            mobile_requirements: MobileRequirements {
                max_file_size_mb: 5.0,
                max_vertex_count: 50_000,
                max_texture_size: 1024,
                required_compression: true,
            },
        };
        
        Self {
            base_path,
            rules,
            scanned_assets: Vec::new(),
            validation_report: ValidationReport {
                total_scanned: 0,
                core_assets: 0,
                library_assets: 0,
                generated_assets: 0,
                approved: 0,
                rejected: 0,
                needs_review: 0,
                mobile_compatible: 0,
                performance_warnings: Vec::new(),
                validation_errors: Vec::new(),
            },
        }
    }
    
    /// Scan all assets in the three-tier structure
    pub fn scan_all(&mut self) -> Result<&ValidationReport, String> {
        // Clear previous scan
        self.scanned_assets.clear();
        self.validation_report = ValidationReport {
            total_scanned: 0,
            core_assets: 0,
            library_assets: 0,
            generated_assets: 0,
            approved: 0,
            rejected: 0,
            needs_review: 0,
            mobile_compatible: 0,
            performance_warnings: Vec::new(),
            validation_errors: Vec::new(),
        };
        
        // Scan each category
        self.scan_category("core", AssetCategory::Core)?;
        self.scan_category("library", AssetCategory::Library)?;
        self.scan_category("generated", AssetCategory::Generated)?;
        
        Ok(&self.validation_report)
    }
    
    /// Scan a specific asset category
    pub fn scan_category(&mut self, subdir: &str, category: AssetCategory) -> Result<(), String> {
        let category_path = self.base_path.join(subdir);
        
        if !category_path.exists() {
            return Ok(()); // Category directory doesn't exist yet
        }
        
        for entry in WalkDir::new(&category_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip directories
            if path.is_dir() {
                continue;
            }
            
            // Skip forbidden patterns
            let filename = path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            if self.rules.forbidden_patterns.iter().any(|p| filename.contains(p)) {
                continue;
            }
            
            // Parse and validate asset
            let mut metadata = CoreAssetRules::parse_filename(filename, category.clone());
            metadata.path = path.to_path_buf();
            
            // Get file size
            if let Ok(file_metadata) = std::fs::metadata(path) {
                metadata.file_size_mb = file_metadata.len() as f32 / (1024.0 * 1024.0);
            }
            
            // Validate asset
            self.validate_asset(&mut metadata);
            
            // Update report counters
            self.validation_report.total_scanned += 1;
            match category {
                AssetCategory::Core => self.validation_report.core_assets += 1,
                AssetCategory::Library => self.validation_report.library_assets += 1,
                AssetCategory::Generated => self.validation_report.generated_assets += 1,
            }
            
            match metadata.validation_status {
                ValidationStatus::Approved => self.validation_report.approved += 1,
                ValidationStatus::Rejected => self.validation_report.rejected += 1,
                ValidationStatus::NeedsReview => self.validation_report.needs_review += 1,
                ValidationStatus::Pending => {}
            }
            
            if metadata.mobile_compatible {
                self.validation_report.mobile_compatible += 1;
            }
            
            self.scanned_assets.push(metadata);
        }
        
        Ok(())
    }
    
    /// Validate an asset against rules
    fn validate_asset(&mut self, metadata: &mut AssetMetadata) {
        let mut issues = Vec::new();
        
        // Check file size
        if metadata.file_size_mb > self.rules.max_file_size_mb {
            issues.push(format!(
                "File size {:.1}MB exceeds limit of {:.1}MB",
                metadata.file_size_mb, self.rules.max_file_size_mb
            ));
            metadata.mobile_compatible = false;
        }
        
        // Check mobile compatibility
        if metadata.file_size_mb > self.rules.mobile_requirements.max_file_size_mb {
            metadata.mobile_compatible = false;
            self.validation_report.performance_warnings.push(format!(
                "{}: Not mobile compatible (size: {:.1}MB)",
                metadata.filename, metadata.file_size_mb
            ));
        }
        
        // Check format requirements
        if let Some(ext) = metadata.path.extension() {
            let ext_str = ext.to_str().unwrap_or("").to_lowercase();
            
            match metadata.asset_type {
                AssetType::Model3D => {
                    if !["glb", "gltf"].contains(&ext_str.as_str()) {
                        issues.push(format!("Model must be .glb or .gltf, not .{}", ext_str));
                    }
                }
                AssetType::Texture | AssetType::HexTile => {
                    if !["png", "jpg", "jpeg"].contains(&ext_str.as_str()) {
                        issues.push(format!("Texture must be .png or .jpg, not .{}", ext_str));
                    }
                }
                AssetType::Audio => {
                    if !["ogg", "mp3", "wav"].contains(&ext_str.as_str()) {
                        issues.push(format!("Audio must be .ogg, .mp3, or .wav, not .{}", ext_str));
                    }
                }
                _ => {}
            }
        }
        
        // Calculate performance score
        metadata.performance_score = self.calculate_performance_score(metadata);
        
        // Set validation status
        if issues.is_empty() {
            if metadata.category == AssetCategory::Core {
                metadata.validation_status = ValidationStatus::Approved; // Core assets are pre-approved
            } else if metadata.performance_score > 0.7 {
                metadata.validation_status = ValidationStatus::Approved;
            } else {
                metadata.validation_status = ValidationStatus::NeedsReview;
            }
        } else {
            metadata.validation_status = ValidationStatus::Rejected;
            for issue in issues {
                self.validation_report.validation_errors.push(format!(
                    "{}: {}",
                    metadata.filename, issue
                ));
            }
        }
    }
    
    fn calculate_performance_score(&self, metadata: &AssetMetadata) -> f32 {
        let mut score = 1.0;
        
        // Penalize large files
        if metadata.file_size_mb > 5.0 {
            score -= 0.2;
        } else if metadata.file_size_mb > 2.0 {
            score -= 0.1;
        }
        
        // Bonus for mobile compatibility
        if metadata.mobile_compatible {
            score += 0.1;
        }
        
        // Bonus for optimized tags
        if metadata.tags.contains(&"mobile_optimized".to_string()) {
            score += 0.1;
        }
        
        // Core assets get a bonus
        if matches!(metadata.category, AssetCategory::Core) {
            score += 0.2;
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Get assets by category
    pub fn get_assets_by_category(&self, category: AssetCategory) -> Vec<&AssetMetadata> {
        self.scanned_assets
            .iter()
            .filter(|a| a.category == category)
            .collect()
    }
    
    /// Get assets by dread level
    pub fn get_assets_by_dread(&self, dread_level: DreadLevel) -> Vec<&AssetMetadata> {
        self.scanned_assets
            .iter()
            .filter(|a| a.dread_level == Some(dread_level))
            .collect()
    }
    
    /// Export scan results to JSON
    pub fn export_to_json(&self, path: &Path) -> Result<(), String> {
        let data = serde_json::json!({
            "scan_timestamp": chrono::Utc::now().to_rfc3339(),
            "base_path": self.base_path,
            "validation_report": self.validation_report,
            "assets": self.scanned_assets,
            "rules": self.rules,
        });
        
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        std::fs::write(path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_filename() {
        let metadata = CoreAssetRules::parse_filename(
            "hex_grassland_dread0.glb",
            AssetCategory::Generated
        );
        
        assert_eq!(metadata.asset_type, AssetType::HexTile);
        assert_eq!(metadata.dread_level, Some(DreadLevel::Peace));
        assert!(metadata.tags.contains(&"grassland".to_string()));
    }
    
    #[test]
    fn test_extract_dread_level() {
        assert_eq!(
            CoreAssetRules::extract_dread_level("something_dread2_test"),
            Some(DreadLevel::Dread)
        );
        assert_eq!(
            CoreAssetRules::extract_dread_level("horror_variant"),
            Some(DreadLevel::Horror)
        );
        assert_eq!(
            CoreAssetRules::extract_dread_level("normal_asset"),
            None
        );
    }
}
