"""Container models for spatial and relationship indexing.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

from collections import defaultdict
from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.models.base import HexKey
from generator.analysis.models.dungeons import DungeonArea
from generator.analysis.models.regions import RegionHexTile
from generator.analysis.models.settlements import SettlementEstablishment
from generator.analysis.models.factions import FactionEntity


class DungeonContainer(BaseModel):
    """Container for dungeon areas with spatial and navigation indexes."""
    dungeon_uuid: str
    dungeon_name: str | None = None
    entrance_hex: HexKey | None = None
    areas: list[DungeonArea] = Field(default_factory=list)
    
    # Indexes for efficient lookups
    neighbors: dict[str, list[str]] = Field(default_factory=dict)  # area_key -> list[area_key]
    by_area: dict[str, DungeonArea] = Field(default_factory=dict)  # area_key -> area
    by_hex: dict[HexKey, list[str]] = Field(default_factory=dict)  # hex_key -> list[area_key]

    def build_indexes(self) -> None:
        """Build spatial and navigation indexes from areas."""
        # Build area lookup
        for area in self.areas:
            if area.area_number is not None:
                area_key = f"{self.dungeon_uuid}:{area.area_number}"
                self.by_area[area_key] = area
        
        # Build neighbor connections
        nb: defaultdict[str, list[str]] = defaultdict(list)
        for area in self.areas:
            if area.area_number is not None:
                area_key = f"{self.dungeon_uuid}:{area.area_number}"
                for connected_area in area.connected_areas:
                    connected_key = f"{self.dungeon_uuid}:{connected_area}"
                    nb[area_key].append(connected_key)
        self.neighbors = dict(nb)
        
        # Build hex index
        hx: defaultdict[HexKey, list[str]] = defaultdict(list)
        for area in self.areas:
            if area.area_number is not None and area.entrance_hex:
                area_key = f"{self.dungeon_uuid}:{area.area_number}"
                hx[area.entrance_hex].append(area_key)
        self.by_hex = dict(hx)

    def get_area(self, area_number: int) -> DungeonArea | None:
        """Get area by number."""
        area_key = f"{self.dungeon_uuid}:{area_number}"
        return self.by_area.get(area_key)

    def get_connected_areas(self, area_number: int) -> list[DungeonArea]:
        """Get all areas connected to the given area."""
        area_key = f"{self.dungeon_uuid}:{area_number}"
        connected_keys = self.neighbors.get(area_key, [])
        return [self.by_area[key] for key in connected_keys if key in self.by_area]


class RegionContainer(BaseModel):
    """Container for regional entities with spatial and relationship indexes."""
    region_uuid: str
    region_name: str | None = None
    
    # Entity collections
    hex_tiles: list[RegionHexTile] = Field(default_factory=list)
    settlements: list[SettlementEstablishment] = Field(default_factory=list)
    factions: list[FactionEntity] = Field(default_factory=list)
    dungeon_containers: list[DungeonContainer] = Field(default_factory=list)
    
    # Spatial indexes
    by_hex: dict[HexKey, dict[str, list[str]]] = Field(default_factory=dict)  # hex -> {entity_type: [uuids]}
    
    # Entity indexes
    settlements_by_uuid: dict[str, SettlementEstablishment] = Field(default_factory=dict)
    factions_by_uuid: dict[str, FactionEntity] = Field(default_factory=dict)
    dungeons_by_uuid: dict[str, DungeonContainer] = Field(default_factory=dict)

    def build_indexes(self) -> None:
        """Build spatial and entity indexes from all entities."""
        # Build hex index with entity type categorization
        hx: defaultdict[HexKey, defaultdict[str, list[str]]] = defaultdict(lambda: defaultdict(list))
        
        # Index hex tiles
        for tile in self.hex_tiles:
            if tile.hex_key:
                hx[tile.hex_key]["tiles"].append(tile.entity_uuid)
                
                # Add referenced entities from the tile
                for settlement_uuid in tile.settlement_uuids:
                    hx[tile.hex_key]["settlements"].append(settlement_uuid)
                for dungeon_uuid in tile.dungeon_uuids:
                    hx[tile.hex_key]["dungeons"].append(dungeon_uuid)
                for faction_uuid in tile.faction_uuids:
                    hx[tile.hex_key]["factions"].append(faction_uuid)
        
        # Index settlements
        for settlement in self.settlements:
            self.settlements_by_uuid[settlement.entity_uuid] = settlement
            if settlement.hex_key:
                hx[settlement.hex_key]["settlements"].append(settlement.entity_uuid)
        
        # Index factions
        for faction in self.factions:
            self.factions_by_uuid[faction.entity_uuid] = faction
            if faction.stronghold_hex:
                hx[faction.stronghold_hex]["factions"].append(faction.entity_uuid)
        
        # Index dungeon containers
        for dungeon in self.dungeon_containers:
            self.dungeons_by_uuid[dungeon.dungeon_uuid] = dungeon
            if dungeon.entrance_hex:
                hx[dungeon.entrance_hex]["dungeons"].append(dungeon.dungeon_uuid)
            # Also build dungeon's internal indexes
            dungeon.build_indexes()
        
        # Convert defaultdicts to regular dicts
        self.by_hex = {k: dict(v) for k, v in hx.items()}

    def get_entities_at_hex(self, hex_key: HexKey) -> dict[str, list[Any]]:
        """Get all entities at a specific hex coordinate."""
        result: dict[str, list[Any]] = {
            "tiles": [],
            "settlements": [],
            "dungeons": [],
            "factions": []
        }
        
        hex_data = self.by_hex.get(hex_key, {})
        
        # Resolve tile entities
        for tile_uuid in hex_data.get("tiles", []):
            for tile in self.hex_tiles:
                if tile.entity_uuid == tile_uuid:
                    result["tiles"].append(tile)
                    break
        
        # Resolve settlement entities
        for settlement_uuid in hex_data.get("settlements", []):
            settlement = self.settlements_by_uuid.get(settlement_uuid)
            if settlement:
                result["settlements"].append(settlement)
        
        # Resolve dungeon entities
        for dungeon_uuid in hex_data.get("dungeons", []):
            dungeon = self.dungeons_by_uuid.get(dungeon_uuid)
            if dungeon:
                result["dungeons"].append(dungeon)
        
        # Resolve faction entities
        for faction_uuid in hex_data.get("factions", []):
            faction = self.factions_by_uuid.get(faction_uuid)
            if faction:
                result["factions"].append(faction)
        
        return result

    def get_faction_territories(self, faction_uuid: str) -> dict[str, list[str]]:
        """Get all territories controlled by a faction."""
        faction = self.factions_by_uuid.get(faction_uuid)
        if not faction:
            return {"regions": [], "settlements": []}
        
        return {
            "regions": faction.controlled_regions,
            "settlements": faction.controlled_settlements
        }
