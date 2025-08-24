//! HBF to Dragon's Labyrinth Converter
//! 
//! This crate analyzes HBF (HexRoll Blob Format) SQLite databases and generates
//! ORM models plus data files for Dragon's Labyrinth horror RPG.

pub mod analyzer;
// TODO: Fix these modules after analysis is working
// pub mod generators;
// pub mod templates; 
// pub mod transformers;

// Re-export key types
pub use analyzer::{HbfAnalyzer, AnalysisReport, TableInfo, ColumnInfo, ForeignKeyInfo};

/// Simple analysis-focused crate
/// The goal is to understand HBF structure to generate proper ORM models
pub struct HbfAnalysisToolkit {
    analyzer: HbfAnalyzer,
}

impl HbfAnalysisToolkit {
    pub fn new(hbf_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let analyzer = HbfAnalyzer::new(hbf_path)?;
        Ok(Self { analyzer })
    }
    
    pub fn analyze(&self, depth: u8) -> anyhow::Result<AnalysisReport> {
        self.analyzer.analyze_structure(depth)
    }
    
    pub fn analyze_and_save(&self, depth: u8, output_path: impl AsRef<std::path::Path>) -> anyhow::Result<AnalysisReport> {
        let report = self.analyzer.analyze_structure(depth)?;
        report.save_report(output_path)?;
        Ok(report)
    }
}
