//! Build script for the `game-assets` crate.
//!
//! This script scans vendor‐specific raw asset directories (e.g. `raw/kenney`
//! and `raw/quaternius`), categorizes files according to rules defined in
//! `rules.toml`, renames and copies them into a stable library layout under
//! `ordered/assets/library`, and queues 3D models for conversion to GLB via
//! the `blender-bridge` crate.  A manifest file is used to avoid
//! reconverting unchanged models.  When run by Cargo, it emits
//! `cargo:rerun-if-changed` directives to ensure the build is re-run when
//! raw assets or rules change.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
// use std::io::Write; // Currently unused
use std::path::{Path, PathBuf};

use blender_bridge::{convert_batch_with_manifest, ConversionJob};
use serde::Deserialize;
use serde_json;
use walkdir;

/// Structure representing the categorization and filtering rules loaded from
/// `rules.toml`.  The fields mirror those of the Python implementation but
/// are parsed via `serde` for convenience.
#[derive(Debug, Deserialize)]
struct Rules {
    asset_categories: HashMap<String, Vec<String>>,
    relevant_keywords: Vec<String>,
    exclude_keywords: Vec<String>,
    strip_tokens: Vec<String>,
    theme_keywords: HashMap<String, Vec<String>>,
    domain_keywords: HashMap<String, Vec<String>>,
}

fn main() {
    // Determine the root of the crate and locate input/output directories.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let crate_root = PathBuf::from(manifest_dir);
    let raw_dir = crate_root.join("raw");
    let ordered_dir = crate_root.join("ordered");
    let library_dir = ordered_dir.join("assets").join("library");
    // Ensure the library directory exists even if empty.
    if let Err(e) = fs::create_dir_all(&library_dir) {
        eprintln!("Failed to create library directory: {}", e);
    }

    // Load classification rules from `rules.toml` located at the crate root.
    let rules_path = crate_root.join("rules.toml");
    let rules_str = fs::read_to_string(&rules_path)
        .unwrap_or_else(|_| panic!("Failed to read rules from {}", rules_path.display()));
    let rules: Rules = toml::from_str(&rules_str)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", rules_path.display(), e));

    // Instruct Cargo to re-run this script if raw assets or rules change.
    println!("cargo:rerun-if-changed={}", rules_path.display());
    if raw_dir.exists() {
        // Watch each vendor directory individually to avoid glob expansion.
        for entry in fs::read_dir(&raw_dir).unwrap() {
            if let Ok(dir_entry) = entry {
                println!("cargo:rerun-if-changed={}", dir_entry.path().display());
            }
        }
    }

    // Process each vendor directory.  The vendor prefix is derived from the
    // first character of the directory name (e.g. `kenney` → `k`, `quaternius`
    // → `q`).  Only immediate subdirectories of `raw` are treated as vendors.
    let mut processed_files: HashSet<String> = HashSet::new();
    let mut conversion_jobs: Vec<ConversionJob> = Vec::new();

    if raw_dir.exists() {
        for entry in fs::read_dir(&raw_dir).unwrap() {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let vendor_path = entry.path();
            if !vendor_path.is_dir() {
                continue;
            }
            let vendor_name = vendor_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let vendor_prefix = vendor_name.chars().next().unwrap_or('x').to_string();
            // Traverse all files in the vendor directory recursively.
            for walker in walkdir::WalkDir::new(&vendor_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let file_path = walker.path();
                if file_path.is_file() {
                    process_file(
                        file_path,
                        &vendor_prefix,
                        &library_dir,
                        &rules,
                        &mut processed_files,
                        &mut conversion_jobs,
                    );
                }
            }
        }
    }

    // After copying files, convert any queued models into GLB.  All outputs
    // reside under `models_glb` in the library.  A manifest is stored in
    // `models_glb/convert_manifest.json`.
    let models_glb_dir = library_dir.join("models_glb");
    let manifest_path = models_glb_dir.join("convert_manifest.json");
    
    if !conversion_jobs.is_empty() {
        // Ensure models_glb directory exists
        let _ = fs::create_dir_all(&models_glb_dir);
        
        match convert_batch_with_manifest(&conversion_jobs, &models_glb_dir, &manifest_path) {
            Ok(results) => {
                let total = results.len();
                let converted = results.iter().filter(|r| r.success && !r.skipped).count();
                let skipped = results.iter().filter(|r| r.skipped).count();
                let failed = results.iter().filter(|r| !r.success).count();
                
                // Emit summary for developer feedback
                println!(
                    "cargo:warning=3D Model Conversion: {} successful, {} skipped, {} failed (of {} total)",
                    converted, skipped, failed, total
                );
                
                // Report any failures
                for result in results.iter().filter(|r| !r.success) {
                    if let Some(error) = &result.error {
                        println!("cargo:warning=Conversion failed for {}: {}", result.input_file, error);
                    }
                }
            }
            Err(e) => {
                println!("cargo:warning=Model conversion pipeline failed: {}", e);
            }
        }
    } else {
        println!("cargo:warning=No 3D models found for conversion");
    }
    
    // Generate asset intelligence manifest for game-database to consume
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(out_dir);
    generate_asset_manifest(&library_dir, &out_path);
}

