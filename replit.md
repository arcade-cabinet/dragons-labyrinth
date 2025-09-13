# Dragon's Labyrinth - Enhanced Seeding System with Training Data

## Overview

Dragon's Labyrinth features a sophisticated seeding system with training data feedback loop that transforms D&D content into horror RPG assets. The system uses standalone Rust binaries to process HBF databases, generate RON asset structures, and create Replit-ready prompts for 3D model and dialogue generation.

**Core Innovation**: Self-improving training data system that learns from examples and enhances categorization with each iteration.

## Enhanced Seeding Architecture

### Three-Stage Pipeline

```
HBF Database (70K+ D&D entities)
    ↓ hbf-analyzer (training-enhanced categorization)
Categorized JSON Files (9 categories)
    ↓ ron-generator (cosmic-cults asset structure)
RON Asset Files (units, buildings, terrain, leaders)
    ↓ replit-prompter (training-aware prompt generation)
Replit Generation Templates (3D models + dialogue)
```

### Training Data Feedback Loop

The system includes comprehensive training data that teaches pattern recognition:

```
training_data/
├── characters/npcs.toml           # NPCs and named individuals
├── creatures/monsters.toml        # Monsters, beasts, encounters
├── items/treasure.toml           # Equipment, treasure, currency
├── spells/magic_systems.toml     # Magic, enchantments, arcane
├── mechanics/dice_rules.toml     # Game systems, probability
└── locations/                    # Adventure sites
    ├── dungeons.toml             # Lairs, crypts, temples
    ├── regions.toml              # Biomes, environmental areas
    ├── settlements.toml          # Villages, towns, communities
    └── factions.toml             # Guilds, orders, organizations
```

## Binary Tools Usage

### 1. hbf-analyzer v1.0.0 - Enhanced Database Analysis

**Purpose**: Analyze HBF databases with training data enhancement for sophisticated categorization.

**Basic Usage**:
```bash
cd crates/dl_seeds

# Analyze entire database with training data enhancement
cargo run --bin hbf-analyzer -- --database game.hbf --output analysis/ analyze-all --reports

# Query specific categories (9 categories supported)
cargo run --bin hbf-analyzer -- --database game.hbf query --category creatures --limit 5
cargo run --bin hbf-analyzer -- --database game.hbf query --category characters --limit 5
cargo run --bin hbf-analyzer -- --database game.hbf query --category mechanics --limit 3

# Inspect specific entities
cargo run --bin hbf-analyzer -- --database game.hbf inspect-html [UUID] --verbose

# Export for external processing
cargo run --bin hbf-analyzer -- --database game.hbf export --format json --category creatures
```

**Training Data Integration**:
- Automatically loads 45 training examples from `training_data/` directory
- Enhances categorization with pattern recognition
- Reports training data impact in analysis results
- Supports unlimited training example expansion

**Enhanced Categories**:
- **Original**: regions, settlements, factions, dungeons
- **Enhanced**: characters, creatures, items, spells, mechanics
- Each category uses training examples for improved recognition

### 2. ron-generator v1.0.0 - Cosmic-Cults Asset Structure

**Purpose**: Convert analyzed HBF data into organized RON files with cosmic-cults methodology and Dragon's Labyrinth horror themes.

**Basic Usage**:
```bash
# Generate all asset categories with corruption bands
cargo run --bin ron-generator -- --input analysis/ --output assets/ generate-all --corruption-bands

# Generate specific asset category with faction filtering
cargo run --bin ron-generator -- --input analysis/ --output assets/ generate --category units --faction "Crimson"

# Generate upgrade progression chains
cargo run --bin ron-generator -- --input analysis/ --output assets/ upgrades --auto-detect

# Validate existing RON structure
cargo run --bin ron-generator -- validate --assets-path existing_assets/
```

**Asset Organization**:
```
assets/
├── units/              # Character units by faction
│   ├── faction_name/   # Faction-specific directories
│   │   └── unit.meta.ron
├── buildings/          # Structures by settlement
├── leaders/            # Faction leaders and commanders
├── terrain/            # Environmental assets by biome
└── upgrade_chains/     # Progression relationships
```

**Cosmic-Cults Features**:
- Sophisticated RON metadata with upgrade paths
- Animation sockets and technical specifications
- Corruption band assignment (1-5 progression)
- Horror theme integration for asset generation

### 3. replit-prompter v1.0.0 - Training-Enhanced Prompt Generation

**Purpose**: Create Replit-ready prompts for 3D model and dialogue generation with RON asset integration.

**Basic Usage**:
```bash
# Generate all prompt templates with corruption themes
cargo run --bin replit-prompter -- --input analysis/ --assets assets/ --output prompts/ generate-all --corruption-themes

# Generate 3D model prompts only
cargo run --bin replit-prompter -- --input analysis/ --assets assets/ --output prompts/ models --category units --faction "Crimson"

# Generate Yarnspinner dialogue prompts
cargo run --bin replit-prompter -- --input analysis/ --output prompts/ dialogue --companion-trauma

# Generate upgrade progression documentation
cargo run --bin replit-prompter -- --assets assets/ --output prompts/ progressions --visual-guides
```

**Output Structure**:
```
prompts/
├── model_prompts/           # 3D model generation templates
│   ├── faction_name/        # Organized by faction
│   │   └── entity_prompt.md
├── dialogue_prompts/        # Yarnspinner dialogue templates
│   └── character_dialogue.md
└── progression_guides/      # Upgrade progression documentation
    ├── upgrade_progressions.md
    └── visual_progression_guide.md
```

**RON Enhancement Features**:
- Reads existing RON files to enhance prompts with technical specifications
- Category filtering for specialized prompt generation
- Training-aware content analysis for speech patterns
- Asset-enhanced progression documentation

