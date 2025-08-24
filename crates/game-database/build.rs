//! Build script for Dragon's Labyrinth Database
//! 
//! Creates SQLite database at build time using database-orm models.
//! Database is created in OUT_DIR and ships with release.
//! Runtime uses bevy_sqlx for ECS integration.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
// std::time imports no longer needed - using chrono::DateTime

use sea_orm::{Database, DatabaseConnection, Set};
use walkdir::WalkDir;

use database_orm::{
    self,
    sea_orm::{self, ActiveModelTrait},
    assets::{self, attribution},
    ai_workflows, generated_assets,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use game_assets::{AssetManifest, AssetType as ManifestAssetType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../assets");
    println!("cargo:rerun-if-changed=../database-orm/src");
    println!("cargo:rerun-if-changed=../game-assets/ordered/assets");
    
    // Get OUT_DIR for database creation
    let out_dir = env::var("OUT_DIR")?;
    let db_path = PathBuf::from(&out_dir).join("game_database.db");
    
    // Get project root and connect to game-assets output
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let project_root = PathBuf::from(&manifest_dir).parent().unwrap().parent().unwrap().to_path_buf();
    let assets_dir = project_root.join("crates/game-assets/ordered/assets");
    
    // Try to load asset manifest from game-assets build output
    let game_assets_out = project_root.join("target/debug/build");
    let asset_manifest = find_and_load_asset_manifest(&game_assets_out);
    
    // Create database connection using SeaORM
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let db = Database::connect(&db_url).await?;
    
    // Create all tables using database-orm with proper error handling
    match database_orm::create_all_tables(&db).await {
        Ok(_) => println!("All tables created successfully!"),
        Err(e) => {
            // Tables might already exist - this is OK for build scripts
            println!("Table creation result: {} (tables may already exist)", e);
        }
    }
    
    // If we have an asset manifest, use AI-enriched data
    if let Some(manifest) = asset_manifest {
        process_asset_manifest(&db, &manifest).await?;
        println!("cargo:warning=Processed AI-enriched asset manifest with {} assets and {} gaps", 
            manifest.assets.len(), manifest.gaps.len());
        
        // Create generation queue in database from manifest
        create_generation_queue(&db, &manifest).await?;
        println!("cargo:warning=Created generation queue with {} prompts", manifest.generation_queue.len());
    } else {
        // Fall back to simple file scanning
        if assets_dir.exists() {
            scan_assets_with_orm(&db, &assets_dir).await?;
            println!("cargo:warning=Indexed assets from game-assets library: {}", assets_dir.display());
        } else {
            println!("cargo:warning=Game-assets directory not found, creating empty database");
        }
    }
    
    // Close database connection
    db.close().await?;
    
    // Export database path for runtime
    println!("cargo:rustc-env=GAME_DATABASE_PATH={}", db_path.display());
    println!("Game database created successfully with asset indexing using database-orm");
    
    Ok(())
}

/// Scan assets directory and insert into database using SeaORM
async fn scan_assets_with_orm(db: &DatabaseConnection, assets_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut stats = ScanStats::default();
    
    // Scan each category directory
    for category in &["core", "library", "generated"] {
        let category_path = assets_dir.join(category);
        if !category_path.exists() {
            continue;
        }
        
        scan_category_with_orm(db, &category_path, category, &mut stats).await?;
    }
    
    println!("Asset scan complete: {} scanned, {} added", stats.scanned, stats.added);
    Ok(())
}

/// Scan a category directory using SeaORM
async fn scan_category_with_orm(
    db: &DatabaseConnection,
    category_path: &Path,
    category: &str,
    stats: &mut ScanStats,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(category_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }
        
        // Skip non-asset files
        if !is_asset_file(path) {
            continue;
        }
        
        stats.scanned += 1;
        
        // Create asset record using SeaORM
        let asset_data = create_asset_data(path, category)?;
        
        // Insert asset using SeaORM ActiveModel with proper types
        let asset_model = assets::ActiveModel {
            id: Set(asset_data.id),
            path: Set(asset_data.path.clone()),
            category: Set(asset_data.category),
            asset_type: Set(asset_data.asset_type),
            filename: Set(asset_data.filename.clone()),
            display_name: Set(asset_data.display_name),
            tags: Set(asset_data.tags),
            file_size_bytes: Set(asset_data.file_size_bytes),
            dread_level: Set(asset_data.dread_level),
            mobile_compatible: Set(asset_data.mobile_compatible),
            performance_score: Set(asset_data.performance_score),
            last_modified: Set(asset_data.last_modified),
            indexed_at: Set(Utc::now()),
        };
        
        // Insert asset with conflict handling (ignore duplicates based on unique path constraint)
        match asset_model.insert(db).await {
            Ok(_) => {
                // Asset inserted successfully
            },
            Err(e) => {
                // Check if this is a unique constraint error, which is fine for build scripts
                let error_str = e.to_string();
                if error_str.contains("UNIQUE constraint failed") || error_str.contains("assets.path") {
                    // Asset already exists - skip silently
                    continue;
                } else {
                    // Some other error - propagate it
                    return Err(e.into());
                }
            }
        }
        
        // Create attribution record if available
        if let Some(attribution_data) = extract_attribution_info(path, &asset_data.filename) {
            let attribution_model = attribution::ActiveModel {
                id: Set(attribution_data.id),
                asset_id: Set(asset_data.id),
                source_library: Set(attribution_data.source_library),
                vendor_prefix: Set(attribution_data.vendor_prefix),
                original_filename: Set(attribution_data.original_filename),
                original_path: Set(attribution_data.original_path),
                license_type: Set(attribution_data.license_type),
                attribution_required: Set(attribution_data.attribution_required),
                attribution_text: Set(attribution_data.attribution_text),
                source_url: Set(attribution_data.source_url),
                converted_from: Set(attribution_data.converted_from),
                conversion_manifest_path: Set(attribution_data.conversion_manifest_path),
                indexed_at: Set(Utc::now()),
            };
            
            // Insert attribution with conflict handling
            match attribution_model.insert(db).await {
                Ok(_) => {
                    // Attribution inserted successfully
                },
                Err(e) => {
                    // Check if this is a unique constraint error
                    let error_str = e.to_string();
                    if error_str.contains("UNIQUE constraint failed") {
                        // Attribution already exists - skip silently 
                    } else {
                        // Some other error - log but continue (attribution is optional)
                        eprintln!("Warning: Failed to insert attribution for {}: {}", asset_data.path, e);
                    }
                }
            }
        }
        
        stats.added += 1;
    }
    
    Ok(())
}

