# Dragon's Labyrinth - Bevy Agent Specification

## Core Game Concept
A horror RPG with inverted power mechanics where players grow weaker as they progress, exploring an infinite hex-based world with psychological horror through companion trauma and mathematical progression of dread.

## Key Mechanics

### Inverted Power System
- Health is currency - every action costs HP
- Winning battles makes you permanently weaker (reduce max HP)
- Progression bands increase curse level (1-20 Peace, 21-40 Unease, 41-60 Dread, 61-120 Terror, 121-180 Horror)
- Distance from origin determines difficulty and horror intensity

### Hex-Based World
- Infinite procedural hex map extending in all directions
- Movement: Q/W/E/A/S/D for six hex directions
- Tiles loaded from pre-generated worldbook.json
- Biomes transition based on progression bands
- Points of Interest (villages, shrines, lairs, ruins, dungeons, camps, forges, portals)

### Companion System
- NPCs accumulate psychological trauma from events
- Trauma affects dialogue, behavior, and may cause betrayal
- Permanent scars that can't be healed
- Redemption possible through Forge system at great cost

### Combat System
- No traditional XP or leveling
- Every action in combat costs HP (attack, defend, flee)
- Enemies defined in worldbook.json, not generated at runtime
- Victory reduces max HP permanently
- Death is retreat, not game over

## Technical Requirements

### Dependencies
- bevy = "0.16"
- bevy_ecs_tilemap = "0.16" (for efficient hex rendering)
- avian2d = "0.2" (for physics-based movement and collisions)
- mapgen = "0.6" (for dungeon generation algorithms at POIs)
- serde/serde_json (for loading worldbook.json)

### Architecture Patterns

#### ECS Bridge Pattern
1. Load worldbook.json at startup
2. Transform JSON data into ECS components dynamically
3. Systems query components, never the worldbook directly
4. No runtime generation - everything pre-determined

#### Physics-First Movement
- Player entity with RigidBody::Dynamic and Collider
- Movement via LinearVelocity, not Transform manipulation
- NPCs/creatures use Sensor colliders for interactions
- CollisionStarted events trigger encounters

#### Content Pipeline
- Python generates worldbook.json with all world data
- Python generates texture atlas for biomes/POIs
- Rust/Bevy only loads and uses pre-generated content
- Hot-reload support (R key) for rapid iteration

### Core Systems Needed

1. **World Loading System**
   - Parse worldbook.json
   - Create hex tile entities with components
   - Spawn NPCs, creatures, POIs from data

2. **Physics Movement System**
   - Handle Q/W/E/A/S/D input
   - Apply velocity to player RigidBody
   - Smooth hex-based movement

3. **Combat System**
   - Query creatures at player position
   - Health-as-currency mechanics
   - Inverted progression (reduce max HP on victory)

4. **Companion Trauma System**
   - Track trauma levels
   - Modify NPC behavior based on trauma
   - Breaking point mechanics

5. **Dungeon Generation System**
   - Use mapgen at dungeon POIs
   - Generate tiles as physics entities
   - Fog of war implementation

6. **UI Systems**
   - Health/curse display
   - Dialogue system
   - Shop interface
   - Quest log

## Data Structure (worldbook.json)
```json
{
  "regions": [
    {
      "band": "Peace",
      "tiles": [
        {"q": 0, "r": 0, "biome": "wet_meadow", "poi": "village"}
      ],
      "npcs": [
        {"id": "vicar", "name": "Under-Vicar Marn", "q": 0, "r": 0}
      ],
      "creatures": [
        {"id": "lost_soul", "name": "Lost Villager", "q": 1, "r": 0, "hp": 20}
      ]
    }
  ]
}
```

## Visual Style
- Muted, painterly hex tiles (256x256)
- Fog and shadow overlays increase with distance
- Simple 2D sprites for characters
- UI uses semi-transparent overlays

## Controls
- Q/W/E/A/S/D - Hex movement
- Space - Interact with NPCs
- T - Open shop
- Q - Quest log
- Enter - Enter dungeon
- Esc - Exit dungeon
- R - Hot reload worldbook

## Success Criteria
- Smooth physics-based hex movement
- Combat that weakens the player
- NPCs with persistent trauma
- Dungeons generated at runtime using mapgen
- All content loaded from worldbook.json
- No runtime generation of world content
