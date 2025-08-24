//! ORM model and data file generators using minijinja2 templates

use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::transformers::{DragonSettlement, DragonHexTile, DragonNpc, DragonDungeon, DragonItem, DragonFaction};
use crate::templates::TemplateEngine;

/// ORM model generator using templates
pub struct OrmModelGenerator {
    output_dir: PathBuf,
    template_engine: TemplateEngine,
}

impl OrmModelGenerator {
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            template_engine: TemplateEngine::new(),
        }
    }
    
    pub async fn generate_settlement_model(&self, settlements: &[DragonSettlement]) -> Result<()> {
        let output_path = self.output_dir.join("settlements.rs");
        self.template_engine.generate_model("settlement", settlements, &output_path).await
    }
    
    pub async fn generate_hex_tile_model(&self, hex_tiles: &[DragonHexTile]) -> Result<()> {
        let output_path = self.output_dir.join("hex_tiles.rs");
        self.template_engine.generate_model("hex_tile", hex_tiles, &output_path).await
    }
    
    pub async fn generate_npc_model(&self, npcs: &[DragonNpc]) -> Result<()> {
        let output_path = self.output_dir.join("npcs.rs");
        self.template_engine.generate_model("npc", npcs, &output_path).await
    }
    
    pub async fn generate_dungeon_model(&self, dungeons: &[DragonDungeon]) -> Result<()> {
        let output_path = self.output_dir.join("dungeons.rs");
        self.template_engine.generate_model("dungeon", dungeons, &output_path).await
    }
    
    pub async fn generate_item_model(&self, items: &[DragonItem]) -> Result<()> {
        let output_path = self.output_dir.join("items.rs");
        self.template_engine.generate_model("item", items, &output_path).await
    }
    
    pub async fn generate_faction_model(&self, factions: &[DragonFaction]) -> Result<()> {
        let output_path = self.output_dir.join("factions.rs");
        self.template_engine.generate_model("faction", factions, &output_path).await
    }
    
    pub async fn generate_models_mod(&self) -> Result<()> {
        let output_path = self.output_dir.join("mod.rs");
        self.template_engine.generate_models_module(&output_path).await
    }
}

/// Data file generator for loading actual entity data
pub struct DataFileGenerator {
    output_dir: PathBuf,
}

impl DataFileGenerator {
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }
    
    pub async fn generate_settlement_data(&self, settlements: &[DragonSettlement]) -> Result<()> {
        let output_path = self.output_dir.join("settlement_data.json");
        let json = serde_json::to_string_pretty(settlements)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
    
    pub async fn generate_hex_tile_data(&self, hex_tiles: &[DragonHexTile]) -> Result<()> {
        let output_path = self.output_dir.join("hex_tile_data.json");
        let json = serde_json::to_string_pretty(hex_tiles)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
    
    pub async fn generate_npc_data(&self, npcs: &[DragonNpc]) -> Result<()> {
        let output_path = self.output_dir.join("npc_data.json");
        let json = serde_json::to_string_pretty(npcs)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
    
    pub async fn generate_dungeon_data(&self, dungeons: &[DragonDungeon]) -> Result<()> {
        let output_path = self.output_dir.join("dungeon_data.json");
        let json = serde_json::to_string_pretty(dungeons)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
    
    pub async fn generate_item_data(&self, items: &[DragonItem]) -> Result<()> {
        let output_path = self.output_dir.join("item_data.json");
        let json = serde_json::to_string_pretty(items)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
    
    pub async fn generate_faction_data(&self, factions: &[DragonFaction]) -> Result<()> {
        let output_path = self.output_dir.join("faction_data.json");
        let json = serde_json::to_string_pretty(factions)?;
        std::fs::write(&output_path, json)?;
        Ok(())
    }
}