/// Asset data structure for creation - using proper SeaORM types
#[derive(Debug)]
struct AssetData {
    id: Uuid,
    path: String,
    category: String,
    asset_type: String,
    filename: String,
    display_name: String,
    tags: String,
    file_size_bytes: i64,
    dread_level: Option<i32>,
    mobile_compatible: bool,
    performance_score: f32,
    last_modified: DateTime<Utc>,
}

/// Attribution information structure - using proper SeaORM types
#[derive(Debug)]
struct AttributionData {
    id: Uuid,
    source_library: String,
    vendor_prefix: String,
    original_filename: Option<String>,
    original_path: Option<String>,
    license_type: String,
    attribution_required: bool,
    attribution_text: Option<String>,
    source_url: Option<String>,
    converted_from: Option<String>,
    conversion_manifest_path: Option<String>,
}

fn create_asset_data(path: &Path, category: &str) -> Result<AssetData, Box<dyn std::error::Error>> {
    let filename = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    // Generate unique ID using UUID
    let id = Uuid::new_v4();
    
    // Extract metadata
    let metadata = fs::metadata(path)?;
    let file_size_bytes = metadata.len() as i64;
    let last_modified = DateTime::<Utc>::from(metadata.modified()?);
    
    // Determine asset type
    let asset_type = determine_asset_type(path);
    
    // Extract display name
    let display_name = stem
        .replace('_', " ")
        .replace('-', " ");
    
    // Extract tags
    let tags = extract_tags(stem);
    
    // Extract dread level if present
    let dread_level = extract_dread_level(stem);
    
    // Calculate mobile compatibility
    let mobile_compatible = file_size_bytes < 5 * 1024 * 1024; // 5MB limit
    
    // Calculate performance score
    let performance_score = calculate_performance_score(file_size_bytes, &asset_type);
    
    Ok(AssetData {
        id,
        path: path.to_string_lossy().to_string(),
        category: category.to_string(),
        asset_type,
        filename,
        display_name,
        tags,
        file_size_bytes,
        dread_level,
        mobile_compatible,
        performance_score,
        last_modified,
    })
}