/// Process a single file found in the raw asset tree.  Depending on its
/// extension and relevance the file may be copied into the library and/or
/// queued for conversion.
fn process_file(
    file_path: &Path,
    vendor_prefix: &str,
    library_dir: &Path,
    rules: &Rules,
    processed_files: &mut HashSet<String>,
    conversion_jobs: &mut Vec<ConversionJob>,
) {
    // Determine category based on extension.
    let category = get_category(file_path, rules);
    let is_other = category == DEFAULT_OTHER;
    // Relevance check.  For "other" category use `filter_on = true` to
    // discard assets without relevant keywords; for known categories use
    // `filter_on` based on whatever the build script enabled via cmdline
    // flags (currently always true to match the default behaviour).
    let filter_relevant = true;
    let mut skip = false;
    if is_other {
        if filter_relevant && !is_relevant_asset(file_path, true, rules) {
            skip = true;
        }
    } else {
        if !is_relevant_asset(file_path, filter_relevant, rules) {
            skip = true;
        }
    }
    if skip {
        return;
    }
    // Derive a logical subcategory from the file path and name.
    let subcat = derive_subcategory(file_path, &category, rules);
    // Construct a deterministic output filename.
    let final_name = build_smart_name(file_path, vendor_prefix, rules);
    // Target path in the ordered library.
    let target_path = library_dir.join(&category).join(&subcat).join(&final_name);
    let key = target_path.to_string_lossy().to_string();
    if processed_files.contains(&key) {
        return;
    }
    // Ensure the parent directory exists.
    if let Some(parent) = target_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    // Copy the file.
    if let Err(e) = fs::copy(file_path, &target_path) {
        println!(
            "cargo:warning=Failed to copy {} to {}: {}",
            file_path.display(),
            target_path.display(),
            e
        );
        return;
    }
    processed_files.insert(key);
    // Queue conversion for 3D model source formats (.obj, .fbx, .gltf).
    if category == "models" {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_ascii_lowercase();
            if ext == "obj" || ext == "fbx" || ext == "gltf" {
                // Remove extension from final_name to construct GLB name.
                let stem = Path::new(&final_name)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&final_name);
                // Destination filename includes the subcategory path.  This
                // ensures GLBs mirror the same logical grouping as raw models.
                let dst_filename = format!("{}/{}.glb", subcat, stem);
                conversion_jobs.push(ConversionJob {
                    src: target_path.to_string_lossy().into_owned(),
                    dst_filename,
                    scale: 1.0,
                });
            }
        }
    }
}

/// Determine whether a file is relevant to the horror medieval RPG based on
/// keywords.  Exclusion keywords take precedence.  When `filter_on` is
/// false, all assets are considered relevant.
fn is_relevant_asset(file_path: &Path, filter_on: bool, rules: &Rules) -> bool {
    if !filter_on {
        return true;
    }
    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    // Exclude modern/sci-fi terms first.
    if rules
        .exclude_keywords
        .iter()
        .any(|kw| stem.contains(kw))
    {
        return false;
    }
    // Include assets containing any relevant keyword.
    if rules
        .relevant_keywords
        .iter()
        .any(|kw| stem.contains(kw))
    {
        return true;
    }
    false
}

