//! Test HBF entity extraction efficiency
//!
//! This test verifies that the new orchestration system can extract
//! significantly more entities than the previous 331 count, targeting 600+.

use std::path::PathBuf;
use anyhow::Result;
use dl_analysis::orchestration::RawEntities;
use dl_analysis::clusters::EntityCluster;

fn main() -> Result<()> {
    // Look for HBF database in current directory
    let hbf_path = PathBuf::from("game.hbf");
    
    if !hbf_path.exists() {
        println!("❌ HBF database not found at: {}", hbf_path.display());
        println!("Please ensure game.hbf is in the current directory");
        return Ok(());
    }

    println!("🔍 Testing HBF entity extraction efficiency...");
    println!("📂 HBF database: {}", hbf_path.display());
    println!("");

    // Create RawEntities container
    let mut entities = RawEntities::new();

    // Initialize audit system
    let audit_system = std::env::var("AUDIT_REPORTS_DIR")
        .ok()
        .map(|dir| dl_audit::AuditSystem::new(dir));

    // Load all entities from HBF database
    println!("⚡ Loading entities from HBF database...");
    entities.load_from_hbf_database(&hbf_path, audit_system.as_ref())?;

    // Generate analysis summary
    let summary = entities.get_analysis_summary(audit_system.as_ref());
    
    println!("📊 EXTRACTION RESULTS:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 Total entities extracted: {}", entities.total_entities);
    println!("📂 Regions: {} clusters", entities.regions.len());
    println!("🏘️  Settlements: {} clusters", entities.settlements.len()); 
    println!("⚔️  Factions: {} clusters", entities.factions.len());
    println!("🏰 Dungeons: {} clusters", entities.dungeons.len());
    println!("❓ Uncategorized: {} entities", entities.uncategorized.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("");

    // Efficiency assessment
    let target_count = 600;
    let efficiency_percentage = (entities.total_entities as f32 / target_count as f32) * 100.0;
    
    if entities.total_entities >= target_count {
        println!("✅ SUCCESS: Extracted {} entities (target: {})", entities.total_entities, target_count);
        println!("🎯 Efficiency: {:.1}% of target achieved!", efficiency_percentage);
    } else {
        println!("⚠️  PARTIAL: Extracted {} entities (target: {})", entities.total_entities, target_count);
        println!("📊 Efficiency: {:.1}% of target achieved", efficiency_percentage);
        println!("💡 Need to improve entity extraction or categorization");
    }
    
    println!("");
    println!("🔍 DETAILED BREAKDOWN:");
    
    // Show detailed counts for each category - handle each type separately
    
    // Regions
    if !entities.regions.is_empty() {
        println!("  📂 Regions:");
        let mut total_entities = 0;
        for (name, cluster) in &entities.regions {
            let count = cluster.base.entities.len();
            total_entities += count;
            if count > 0 {
                println!("    {} {} entities: {}", 
                       if cluster.can_generate_models() { "✓" } else { "○" }, 
                       name, count);
            }
        }
        println!("    └─ Total regions entities: {}", total_entities);
    }
    
    // Settlements  
    if !entities.settlements.is_empty() {
        println!("  🏘️  Settlements:");
        let mut total_entities = 0;
        for (name, cluster) in &entities.settlements {
            let count = cluster.base.entities.len();
            total_entities += count;
            if count > 0 {
                println!("    {} {} entities: {}", 
                       if cluster.can_generate_models() { "✓" } else { "○" }, 
                       name, count);
            }
        }
        println!("    └─ Total settlements entities: {}", total_entities);
    }
    
    // Factions
    if !entities.factions.is_empty() {
        println!("  ⚔️  Factions:");
        let mut total_entities = 0;
        for (name, cluster) in &entities.factions {
            let count = cluster.base.entities.len();
            total_entities += count;
            if count > 0 {
                println!("    {} {} entities: {}", 
                       if cluster.can_generate_models() { "✓" } else { "○" }, 
                       name, count);
            }
        }
        println!("    └─ Total factions entities: {}", total_entities);
    }
    
    // Dungeons
    if !entities.dungeons.is_empty() {
        println!("  🏰 Dungeons:");
        let mut total_entities = 0;
        for (name, cluster) in &entities.dungeons {
            let count = cluster.base.entities.len();
            total_entities += count;
            if count > 0 {
                println!("    {} {} entities: {}", 
                       if cluster.can_generate_models() { "✓" } else { "○" }, 
                       name, count);
            }
        }
        println!("    └─ Total dungeons entities: {}", total_entities);
    }

    if !entities.uncategorized.is_empty() {
        println!("  ❓ Uncategorized: {} entities", entities.uncategorized.len());
        
        // Show sample uncategorized entities for debugging
        println!("    Sample uncategorized entities:");
        for (i, entity) in entities.uncategorized.iter().take(5).enumerate() {
            let preview = if entity.raw_value.len() > 50 {
                format!("{}...", &entity.raw_value[..50])
            } else {
                entity.raw_value.clone()
            };
            println!("    {}. {} -> {}", i + 1, entity.uuid, preview);
        }
    }

    println!("");
    
    // Compare to previous efficiency
    let previous_count = 331;
    let improvement = entities.total_entities as i32 - previous_count;
    let improvement_percentage = (improvement as f32 / previous_count as f32) * 100.0;
    
    println!("📈 EFFICIENCY COMPARISON:");
    println!("   Previous system: {} entities", previous_count);
    println!("   Current system:  {} entities", entities.total_entities);
    if improvement > 0 {
        println!("   ✅ Improvement: +{} entities (+{:.1}%)", improvement, improvement_percentage);
    } else {
        println!("   ❌ Regression: {} entities ({:.1}%)", improvement, improvement_percentage);
    }

    println!("");
    println!("📋 Analysis Notes:");
    for note in &summary.notes {
        println!("   • {}", note);
    }

    Ok(())
}
