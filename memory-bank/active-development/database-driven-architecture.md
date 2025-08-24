# Database-Driven Game Engine Architecture

## Architectural Revelation: The Database IS the Engine

With the HBF integration complete and 70,000+ entities now available, we've reached a crucial architectural insight:

**game-database** = THE ACTUAL GAME ENGINE
**game-engine** = PLAYER INTERFACE FACILITATOR  
**player-database** = PLAYER STATE MANAGER (to be built)

## Current Data Richness

### World Content (game-database)
- **2,000+ Hex Tiles**: Complete world geography with corruption/dread
- **50+ Settlements**: Taverns, inns, shops with economic systems
- **30+ Dungeons**: Multi-room complexes with environmental storytelling
- **100+ NPCs**: Full personalities, dialogue trees, trade systems
- **Weather Systems**: Regional tables with seasonal mechanics
- **D&D 5e Creatures**: Complete stat blocks ready for combat
- **Faction Data**: Political relationships extracted from HBF
- **Encounter Tables**: Probability-based spawning systems

### System Capabilities Now Possible
With this data richness, game-database can contain the actual game systems:

## Proposed System Architecture

### game-database (The True Engine)
**Core Game Systems:**
- `hex_rendering`: Query tiles → bevy_ecs_tilemap rendering
- `combat_engine`: Query creatures → D&D 5e combat mechanics  
- `settlement_systems`: Query NPCs → dialogue/trade/inn mechanics
- `weather_engine`: Query weather → environmental effects
- `faction_systems`: Query relationships → political mechanics
- `dungeon_systems`: Query rooms → 3D rendering/navigation
- `encounter_spawning`: Query tables → probability-based encounters
- `corruption_spread`: Query corruption → world state updates

**Database-Driven Logic:**
```rust
// All game logic driven by database queries
impl HexRenderingSystem {
    async fn render_hex_tiles(&self, viewport: Viewport) -> Vec<TileRenderData> {
        let tiles = self.db.get_tiles_in_viewport(viewport).await?;
        tiles.into_iter().map(|tile| TileRenderData {
            position: (tile.q, tile.r),
            texture: self.get_biome_texture(&tile.biome_type),
            corruption_overlay: tile.corruption_level,
            dread_effects: tile.dread_intensity,
        }).collect()
    }
}

impl CombatSystem {
    async fn spawn_encounter(&self, hex_pos: HexPosition) -> CombatEncounter {
        let encounters = self.db.get_encounters_for_hex(hex_pos).await?;
        let encounter = encounters.roll_random();
        let creatures = self.db.get_creatures_by_name(&encounter.creature_name).await?;
        CombatEncounter::new(creatures, encounter.quantity)
    }
}

impl SettlementSystem {
    async fn enter_inn(&self, settlement_id: Uuid) -> InnInterface {
        let settlement = self.db.get_settlement(settlement_id).await?;
        let npcs = self.db.get_npcs_in_settlement(settlement_id).await?;
        let services = settlement.services.unwrap_or_default();
        let weather = self.db.get_current_weather(settlement_id).await?;
        InnInterface::new(settlement, npcs, services, weather)
    }
}
```

### game-engine (Interface Facilitator)
**Player Interface Systems:**
- Input handling (keyboard/mouse)
- Camera management
- UI rendering (menus, HUD)
- Asset loading coordination
- Bevy app setup and plugin management
- Communication bridge between databases

**Facilitator Role:**
```rust
// game-engine coordinates between systems
impl GameEngineFacilitator {
    fn handle_player_move(&mut self, direction: HexDirection) {
        // Update player position in player-database
        self.player_db.move_player(direction).await?;
        
        // Query new world state from game-database  
        let new_hex = self.game_db.get_hex_at_player_position().await?;
        let encounters = self.game_db.check_for_encounters(new_hex).await?;
        
        // Update rendering
        self.hex_renderer.update_viewport(new_hex).await?;
    }
}
```

### player-database (To Be Built)
**Player-Specific Data:**
- Save slots and player progression
- Companion relationships and trust levels
- Player choices and consequences
- Discovery state (what's been explored)
- Inventory and equipment
- Philosophy progression
- Death scars and permanent effects

**Player State Management:**
```rust
// Separate from world data
impl PlayerDatabase {
    async fn get_companion_trust(&self, companion_id: Uuid) -> f32;
    async fn record_player_choice(&self, choice: PhilosophyChoice);
    async fn update_discovery(&self, hex_pos: HexPosition, discovered: bool);
    async fn get_player_inventory(&self) -> Vec<Item>;
}
```

## Implementation Benefits

### Data-Driven Design
- **All game logic backed by rich database content**
- **No hardcoded encounters, NPCs, or locations**
- **Procedural content from HBF seamlessly integrated**
- **Weather/faction systems driven by real data**

### Separation of Concerns
- **World Logic** (game-database): Immutable, shared content
- **Player State** (player-database): Mutable, per-save-slot data  
- **Interface** (game-engine): Input/output and coordination

### Scalability
- **Multiple save slots** easily managed in player-database
- **World updates** don't affect player saves
- **Database queries** enable complex game mechanics
- **ECS integration** via bevy_sqlx for performance

## Systems to Implement

### Immediate Priority
1. **Hex Rendering System**: Query tiles → visual representation
2. **Combat Engine**: Query creatures → D&D 5e mechanics
3. **Settlement Systems**: Query NPCs → social interactions
4. **Weather Engine**: Query weather → environmental effects

### Secondary Priority  
5. **Faction Systems**: Extract and implement political mechanics
6. **Dungeon Navigation**: Query rooms → 3D map transitions
7. **Encounter Spawning**: Query tables → probability-based events
8. **Corruption Engine**: Query corruption → world state changes

### Player Systems (New Crate)
9. **Player Database**: Save slots, progression, relationships
10. **Discovery System**: What player has found/explored
11. **Inventory Management**: Player-specific items and equipment
12. **Choice Consequences**: Philosophy and dialogue impacts

## Technical Architecture

### Database Queries Drive Everything
Instead of hardcoded game logic, every system queries the database:
- Combat encounters from creature tables
- NPC dialogue from personality systems  
- Weather effects from seasonal tables
- Faction relationships from political data
- Dungeon layouts from room/doorway data

### ECS Integration via bevy_sqlx
Database entities become ECS components automatically:
```rust
// Database entity → ECS component
#[derive(Component, FromRow)]
struct Settlement {
    name: String,
    settlement_type: String,
    prosperity_level: i32,
    corruption_influence: f32,
    // ... all database fields
}

// Query database → spawn ECS entities
for settlement in db.get_all_settlements().await? {
    commands.spawn(settlement);
}
```

## Implementation Plan

### Phase 1: Core Systems (game-database)
- Hex rendering system with tile queries
- Combat system with creature stat integration  
- Settlement interaction with NPC dialogue
- Weather effects with environmental impact

### Phase 2: Advanced Systems (game-database)
- Faction/cult/militia political mechanics
- Dungeon navigation with room transitions
- Encounter spawning with probability tables
- Corruption spread with database updates

### Phase 3: Player Systems (new crate)
- player-database crate creation
- Save slot management
- Player progression tracking
- Discovery and exploration state

This architecture makes Dragon's Labyrinth truly data-driven, where the rich HBF content becomes the living, breathing world engine powered by database queries and ECS integration.