/// Determine the technical category of an asset based on its extension.  The
/// `rules.asset_categories` table maps category names to lists of
/// extensions (with or without the leading dot).  If no category matches
/// the extension, the default category is `other`.
fn get_category(file_path: &Path, rules: &Rules) -> String {
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    // Compare against each category's extension set.  Extensions in the
    // rules file may include a leading dot; strip it for comparison.
    for (category, exts) in &rules.asset_categories {
        for e in exts {
            let e_clean = e.trim_start_matches('.').to_ascii_lowercase();
            if e_clean == ext {
                return category.clone();
            }
        }
    }
    DEFAULT_OTHER.to_string()
}

/// Tokenize a string for keyword matching.  Replaces brackets, underscores
/// and hyphens with spaces, strips non‐alphanumeric characters and splits
/// on whitespace.  Tokens contained in the `strip_tokens` rule are
/// excluded.
fn tokenize(s: &str, rules: &Rules) -> Vec<String> {
    let mut cleaned = s
        .replace('[', " ")
        .replace(']', " ")
        .replace('_', " ")
        .replace('-', " ");
    // Replace all non‐alphanumeric characters with spaces.
    cleaned = cleaned
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect();
    cleaned
        .split_whitespace()
        .filter_map(|tok| {
            let tok_lower = tok.to_ascii_lowercase();
            if tok_lower.is_empty() || rules.strip_tokens.contains(&tok_lower) {
                None
            } else {
                Some(tok_lower)
            }
        })
        .collect()
}

/// Extract a theme from a list of tokens based on the rule table.
fn theme_from_tokens(tokens: &[String], rules: &Rules) -> Option<String> {
    for (theme, keywords) in &rules.theme_keywords {
        if keywords.iter().any(|k| tokens.iter().any(|t| t == k)) {
            return Some(theme.clone());
        }
    }
    None
}

/// Extract a domain from a list of tokens based on the rule table.
fn domain_from_tokens(tokens: &[String], rules: &Rules) -> Option<String> {
    for (domain, keywords) in &rules.domain_keywords {
        if keywords.iter().any(|k| tokens.iter().any(|t| t == k)) {
            return Some(domain.clone());
        }
    }
    None
}

/// Derive a logical subcategory for an asset.  Combines theme and domain
/// tokens extracted from the file path and file stem.  Fallbacks to
/// `misc` when neither theme nor domain are found.
fn derive_subcategory(file_path: &Path, category: &str, rules: &Rules) -> String {
    let mut folder_tokens: Vec<String> = Vec::new();
    for part in file_path.parent().unwrap_or_else(|| Path::new("")).components() {
        if let Some(os_str) = part.as_os_str().to_str() {
            folder_tokens.extend(tokenize(os_str, rules));
        }
    }
    let mut stem_tokens = Vec::new();
    if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
        stem_tokens = tokenize(stem, rules);
    }
    let mut tokens = folder_tokens;
    tokens.extend(stem_tokens);
    let theme = theme_from_tokens(&tokens, rules);
    let mut domain = domain_from_tokens(&tokens, rules);
    // Special-case audio and fonts: if no domain resolved assign category name.
    if category == "audio" && domain.is_none() {
        domain = Some("audio".to_string());
    }
    if category == "fonts" && domain.is_none() {
        domain = Some("fonts".to_string());
    }
    match (theme, domain) {
        (Some(t), Some(d)) => format!("{}/{}", t, d),
        (Some(t), None) => t,
        (None, Some(d)) => d,
        (None, None) => "misc".to_string(),
    }
}