fn extract_attribution_info(path: &Path, filename: &str) -> Option<AttributionData> {
    // Extract vendor prefix from filename (e.g., "k_" for Kenney, "q_" for Quaternius)
    let vendor_prefix = if let Some(first_char) = filename.chars().next() {
        if filename.chars().nth(1) == Some('_') {
            first_char.to_string()
        } else {
            return None; // Not a vendored asset
        }
    } else {
        return None;
    };
    
    // Map vendor prefix to library information
    let (source_library, source_url) = match vendor_prefix.as_str() {
        "k" => ("Kenney CC0 Assets", Some("https://www.kenney.nl/assets".to_string())),
        "q" => ("Quaternius CC0 Assets", Some("https://quaternius.com".to_string())),
        _ => return None, // Unknown vendor
    };
    
    // Check if this is a converted GLB file
    let converted_from = if path.extension().and_then(|e| e.to_str()) == Some("glb") {
        // Try to find the original extension in the stem
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem.contains("_obj") || stem.contains("_fbx") || stem.contains("_gltf") {
            Some("3D model conversion".to_string())
        } else {
            None
        }
    } else {
        None
    };
    
    Some(AttributionData {
        id: Uuid::new_v4(),
        source_library: source_library.to_string(),
        vendor_prefix,
        original_filename: None, // Could be extracted from path analysis
        original_path: None, // Could be stored during processing
        license_type: "CC0".to_string(),
        attribution_required: false, // CC0 doesn't require attribution
        attribution_text: None, // CC0 doesn't require attribution text
        source_url,
        converted_from,
        conversion_manifest_path: None, // Could reference blender-bridge manifest
    })
}

fn is_asset_file(path: &Path) -> bool {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    matches!(
        ext.as_str(),
        // 3D Models
        "glb" | "gltf" | "fbx" | "obj" | "dae" |
        // Images
        "png" | "jpg" | "jpeg" | "tga" | "bmp" |
        // Audio
        "ogg" | "mp3" | "wav" | "flac" |
        // Video
        "mp4" | "webm" | "avi" | "mov" |
        // Fonts
        "ttf" | "otf" | "woff" | "woff2" |
        // Other
        "yarn"
    )
}

fn determine_asset_type(path: &Path) -> String {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match ext.as_str() {
        "glb" | "gltf" | "fbx" | "obj" | "dae" => "model",
        "png" | "jpg" | "jpeg" | "tga" | "bmp" => "texture",
        "ogg" | "mp3" | "wav" | "flac" => "audio",
        "mp4" | "webm" | "avi" | "mov" => "video",
        "ttf" | "otf" | "woff" | "woff2" => "font",
        "yarn" => "dialogue",
        _ => "other"
    }.to_string()
}

fn extract_tags(name: &str) -> String {
    let mut tags = Vec::new();
    let name_lower = name.to_lowercase();
    
    // Horror tags
    if name_lower.contains("corrupt") { tags.push("corruption"); }
    if name_lower.contains("dark") { tags.push("dark"); }
    if name_lower.contains("horror") { tags.push("horror"); }
    if name_lower.contains("nightmare") { tags.push("nightmare"); }
    
    // Biome tags
    for biome in &["grassland", "forest", "swamp", "mountain", "dungeon", "village", "ruins"] {
        if name_lower.contains(biome) {
            tags.push(biome);
        }
    }
    
    // Companion tags
    for companion in &["einar", "mira", "sorin", "tamara"] {
        if name_lower.contains(companion) {
            tags.push(companion);
        }
    }
    
    // Special tags
    if name_lower.contains("hex") { tags.push("hex"); }
    if name_lower.contains("tile") { tags.push("tile"); }
    if name_lower.contains("companion") { tags.push("companion"); }
    
    serde_json::to_string(&tags).unwrap_or_else(|_| "[]".to_string())
}

