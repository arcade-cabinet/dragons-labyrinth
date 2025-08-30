"""
Maps Integration - Receive spatial data from entities processors.

Handles spatial relationships: hex coordinates, settlements, dungeons, 
travel routes, and biome mapping from entities → maps tables.
"""

from __future__ import annotations

import json
from typing import Any

from sqlmodel import Session
from generator.maps.models import HexTiles, MapRegions
from generator.statistics import RunStatistics


def integrate_from_entities_processors(
    session: Session, 
    processing_results: dict[str, Any]
) -> RunStatistics:
    """
    Integrate spatial data from entities processors into maps tables.
    
    Args:
        session: SQLModel database session
        processing_results: Results from entities transformer/processors
        
    Returns:
        RunStatistics with integration results
    """
    
    run_stats = RunStatistics(subpackage="maps")
    
    hex_tiles_created = 0
    regions_mapped = 0
    settlements_placed = 0
    dungeons_placed = 0
    
    # Process each category from entities
    for processor_type, processor_result in processing_results.items():
        if "error" in processor_result:
            run_stats.add_error(f"{processor_type}: {processor_result['error']}")
            continue
            
        try:
            if processor_type == "regions":
                region_stats = _integrate_region_spatial_data(session, processor_result)
                hex_tiles_created += region_stats.get("hex_tiles_created", 0)
                regions_mapped += region_stats.get("regions_mapped", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "settlements":
                settlement_stats = _integrate_settlement_spatial_data(session, processor_result)
                settlements_placed += settlement_stats.get("settlements_placed", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "dungeons":
                dungeon_stats = _integrate_dungeon_spatial_data(session, processor_result)
                dungeons_placed += dungeon_stats.get("dungeons_placed", 0)
                run_stats.items_processed += 1
                
            run_stats.items_stored += 1
            
        except Exception as e:
            run_stats.add_error(f"Failed to integrate {processor_type}: {str(e)}")
    
    # Store custom metrics
    run_stats.add_metric("hex_tiles_created", hex_tiles_created)
    run_stats.add_metric("regions_mapped", regions_mapped)
    run_stats.add_metric("settlements_placed", settlements_placed)
    run_stats.add_metric("dungeons_placed", dungeons_placed)
    run_stats.add_metric("database_ready_for_godot_sqlite", True)
    
    session.commit()
    run_stats.finish()
    return run_stats


def _integrate_region_spatial_data(session: Session, region_result: dict[str, Any]) -> dict[str, int]:
    """Integrate region spatial data into maps tables."""
    
    stats = {"hex_tiles_created": 0, "regions_mapped": 0}
    
    world_hooks = region_result.get("world_hooks", {})
    region_data = region_result.get("region_data", {})
    
    # Extract hex data if available
    if "hex_tiles" in world_hooks:
        for hex_data in world_hooks["hex_tiles"]:
            hex_tile = _create_or_update_hex_tile(session, hex_data, region_data)
            if hex_tile:
                stats["hex_tiles_created"] += 1
    
    # Create region mapping
    region_name = region_data.get("name", "Unknown Region")
    if region_name != "Unknown Region":
        region_mapping = _create_or_update_region_mapping(session, region_name, region_data, world_hooks)
        if region_mapping:
            stats["regions_mapped"] += 1
    
    return stats


def _integrate_settlement_spatial_data(session: Session, settlement_result: dict[str, Any]) -> dict[str, int]:
    """Integrate settlement spatial data into maps tables."""
    
    stats = {"settlements_placed": 0}
    
    world_hooks = settlement_result.get("world_hooks", {})
    settlement_data = settlement_result.get("settlement_data", {})
    
    # Update hex tile with settlement
    if "hex_coordinate" in world_hooks:
        hex_coord = world_hooks["hex_coordinate"]
        hex_tile = session.query(HexTiles).filter(HexTiles.hex_coordinate == hex_coord).first()
        
        if hex_tile:
            hex_tile.has_settlement = True
            hex_tile.entity_count += 1
            
            # Update world_hooks with settlement data
            current_hooks = json.loads(hex_tile.world_hooks) if hex_tile.world_hooks else {}
            current_hooks.update({
                "settlement_name": settlement_data.get("name", ""),
                "settlement_scale": settlement_data.get("scale_hint", "village"),
                "settlement_services": settlement_data.get("service_types", [])
            })
            hex_tile.world_hooks = json.dumps(current_hooks)
            
            stats["settlements_placed"] += 1
    
    return stats


def _integrate_dungeon_spatial_data(session: Session, dungeon_result: dict[str, Any]) -> dict[str, int]:
    """Integrate dungeon spatial data into maps tables."""
    
    stats = {"dungeons_placed": 0}
    
    world_hooks = dungeon_result.get("world_hooks", {})
    dungeon_data = dungeon_result.get("dungeon_data", {})
    
    # Update hex tile with dungeon
    if "hex_coordinate" in world_hooks:
        hex_coord = world_hooks["hex_coordinate"]
        hex_tile = session.query(HexTiles).filter(HexTiles.hex_coordinate == hex_coord).first()
        
        if hex_tile:
            hex_tile.has_dungeon = True
            hex_tile.entity_count += 1
            
            # Update world_hooks with dungeon data
            current_hooks = json.loads(hex_tile.world_hooks) if hex_tile.world_hooks else {}
            current_hooks.update({
                "dungeon_name": dungeon_data.get("name", ""),
                "dungeon_type": dungeon_data.get("dungeon_type", "unknown"),
                "horror_intensity": dungeon_data.get("horror_intensity", "none"),
                "entrance_types": world_hooks.get("entrances", [])
            })
            hex_tile.world_hooks = json.dumps(current_hooks)
            
            stats["dungeons_placed"] += 1
    
    return stats


def _create_or_update_hex_tile(session: Session, hex_data: dict[str, Any], region_data: dict[str, Any]) -> HexTiles | None:
    """Create or update hex tile from spatial data."""
    
    # Extract coordinates
    x = hex_data.get("x", 0)
    y = hex_data.get("y", 0)
    
    # Build coordinate string
    coordinate = _build_hex_coordinate(x, y)
    
    # Check if exists
    existing = session.query(HexTiles).filter(HexTiles.hex_coordinate == coordinate).first()
    
    if existing:
        # Update existing tile
        _update_hex_tile_from_data(existing, hex_data, region_data)
        return existing
    else:
        # Create new tile
        return _create_hex_tile_from_data(session, coordinate, x, y, hex_data, region_data)


def _build_hex_coordinate(x: int, y: int) -> str:
    """Build hex coordinate string from x,y coordinates."""
    
    if x == 0 and y == 0:
        return "BASE"
    elif x > 0 and y == 0:
        return f"E{x}"
    elif x < 0 and y == 0:
        return f"W{abs(x)}"
    elif x == 0 and y > 0:
        return f"N{y}"
    elif x == 0 and y < 0:
        return f"S{abs(y)}"
    else:
        # Complex coordinates
        x_dir = "E" if x > 0 else "W"
        y_dir = "N" if y > 0 else "S"
        return f"{x_dir}{abs(x)}{y_dir}{abs(y)}"


def _create_hex_tile_from_data(
    session: Session, 
    coordinate: str, 
    x: int, 
    y: int, 
    hex_data: dict[str, Any], 
    region_data: dict[str, Any]
) -> HexTiles:
    """Create new hex tile from spatial data."""
    
    # Extract biome from type
    hex_type = hex_data.get("type", "")
    biome = hex_type.replace("Hex", "") if hex_type.endswith("Hex") else "unknown"
    
    # Determine features
    feature = hex_data.get("feature", "")
    has_settlement = feature in ["Village", "Town", "City", "Inn", "Residency"]
    has_dungeon = feature == "Dungeon"
    has_npc = bool(hex_data.get("label"))
    
    # Build world_hooks
    world_hooks = {
        "hex_uuid": hex_data.get("uuid", ""),
        "feature": feature,
        "label": hex_data.get("label", ""),
        "rivers": hex_data.get("rivers", []),
        "trails": hex_data.get("trails", []),
        "harbor": hex_data.get("harbor", False),
        "region_id": hex_data.get("region", ""),
        "realm_id": hex_data.get("realm", ""),
        "biome": biome
    }
    
    hex_tile = HexTiles(
        hex_coordinate=coordinate,
        coordinate_type="complex" if (x != 0 and y != 0) else "cardinal",
        grid_x=x,
        grid_y=y,
        distance_from_base=abs(x) + abs(y),
        base_tile_type=biome.lower(),
        biome_type=biome.lower(),
        has_settlement=has_settlement,
        has_dungeon=has_dungeon,
        has_npc=has_npc,
        entity_count=1,
        travel_routes=_extract_travel_routes(hex_data),
        world_hooks=json.dumps(world_hooks)
    )
    
    session.add(hex_tile)
    return hex_tile


def _update_hex_tile_from_data(hex_tile: HexTiles, hex_data: dict[str, Any], region_data: dict[str, Any]) -> None:
    """Update existing hex tile with new data."""
    
    # Merge world_hooks
    current_hooks = json.loads(hex_tile.world_hooks) if hex_tile.world_hooks else {}
    
    new_hooks = {
        "hex_uuid": hex_data.get("uuid", ""),
        "feature": hex_data.get("feature", ""),
        "label": hex_data.get("label", ""),
        "rivers": hex_data.get("rivers", []),
        "trails": hex_data.get("trails", []),
        "harbor": hex_data.get("harbor", False),
        "region_id": hex_data.get("region", ""),
        "realm_id": hex_data.get("realm", "")
    }
    
    current_hooks.update(new_hooks)
    hex_tile.world_hooks = json.dumps(current_hooks)
    
    # Update entity counts
    feature = hex_data.get("feature", "")
    if feature in ["Village", "Town", "City", "Inn", "Residency"]:
        hex_tile.has_settlement = True
    if feature == "Dungeon":
        hex_tile.has_dungeon = True
    if hex_data.get("label"):
        hex_tile.has_npc = True
    
    hex_tile.entity_count += 1


def _extract_travel_routes(hex_data: dict[str, Any]) -> list[str]:
    """Extract travel route types from hex data."""
    
    routes = []
    
    if hex_data.get("rivers"):
        routes.append("river")
    if hex_data.get("trails"):
        routes.append("trail")
    if hex_data.get("harbor"):
        routes.append("bridge")
        
    return routes


def _create_or_update_region_mapping(
    session: Session, 
    region_name: str, 
    region_data: dict[str, Any], 
    world_hooks: dict[str, Any]
) -> MapRegions | None:
    """Create or update region mapping."""
    
    # Check if region already exists
    existing = session.query(MapRegions).filter(MapRegions.region_name == region_name).first()
    
    if existing:
        return existing
    
    # Create new region mapping
    region_mapping = MapRegions(
        map_region_id=region_name.lower().replace(" ", "_"),
        world_region_id=world_hooks.get("region_id", ""),
        region_name=region_name,
        min_hex_x=0,  # Will be calculated when hex tiles are processed
        max_hex_x=0,
        min_hex_y=0,
        max_hex_y=0,
        center_coordinate="BASE",
        region_type="wilderness",  # Default, can be enhanced
        total_hexes=0,
        dominant_biome=region_data.get("dominant_biome", "forest"),
        corruption_average=0.0,
        entity_density=0.0
    )
    
    session.add(region_mapping)
    return region_mapping


def get_integration_statistics(session: Session) -> dict[str, Any]:
    """Get statistics about integrated spatial data for reporting."""
    
    hex_count = session.query(HexTiles).count()
    regions_count = session.query(MapRegions).count()
    
    # Count hex tiles by type
    settlement_hexes = session.query(HexTiles).filter(HexTiles.has_settlement == True).count()
    dungeon_hexes = session.query(HexTiles).filter(HexTiles.has_dungeon == True).count()
    river_hexes = session.query(HexTiles).filter(HexTiles.travel_routes.contains("river")).count()
    trail_hexes = session.query(HexTiles).filter(HexTiles.travel_routes.contains("trail")).count()
    
    return {
        "total_hex_tiles": hex_count,
        "total_regions": regions_count,
        "settlement_hexes": settlement_hexes,
        "dungeon_hexes": dungeon_hexes,
        "river_hexes": river_hexes,
        "trail_hexes": trail_hexes,
        "database_ready_for_godot_sqlite": True
    }
