"""
Simple Entity Integration - Direct population of 5-table schema.

Replaces complex sprites/world/encounters/maps integration modules with 
single simple integration that populates 5 tables directly from 
REAL ML-processed entity data. No fake data, no JSON world_hooks.
"""

from __future__ import annotations

import hashlib
from typing import Any

from sqlmodel import Session, select
from generator.entities.models import HexTiles, Entities, Companions, Encounters, Assets
from generator.statistics import RunStatistics


def integrate_from_entities_processors(
    session: Session, 
    processing_results: dict[str, Any]
) -> RunStatistics:
    """
    Simple integration that populates 5 tables directly from entity processor results.
    
    Uses REAL ML-processed entity data, not fake placeholder data.
    Direct table population for godot-sqlite access.
    
    Args:
        session: SQLModel database session
        processing_results: Results from entities processors (regions, settlements, factions, dungeons)
        
    Returns:
        RunStatistics with integration results
    """
    
    run_stats = RunStatistics(subpackage="entities")
    
    hex_tiles_created = 0
    entities_created = 0
    companions_created = 0
    encounters_created = 0
    assets_created = 0
    
    # Process each processor result
    for processor_type, processor_result in processing_results.items():
        if "error" in processor_result:
            run_stats.add_error(f"{processor_type}: {processor_result['error']}")
            continue
            
        try:
            # Get ML results - REQUIRE real data
            if "ml_results" not in processor_result:
                run_stats.add_error(f"No ML results for {processor_type} - skipping fake data")
                continue
                
            if "cluster_name" not in processor_result:
                run_stats.add_error(f"No cluster name for {processor_type}")
                continue
            
            ml_results = processor_result["ml_results"]
            cluster_name = processor_result["cluster_name"]
            
            if "entities" not in ml_results:
                run_stats.add_error(f"No entities in ML results for {processor_type}")
                continue
            
            entities_data = ml_results["entities"]
            if not entities_data:
                run_stats.add_error(f"Empty entities list for {processor_type}")
                continue
            
            # Process each REAL entity from ML results
            for entity_data in entities_data:
                if not isinstance(entity_data, dict):
                    continue
                    
                if "uuid" not in entity_data:
                    continue
                    
                entity_uuid = entity_data["uuid"]
                extracted_data = entity_data.get("extracted_data", {})
                
                # Generate cube coordinates from cluster name (simple hash-based)
                coords = _generate_cube_coordinates(cluster_name, entity_uuid)
                
                # Create hex tile if needed
                hex_tile_created = _create_hex_tile_if_needed(session, coords, processor_type, extracted_data)
                if hex_tile_created:
                    hex_tiles_created += 1
                
                # Create entity record
                entity_created = _create_entity_record(session, entity_uuid, cluster_name, processor_type, coords, extracted_data)
                if entity_created:
                    entities_created += 1
                
                # Create companions if entity type supports it
                companion_created = _create_companion_if_applicable(session, entity_uuid, cluster_name, processor_type, extracted_data)
                if companion_created:
                    companions_created += 1
                
                # Create encounters if entity has encounter data
                encounter_created = _create_encounter_if_applicable(session, entity_uuid, cluster_name, processor_type, coords, extracted_data)
                if encounter_created:
                    encounters_created += 1
                
                # Create asset record
                asset_created = _create_asset_record(session, entity_uuid, processor_type, extracted_data)
                if asset_created:
                    assets_created += 1
            
            run_stats.items_processed += 1
            run_stats.items_stored += 1
            
        except Exception as e:
            run_stats.add_error(f"Failed to integrate {processor_type}: {str(e)}")
    
    # Store metrics
    run_stats.add_metric("hex_tiles_created", hex_tiles_created)
    run_stats.add_metric("entities_created", entities_created)
    run_stats.add_metric("companions_created", companions_created)
    run_stats.add_metric("encounters_created", encounters_created)
    run_stats.add_metric("assets_created", assets_created)
    run_stats.add_metric("database_ready_for_godot_sqlite", True)
    
    session.commit()
    run_stats.finish()
    return run_stats