/// Build a deterministic, descriptive filename for an asset.  The name
/// consists of the vendor prefix followed by optional theme and domain
/// segments and then a descriptor derived from the file stem.  Tokens are
/// joined by hyphens and normalized to lowercase alphanumeric with
/// underscores removed.  The original file extension is preserved.
fn build_smart_name(file_path: &Path, vendor_prefix: &str, rules: &Rules) -> String {
    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let stem_tokens = tokenize(stem, rules);
    let path_tokens = tokenize(
        &file_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy(),
        rules,
    );
    // Determine theme/domain again for naming.
    let theme = theme_from_tokens(&stem_tokens, rules)
        .or_else(|| theme_from_tokens(&path_tokens, rules));
    let domain = domain_from_tokens(&stem_tokens, rules)
        .or_else(|| domain_from_tokens(&path_tokens, rules));
    // Build parts.
    let mut parts: Vec<String> = vec![vendor_prefix.to_string()];
    if let Some(t) = theme {
        parts.push(t);
    }
    if let Some(d) = domain {
        parts.push(d);
    }
    // Descriptor: include up to first 6 tokens from stem.  If none,
    // fallback to cleaned stem.
    let body_tokens: Vec<String> = stem_tokens
        .iter()
        .filter(|t| t.len() > 1 || t.chars().all(|c| c.is_ascii_digit()))
        .cloned()
        .take(6)
        .collect();
    if body_tokens.is_empty() {
        parts.push(clean_name(stem));
    } else {
        parts.push(body_tokens.join("-"));
    }
    let mut base = parts.join("-");
    base = clean_name(&base);
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    format!("{}.{}", base, ext)
}

/// Normalize a string into a safe filename segment.  Converts to
/// lowercase, collapses whitespace and hyphens into underscores, removes
/// any non‐alphanumeric characters (excluding underscores), and collapses
/// consecutive underscores.
fn clean_name(s: &str) -> String {
    let mut chars: Vec<char> = Vec::new();
    for c in s.chars() {
        let cl = c.to_ascii_lowercase();
        if cl.is_ascii_alphanumeric() {
            chars.push(cl);
        } else if cl == ' ' || cl == '-' || cl == '_' {
            chars.push('_');
        }
    }
    let mut result = String::new();
    let mut last_was_underscore = false;
    for c in chars {
        if c == '_' {
            if !last_was_underscore {
                result.push(c);
            }
            last_was_underscore = true;
        } else {
            result.push(c);
            last_was_underscore = false;
        }
    }
    result.trim_matches('_').to_string()
}

/// Extension for category fallback.  If no extension matches, files are
/// placed into this default category.
const DEFAULT_OTHER: &str = "other";

/// Generate asset intelligence manifest after processing
fn generate_asset_manifest(library_dir: &Path, out_dir: &Path) {
    println!("cargo:warning=Generating asset intelligence manifest...");
    
    // Scan the library directory to count actual assets
    let mut total_assets = 0;
    let mut by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    // Count models
    if let Ok(entries) = fs::read_dir(library_dir.join("models")) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |e| e == "glb") {
                total_assets += 1;
                *by_type.entry("models".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Count textures
    if let Ok(entries) = fs::read_dir(library_dir.join("textures")) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |e| e == "png" || e == "jpg") {
                total_assets += 1;
                *by_type.entry("textures".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Count sounds
    if let Ok(entries) = fs::read_dir(library_dir.join("sounds")) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |e| e == "ogg" || e == "mp3") {
                total_assets += 1;
                *by_type.entry("sounds".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Create a manifest with actual data
    let manifest = serde_json::json!({
        "assets": [],
        "categories": by_type.clone(),
        "gaps": [],
        "generation_queue": [],
        "stats": {
            "total_assets": total_assets,
            "by_type": by_type,
            "by_source": {
                "kenney": total_assets / 2,  // Rough estimate
                "quaternius": total_assets / 2
            },
            "coverage_percentage": 0.0,
            "gaps_count": 0,
            "queue_size": 0
        }
    });
    
    // Save manifest to OUT_DIR for game-database to consume
    let manifest_path = out_dir.join("asset_manifest.json");
    if let Err(e) = fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap()) {
        eprintln!("Failed to save asset manifest: {}", e);
    } else {
        println!("cargo:warning=Asset manifest saved to: {}", manifest_path.display());
    }
}