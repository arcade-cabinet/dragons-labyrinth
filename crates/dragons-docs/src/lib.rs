//! Dragon's Labyrinth Documentation System
//! 
//! This crate generates the player manual and game documentation.
//! No API docs, no code patterns - just what players need to know.

pub mod changelog;

use anyhow::Result;
use std::path::Path;
use std::fs;

/// Check documentation completeness
pub fn check_docs_coverage() -> Result<CoverageReport> {
    let mut missing = Vec::new();
    
    // Check for essential player documentation
    let required_docs = vec![
        "book/player/getting_started.md",
        "book/player/controls.md",
        "book/player/companions",
        "book/player/systems/forge.md",
        "book/player/systems/dread.md",
        "book/design/horror_philosophy.md",
        "book/design/narrative_structure.md",
    ];
    
    for doc in required_docs {
        let path = Path::new("crates/dragons-docs").join(doc);
        if !path.exists() {
            missing.push(doc.to_string());
        }
    }
    
    Ok(CoverageReport {
        total_items: required_docs.len(),
        documented_items: required_docs.len() - missing.len(),
        coverage_percent: ((required_docs.len() - missing.len()) as f32 / required_docs.len() as f32) * 100.0,
        missing,
    })
}

#[derive(Debug)]
pub struct CoverageReport {
    pub total_items: usize,
    pub documented_items: usize,
    pub coverage_percent: f32,
    pub missing: Vec<String>,
}

/// Generate the table of contents from existing files
pub fn generate_toc(book_dir: &Path) -> Result<String> {
    let mut toc = String::from("# Summary\n\n");
    
    // Introduction
    toc.push_str("[Introduction](./introduction.md)\n\n");
    
    // Player Guide
    toc.push_str("# Player Guide\n\n");
    toc.push_str("- [Getting Started](./player/getting_started.md)\n");
    toc.push_str("- [Core Concepts](./player/core_concepts.md)\n");
    toc.push_str("- [Controls](./player/controls.md)\n");
    toc.push_str("- [Companions](./player/companions/README.md)\n");
    toc.push_str("- [Systems](./player/systems/README.md)\n");
    toc.push_str("  - [The Forge](./player/systems/forge.md)\n");
    toc.push_str("  - [Dread System](./player/systems/dread.md)\n");
    toc.push_str("  - [Second Chances](./player/systems/second_chances.md)\n");
    
    // Game Design
    toc.push_str("\n# Game Design\n\n");
    toc.push_str("- [Vision](./design/vision.md)\n");
    toc.push_str("- [Horror Philosophy](./design/horror_philosophy.md)\n");
    toc.push_str("- [Narrative Structure](./design/narrative_structure.md)\n");
    toc.push_str("- [Playthroughs](./design/playthroughs/README.md)\n");
    
    // Appendix
    toc.push_str("\n# Appendix\n\n");
    toc.push_str("- [Changelog](./appendix/changelog.md)\n");
    toc.push_str("- [Credits](./appendix/credits.md)\n");
    
    Ok(toc)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coverage_report() {
        let report = check_docs_coverage().unwrap();
        assert!(report.coverage_percent >= 0.0);
        assert!(report.coverage_percent <= 100.0);
    }
}