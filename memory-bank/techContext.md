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
├── ai/                  # Python generation pipeline
├── apps/                # Rust applications
│   └── game/           # Main game executable
├── crates/             # Rust libraries
│   └── world/          # Game world logic
├── content/            # Source markdown files
├── build/              # Generated JSON data
└── memory-bank/        # Project documentation
```

### Build Artifacts
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
└── image_plan.json    # Asset generation plan
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
- **Rust**: Cargo workspaces
- **Python**: uv/hatch for dependencies
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
```

### Python Dependencies
```toml
# Core
openai = ">=1.50.0"          # AI integration
pydantic = ">=2.8.0"         # Data validation
typer = ">=0.12.0"           # CLI framework

# Processing
pillow = ">=10.4.0"          # Image manipulation
beautifulsoup4 = ">=4.12.3"  # HTML parsing
jinja2 = ">=3.1.4"           # Template engine

# Data
pandas = ">=2.2.0"           # Data analysis
sqlalchemy = ">=2.0.0"       # Database access
sqlmodel = ">=0.0.24"        # ORM

# Utilities
rich = ">=13.9.0"            # Terminal output
tqdm = ">=4.66.0"            # Progress bars
requests = ">=2.32.0"        # HTTP requests
```

## Build & Run Commands

### Python Content Generation
```bash
# Install dependencies
pip install -e .

# Generate game content
python -m ai canonize        # Convert markdown to canon
python -m ai plan           # Create world plan
python -m ai expand         # Generate regions
python -m ai image-plan     # Design assets
python -m ai images         # Generate tilesets
python -m ai narrative      # Expand dialogue
```

### Rust Game Development
```bash
# Build game
cargo build --release

# Run game
cargo run -p game

# Run with hot-reload testing
cargo watch -x "run -p game"
```

### Development Workflow
```bash
# 1. Edit content
vim content/Architecture.md

# 2. Regenerate
python -m ai canonize && python -m ai plan && python -m ai expand

# 3. Run game
cargo run -p game

# 4. Hot-reload test (press R in game)
```

## Architecture Decisions

### Why Rust + Bevy (not Godot)
- **Performance**: Native performance, no GDScript overhead
- **Architecture**: ECS fits our component-heavy design
- **Simplicity**: Direct JSON loading vs Godot resources
- **Control**: Full control over rendering pipeline
- **Hot-Reload**: Simpler implementation than Godot

### Why Python Generation (not Rust)
- **AI Libraries**: Better OpenAI integration
- **Flexibility**: Rapid prototyping of generation logic
- **Ecosystem**: Rich data processing libraries
- **Separation**: Clear boundary between generation and runtime

### Why JSON (not Database)
- **Simplicity**: No ORM complexity
- **Portability**: Easy to share/version
- **Debugging**: Human-readable format
- **Performance**: Fast loading at startup
- **Hot-Reload**: Simple file watching

## Performance Considerations

### Memory Usage
- **World Data**: ~10MB JSON loaded at startup
- **Textures**: Lazy-loaded as needed
- **Entities**: Component pools for efficiency
- **Target**: <500MB RAM usage

### CPU Usage
- **Hex Math**: Pre-calculated lookup tables
- **Pathfinding**: A* with caching
- **Systems**: Parallel execution where possible
- **Target**: 60 FPS on 2015+ hardware

### GPU Usage
- **2D Rendering**: Minimal requirements
- **Shaders**: Simple hex tile materials
- **Batching**: Instanced rendering for tiles
- **Target**: Intel integrated graphics

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

### Unit Tests
```bash
# Rust tests
cargo test

# Python tests
pytest ai/tests/
```

### Integration Tests
- Full pipeline execution
- World generation validation
- Save/load cycles
- Hot-reload verification

### Performance Tests
- Frame rate monitoring
- Memory leak detection
- Load time benchmarks
- Pathfinding stress tests

## Deployment

### Distribution
- **Format**: Single executable + data folder
- **Size**: ~50MB download
- **Updates**: Replace data files for content
- **Saves**: User home directory

### Content Updates
```bash
# Update content only
python -m ai canonize plan expand
cp -r build/* game_release/data/
```

### Binary Updates
```bash
# Full rebuild
cargo build --release
cp target/release/game game_release/
```

## Security Considerations

### API Keys
- Never commit to repository
- Use environment variables
- Document in .env.example

### User Data
- Saves in user directory only
- No network features planned
- No telemetry or analytics

## Known Limitations

### Current
- Single-threaded AI generation
- No multiplayer support
- Limited to 2D rendering
- English-only content

### Acceptable
- These align with project scope
- Focus on single-player horror
- 2D supports our art style
- Localization post-launch maybe

## Monitoring & Debugging

### Development Tools
- Bevy inspector egui
- Chrome DevTools for WASM
- Python debugger for generation
- VS Code integrated debugging

### Logging
```rust
// Rust
log::info!("Player moved to {:?}", pos);

// Python
logger.info(f"Generated {len(regions)} regions")
```

### Profiling
- Bevy Tracy integration
- Python cProfile for generation
- Memory profiling with Valgrind
- GPU profiling with RenderDoc

## Future Technical Considerations

### Potential Optimizations
- Texture atlasing for tiles
- Chunk-based world loading
- Background content generation
- Shader-based fog of war

### Potential Features
- Steam Workshop support
- Mod loading system
- Replay system
- Achievement integration

### Technical Debt
- Schema implementations needed
- Image generation incomplete
- Combat system placeholder
- Companion AI basic

## Migration Notes

### From Godot to Bevy
- Removed GDScript files
- Removed .tscn/.tres resources  
- Converted to ECS components
- Simplified to JSON data

### From Complex Python
- Removed generator/ subpackage mess
- Simplified to ai/ directory
- Modern Python patterns
- Clear module separation

This technical context provides the foundation for understanding and extending Dragon's Labyrinth's implementation.
