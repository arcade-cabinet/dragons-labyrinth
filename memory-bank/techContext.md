# Technical Context

## Technology Stack

### Core Technologies

#### Rust + Bevy (Game Engine)
- **Version**: Bevy 0.16.1
- **Language**: Rust with 2024 edition
- **Why Bevy**: ECS architecture perfect for component-heavy RPG
- **Key Features**: Hot-reload, 2D rendering, plugin system, shaders

#### Python (Content Generation)
- **Version**: Python 3.13
- **Purpose**: AI-driven content pipeline
- **Key Libraries**: 
  - OpenAI: GPT integration
  - Pydantic: Schema validation
  - Pillow: Image processing
  - Typer: CLI interface

#### Training Data System (New)
- **Format**: TOML-based training files with D&D patterns
- **Purpose**: Self-improving HBF entity categorization
- **Coverage**: 45 training examples across 9 categories
- **Integration**: Feedback loop with pattern recognition

#### OpenAI API
- **Text Model**: Configurable (default: gpt-5.1)
- **Image Model**: Configurable (default: gpt-image-1)
- **Purpose**: Convert markdown to game content
- **Features**: JSON schema enforcement, structured outputs

## Project Structure

### Repository Layout
```
dragons-labyrinth/
├── .clinerules           # AI agent instructions
├── Cargo.toml           # Rust workspace config
├── pyproject.toml       # Python project config
├── crates/dl_seeds/     # Enhanced seeding system
│   ├── training_data/   # 🆕 TOML training files
│   │   ├── characters/  # NPCs and named individuals
│   │   ├── creatures/   # Monsters and beasts
│   │   ├── items/       # Equipment and treasure
│   │   ├── spells/      # Magic systems
│   │   ├── mechanics/   # Game rules and dice
│   │   └── locations/   # All location types
│   └── src/bin/        # Enhanced standalone binaries
├── ai/                  # Python generation pipeline
├── apps/                # Rust applications
│   └── game/           # Main game executable
├── content/            # Source markdown files
├── build/              # Generated JSON data
└── memory-bank/        # Project documentation
```

### Enhanced Build Artifacts
```
build/
├── master/             # Core game data
│   ├── canon.json     # Game rules
│   └── themes.json    # Art bible
├── world/             # World data
│   ├── plan.json      # High-level structure
│   ├── region_*.json  # Detailed regions
│   └── worldbook.json # Complete world
├── features/          # Game features
│   └── shops/         # Shop inventories
├── narrative/         # Story content
├── image_plan.json    # Asset generation plan
└── enhanced_analysis/ # 🆕 Training-enhanced HBF processing
    ├── characters.json # NPCs and named individuals
    ├── creatures.json  # Monsters and encounters
    ├── items.json      # Equipment and treasure
    ├── spells.json     # Magic and enchantments
    ├── mechanics.json  # Rules and probability
    └── reports/        # Split CSV analysis
```

## Development Environment

### Required Tools
- **Rust**: Latest stable via rustup
- **Python**: 3.13 via pyenv/uv
- **Git**: Version control
- **VS Code**: With Rust and Python extensions

### Environment Variables
```bash
OPENAI_API_KEY=sk-...         # Required for AI generation
OPENAI_MODEL=gpt-5.1          # Optional, defaults shown
OPENAI_IMAGE_MODEL=gpt-image-1 # Optional
```

### Package Management
- **Rust**: Cargo workspaces with training data integration
- **Python**: uv/hatch for dependencies
- **Training Data**: TOML files for categorization enhancement
- **Node**: Not required (removed with Godot)

## Key Dependencies

### Rust Dependencies
```toml
[workspace.dependencies]
bevy = "0.16.1"              # Game engine
serde = { version = "1", features = ["derive"] }
serde_json = "1"             # JSON parsing
anyhow = "1"                 # Error handling
rand = "0.8"                 # Random generation
toml = "1"                   # 🆕 Training data parsing
walkdir = "2"                # 🆕 Training file discovery
```