fn extract_dread_level(name: &str) -> Option<i32> {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("dread4") || name_lower.contains("horror") {
        Some(4)
    } else if name_lower.contains("dread3") || name_lower.contains("terror") {
        Some(3)
    } else if name_lower.contains("dread2") || name_lower.contains("dread") {
        Some(2)
    } else if name_lower.contains("dread1") || name_lower.contains("unease") {
        Some(1)
    } else if name_lower.contains("dread0") || name_lower.contains("peace") {
        Some(0)
    } else {
        None
    }
}

fn calculate_performance_score(file_size: i64, asset_type: &str) -> f32 {
    let mut score: f32 = 1.0;
    
    // Penalize large files
    let size_mb = file_size as f32 / (1024.0 * 1024.0);
    if size_mb > 5.0 {
        score -= 0.3;
    } else if size_mb > 2.0 {
        score -= 0.1;
    }
    
    // Bonus for optimized types
    if asset_type == "texture" && size_mb < 1.0 {
        score += 0.1;
    }
    
    score.clamp(0.0, 1.0)
}

#[derive(Default)]
struct ScanStats {
    scanned: i32,
    added: i32,
}

/// Find and load asset manifest from game-assets build output
fn find_and_load_asset_manifest(build_dir: &Path) -> Option<AssetManifest> {
    // Search for the most recent game-assets build directory
    if !build_dir.exists() {
        return None;
    }
    
    // Look for asset_manifest.json in any game-assets-* directory
    for entry in fs::read_dir(build_dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()?.to_str()?.starts_with("game-assets-") {
            let manifest_path = path.join("out/asset_manifest.json");
            if manifest_path.exists() {
                println!("cargo:warning=Found asset manifest at: {}", manifest_path.display());
                
                // Load and parse the manifest
                let json = fs::read_to_string(&manifest_path).ok()?;
                let manifest: AssetManifest = serde_json::from_str(&json).ok()?;
                
                return Some(manifest);
            }
        }
    }
    
    None
}

/// Process AI-enriched asset manifest into database
async fn process_asset_manifest(
    db: &DatabaseConnection,
    manifest: &AssetManifest,
) -> Result<(), Box<dyn std::error::Error>> {
    for asset_entry in &manifest.assets {
        // Convert manifest asset type to string
        let asset_type = match &asset_entry.asset_type {
            ManifestAssetType::Model3D { .. } => "model",
            ManifestAssetType::Texture { .. } => "texture",
            ManifestAssetType::Font { .. } => "font",
            ManifestAssetType::Audio { .. } => "audio",
            ManifestAssetType::Animation { .. } => "animation",
        }.to_string();
        
        // Convert tags to JSON string
        let tags = serde_json::to_string(&asset_entry.tags)?;
        
        // Create asset record with AI-enriched data
        let asset_model = assets::ActiveModel {
            id: Set(Uuid::new_v4()),
            path: Set(asset_entry.path.to_string_lossy().to_string()),
            category: Set(asset_entry.semantic_category.clone().unwrap_or_else(|| "uncategorized".to_string())),
            asset_type: Set(asset_type),
            filename: Set(asset_entry.path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()),
            display_name: Set(asset_entry.path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()),
            tags: Set(tags),
            file_size_bytes: Set(0), // Would need to read from filesystem
            dread_level: Set(asset_entry.dread_compatibility.first().map(|&d| d as i32)),
            mobile_compatible: Set(true), // Could be calculated
            performance_score: Set(asset_entry.style_match_score),
            last_modified: Set(Utc::now()),
            indexed_at: Set(Utc::now()),
        };
        
        // Insert with conflict handling
        match asset_model.insert(db).await {
            Ok(_) => {},
            Err(e) if e.to_string().contains("UNIQUE") => {
                // Asset already exists - OK
            },
            Err(e) => return Err(e.into()),
        }
    }
    
    Ok(())
}

