# Dragon's Labyrinth
## A Horror RPG That Inverts the Power Fantasy

[![Godot](https://img.shields.io/badge/Godot-4.4+-blue.svg)](https://godotengine.org/)
[![Python](https://img.shields.io/badge/Python-3.11+-green.svg)](https://python.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> *"It's a horror RPG. You can feel it like a weight, a chill in the air."*

Dragon's Labyrinth is a revolutionary horror RPG that uses familiar RPG mechanics to deliver genuine psychological terror. Instead of growing stronger, you grow more cursed as you journey toward an inevitable confrontation with an ancient dragon.

![Game Banner - Coming Soon]()

## 🎯 Core Innovation: Inverted Power Progression

**Traditional RPGs:** Weak → Strong → Victory  
**Dragon's Labyrinth:** Hopeful → Cursed → Hunted

- **Mathematical Horror**: Dread increases with distance from your starting door (levels 1-180)
- **Companion Psychology**: Sophisticated trauma system where NPCs break under mounting pressure  
- **Proximity Terror**: The dragon actively hunts you in final encounters
- **Moral Weight**: Every choice affects companions you've grown to care about

## 🌍 Technical Architecture

### Infinite Algorithmic World
- **Hex-Based Overworld**: Infinite procedural world using cube coordinates
- **Layer Cake Tile System**: Base biome + path overlay + interactive features
- **Dual Perspective**: 2.5D hex exploration + 3D first-person horror sequences
- **Database-Driven Content**: 50+ SQLModel tables generating real-time game content

### Revolutionary Content Pipeline
```
HBF Worldbuilding → Python ML Generation → SQLite Database → Godot Resources
```

**Key Technologies:**
- **Godot 4.4+**: Game engine with custom addons
- **Python 3.11+**: ML-driven content generation with OpenAI integration
- **SQLite**: Database with 50+ tables across 8 integrated systems
- **Modern Python**: Type hints, SQLModel, comprehensive cross-system coordination

## 🎮 Game Systems

### Horror Progression (Distance-Based)
- **Peace** (0-20 hex): Everything seems normal, birds singing
- **Unease** (20-40 hex): Something feels wrong, shadows longer
- **Dread** (40-60 hex): Open acknowledgment of approaching terror
- **Terror** (60-120 hex): Reality becomes unreliable, companions breaking
- **Horror** (120+ hex): Hunted by incomprehensible intelligence

### Companion Psychology System
- **Trauma Mechanics**: Companions develop PTSD, phobias, breakdowns
- **Therapy System**: Help companions process trauma through dialogue
- **Abandonment Risk**: Companions may flee or turn against you
- **Emotional Investment**: Deep relationships make loss meaningful

### The Dragon Encounter
- **Not a Boss Fight**: The dragon is a hunter, you are prey
- **First-Person Terror**: Sudden perspective shift to survival horror
- **Proximity Audio**: Spatial sound reveals dragon's location
- **Multiple Endings**: Based on understanding, not combat victory

## 🛠️ Technical Implementation

### Critical Addons Integration
- **hexagon_tilemaplayer**: Infinite hex world with cube coordinate system
- **godot-sqlite**: 50+ table database integration for real-time content
- **dialogic**: Companion psychology dialogue system
- **limboai**: Dragon stalking AI and companion trauma responses

### Python Generator Architecture (Complete ✅)
```python
src/generator/
├── entities/     # 13 tables: NPCs, monsters, biomes from worldbuilding
├── seeds/        # 8 tables: Literature analysis, emotional patterns  
├── psychology/   # 4 tables: ML companion psychology using entities + seeds
├── world/        # 5 tables: Master coordination using all systems
├── maps/         # 5 tables: Hex grid with entity placement + corruption
├── encounters/   # 5 tables: Cross-system scenario generation
├── sprites/      # 5 tables: Character rosters with trauma systems
└── assets/       # 5 tables: OpenAI integration with comprehensive context
```

**Status**: 🏆 **COMPREHENSIVE DATABASE ARCHITECTURE 100% COMPLETE**

### Development Tools
```bash
# Install dependencies
pip install -e .

# Generate world content
python -m src.generator

# Run Godot development
godot --path . --editor

# Test database integration
python -c "from src.generator import run; run()"
```

## 🎨 Asset Generation

### AI-Generated Content (Layer Cake System)
- **Consistent Art Style**: All assets generated with unified parameters
- **Horror Progression**: Visual corruption increases with distance
- **Cross-System Enhancement**: Each system enriches AI prompts for others
- **No Licensing Issues**: 100% original AI-generated content

### Content Categories
- **Hex Tiles**: Biome bases with adjacency rules (no lava next to snow)
- **Creatures**: Mathematical horror progression (Tainted → Corrupted → Nightmare → Unspeakable)
- **NPCs**: Psychology-driven characters with trauma responses
- **Environmental Audio**: Spatial horror soundscapes
- **UI Elements**: Horror-appropriate interface design

## 📖 World Lore: The Lands of Vo'il

### Rich Worldbuilding Foundation
- **600+ Hex Tiles**: Across 27 named regions with political complexity
- **245+ Entities**: NPCs, monsters, locations with full backstories
- **5 Major Factions**: Territorial disputes and alliance networks
- **Geographic Logic**: Natural barriers, trade routes, biome consistency

### The Journey Structure
- **Your Front Door**: First-person opening that haunts players hours later
- **Infinite Exploration**: Hex world extends forever, corruption spreads mathematically  
- **Regional Progression**: 15 major regions with unique horror themes
- **The Dragon's Labyrinth**: Final first-person horror sequence where you're hunted

## 🚀 Getting Started

### Prerequisites
- **Godot 4.4+**: Download from [godotengine.org](https://godotengine.org/)
- **Python 3.11+**: For content generation system
- **Git**: For version control and addon management

### Quick Start
```bash
# Clone the repository
git clone https://github.com/your-username/dragons-labyrinth.git
cd dragons-labyrinth

# Install Python dependencies
pip install -e .

# Generate initial world content (optional)
python -m src.generator

# Open in Godot
godot --path . --editor

# Run the game
godot --path . --main-pack
```

### First Time Setup
1. **Addon Configuration**: Verify hexagon_tilemaplayer and godot-sqlite are enabled
2. **Database Generation**: Run Python generator to create initial content
3. **Test Systems**: Verify hex grid loading and horror progression
4. **Start Playing**: Open your front door and begin the journey

## 🎯 Development Status

### ✅ Completed Systems
- **Python Generator Architecture**: 50+ table comprehensive database complete
- **Cross-System ML Integration**: Revolutionary coordination across 8 subpackages
- **OpenAI Asset Generation**: Working API with cross-system context enhancement
- **Addon Integration**: Critical addons installed and configured
- **Horror Progression Mathematics**: Distance-based corruption algorithms

### 🔄 Current Development
- **Godot Code Transformation**: Converting OpenRPG foundation to horror RPG
- **Database Integration**: Connecting Python generator to Godot via SQLite
- **Hex Grid Implementation**: Layer cake tile system with cube coordinates
- **Companion Psychology**: Trauma/therapy mechanics in Godot
- **Horror Orchestration**: Dread system affecting all game systems

### 🎯 Next Milestones
1. **Core Game Loop**: Hex exploration with horror progression
2. **Companion System**: Full psychology implementation with dialogue
3. **Dragon AI**: Stalking behavior and proximity detection
4. **First-Person Transition**: Seamless perspective shift for labyrinth
5. **Audio Integration**: Spatial horror soundscape system

## 🤝 Contributing

Dragon's Labyrinth is built for the community of developers interested in:
- **Horror Game Design**: Psychological terror through familiar mechanics
- **Procedural Generation**: Infinite content with mathematical consistency
- **AI Integration**: ML-driven game content with cross-system coordination
- **Advanced Godot Techniques**: Hex grids, database integration, dual perspectives

### Development Philosophy
> **"The game is what matters. Everything else is just tooling."**

- **Horror-First Design**: Every system serves the emotional journey
- **Continuous Execution**: Implement immediately, perfect iteratively  
- **Cross-System Integration**: Components enhance each other
- **Mathematical Consistency**: Horror progression through algorithms

### Code Standards
- **Modern Python**: Type hints (`dict[str, Any]`, `str | None`), imports at top
- **GDScript Style**: Follow Godot conventions with static typing
- **Horror Coherence**: All changes must serve the horror progression
- **Performance First**: Infinite world requires memory optimization

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **HBF Worldbuilding**: Foundation worldbuilding data from comprehensive analysis
- **Godot Community**: Engine and addon developers
- **Python ML Ecosystem**: SQLModel, OpenAI, Rich console interfaces
- **Horror Game Pioneers**: Inspiration from psychological horror innovations

---

<p align="center">
  <strong>🐉 Enter the Labyrinth. Face the Dragon. Discover the Truth. 🐉</strong>
</p>

<p align="center">
  <em>"You open your front door to a beautiful morning. Birds singing, sun shining.<br/>
  Everything is perfect. Too perfect.<br/>
  By the time you realize this isn't a normal RPG, it may be too late to turn back."</em>
</p>