### Enhanced dl_seeds Dependencies
```toml
# Training data system
toml = { workspace = true }           # Training file parsing
walkdir = { workspace = true }        # Training directory traversal

# Enhanced categorization
rusqlite = { workspace = true }       # HBF database access
regex = { workspace = true }          # Pattern matching
csv = { workspace = true }            # Split reporting

# AI integration (original)
rust-bert = { workspace = true }      # Pattern recognition models
openai_dive = { workspace = true }    # OpenAI integration
polars = { workspace = true }         # DataFrame processing
```

## Enhanced Build & Run Commands

### Training Data Enhanced Analysis
```bash
# Enhanced HBF analysis with training data
cd crates/dl_seeds
cargo run --bin hbf-analyzer -d game.hbf -o analysis/ analyze-all --reports

# Query specific enhanced categories
cargo run --bin hbf-analyzer -d game.hbf query -c creatures -l 5
cargo run --bin hbf-analyzer -d game.hbf query -c characters -l 5
cargo run --bin hbf-analyzer -d game.hbf query -c mechanics -l 5

# RON generation with enhanced data
cargo run --bin ron-generator -i analysis/ -o assets/ generate-all --corruption-bands

# Replit prompts with RON enhancement
cargo run --bin replit-prompter -i analysis/ -a assets/ -o prompts/ generate-all --corruption-themes
```

### Training Data Management
```bash
# Add new training examples (manual)
vim crates/dl_seeds/training_data/creatures/monsters.toml

# Test training data loading
cargo run --bin hbf-analyzer -d game.hbf analyze-all  # Shows training count

# Validate training data integration
cargo run --bin hbf-analyzer -d game.hbf query        # Shows all 9 categories
```

### Development Workflow Enhanced
```bash
# 1. Edit training data (if needed)
vim crates/dl_seeds/training_data/[category]/[subcategory].toml

# 2. Run enhanced analysis
cargo run --bin hbf-analyzer -d game.hbf -o analysis/ analyze-all --reports

# 3. Generate enhanced RONs
cargo run --bin ron-generator -i analysis/ -o assets/ generate-all --corruption-bands

# 4. Create training-aware prompts
cargo run --bin replit-prompter -i analysis/ -a assets/ -o prompts/ generate-all

# 5. Hot-reload test (press R in game)
cargo run -p game
```

## Architecture Decisions

### Why Training Data System (New)
- **Self-Improvement**: System gets better with each training addition
- **Pattern Recognition**: Teaches system D&D content types systematically
- **Scalability**: Unlimited categorization enhancement potential
- **Quality**: Rich content properly categorized for asset generation
- **Horror Integration**: Training examples include corruption themes

### Why TOML Training Files (vs Database)
- **Human-Readable**: Easy to edit and maintain training examples
- **Version Control**: Training data changes tracked in git
- **External Configuration**: No recompilation needed for training updates
- **Pattern Documentation**: Training files serve as categorization documentation
- **Horror Integration**: Corruption bands and themes in training examples

### Why Enhanced Categorization (vs Basic)
- **D&D Content Diversity**: Original 4 categories insufficient for rich D&D data
- **Asset Generation**: More categories → more diverse Replit prompts
- **Training Feedback**: Enhanced categories enable better training examples
- **Horror RPG Support**: Specialized categories support companion trauma, forge systems

### Why Split Reporting (vs Massive Files)
- **System Performance**: Prevents overload with 70K+ entity datasets
- **Analysis Efficiency**: 100-entity samples enable rapid pattern recognition
- **User Experience**: Manageable files vs massive CSV exports
- **Iterative Development**: Quick analysis cycles for training refinement

## Performance Considerations

### Memory Usage (Enhanced)
- **Training Data**: Minimal overhead (~1MB TOML files)
- **World Data**: ~10MB JSON loaded at startup
- **Textures**: Lazy-loaded as needed
- **Entities**: Component pools for efficiency
- **Target**: <500MB RAM usage maintained

### CPU Usage (Training Enhanced)
- **Training Loading**: One-time TOML parsing cost
- **Pattern Recognition**: Efficient string matching algorithms
- **Enhanced Categorization**: Scalable with training examples
- **Target**: 60 FPS maintained with training enhancement

