# Active Context - Dragon's Labyrinth

## 🔄 CRITICAL ARCHITECTURAL PIVOT DISCOVERED (Jan 3, 2025)

### 🎯 FUNDAMENTAL PROBLEM IDENTIFIED: HEXROLL DATA INCOMPATIBLE WITH 2.5D GAME

The user provided examples of actual hexroll data showing the core issue:
- **Multiple dungeon entrances per hex**: "Stairs leading down into area 14", "area 27", "area 24" 
- **Complex encounter tables**: "1 in 6 chance" with detailed D&D stat blocks
- **Tabletop-specific features**: Weather tables, regional variations, complex descriptions
- **Narrative focus**: "A choral melody drifts on the breeze" - designed for DM reading, not game implementation

**USER QUOTE**: "You know realisticaly the more I turn this all over in my head, using all this hexroll data directly is going to be a nightmare to actually implement in a 2.5D game. There's stuff like multiple entrances, unique descriptions that work well in DND but not in reality...."

### 🏗️ NEW ARCHITECTURE APPROACH: SAMPLE-BASED AI EXTRACTION

**COMPLETE REORGANIZATION REQUIRED:**

#### 1. Move HBF Analysis to dl_seeds
- **FROM**: `crates/dl_analysis/game.hbf` and analysis code  
- **TO**: `crates/dl_seeds`
- **PURPOSE**: Seeds should handle the raw data extraction, not analysis

#### 2. Move AI Transformation to dl_analysis  
- **FROM**: `crates/dl_processors` AI transformation logic
- **TO**: `crates/dl_analysis` 
- **PURPOSE**: Analysis should do the AI extraction and structuring

#### 3. Split dl_analysis/src/seeds.rs into Modular Files
- **FROM**: Monolithic seeds.rs
- **TO**: 
  - `quests.rs` - Quest pattern extraction
  - `dialogue.rs` - Dialogue pattern extraction  
  - `monsters.rs` - Monster/creature extraction
  - `weather.rs` - Weather pattern extraction
  - `biomes.rs` - Biome/environmental extraction
  - `dungeons.rs` - Dungeon structure extraction
  - `regions.rs` - Regional data extraction

#### 4. Move Build Logic
- **FROM**: `crates/dl_processors/build.rs`
- **TO**: `crates/dl_analysis/build.rs`
- **PURPOSE**: Analysis stage should handle the build chain coordination

#### 5. Sample-Based Extraction Approach
**NEW METHOD:**
- **Idempotent Sampling**: Check OUT_DIR for existing `regions.json`, `factions.json`, `dungeons.json`, `settlements.json`
- **Random Sampling**: If missing, randomly shuffle known entity names and grab ~5 samples per category  
- **Optimized Queries**: Use SQL LIKE queries to select specific entities (e.g., "region foo, bar, baz")
- **AI Structure Extraction**: Use AI to extract wealth of structured data from samples, not parse HTML
- **No Predetermined Schema**: Let AI discover top-level fields initially, then review and standardize

**USER QUOTE**: "And then change the AI generation of models part to take those samples for a specific category and instead of generating PARSING rules for the HTML, actually EXTRACT the wealth of data from each and organize it into a structured output that organizes effectively all the data."

### 🚨 CURRENT STATUS: ARCHITECTURAL REDESIGN REQUIRED

#### What Was Completed Before Pivot:
- ✅ **Complete Pipeline Chain**: dl_seeds → dl_analysis → dl_processors → apps/game
- ✅ **70,801 Entities Processed**: Full extraction and categorization operational  
- ✅ **Comprehensive Audit System**: Performance tracking across all stages
- ✅ **Type System Integration**: dl_types::world::HexCoord usage throughout
- ✅ **Build Chain Coordination**: All components properly connected

#### What Needs Complete Redesign:
- ❌ **dl_processors Generates Wrong Code**: Currently creates minimal placeholders instead of complete ECS world
- ❌ **HexCoord Type Mismatches**: utils::hex::HexCoord vs dl_types::world::HexCoord confusion
- ❌ **Hexroll Data Format**: Too complex for 2.5D game implementation
- ❌ **Template Architecture**: Current templates generate massive monolithic files instead of modular components

### 📋 NEW IMPLEMENTATION PLAN

#### Phase 1: Architectural Reorganization
1. **Move HBF and Analysis Logic**: dl_analysis → dl_seeds
2. **Move AI Transformation**: dl_processors → dl_analysis  
3. **Split seeds.rs**: Create modular extraction files
4. **Move Build Logic**: dl_processors/build.rs → dl_analysis/build.rs

#### Phase 2: Sample-Based AI Extraction Implementation
1. **Implement Idempotent Sampling**: Check existing JSON files
2. **Create Random Sampling Logic**: 5 samples per category
3. **Build AI Structure Extraction**: Extract structured data instead of parsing rules
4. **Review and Standardize**: Manual review of AI-discovered fields

#### Phase 3: Modular dl_processors Redesign  
1. **Generate Focused Templates**: components.rs, systems.rs, world.rs, regions.rs, settlements.rs, factions.rs, dungeons.rs
2. **Fix Type Consistency**: Use dl_types::world::HexCoord throughout
3. **Create Complete ECS Code**: Generate full working systems, not placeholders

## WORKING DIRECTORY
**Current**: `/Users/jbogaty/src/dragons-labyrinth`

## IMMEDIATE NEXT STEP
**Set up new task** to implement the architectural reorganization starting with moving the HBF analysis to dl_seeds and creating the modular structure for AI-based sample extraction.

**STATUS: ARCHITECTURAL PIVOT DOCUMENTED - READY FOR REORGANIZATION TASK**