def _generate_cube_coordinates(cluster_name: str, entity_uuid: str) -> tuple[int, int, int]:
    """Generate cube coordinates from cluster name and entity UUID."""
    
    # Use hash to generate consistent coordinates
    hash_input = f"{cluster_name}_{entity_uuid}"
    hash_value = int(hashlib.md5(hash_input.encode()).hexdigest(), 16)
    
    # Generate x and y, calculate z to maintain x+y+z=0
    x = (hash_value % 40) - 20  # Range -20 to 19
    y = ((hash_value >> 8) % 40) - 20  # Range -20 to 19
    z = -(x + y)  # Ensure x+y+z=0
    
    return (x, y, z)


def _create_hex_tile_if_needed(session: Session, coords: tuple[int, int, int], processor_type: str, extracted_data: dict[str, Any]) -> bool:
    """Create hex tile if it doesn't exist."""
    
    x, y, z = coords
    tile_id = f"hex_{x}_{y}_{z}"
    
    # Check if tile already exists
    existing = session.exec(select(HexTiles).where(HexTiles.tile_id == tile_id)).first()
    if existing:
        return False
    
    # Determine biome from processor type and extracted data
    biome_type = _determine_biome_type(processor_type, extracted_data)
    
    # Create new hex tile
    hex_tile = HexTiles(
        tile_id=tile_id,
        cube_x=x,
        cube_y=y,
        cube_z=z,
        biome_type=biome_type,
        has_settlement=(processor_type == "settlements"),
        has_dungeon=(processor_type == "dungeons"),
        data={
            "processor_type": processor_type,
            "distance_from_origin": abs(x) + abs(y) + abs(z),
            "extracted_data_summary": _summarize_extracted_data(extracted_data)
        }
    )
    
    session.add(hex_tile)
    return True


def _create_entity_record(session: Session, entity_uuid: str, cluster_name: str, processor_type: str, coords: tuple[int, int, int], extracted_data: dict[str, Any]) -> bool:
    """Create entity record from ML-processed data."""
    
    x, y, z = coords
    
    # Determine entity type from processor
    entity_type = _map_processor_to_entity_type(processor_type)
    
    # Extract name from data
    entity_name = _extract_entity_name(cluster_name, extracted_data)
    
    entity = Entities(
        entity_id=entity_uuid,
        name=entity_name,
        type=entity_type,
        hex_x=x,
        hex_y=y,
        hex_z=z,
        data={
            "cluster_name": cluster_name,
            "processor_type": processor_type,
            "extracted_data": extracted_data,
            "ml_confidence": extracted_data.get("confidence", 0.0)
        }
    )
    
    session.add(entity)
    return True


def _create_companion_if_applicable(session: Session, entity_uuid: str, cluster_name: str, processor_type: str, extracted_data: dict[str, Any]) -> bool:
    """Create companion record if entity can be a companion."""
    
    # Only settlements and factions can provide companions
    if processor_type not in ["settlements", "factions"]:
        return False
    
    # Check if extracted data indicates NPC presence
    npcs = extracted_data.get("npcs", [])
    if not npcs:
        return False
    
    # Create companion for first NPC
    first_npc = npcs[0] if isinstance(npcs, list) and npcs else {}
    if not isinstance(first_npc, dict):
        return False
    
    companion_name = first_npc.get("name", cluster_name)
    
    companion = Companions(
        companion_id=f"{entity_uuid}_companion",
        name=companion_name,
        loyalty_level=0.6,  # Default starting loyalty
        trauma_tolerance=0.7,  # Default trauma tolerance
        data={
            "source_entity": entity_uuid,
            "source_cluster": cluster_name,
            "npc_data": first_npc
        }
    )
    
    session.add(companion)
    return True