### Enhanced Processing
- **Training Impact**: Measurable categorization improvement
- **Scalable Enhancement**: More training → exponentially better results
- **Split Reporting**: Prevents processing bottlenecks
- **Pattern Caching**: Training data cached during analysis sessions

## Platform Support

### Primary Targets
- **Windows**: 10/11 (64-bit)
- **macOS**: 11+ (Apple Silicon + Intel)
- **Linux**: Ubuntu 20.04+ equivalent

### Future Targets
- **Steam Deck**: Verified compatibility
- **Web**: WASM build (post-launch)
- **Mobile**: Possible but not planned

## Testing Strategy

### Enhanced Unit Tests
```bash
# Rust tests (including training data)
cargo test

# Training data validation
cargo test training_repository

# Enhanced categorization tests
cargo test enhanced_categorization

# Python tests
pytest ai/tests/
```

### Training Data Integration Tests
- Training data loading validation
- Pattern recognition accuracy tests
- Categorization improvement measurement
- End-to-end pipeline with training enhancement

### Performance Tests Enhanced
- Frame rate monitoring with training data
- Memory leak detection with enhanced categorization
- Training data loading performance benchmarks
- Enhanced analysis processing time measurement

## Deployment

### Enhanced Distribution
- **Format**: Single executable + data folder + training data
- **Size**: ~50MB download + training TOML files
- **Updates**: Replace data files + training examples for content enhancement
- **Saves**: User home directory

### Training Data Updates
```bash
# Update training examples only
git pull  # Get new training data
cargo run --bin hbf-analyzer -d game.hbf analyze-all  # Enhanced analysis

# Full content + training update
python -m ai canonize plan expand  # Content generation
cargo run --bin hbf-analyzer analyze-all  # Enhanced categorization
```

### Enhanced Binary Updates
```bash
# Full rebuild with training enhancement
cargo build --release
cp target/release/hbf-analyzer game_release/
cp target/release/ron-generator game_release/
cp target/release/replit-prompter game_release/
cp -r training_data/ game_release/
```

## Security Considerations

### API Keys
- Never commit to repository
- Use environment variables
- Document in .env.example

### Training Data Security
- Training examples versioned in git
- No sensitive content in training files
- TOML files human-readable for security review

### User Data
- Saves in user directory only
- No network features planned
- No telemetry or analytics

## Enhanced Monitoring & Debugging

### Training Data Development Tools
- TOML syntax validation
- Training example effectiveness measurement
- Pattern recognition success rate tracking
- Categorization accuracy monitoring

### Enhanced Logging
```rust
// Training data loading
log::info!("Loaded {} training examples from TOML files", training_repo.total_examples());

// Enhanced categorization
log::info!("Categorization Success Rate: {}%", success_rate);

// Pattern recognition
log::debug!("Training pattern matched: {} → {}", entity_name, category);
```

### Training Data Profiling
- Training data loading performance
- Pattern recognition efficiency measurement
- Enhanced categorization impact analysis
- Training example effectiveness tracking

## Future Technical Considerations

### Training Data Enhancements
- Automated training example generation from successful categorizations
- Machine learning pattern recognition for training refinement
- Training data validation and quality assurance tools
- Advanced pattern matching with fuzzy recognition

### Enhanced Pipeline Optimizations
- Training data caching for improved performance
- Parallel training data processing
- Advanced HTML parsing with ML enhancement
- Pattern recognition optimization algorithms

### Scalability Improvements
- Training data hot-reloading without restart
- Distributed training data repositories
- Training example sharing between development teams
- Advanced categorization with ensemble methods

## Migration Notes

### From Basic to Training-Enhanced Categorization
- Added TOML training data files
- Enhanced entity categorization with 9 categories
- Implemented pattern recognition with training examples
- Integrated horror RPG themes throughout training system

### From Massive CSV to Split Reporting
- Replaced massive CSV exports with 100-entity samples
- Added pattern analysis files for categorization insights
- Implemented manageable reporting preventing system overload
- Maintained full export capability on explicit request

This enhanced technical context provides the foundation for understanding Dragon's Labyrinth's comprehensive training data feedback loop system with self-improving HBF processing capabilities.
