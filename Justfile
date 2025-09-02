# Dragon's Labyrinth - Project Management
# Use `just` to see all available commands

# Default command - show available recipes
default:
    @just --list

# ============================================
# SETUP & CONFIGURATION
# ============================================

# Install all dependencies (Python, Rust, bevy-agent)
install:
    @echo "Installing Python dependencies..."
    pip install -e .
    @echo "Installing Rust dependencies..."
    cargo fetch
    @echo "Installing bevy-agent..."
    cargo install bevy-agent || true
    @echo "Installing just (if needed)..."
    cargo install just || true
    @echo "✅ All dependencies installed!"

# Configure bevy-agent with OpenAI API key
configure-agent key:
    bevy-agent config --openai-key {{key}}
    @echo "✅ bevy-agent configured!"

# Check if everything is properly set up
check:
    @echo "Checking Python..."
    @python --version
    @echo "Checking Rust..."
    @cargo --version
    @echo "Checking bevy-agent..."
    @bevy-agent --version || echo "⚠️  bevy-agent not installed"
    @echo "Checking OpenAI key..."
    @[ -n "$OPENAI_API_KEY" ] && echo "✅ OPENAI_API_KEY is set" || echo "⚠️  OPENAI_API_KEY not set"

# ============================================
# WORLDBUILDING (Python AI Pipeline)
# ============================================

# Generate all world content from markdown
world-all: world-canon world-plan world-expand world-images

# Convert Architecture.md to canon.json
world-canon:
    @echo "📖 Converting Architecture.md to canon..."
    python -m ai canonize

# Generate world plan from canon
world-plan:
    @echo "🗺️  Creating world plan..."
    python -m ai plan

# Expand regions with details
world-expand:
    @echo "🌍 Expanding world regions..."
    python -m ai expand

# Generate image assets
world-images:
    @echo "🎨 Generating tileset images..."
    python -m ai image-plan
    python -m ai images

# Generate NPC dialogue
world-dialogue:
    @echo "💬 Expanding NPC dialogue..."
    python -m ai narrative

# Clean generated world data
world-clean:
    @echo "🧹 Cleaning generated world data..."
    rm -rf build/world/*.json
    rm -rf build/master/*.json
    rm -rf build/features/**/*.json
    @echo "✅ World data cleaned!"

# ============================================
# CODE GENERATION (bevy-agent)
# ============================================

# Generate all game systems with bevy-agent
generate-all: generate-base generate-movement generate-combat generate-companions generate-forge

# Create base game structure
generate-base:
    @echo "🎮 Creating base game structure..."
    bevy-agent create "Horror RPG with hex world loaded from build/world/worldbook.json using bevy_ecs_tilemap"

# Generate hex movement system
generate-movement:
    @echo "🚶 Generating hex movement system..."
    bevy-agent add "Hex movement with Q/W/E/A/S/D keys using bevy_ecs_tilemap HexCoordSystem::Row"

# Generate inverted combat system
generate-combat:
    @echo "⚔️  Generating inverted combat system..."
    bevy-agent add "Combat where attacks cost HP. Victory reduces max HP. Load enemies from worldbook.json"

# Generate companion system
generate-companions:
    @echo "👥 Generating companion trauma system..."
    bevy-agent add "Companion trauma tracking 0.0-1.0 with breaking points. Load from worldbook NPCs"

# Generate forge redemption system
generate-forge:
    @echo "🔨 Generating forge redemption system..."
    bevy-agent add "Forge at shrine POIs: trade 50% HP for companion resurrection, companion for trauma reset"

# Generate procedural dungeons
generate-dungeons:
    @echo "🏰 Generating dungeon system..."
    bevy-agent add "Procedural dungeons using mapgen at dungeon POIs from worldbook.json"

# Improve existing code
improve target="performance":
    @echo "🔧 Improving {{target}}..."
    bevy-agent improve {{target}}

# ============================================
# BUILD & RUN
# ============================================

# Build the game
build:
    @echo "🔨 Building game..."
    cargo build --release -p game

# Run the game in development mode
run:
    @echo "🎮 Running game..."
    cargo run -p game

