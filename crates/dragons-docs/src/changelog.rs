//! Changelog generation using git-cliff or similar

use anyhow::Result;
use chrono::Utc;
use std::process::Command;

/// Generate a changelog from git history
pub fn generate_changelog() -> Result<String> {
    // Try to use git-cliff if available
    if let Ok(output) = Command::new("git-cliff")
        .arg("--current")
        .arg("--unreleased")
        .output()
    {
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
    }
    
    // Fall back to git log parsing
    generate_from_git_log()
}

/// Generate changelog from raw git log
fn generate_from_git_log() -> Result<String> {
    let mut changelog = String::from("# Changelog\n\n");
    changelog.push_str("All notable changes to Dragon's Labyrinth will be documented in this file.\n\n");
    changelog.push_str("The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\n");
    changelog.push_str("and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n");
    
    // Get recent commits
    if let Ok(output) = Command::new("git")
        .args(&["log", "--oneline", "--pretty=format:%s", "-20"])
        .output()
    {
        if output.status.success() {
            let commits = String::from_utf8_lossy(&output.stdout);
            
            changelog.push_str("## [Unreleased]\n\n");
            
            let mut added = Vec::new();
            let mut changed = Vec::new();
            let mut fixed = Vec::new();
            
            for line in commits.lines() {
                let lower = line.to_lowercase();
                if lower.starts_with("add") || lower.starts_with("feat") {
                    added.push(format!("- {}", line));
                } else if lower.starts_with("fix") {
                    fixed.push(format!("- {}", line));
                } else if lower.starts_with("change") || lower.starts_with("update") {
                    changed.push(format!("- {}", line));
                }
            }
            
            if !added.is_empty() {
                changelog.push_str("### Added\n");
                for item in &added {
                    changelog.push_str(&format!("{}\n", item));
                }
                changelog.push_str("\n");
            }
            
            if !changed.is_empty() {
                changelog.push_str("### Changed\n");
                for item in &changed {
                    changelog.push_str(&format!("{}\n", item));
                }
                changelog.push_str("\n");
            }
            
            if !fixed.is_empty() {
                changelog.push_str("### Fixed\n");
                for item in &fixed {
                    changelog.push_str(&format!("{}\n", item));
                }
                changelog.push_str("\n");
            }
        }
    }
    
    // Add initial release section
    changelog.push_str(&format!(
        "## [0.1.0] - {}\n\n### Initial Release\n",
        Utc::now().format("%Y-%m-%d")
    ));
    changelog.push_str("- Core game engine with Bevy 0.16.1\n");
    changelog.push_str("- Horror progression system (dread levels 0-4)\n");
    changelog.push_str("- 12 companion psychological models\n");
    changelog.push_str("- Hex-based board system\n");
    changelog.push_str("- Forge trial mechanics\n");
    changelog.push_str("- Environmental decay system\n");
    changelog.push_str("- Mount bonding mechanics\n");
    changelog.push_str("- AI-powered content generation pipeline\n");
    
    Ok(changelog)
}
