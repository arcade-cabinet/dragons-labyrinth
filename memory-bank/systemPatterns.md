# System Patterns & Architecture

## Core Architecture Pattern

### Training Data Enhanced Content-Driven Development
```
Training Data (TOML patterns + examples)
    ↓
HBF Database Analysis (enhanced categorization)
    ↓
Enhanced JSON Data (9 categories vs 4)
    ↓
RON Asset Generation (cosmic-cults structure)
    ↓
Replit Prompt Enhancement (training-aware)
    ↓
Game Runtime (Rust/Bevy)
```

## Technical Patterns

### 1. Training Data Feedback Loop Pattern
**Framework**: TOML-based training with pattern recognition

**Core Concepts**:
- **Training Examples**: Real D&D content with categorization patterns
- **Pattern Matching**: Marker-based recognition with positive/negative indicators
- **Self-Improvement**: Training additions automatically enhance categorization
- **Horror Integration**: Corruption bands and trauma themes in all categories

**Training Structure**:
```toml
[[examples]]
name = "5 Onis"
content_patterns = ["number + creature", "group encounter"]
markers = ["quantity prefix", "creature name", "encounter group"]
corruption_band = 3
horror_theme = "demonic_presence"
```

### 2. Enhanced Entity Categorization Pattern
**Purpose**: Comprehensive D&D content recognition with training enhancement

**Implementation**:
```rust
// Training-enhanced categorization
fn categorize_entity_with_training(&self, entity: &RawEntity, training: &TrainingRepository) -> Option<EntityCategory> {
    // Check training patterns for each category
    for char_training in &training.characters {
        if self.matches_training_patterns(&content, &name, &example.markers) {
            return Some(EntityCategory::Characters);
        }
    }
    // Fall back to original categorization
    self.categorize_entity(entity)
}
```

**Categories**:
- Primary: Characters, Creatures, Items, Spells, Mechanics
- Locations: Regions, Settlements, Factions, Dungeons
- Each with subcategory specialization support

### 3. Split Reporting Pattern
**Purpose**: Manageable data analysis preventing massive CSV files

**Implementation**:
- Sample-based reporting (100 entities vs 68K+)
- Pattern analysis files for categorization insights
- Chunked exports only on explicit request

**Benefits**:
- Prevents system overload with massive datasets
- Enables rapid analysis iteration
- Provides actionable categorization insights

### 4. Self-Improving Architecture Pattern
**Concept**: System gets better with each training addition

**Enhancement Cycle**:
```
1. Add Training Examples (TOML files)
    ↓
2. Enhanced Pattern Recognition
    ↓  
3. Improved Categorization Results
    ↓
4. Better Asset Generation
    ↓
5. Feedback for Next Training Cycle
```

**Scalability**: Unlimited training expansion supported

### 5. Cosmic-Cults Integration Pattern
**Purpose**: Sophisticated asset organization with horror RPG themes

**RON Structure Enhancement**:
```rust
ModelMetadata {
    // Cosmic-cults standard
    animations, sockets, upgrades_to,
    // Dragon's Labyrinth additions
    corruption_band: Option<u8>,
    horror_theme: Option<String>,
    forge_material: Option<String>,
}
```

**Horror RPG Transformation**:
- Corruption progression through 5 bands
- Companion trauma integration
- Forge material system support

## Data Flow Patterns

### 1. Enhanced Analysis Pipeline Pattern
```
HBF Database (70,801 entities)
    ↓ Training Data Loading (45 examples)
Enhanced Categorization (9 categories)
    ↓ Pattern Recognition
Categorized Entities (2,091 current, scalable)
    ↓ Asset Organization
RON Files (cosmic-cults structure)
    ↓ Prompt Enhancement
Replit Templates (training-aware)
```

### 2. Training Data Repository Pattern
**Structure**: Category-based TOML files with comprehensive examples

**Loading Strategy**:
```rust
TrainingRepository::load_from_directory()
  ├── characters/npcs.toml
  ├── creatures/monsters.toml  
  ├── items/treasure.toml
  ├── spells/magic_systems.toml
  ├── mechanics/dice_rules.toml
  └── locations/*.toml (4 subcategories)
```

**Pattern Matching**:
- Marker-based recognition for content types
- Positive/negative indicator systems
- Horror RPG theme integration

### 3. Self-Improving Categorization Pattern
**Concept**: System learns from training examples

**Enhancement Process**:
- Training examples teach pattern recognition
- Successful patterns improve categorization accuracy
- More training data → exponentially better results
- Feedback loop creates continuous improvement

## Enhanced Game System Patterns

### 1. Training-Aware Asset Generation Pattern
**Integration**: Training data influences entire asset pipeline