# Run with hot-reload watching
watch:
    @echo "👁️  Running with hot-reload..."
    cargo watch -x "run -p game"

# Build and run optimized
play: build
    @echo "🎮 Running optimized build..."
    ./target/release/game

# ============================================
# TESTING
# ============================================

# Run all tests
test:
    @echo "🧪 Running tests..."
    cargo test --all
    pytest ai/tests/ || true

# Test Python AI pipeline
test-python:
    @echo "🐍 Testing Python pipeline..."
    pytest ai/tests/ -v

# Test Rust game code
test-rust:
    @echo "🦀 Testing Rust code..."
    cargo test --all

# Run clippy lints
lint:
    @echo "📝 Running clippy..."
    cargo clippy --all -- -D warnings

# Format all code
format:
    @echo "✨ Formatting code..."
    cargo fmt --all
    black ai/ || true
    @echo "✅ Code formatted!"

# ============================================
# DEVELOPMENT WORKFLOWS
# ============================================

# Complete development cycle
dev: world-all generate-all build run

# Quick iteration (skip world generation)
quick: build run

# Clean and rebuild everything
fresh: clean world-all generate-all build

# Clean all generated files
clean:
    @echo "🧹 Cleaning all generated files..."
    cargo clean
    rm -rf build/**/*.json
    rm -rf target/
    @echo "✅ All clean!"

# ============================================
# PROJECT MANAGEMENT
# ============================================

# Show project statistics
stats:
    @echo "📊 Project Statistics:"
    @echo "Lines of Rust code:"
    @find crates apps -name "*.rs" | xargs wc -l | tail -1
    @echo "Lines of Python code:"
    @find ai -name "*.py" | xargs wc -l | tail -1
    @echo "Generated world files:"
    @ls -la build/world/*.json | wc -l
    @echo "Asset files:"
    @ls -la apps/game/assets/**/* | wc -l

# Update memory bank documentation
docs:
    @echo "📚 Updating documentation..."
    @echo "Check memory-bank/ for all documentation"

# Git status
status:
    @git status -s

# Commit with conventional commit message
commit message:
    git add -A
    git commit -m "{{message}}"

# Push to origin
push:
    git push origin main

# ============================================
# ASSET MANAGEMENT
# ============================================

# Copy atlas to game assets
atlas-copy:
    @echo "📦 Copying atlas to game assets..."
    cp build/atlas/atlas.png apps/game/assets/atlas/
    cp build/atlas/atlas.json apps/game/assets/atlas/
    @echo "✅ Atlas copied!"

# List all biome tiles
list-biomes:
    @echo "🌍 Available biome tiles:"
    @ls -la apps/game/assets/biomes/

# List all POI icons
list-icons:
    @echo "📍 Available POI icons:"
    @ls -la apps/game/assets/icons/

# ============================================
# UTILITIES
# ============================================

# Open project in VS Code
code:
    code .

# Start Python REPL with project context
repl:
    python -i -c "from ai import *; print('AI modules loaded')"

# Check worldbook.json validity
validate-world:
    @echo "🔍 Validating worldbook.json..."
    python -c "import json; json.load(open('build/world/worldbook.json')); print('✅ Valid JSON')"

# Backup important files
backup:
    @echo "💾 Creating backup..."
    tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz \
        content/ \
        memory-bank/ \
        build/world/ \
        .bevy-agent.json
    @echo "✅ Backup created!"

# ============================================
# HELP & INFO
# ============================================

# Show detailed help for a command
help command:
    @just --show {{command}}

# Show project info
info:
    @echo "🐉 Dragon's Labyrinth"
    @echo "Horror RPG with inverted power mechanics"
    @echo ""
    @echo "Key Commands:"
    @echo "  just world-all    - Generate all worldbuilding"
    @echo "  just generate-all - Generate all game code"
    @echo "  just run         - Run the game"
    @echo "  just dev         - Complete dev cycle"
    @echo ""
    @echo "See 'just' for all commands"

# Open documentation
docs-open:
    @echo "Opening documentation..."
    @[ -f memory-bank/projectbrief.md ] && ${EDITOR:-code} memory-bank/projectbrief.md
