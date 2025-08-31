"""
Unified Entity Models - Simple 5-table architecture for Godot integration.

Replaces complex 50+ table architecture with simple tables that match
Godot autoload script expectations. Direct SQLite integration via godot-sqlite addon.
"""

from __future__ import annotations

from datetime import datetime
from typing import Any
from sqlmodel import SQLModel, Field, Column, JSON, DateTime


class HexTiles(SQLModel, table=True):
    """
    Hex tile spatial data for hexagon_tilemaplayer addon.
    
    Uses cube coordinates (x+y+z=0) as expected by Godot hex addon.
    Simple spatial data without complex relationships.
    """
    __tablename__ = "game_hex_tiles"
    
    # Primary identification with cube coordinates
    tile_id: str = Field(primary_key=True, description="Unique tile identifier")
    cube_x: int = Field(description="Cube coordinate X (x+y+z=0)")
    cube_y: int = Field(description="Cube coordinate Y (x+y+z=0)")  
    cube_z: int = Field(description="Cube coordinate Z (x+y+z=0)")
    
    # Simple tile properties for Godot
    biome_type: str = Field(description="Biome type (forest, mountain, etc.)")
    has_settlement: bool = Field(default=False, description="Contains settlement")
    has_dungeon: bool = Field(default=False, description="Contains dungeon")
    
    # Simple data for autoload scripts
    data: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    created_at: datetime = Field(default_factory=datetime.utcnow, sa_column=Column(DateTime))


class Entities(SQLModel, table=True):
    """
    Core entity data from ML processing.
    
    Stores all entities (NPCs, monsters, settlements, dungeons, etc.) in single table.
    Simple structure for direct godot-sqlite queries.
    """
    __tablename__ = "game_entities"
    
    # Primary identification
    entity_id: str = Field(primary_key=True, description="Unique entity identifier")
    name: str = Field(description="Entity name")
    type: str = Field(description="Entity type (npc, monster, settlement, dungeon)")
    
    # Spatial location (cube coordinates)
    hex_x: int = Field(description="Hex cube coordinate X")
    hex_y: int = Field(description="Hex cube coordinate Y") 
    hex_z: int = Field(description="Hex cube coordinate Z")
    
    # Entity data from ML processing
    data: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    created_at: datetime = Field(default_factory=datetime.utcnow, sa_column=Column(DateTime))


class Companions(SQLModel, table=True):
    """
    Companion psychology data for companion psychology system.
    
    Simple companion data for psychology autoload script.
    No complex relationships, just direct data access.
    """
    __tablename__ = "game_companions"
    
    # Primary identification
    companion_id: str = Field(primary_key=True, description="Unique companion identifier")
    name: str = Field(description="Companion name")
    
    # Psychology system data
    loyalty_level: float = Field(default=0.5, description="Loyalty level (0-1)")
    trauma_tolerance: float = Field(default=0.8, description="Trauma tolerance (0-1)")
    
    # Companion data
    data: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    created_at: datetime = Field(default_factory=datetime.utcnow, sa_column=Column(DateTime))


class Encounters(SQLModel, table=True):
    """
    Encounter data for encounter system.
    
    Simple encounter definitions for direct Godot loading.
    Spatial coordinates for hex-based encounters.
    """
    __tablename__ = "game_encounters"
    
    # Primary identification
    encounter_id: str = Field(primary_key=True, description="Unique encounter identifier")
    name: str = Field(description="Encounter name")
    type: str = Field(description="Encounter type (combat, dialogue, event)")
    
    # Spatial location
    hex_x: int = Field(description="Encounter hex cube coordinate X")
    hex_y: int = Field(description="Encounter hex cube coordinate Y")
    hex_z: int = Field(description="Encounter hex cube coordinate Z")
    
    # Encounter data
    data: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    created_at: datetime = Field(default_factory=datetime.utcnow, sa_column=Column(DateTime))


class Assets(SQLModel, table=True):
    """
    Asset data for asset catalog system.
    
    Simple asset references with entity associations.
    Direct asset loading for Godot scenes.
    """
    __tablename__ = "game_assets"
    
    # Primary identification
    asset_id: str = Field(primary_key=True, description="Unique asset identifier")
    entity_id: str = Field(description="Associated entity ID")
    
    # Asset properties
    asset_path: str = Field(description="Asset file path")
    asset_type: str = Field(description="Asset type (sprite, model, audio)")
    
    # Asset data
    data: dict[str, Any] = Field(default_factory=dict, sa_column=Column(JSON))
    created_at: datetime = Field(default_factory=datetime.utcnow, sa_column=Column(DateTime))


# Simple function to create all tables
def create_all_tables(engine):
    """Create all tables using the simple 5-table schema."""
    SQLModel.metadata.create_all(engine)


# Simple function to get table counts (using modern SQLModel syntax)
def get_table_stats(session):
    """Get simple statistics about table contents."""
    from sqlmodel import select
    
    return {
        "hex_tiles": len(session.exec(select(HexTiles)).all()),
        "entities": len(session.exec(select(Entities)).all()), 
        "companions": len(session.exec(select(Companions)).all()),
        "encounters": len(session.exec(select(Encounters)).all()),
        "assets": len(session.exec(select(Assets)).all())
    }