**Enhancement Flow**:
- Training examples → improved categorization
- Enhanced categorization → richer RON metadata
- Training themes → horror RPG asset specifications
- Pattern success → asset generation quality improvement

### 2. Horror RPG Training Integration Pattern
**Structure**: Each training example includes corruption and horror themes

**Implementation**:
```toml
corruption_band = 3              # 1-5 progression system
horror_theme = "demonic_presence" # Asset generation theme
```

**Benefits**:
- Consistent horror progression across all assets
- Training data directly drives horror RPG transformation
- Corruption band assignment for systematic progression

### 3. Comprehensive Category Support Pattern
**Architecture**: 9 categories vs original 4

**Category Enhancement**:
- **Original 4**: Regions, Settlements, Factions, Dungeons
- **Enhanced 5**: Characters, Creatures, Items, Spells, Mechanics
- **Subcategory Support**: Location subcategories with specialized patterns

**Scalability**: Unlimited category and subcategory expansion

## Training Data Patterns

### 1. TOML Training File Pattern
**Structure**: Standardized training data format

**Template**:
```toml
[category]
name = "category_name"
subcategory = "subcategory_name" 
description = "category description"

[[examples]]
name = "example_name"
content_patterns = ["pattern descriptions"]
markers = ["recognition markers"]
corruption_band = 1-5
horror_theme = "theme_name"

[patterns]
positive_indicators = ["content that indicates this category"]
negative_indicators = ["content that excludes this category"]
```

### 2. Pattern Recognition Enhancement Pattern
**Concept**: Training examples teach system content recognition

**Matching Strategy**:
- Example name matching for direct recognition
- Marker pattern matching for content analysis
- Positive indicator matching for category classification
- Negative indicator exclusion for accuracy

### 3. Horror RPG Integration Pattern
**Purpose**: Every training example contributes to horror game development

**Integration Points**:
- Corruption band assignment (1-5 progression)
- Horror theme specification for asset generation
- Transformation evolution guidelines
- Asset specification requirements

## Performance Patterns

### 1. Scalable Training Pattern
**Concept**: System performance improves with training additions

**Scaling Benefits**:
- More examples → better pattern recognition
- Enhanced categorization → richer asset generation
- Training diversity → comprehensive content coverage
- Unlimited expansion without performance degradation

### 2. Memory-Optimized Training Pattern
**Implementation**: Training data loaded efficiently without memory impact

**Optimization Strategy**:
- TOML parsing on-demand during analysis
- Training data cached for analysis session
- No memory penalty for training expansion
- Maintains 97% RAM reduction from architectural consolidation

### 3. Split Processing Pattern
**Purpose**: Handle massive datasets with manageable processing

**Implementation**:
- Sample-based analysis for rapid iteration
- Full processing available on explicit request
- Pattern analysis for categorization insights
- Prevents system overload with 70K+ entity datasets

## Content Enhancement Patterns

### 1. D&D Content Recognition Pattern
**Approach**: Training data teaches system D&D content types

**Content Categories**:
- Character names and NPC patterns
- Creature encounters and monster formats
- Treasure and equipment descriptions
- Spell names and magic system content
- Game mechanics and probability notation

### 2. HTML Parsing Enhancement Pattern
**Strategy**: Extract meaningful content from D&D HTML structure

**Parsing Hierarchy**:
1. Hidden doc-title divs (HBF specific)
2. Header tags (h1, h2, h3, h4)
3. Bold/strong tags for emphasis
4. Clean text extraction as fallback

### 3. Horror RPG Transformation Pattern
**Integration**: Every D&D element enhanced with horror themes

**Transformation Guidelines**:
- Corruption progression (1-5 bands)
- Companion trauma indicators
- Environmental corruption evolution
- Forge material system integration

## Development Patterns

### 1. Training-First Development Pattern
**Workflow**: Create training examples before implementing features

**Benefits**:
- Clear categorization goals before implementation
- Training data drives feature development
- Pattern recognition improves iteratively
- Quality assurance through training validation

### 2. Feedback Loop Integration Pattern
**Concept**: Analysis results inform training improvements

**Cycle**:
```
Training Examples → Analysis Results → Pattern Insights → Training Refinement → Enhanced Results
```

### 3. Comprehensive Coverage Pattern
**Strategy**: Ensure training data for every category

**Implementation**:
- Primary categories with multiple examples
- Subcategory specialization with focused training
- Pattern diversity for comprehensive recognition
- Horror RPG theme integration throughout

These enhanced patterns form the foundation of Dragon's Labyrinth's self-improving training data system, ensuring sophisticated categorization with unlimited enhancement potential through operational feedback loops.
