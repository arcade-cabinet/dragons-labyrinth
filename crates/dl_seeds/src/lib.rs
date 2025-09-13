//! Seeds crate for Dragon's Labyrinth - Comprehensive Seeding System
//! 
//! This crate provides the unified seeding system that performs ALL HBF data processing,
//! orchestration, analytics, AI analysis, and organized JSON pool generation at build time.
//! 
//! ## Consolidated Architecture
//! 
//! This crate combines functionality from:
//! - TOML sampling and AI/ML processing (original dl_seeds)
//! - Entity orchestration and clustering (from dl_analysis)
//! - ECS code generation (from dl_processors)  
//! - DataFrame auditing (from dl_audit)
//! 
//! ## Build-time Processing
//! - Samples representative HBF data from database
//! - Performs AI analysis and transformation
//! - Generates organized JSON data pools
//! - Creates ECS components and systems
//! 
//! ## Runtime Support
//! - Provides runtime AI analysis engine for organized pools
//! - Supports dynamic seed generation during gameplay

// Core seeding modules (original functionality)
pub mod books;
pub mod entities;
pub mod ai_client;
pub mod regions;
pub mod settlements;
pub mod dungeons;
pub mod factions;

// Consolidated functionality modules (from other crates)
pub mod analysis;      // From dl_analysis/src/ai_analysis.rs
pub mod containers;    // From dl_analysis/src/containers.rs
pub mod templates;     // From dl_processors/src/templates.rs
pub mod orchestration; // Enhanced orchestration (already exists)

// New runtime analysis functionality
pub mod runtime_analysis;
pub mod data_pools;

// Consolidated modules from dl_types
pub mod audit;         // From dl_types/src/audit.rs
pub mod dialogue;      // From dl_types/src/seeds/dialogue.rs
pub mod linguistics;   // From dl_types/src/seeds/linguistics.rs
pub mod components;    // From dl_types/src/processing/components.rs

use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;

// Re-export consolidated types
pub use analysis::AiAnalysisClient;
pub use containers::{HexContainer, DungeonContainer, ClusteringContainer};
pub use templates::TemplateManager;
pub use runtime_analysis::SeedAnalysisEngine;
pub use data_pools::CategorizedDataPools;

/// Comprehensive seeding system that combines orchestration, analysis, and generation
pub struct ComprehensiveSeeder {
    pub books: books::BooksManager,
    pub ai_client: analysis::AiAnalysisClient,
    pub template_manager: templates::TemplateManager,
    pub hex_container: containers::HexContainer,
    pub clustering_container: containers::ClusteringContainer,
}

impl ComprehensiveSeeder {
    /// Initialize the complete seeding system
    pub async fn new() -> Result<Self> {
        println!("Initializing comprehensive Dragon's Labyrinth seeding system...");
        
        let ai_client = analysis::AiAnalysisClient::new()?;
        let template_manager = templates::TemplateManager::new()?;
        
        Ok(Self {
            books: books::BooksManager::new(),
            ai_client,
            template_manager,
            hex_container: containers::HexContainer::new(),
            clustering_container: containers::ClusteringContainer::new(),
        })
    }

    /// Run complete build-time processing pipeline
    pub async fn run_build_pipeline(&mut self, out_dir: &Path) -> Result<()> {
        println!("Running comprehensive build pipeline...");
        
        // Stage 1: Generate seeds from texts
        self.books = books::BooksManager::generate_seeds_from_texts(out_dir)?;
        
        // Stage 2: Run orchestration and clustering
        // (Implementation will be enhanced in Phase 2)
        
        // Stage 3: Perform AI analysis
        // (Implementation will be enhanced in Phase 3)
        
        // Stage 4: Generate organized pools
        self.generate_organized_pools(out_dir).await?;
        
        // Stage 5: Generate ECS code
        // (Implementation will be enhanced in Phase 3)
        
        println!("Build pipeline completed successfully");
        Ok(())
    }

    /// Generate organized JSON data pools for runtime consumption
    pub async fn generate_organized_pools(&mut self, out_dir: &Path) -> Result<()> {
        println!("Generating organized JSON data pools...");
        
        let pools_dir = out_dir.join("organized_pools");
        std::fs::create_dir_all(&pools_dir)?;
        
        // Create categorized pools from seeding data
        let pools = data_pools::CategorizedDataPools::from_seeds(
            &self.books.world_seeds,
            &self.books.quest_seeds,
            &self.books.dialogue_seeds,
        );
        
        // Save organized pools to disk
        pools.save_to_dir(&pools_dir)?;
        
        println!("Organized pools generated in: {}", pools_dir.display());
        Ok(())
    }
}

/// Legacy SeedsManager for backwards compatibility
pub struct SeedsManager {
    comprehensive: ComprehensiveSeeder,
}

impl SeedsManager {
    /// Generate all seeds from TOML samples using AI transformation
    pub async fn generate_from_toml(out_dir: &Path) -> Result<Self> {
        println!("Generating Dragon's Labyrinth seeds from TOML samples...");
        
        let mut comprehensive = ComprehensiveSeeder::new().await?;
        comprehensive.run_build_pipeline(out_dir).await?;
        
        Ok(Self { comprehensive })
    }
    
    /// Get world seeds from literature
    pub fn get_world_seeds(&self) -> &[books::WorldSeed] {
        &self.comprehensive.books.world_seeds
    }
    
    /// Get quest seeds from literature  
    pub fn get_quest_seeds(&self) -> &[books::QuestSeed] {
        &self.comprehensive.books.quest_seeds
    }
    
    /// Get dialogue seeds from literature
    pub fn get_dialogue_seeds(&self) -> &[books::DialogueSeed] {
        &self.comprehensive.books.dialogue_seeds
    }
}

/// Re-export key types for external usage
pub use books::{BooksManager, BookRecord, WorldSeed, QuestSeed, DialogueSeed};
pub use entities::*;
pub use regions::*;
pub use settlements::*;
pub use dungeons::*;
pub use factions::*;