def _create_encounter_if_applicable(session: Session, entity_uuid: str, cluster_name: str, processor_type: str, coords: tuple[int, int, int], extracted_data: dict[str, Any]) -> bool:
    """Create encounter record if entity has encounter data."""
    
    x, y, z = coords
    
    # Check for encounter data in extracted_data
    encounters_data = extracted_data.get("encounters", [])
    if not encounters_data:
        return False
    
    # Create encounter for first encounter data
    first_encounter = encounters_data[0] if isinstance(encounters_data, list) and encounters_data else {}
    if not isinstance(first_encounter, dict):
        return False
    
    encounter_name = first_encounter.get("name", f"{cluster_name} Encounter")
    encounter_type = _determine_encounter_type(processor_type, first_encounter)
    
    encounter = Encounters(
        encounter_id=f"{entity_uuid}_encounter",
        name=encounter_name,
        type=encounter_type,
        hex_x=x,
        hex_y=y,
        hex_z=z,
        data={
            "source_entity": entity_uuid,
            "source_cluster": cluster_name,
            "encounter_data": first_encounter,
            "processor_type": processor_type
        }
    )
    
    session.add(encounter)
    return True


def _create_asset_record(session: Session, entity_uuid: str, processor_type: str, extracted_data: dict[str, Any]) -> bool:
    """Create asset record for entity."""
    
    # Generate asset path based on processor type
    asset_path = f"res://art/{processor_type}/{entity_uuid}.png"
    
    asset = Assets(
        asset_id=f"{entity_uuid}_asset",
        entity_id=entity_uuid,
        asset_path=asset_path,
        asset_type="sprite",
        data={
            "processor_type": processor_type,
            "generated": False,  # Will be set to True when asset is actually generated
            "extracted_data_summary": _summarize_extracted_data(extracted_data)
        }
    )
    
    session.add(asset)
    return True


def _determine_biome_type(processor_type: str, extracted_data: dict[str, Any]) -> str:
    """Determine biome type from processor and data."""
    
    # Check if biome is explicitly mentioned in extracted data
    if "biome" in extracted_data:
        return extracted_data["biome"]
    
    # Default biomes by processor type
    biome_map = {
        "regions": "forest",
        "settlements": "plains", 
        "factions": "plains",
        "dungeons": "underground"
    }
    
    return biome_map.get(processor_type, "forest")


def _map_processor_to_entity_type(processor_type: str) -> str:
    """Map processor type to entity type."""
    
    type_map = {
        "regions": "region",
        "settlements": "settlement",
        "factions": "faction", 
        "dungeons": "dungeon"
    }
    
    return type_map.get(processor_type, "unknown")


def _extract_entity_name(cluster_name: str, extracted_data: dict[str, Any]) -> str:
    """Extract meaningful name from extracted data."""
    
    # Try various name fields
    for field in ["name", "title", "label"]:
        if field in extracted_data and extracted_data[field]:
            return str(extracted_data[field])
    
    # Fallback to cluster name
    return cluster_name


def _determine_encounter_type(processor_type: str, encounter_data: dict[str, Any]) -> str:
    """Determine encounter type from processor and encounter data."""
    
    if "type" in encounter_data:
        return encounter_data["type"]
    
    # Default encounter types by processor
    type_map = {
        "dungeons": "combat",
        "settlements": "dialogue",
        "factions": "dialogue",
        "regions": "event"
    }
    
    return type_map.get(processor_type, "event")


def _summarize_extracted_data(extracted_data: dict[str, Any]) -> dict[str, Any]:
    """Create summary of extracted data for storage."""
    
    summary = {}
    
    # Count various data types
    for key, value in extracted_data.items():
        if isinstance(value, list):
            summary[f"{key}_count"] = len(value)
        elif isinstance(value, dict):
            summary[f"{key}_keys"] = list(value.keys())[:5]  # First 5 keys only
        elif isinstance(value, str) and len(value) > 100:
            summary[f"{key}_length"] = len(value)
        else:
            summary[key] = value
    
    return summary


def get_simple_statistics(session: Session) -> dict[str, Any]:
    """Get simple statistics about the 5-table database."""
    
    from generator.entities.models import get_table_stats
    
    table_stats = get_table_stats(session)
    
    # Add some basic relationship stats
    entities_with_coords = len(session.exec(select(Entities).where(
        Entities.hex_x.is_not(None),
        Entities.hex_y.is_not(None), 
        Entities.hex_z.is_not(None)
    )).all())
    
    return {
        **table_stats,
        "entities_with_coordinates": entities_with_coords,
        "database_schema": "5_simple_tables",
        "godot_integration_ready": True,
        "hexagon_tilemaplayer_compatible": True,
        "godot_sqlite_compatible": True
    }