## Training Data System

### Adding Training Examples

Training examples teach the system to recognize D&D content patterns:

```toml
# Example: creatures/monsters.toml
[[examples]]
name = "5 Onis"
content_patterns = ["number + creature", "group encounter", "japanese monster"]
markers = ["quantity prefix", "creature name", "encounter group"]
corruption_band = 3
horror_theme = "demonic_presence"
```

### Training Categories

Each category includes:
- **Examples**: Real D&D content with pattern descriptions
- **Markers**: Recognition indicators for pattern matching
- **Patterns**: Positive/negative indicators for classification
- **Horror Integration**: Corruption bands and themes for asset generation

### Expanding Training Data

1. **Identify Patterns**: Run analysis to see uncategorized entities
2. **Add Examples**: Create training examples in appropriate TOML files
3. **Test Enhancement**: Re-run analysis to see improved categorization
4. **Iterate**: Continuously improve with more training examples

## End-to-End Workflow

### Complete Asset Generation Pipeline

```bash
# 1. Enhanced HBF Analysis with Training Data
cd crates/dl_seeds
cargo run --bin hbf-analyzer -- --database game.hbf --output analysis/ analyze-all --reports

# 2. RON Asset Generation with Cosmic-Cults Structure
cargo run --bin ron-generator -- --input analysis/ --output assets/ generate-all --corruption-bands

# 3. Replit Prompt Generation with RON Enhancement
cargo run --bin replit-prompter -- --input analysis/ --assets assets/ --output prompts/ generate-all --corruption-themes
```

### Using Generated Assets in Replit

1. **3D Model Generation**:
   - Open `prompts/model_prompts/faction_name/entity_prompt.md`
   - Copy the "Primary Generation Prompt" section
   - Use with Replit's 3D model generation feature
   - Apply technical specifications for proper GLB export

2. **Dialogue Generation**:
   - Open `prompts/dialogue_prompts/character_dialogue.md`
   - Use personality prompt as base for character voice
   - Create Yarnspinner `.yarn` files with suggested interactions
   - Include trauma indicators for dynamic dialogue progression

3. **Progression Planning**:
   - Reference `prompts/progression_guides/upgrade_progressions.md`
   - Follow faction-specific upgrade chains
   - Use visual progression guide for model variations

## Horror RPG Integration

### Dragon's Labyrinth Enhancements

The system transforms D&D content into horror RPG assets through:

**Corruption Progression (5 Bands)**:
1. **Peace to Unease**: Subtle wrongness, barely perceptible asymmetries
2. **Unease to Dread**: Growing wrongness, unnatural shadows
3. **Dread to Terror**: Manifest corruption, non-Euclidean elements
4. **Terror to Horror**: Advanced corruption, reality-bending elements
5. **Final Horror**: Total transformation, cosmic horror manifestation

**Companion Psychology System**:
- Training examples include trauma indicators
- Dialogue prompts feature psychological progression
- Asset specifications support companion deterioration

**Forge System Integration**:
- Training data includes forge material assignments
- RON assets specify redemption/corruption materials
- Prompts enhance equipment with forge system themes

## Advanced Features

### Training Data Feedback Loop

The system continuously improves through training examples:

1. **Analysis Results** → Identify successful/unsuccessful categorizations
2. **Training Enhancement** → Add examples for missed content types
3. **Improved Recognition** → Better categorization in next analysis cycle
4. **Asset Quality** → Enhanced RON generation and Replit prompts

### Cosmic-Cults Methodology

Inspired by cosmic-cults' sophisticated development approach:

- **Progressive Development**: Clear upgrade chains with visual evolution
- **Technical Specifications**: Optimized poly counts, texture resolutions
- **Sophisticated Metadata**: Animation sockets, material properties
- **Performance Focus**: Web-optimized asset generation

### Self-Improving Architecture

The system scales categorization quality with training additions:

- **Current**: 2,091 entities categorized from 70K+ database
- **Scalable**: Architecture supports exponential improvement with more training
- **Measurable**: Training impact visible in categorization success rates
- **Unlimited**: No performance penalty for training expansion

## Development Workflow

### Daily Development

```bash
# 1. Check current categorization status
cargo run --bin hbf-analyzer -- --database game.hbf analyze-all

# 2. Add training examples for missed content
vim crates/dl_seeds/training_data/creatures/monsters.toml

# 3. Test improved categorization
cargo run --bin hbf-analyzer -- --database game.hbf --output analysis/ analyze-all --reports

# 4. Generate enhanced assets
cargo run --bin ron-generator -- --input analysis/ --output assets/ generate-all --corruption-bands

# 5. Create Replit prompts
cargo run --bin replit-prompter -- --input analysis/ --assets assets/ --output prompts/ generate-all
```

### Iterative Enhancement

1. **Analysis**: Identify uncategorized content patterns
2. **Training**: Add examples to appropriate TOML files
3. **Validation**: Test enhanced categorization results
4. **Asset Generation**: Create RON files with improved data
5. **Prompt Creation**: Generate training-enhanced Replit templates

The system creates a continuous improvement cycle where more training data leads to better categorization, richer assets, and higher quality Replit prompts for 3D generation.

## System Requirements

### Development Environment
- **Rust**: Latest stable (for standalone binaries)
- **Cargo**: Workspace management with standalone binary support
- **TOML**: Training data configuration files
- **SQLite**: HBF database access (rusqlite crate)

### Optional Enhancements
- **Replit**: 3D model generation using generated prompts
- **Yarnspinner**: Dialogue system using generated character prompts
- **Git**: Version control for training data and system evolution

The enhanced seeding system provides a sophisticated foundation for horror RPG development with unlimited improvement potential through training data enhancement.
