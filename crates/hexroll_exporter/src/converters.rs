//! HBF file conversion and parsing

use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::hbf_import::HbfParser;

/// HBF converter that reads and parses .hbf files
pub struct HbfConverter {
    hbf_file: PathBuf,
}

impl HbfConverter {
    pub async fn new<P: AsRef<Path>>(hbf_path: P) -> Result<Self> {
        let hbf_file = hbf_path.as_ref().to_path_buf();
        
        if !hbf_file.exists() {
            return Err(anyhow::anyhow!("HBF file not found: {}", hbf_file.display()));
        }
        
        Ok(Self { hbf_file })
    }
    
    /// Extract all entities from HBF file
    pub async fn extract_all_entities(&self) -> Result<crate::transformers::BaseFantasyData> {
        // Use the moved hbf_import functionality
        let parser = HbfParser::new(&self.hbf_file)?;
        let entities = parser.parse_all_entities().await?;
        
        Ok(crate::transformers::BaseFantasyData::from_hbf_entities(entities))
    }
}