/// Create generation queue in database from manifest gaps and prompts
async fn create_generation_queue(
    db: &DatabaseConnection,
    manifest: &AssetManifest,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create AI workflow for asset generation
    let workflow_id = Uuid::new_v4();
    let workflow = ai_workflows::ActiveModel {
        id: Set(workflow_id),
        workflow_type: Set("asset_generation".to_string()),
        workflow_name: Set("Asset Manifest Processing".to_string()),
        agent_name: Set("AssetIntelligence".to_string()),
        input_parameters: Set(serde_json::json!({
            "gaps_count": manifest.gaps.len(),
            "coverage_percentage": manifest.stats.coverage_percentage,
        })),
        generation_prompts: Set(serde_json::json!([])),
        target_dread_level: Set(0),
        content_category: Set(Some("assets".to_string())),
        status: Set("pending".to_string()),
        progress_percentage: Set(0.0),
        priority_level: Set(5),
        created_at: Set(Utc::now()),
        started_at: Set(None),
        completed_at: Set(None),
        estimated_completion: Set(None),
        api_calls_made: Set(0),
        tokens_consumed: Set(0),
        processing_time_seconds: Set(0.0),
        api_provider: Set(None),
        model_used: Set(None),
        generated_assets: Set(None),
        output_metadata: Set(None),
        output_file_count: Set(0),
        output_total_size_bytes: Set(0),
        quality_score: Set(None),
        requires_human_review: Set(false),
        human_approved: Set(None),
        review_notes: Set(None),
        error_log: Set(None),
        debug_information: Set(None),
        failure_reason: Set(None),
        retry_count: Set(0),
        max_retries: Set(3),
        depends_on_workflows: Set(None),
        triggers_workflows: Set(None),
        is_blocking: Set(false),
        cache_key: Set(None),
        cache_hit: Set(false),
        content_hash: Set(None),
        build_target: Set(None),
        output_directory: Set(None),
        integrated_with_build: Set(false),
        cpu_usage_peak: Set(None),
        memory_usage_peak_mb: Set(None),
        network_requests_made: Set(0),
        performance_metrics: Set(None),
    };
    
    workflow.insert(db).await?;
    
    // Create generation tasks for each prompt
    for (idx, prompt) in manifest.generation_queue.iter().enumerate() {
        let asset = generated_assets::ActiveModel {
            id: Set(Uuid::new_v4()),
            ai_workflow_id: Set(Some(workflow_id)),
            asset_name: Set(format!("Generated_{}", prompt.id)),
            asset_type: Set(format!("{:?}", prompt.prompt_type)),
            file_format: Set("pending".to_string()),
            file_path: Set("pending".to_string()),
            content_hash: Set("pending".to_string()),
            target_dread_level: Set(prompt.target_specs.dread_levels.first().copied().unwrap_or(0) as i32),
            generation_agent: Set("pending".to_string()),
            generation_prompts: Set(serde_json::json!([prompt.prompt_text])),
            generation_parameters: Set(serde_json::json!({
                "target_specs": prompt.target_specs,
                "dependencies": prompt.dependencies,
            })),
            api_model_used: Set(None),
            file_size_bytes: Set(0),
            creation_timestamp: Set(Utc::now()),
            last_modified: Set(Utc::now()),
            quality_score: Set(None),
            horror_intensity: Set(None),
            emotional_impact: Set(None),
            content_description: Set(Some(prompt.prompt_text.clone())),
            content_tags: Set(None),
            type_specific_metadata: Set(None),
            audio_duration_seconds: Set(None),
            audio_sample_rate: Set(None),
            audio_bitrate: Set(None),
            image_width: Set(None),
            image_height: Set(None),
            image_color_depth: Set(None),
            model_vertex_count: Set(None),
            model_face_count: Set(None),
            model_material_count: Set(None),
            is_integrated: Set(false),
            integration_status: Set("pending".to_string()),
            usage_count: Set(0),
            referenced_by: Set(None),
            version_number: Set(1),
            is_current_version: Set(true),
            parent_asset_id: Set(None),
            human_reviewed: Set(false),
            human_approved: Set(None),
            review_notes: Set(None),
            automated_tests_passed: Set(false),
            validation_results: Set(None),
            license_type: Set("AI_Generated".to_string()),
            attribution_required: Set(None),
            source_inspiration: Set(None),
            compression_applied: Set(false),
            original_file_size_bytes: Set(None),
            compression_ratio: Set(None),
            build_target: Set(None),
            included_in_build: Set(false),
            build_priority: Set(5),
            depends_on_assets: Set(None),
            required_by_assets: Set(None),
            cache_key: Set(None),
            preload_recommended: Set(false),
            memory_usage_mb: Set(None),
        };
        
        asset.insert(db).await?;
    }
    
    Ok(())
}