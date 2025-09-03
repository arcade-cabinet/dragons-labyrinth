```python
# Absolute imports - NO wildcards

from typing import List, Dict, Any, Optional
from pydantic import BaseModel, Field

from generator.processors.models.dungeons import MonsterStatBlock
from generator.processors.models.dungeons import TreasureItem
from generator.processors.models.dungeons import Trap
from generator.processors.models.dungeons import DungeonArea


class DungeonConnection(BaseModel):
    """
    Represents a connection between two dungeon areas (e.g., door, passage, stairway, teleporter).
    """
    connection_uuid: str = Field(..., description="Unique identifier for this connection")
    from_area_uuid: str = Field(..., description="UUID of the source DungeonArea")
    to_area_uuid: str = Field(..., description="UUID of the destination DungeonArea")
    connection_type: str = Field(..., description="Type of connection (door, passage, stairs, teleporter, etc.)")
    description: Optional[str] = Field(None, description="Narrative or mechanical notes about the connection")
    is_locked: bool = Field(False, description="Whether the connection is locked or requires a key")
    trap: Optional[Trap] = Field(None, description="Trap associated with this connection, if any")


class DungeonProgressionNode(BaseModel):
    """
    Represents a key progression point in the dungeon (entry, boss, treasure vault, etc.).
    """
    area_uuid: str = Field(..., description="UUID of the DungeonArea")
    node_type: str = Field(..., description="Type of progression node (entry, boss, treasure, exit, etc.)")
    description: Optional[str] = Field(None, description="Narrative or mechanical notes about this node")
    recommended_level: Optional[int] = Field(None, description="Recommended party level for this node")


class OverworldLink(BaseModel):
    """
    Represents the connection between the dungeon and the overworld (e.g., hex tile entrance).
    """
    overworld_hex_id: str = Field(..., description="ID of the overworld hex tile")
    entrance_area_uuid: str = Field(..., description="UUID of the DungeonArea that serves as the entrance")
    description: Optional[str] = Field(None, description="Narrative description of the entrance")


class DungeonContainer(BaseModel):
    """
    Master model integrating all dungeon areas, encounters, treasures, connections, and progression.
    """
    dungeon_uuid: str = Field(..., description="Unique identifier for this dungeon complex")
    name: str = Field(..., description="Dungeon name")
    description: str = Field(..., description="High-level narrative description and lore")
    areas: List[DungeonArea] = Field(..., description="All areas (rooms, passages, chambers) in the dungeon")
    connections: List[DungeonConnection] = Field(..., description="All connections between areas")
    progression: List[DungeonProgressionNode] = Field(..., description="Key progression nodes (entry, boss, treasure, exit)")
    overworld_link: OverworldLink = Field(..., description="Link to overworld hex grid")
    monsters: List[MonsterStatBlock] = Field(..., description="All monster stat blocks present in the dungeon")
    treasures: List[TreasureItem] = Field(..., description="All treasure items distributed throughout the dungeon")
    traps: List[Trap] = Field(..., description="All traps present in the dungeon")
    navigation_map: Dict[str, List[str]] = Field(
        ..., 
        description="Mapping of area UUIDs to lists of connected area UUIDs for navigation"
    )
    area_encounters: Dict[str, List[MonsterStatBlock]] = Field(
        ..., 
        description="Mapping of area UUIDs to lists of monster encounters"
    )
    area_treasures: Dict[str, List[TreasureItem]] = Field(
        ..., 
        description="Mapping of area UUIDs to lists of treasure items"
    )
    area_traps: Dict[str, List[Trap]] = Field(
        ..., 
        description="Mapping of area UUIDs to lists of traps"
    )
    area_descriptions: Dict[str, str] = Field(
        ..., 
        description="Mapping of area UUIDs to narrative descriptions (environmental storytelling, lore, etc.)"
    )
    difficulty_by_depth: Dict[int, str] = Field(
        ..., 
        description="Mapping of dungeon depth/level to difficulty rating or description"
    )
    plot_hooks: List[str] = Field(
        default_factory=list,
        description="Narrative plot hooks and storylines tied to the dungeon"
    )
    metadata: Dict[str, Any] = Field(
        default_factory=dict,
        description="Additional metadata, such as creation date, designer notes, tags, etc."
    )
```

---

**Explanation of Integration:**

- **areas**: List of all `DungeonArea` objects (rooms, passages, chambers, etc.).
- **connections**: List of `DungeonConnection` objects, each mapping two areas and describing the connection (door, passage, stairs, etc.), including traps.
- **progression**: List of `DungeonProgressionNode` objects, marking entry, boss, treasure, and exit points for progression tracking and difficulty scaling.
- **overworld_link**: `OverworldLink` object connecting the dungeon to the overworld hex grid.
- **monsters/treasures/traps**: Aggregated lists of all monsters, treasures, and traps present in the dungeon, for quick reference and integration.
- **navigation_map**: Dict mapping each area UUID to a list of connected area UUIDs, enabling area-to-area movement and layout navigation.
- **area_encounters/area_treasures/area_traps**: Dicts mapping area UUIDs to their respective monsters, treasures, and traps, integrating combat and economic systems.
- **area_descriptions**: Dict mapping area UUIDs to narrative/environmental descriptions for storytelling and lore.
- **difficulty_by_depth**: Dict mapping dungeon depth (e.g., level/floor number) to difficulty rating, supporting progression and encounter scaling.
- **plot_hooks**: List of narrative hooks and storylines for integration with the overworld and campaign.
- **metadata**: Arbitrary metadata for extensibility.

This model provides a comprehensive, extensible container for all dungeon gameplay, navigation, combat, treasure, and narrative integration.