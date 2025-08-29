# HBF Integration Strategy: Slice-by-Slice Analysis & Transformation

## Strategic Pivot: From Big Picture to Focused Slices

Based on feedback, we're pivoting from attempting the entire transformation to a **slice-by-slice approach** where each ECS system/component gets dedicated analysis and transformation with its own new_task.

## Target ECS Systems & Components

### Hierarchical System Structure
```
Region (System)
├── Biome (Component - per hex tile)
├── Weather Tables (Component)
└── Hex Count Metadata (Component)

Dungeon (System) 
├── Cave (Component)
├── Temple (Component)
└── Tomb (Component)

Settlement (System)
├── City (Component)
├── Town (Component)
└── Village (Component)

Dwelling (System)
├── Farms & Cabins (Component)
└── Strongholds (Component)

Factions (System)
├── Cults (Component)
├── Militias (Component)
└── Syndicates (Component)

Standalone Components:
├── Monster (Component)
└── Inn (Component)
```

## HTML Richness Discovery Requirements

### What We Must Fully Understand:
1. **Probability Tables** - How they're encoded in HTML
2. **Weather Tables** - Seasonal/regional variations
3. **Hyperlink Associations** - Entity cross-references
4. **Crawl Maps** - Dungeon room connections
5. **Refs vs Entities** - Relationship encoding
6. **JSON vs HTML** - Different data encoding methods

### Deep Analysis Per Slice:
- Extract ALL probability tables for the slice
- Map ALL hyperlink references 
- Understand JSON structure (if any)
- Document HTML patterns unique to slice
- Test relationship resolution
- Validate against game-design-bible integration

## Infrastructure Setup Phase

### 1. Dependencies & Agent Setup
```python
# pyproject.toml additions needed:
[project.dependencies]
langchain = "^0.3.0"
langgraph = "^0.2.0"
langchain-openai = "^0.2.0"
faiss-cpu = "^1.8.0"
tiktoken = "^0.8.0"
```

### 2. Standards Alignment
```python
# Extend types.py with professor-pixels standards
# Extend models.py with proper Field descriptions
# Replace all Optional[Type] → Type | None
# Replace all List[Type] → list[Type]
# Use auto() for all enums
# Add ConfigDict instead of Config class
```

### 3. Agent Architecture
```python
# src/dragons_labyrinth/agent.py
class HBFAnalysisAgent(BaseComponent):
    """Main orchestrator for HBF slice-by-slice analysis"""
    
    def build_slice_workflow(self, slice_type: str) -> StateGraph:
        # Create workflow for specific slice (Region, Dungeon, etc.)
        
    def execute_slice_analysis(self, slice_type: str) -> SliceAnalysisResult:
        # Run analysis workflow with human review checkpoints
```

### 4. Subpackage Restructure

#### Analysis Subpackage
```
src/dragons_labyrinth/hbf/analysis/
├── __init__.py
├── region_analysis.py      # Region slice workflow
├── dungeon_analysis.py     # Dungeon slice workflow  
├── settlement_analysis.py  # Settlement slice workflow
├── faction_analysis.py     # Faction slice workflow
├── monster_analysis.py     # Monster slice workflow
└── base_analyzer.py        # Shared analysis patterns
```

#### Transformers Subpackage
```
src/dragons_labyrinth/hbf/transformers/
├── __init__.py
├── region_transformer.py      # Region → Bevy components
├── dungeon_transformer.py     # Dungeon → Bevy components
├── settlement_transformer.py  # Settlement → Bevy components
├── faction_transformer.py     # Faction → Bevy components
├── monster_transformer.py     # Monster → Bevy components
└── base_transformer.py        # Shared transformation patterns
```

## Incremental SQLite Strategy

### After Each Slice Processing:
1. **Extract slice entities** from original HBF
2. **Transform slice** using langchain workflows
3. **Save new HBF copy** WITHOUT processed entities
4. **Document slice results** in memory-bank
5. **Validate integration** with game-design-bible

### Database Evolution:
```
game.hbf (70,801 entities)
↓ Process Regions
game_post_regions.hbf (70,801 - region_count entities)
↓ Process Dungeons  
game_post_dungeons.hbf (remaining - dungeon_count entities)
↓ Process Settlements
game_post_settlements.hbf (remaining - settlement_count entities)
... until 0 entities remain
```

## Integration with Horror RPG Experience

### Each Slice Must Consider:
1. **Dread Level Integration** - How does this system respond to 0-4 dread?
2. **Philosophy Path Integration** - How does this connect to Strength/Harmony/Light/Dark?
3. **Companion Psychology** - How do companions react to these elements?
4. **Environmental Decay** - How does corruption affect this system?
5. **Narrative Threading** - How does this create horror story moments?

### Example: Region Integration
```rust
// Not just data extraction, but horror integration
struct Region {
    name: String,
    biomes: Vec<Entity>,
    weather_tables: WeatherSystem,
    
    // Horror integration
    corruption_spread_rate: f32,    // How fast decay spreads here
    dread_amplification: f32,       // Regional dread multiplier  
    companion_stress_factors: Vec<StressTrigger>, // What traumatizes here
    philosophy_resonance: PathResonance,  // Which paths this supports
}
```

## Execution Plan

### Phase 1: Infrastructure (Current Task)
- [ ] Add langchain/langgraph dependencies
- [ ] Create agent.py with workflow foundations
- [ ] Align types.py and models.py with standards
- [ ] Restructure analysis.py → analysis/ subpackage
- [ ] Restructure game_transformer.py → transformers/ subpackage
- [ ] Document infrastructure in memory-bank

### Phase 2: First Slice (New Task)
- [ ] Focus on Region slice only
- [ ] Deep HTML analysis of region data
- [ ] Understand region JSON structure  
- [ ] Map region hyperlinks
- [ ] Extract region probability tables
- [ ] Create region workflow with human review
- [ ] Transform regions to Bevy components
- [ ] Integrate with horror RPG systems
- [ ] Save post-regions HBF

### Phase 3-N: Remaining Slices (Separate New Tasks)
- [ ] Dungeon slice (separate new_task)
- [ ] Settlement slice (separate new_task)
- [ ] Faction slice (separate new_task)
- [ ] Monster slice (separate new_task)
- [ ] Inn slice (separate new_task)
- [ ] Dwelling slice (separate new_task)

## Success Criteria Per Slice

### Analysis Phase:
- [ ] 100% understanding of HTML structure for slice
- [ ] All probability tables extracted and documented
- [ ] All hyperlinks mapped and validated
- [ ] All refs/entity relationships understood
- [ ] Integration points with horror systems identified

### Transformation Phase:
- [ ] Bevy components generated with horror integration
- [ ] Human review checkpoint passed
- [ ] Code compiles and validates
- [ ] Memory-bank documentation complete
- [ ] HBF database updated (entities removed)

## Memory Bank Integration

### Documentation Per Slice:
```
memory-bank/hbf-slices/
├── region-analysis.md      # Region deep dive
├── region-integration.md   # Horror RPG integration
├── dungeon-analysis.md     # Dungeon deep dive  
├── dungeon-integration.md  # Horror RPG integration
└── ... (one pair per slice)
```

### Game Design Bible Cross-Reference:
Each slice analysis must reference and extend:
- Dread progression mechanics
- Philosophy path implications  
- Companion psychology triggers
- Environmental decay systems
- Narrative horror threading

This approach ensures we're not just extracting data but truly integrating the rich HBF content into our horror RPG experience as a primary system component.
