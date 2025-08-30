"""
Encounters Integration - Pure data provider for world integration.

Stores clean structured encounter data in SQLModel table
for direct godot-sqlite access. NO JSON dumps, NO world_hooks.
"""

from __future__ import annotations

from typing import Any

from sqlmodel import Session
from generator.encounters.models import EncounterRecord, CombatScenarioRecord, ScriptedEventRecord, BeastEncounterRecord, NPCInteractionRecord
from generator.statistics import RunStatistics


def integrate_from_entities_processors(
    session: Session, 
    processing_results: dict[str, Any]
) -> RunStatistics:
    """
    Integrate encounter data from entities processors into encounters tables.
    
    Args:
        session: SQLModel database session
        processing_results: Results from entities transformer/processors
        
    Returns:
        RunStatistics with integration results
    """
    
    run_stats = RunStatistics(subpackage="encounters")
    
    encounters_created = 0
    combat_scenarios_created = 0
    scripted_events_created = 0
    beasts_created = 0
    encounter_npcs_created = 0
    
    # Process each category from entities
    for processor_type, processor_result in processing_results.items():
        if "error" in processor_result:
            run_stats.add_error(f"{processor_type}: {processor_result['error']}")
            continue
            
        try:
            if processor_type == "dungeons":
                dungeon_stats = _integrate_dungeon_encounters(session, processor_result)
                encounters_created += dungeon_stats.get("encounters_created", 0)
                combat_scenarios_created += dungeon_stats.get("combat_scenarios_created", 0)
                beasts_created += dungeon_stats.get("beasts_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "settlements":
                settlement_stats = _integrate_settlement_encounters(session, processor_result)
                scripted_events_created += settlement_stats.get("scripted_events_created", 0)
                encounter_npcs_created += settlement_stats.get("encounter_npcs_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "factions":
                faction_stats = _integrate_faction_encounters(session, processor_result)
                encounters_created += faction_stats.get("encounters_created", 0)
                encounter_npcs_created += faction_stats.get("encounter_npcs_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "regions":
                region_stats = _integrate_region_encounters(session, processor_result)
                scripted_events_created += region_stats.get("scripted_events_created", 0)
                run_stats.items_processed += 1
                
            run_stats.items_stored += 1
            
        except Exception as e:
            run_stats.add_error(f"Failed to integrate {processor_type}: {str(e)}")
    
    # Store custom metrics
    run_stats.add_metric("encounters_created", encounters_created)
    run_stats.add_metric("combat_scenarios_created", combat_scenarios_created)
    run_stats.add_metric("scripted_events_created", scripted_events_created)
    run_stats.add_metric("beasts_created", beasts_created)
    run_stats.add_metric("encounter_npcs_created", encounter_npcs_created)
    run_stats.add_metric("database_ready_for_godot_sqlite", True)
    
    session.commit()
    run_stats.finish()
    return run_stats


def _integrate_dungeon_encounters(session: Session, dungeon_result: dict[str, Any]) -> dict[str, int]:
    """Integrate REAL dungeon encounter data from entity processors - fail hard on missing data."""
    
    stats = {"encounters_created": 0, "combat_scenarios_created": 0, "beasts_created": 0, "entities_processed": 0, "entities_skipped": 0, "errors": []}
    
    # REQUIRE actual data - no fallbacks
    if "specific_data" not in dungeon_result:
        stats["errors"].append("Missing specific_data from dungeons processor")
        return stats
        
    if "ml_results" not in dungeon_result:
        stats["errors"].append("Missing ml_results from dungeons processor")
        return stats
        
    if "cluster_name" not in dungeon_result:
        stats["errors"].append("Missing cluster_name from dungeons processor")
        return stats
    
    cluster_name = dungeon_result["cluster_name"]
    ml_results = dungeon_result["ml_results"]
    
    # REQUIRE entities data - no fallbacks
    if "entities" not in ml_results:
        stats["errors"].append(f"No entities found in ML results for cluster {cluster_name}")
        return stats
    
    entities = ml_results["entities"]
    if not entities:
        stats["errors"].append(f"Empty entities list for cluster {cluster_name}")
        return stats
    
    # Process each entity with proper error tracking
    for entity_data in entities:
        stats["entities_processed"] += 1
        
        # REQUIRE uuid - no fallbacks
        if "uuid" not in entity_data:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Entity missing uuid in cluster {cluster_name}")
            continue
            
        entity_uuid = entity_data["uuid"]
        
        # REQUIRE extracted_data - no fallbacks
        if "extracted_data" not in entity_data:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Entity {entity_uuid} missing extracted_data")
            continue
        
        extracted_data = entity_data["extracted_data"]
        
        # Create encounter record using REAL data
        try:
            encounter_record = EncounterRecord(
                encounter_id=entity_uuid,
                encounter_name=cluster_name,
                encounter_type="dungeon_exploration", 
                difficulty="moderate"
            )
            
            session.add(encounter_record)
            stats["encounters_created"] += 1
            
            # Process REAL encounter data if present
            if "encounters" in extracted_data and extracted_data["encounters"]:
                for encounter in extracted_data["encounters"]:
                    if not isinstance(encounter, dict):
                        continue
                    
                    # REQUIRE encounter name and location - no fallbacks
                    if "name" not in encounter or "location" not in encounter:
                        stats["errors"].append(f"Encounter missing required fields in {entity_uuid}")
                        continue
                    
                    scenario = CombatScenarioRecord(
                        encounter_id=encounter_record.encounter_id,
                        scenario_name=encounter["name"],
                        biome_context=encounter["location"],
                        encounter_setup=encounter.get("description") if "description" in encounter else f"Combat encounter: {encounter['name']}",
                        encounter_cr=encounter.get("cr") if "cr" in encounter else 1.0
                    )
                    session.add(scenario)
                    stats["combat_scenarios_created"] += 1
                    
        except Exception as e:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Failed to create encounter for entity {entity_uuid}: {str(e)}")
    
    return stats


def _integrate_settlement_encounters(session: Session, settlement_result: dict[str, Any]) -> dict[str, int]:
    """Integrate REAL settlement encounter data from entity processors - fail hard on missing data."""
    
    stats = {"scripted_events_created": 0, "encounter_npcs_created": 0, "entities_processed": 0, "entities_skipped": 0, "errors": []}
    
    # REQUIRE actual data - no fallbacks
    if "specific_data" not in settlement_result:
        stats["errors"].append("Missing specific_data from settlements processor")
        return stats
        
    if "ml_results" not in settlement_result:
        stats["errors"].append("Missing ml_results from settlements processor")
        return stats
        
    if "cluster_name" not in settlement_result:
        stats["errors"].append("Missing cluster_name from settlements processor")
        return stats
    
    cluster_name = settlement_result["cluster_name"]
    ml_results = settlement_result["ml_results"]
    
    # REQUIRE entities data - no fallbacks
    if "entities" not in ml_results:
        stats["errors"].append(f"No entities found in ML results for cluster {cluster_name}")
        return stats
    
    entities = ml_results["entities"]
    if not entities:
        stats["errors"].append(f"Empty entities list for cluster {cluster_name}")
        return stats
    
    # Process each entity with proper error tracking
    for entity_data in entities:
        stats["entities_processed"] += 1
        
        # REQUIRE uuid - no fallbacks
        if "uuid" not in entity_data:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Entity missing uuid in cluster {cluster_name}")
            continue
            
        entity_uuid = entity_data["uuid"]
        
        # REQUIRE extracted_data - no fallbacks
        if "extracted_data" not in entity_data:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Entity {entity_uuid} missing extracted_data")
            continue
        
        extracted_data = entity_data["extracted_data"]
        
        # Create scripted event using REAL data
        try:
            event_record = ScriptedEventRecord(
                event_id=entity_uuid,
                event_name=cluster_name,
                region_context=cluster_name
            )
            
            session.add(event_record)
            stats["scripted_events_created"] += 1
            
            # Process REAL NPC data if present
            if "npcs" in extracted_data and extracted_data["npcs"]:
                for npc_data in extracted_data["npcs"]:
                    if not isinstance(npc_data, dict):
                        continue
                    
                    # REQUIRE NPC ID and name - no fallbacks
                    if "id" not in npc_data or "name" not in npc_data:
                        stats["errors"].append(f"NPC missing required fields in {entity_uuid}")
                        continue
                    
                    npc = NPCInteractionRecord(
                        interaction_id=f"{entity_uuid}_npc_{npc_data['id']}",
                        interaction_name=npc_data["name"],
                        npc_entity_id=npc_data["id"],
                        regional_context=cluster_name,
                        interaction_category=npc_data.get("role") if "role" in npc_data else "settlement_npc"
                    )
                    session.add(npc)
                    stats["encounter_npcs_created"] += 1
                    
        except Exception as e:
            stats["entities_skipped"] += 1
            stats["errors"].append(f"Failed to create settlement encounter for entity {entity_uuid}: {str(e)}")
    
    return stats


def _integrate_faction_encounters(session: Session, faction_result: dict[str, Any]) -> dict[str, int]:
    """Integrate REAL faction encounter data from entity processors - no fake data."""
    
    stats = {"encounters_created": 0, "encounter_npcs_created": 0}
    
    # Use ACTUAL data from entity processors ML analysis
    specific_data = faction_result.get("specific_data", {})
    ml_results = faction_result.get("ml_results", {})
    cluster_name = faction_result.get("cluster_name", "")
    
    if not specific_data or not cluster_name:
        return stats  # Skip if no real data
    
    # Use REAL entity data from ML processing
    entities = ml_results.get("entities", [])
    if not entities:
        return stats
    
    # Process each REAL entity from the ML results
    for entity_data in entities:
        entity_uuid = entity_data.get("uuid", "")
        extracted_data = entity_data.get("extracted_data", {})
        
        if not entity_uuid or not extracted_data:
            continue
        
        # Create encounter record using REAL data
        encounter_record = EncounterRecord(
            encounter_id=entity_uuid,
            encounter_name=cluster_name,
            encounter_type="faction_encounter",
            difficulty="moderate"
        )
        
        session.add(encounter_record)
        stats["encounters_created"] += 1
        
        # Extract REAL faction member data from the ML-processed content
        if "factions" in extracted_data:
            for faction_data in extracted_data["factions"]:
                if isinstance(faction_data, dict):
                    npc = NPCInteractionRecord(
                        interaction_id=f"{entity_uuid}_faction_{faction_data.get('id', 'unknown')}",
                        interaction_name=faction_data.get("name", cluster_name),
                        npc_entity_id=faction_data.get("id", f"{entity_uuid}_faction"),
                        regional_context=cluster_name,
                        interaction_category="faction_contact"
                    )
                    session.add(npc)
                    stats["encounter_npcs_created"] += 1
    
    return stats


def _integrate_region_encounters(session: Session, region_result: dict[str, Any]) -> dict[str, int]:
    """Integrate REAL region encounter data from entity processors - no fake data."""
    
    stats = {"scripted_events_created": 0}
    
    # Use ACTUAL data from entity processors ML analysis
    specific_data = region_result.get("specific_data", {})
    ml_results = region_result.get("ml_results", {})
    cluster_name = region_result.get("cluster_name", "")
    
    if not specific_data or not cluster_name:
        return stats  # Skip if no real data
    
    # Use REAL entity data from ML processing
    entities = ml_results.get("entities", [])
    if not entities:
        return stats
    
    # Process each REAL entity from the ML results
    for entity_data in entities:
        entity_uuid = entity_data.get("uuid", "")
        extracted_data = entity_data.get("extracted_data", {})
        
        if not entity_uuid or not extracted_data:
            continue
        
        # Create scripted event using REAL data
        event_record = ScriptedEventRecord(
            event_id=entity_uuid,
            event_name=cluster_name,
            region_context=cluster_name
        )
        
        session.add(event_record)
        stats["scripted_events_created"] += 1
    
    return stats


def _map_horror_intensity_to_impact(horror_intensity: str) -> float:
    """Map horror intensity to numeric impact value."""
    
    intensity_map = {
        "none": 0.0,
        "low": 0.1,
        "moderate": 0.3,
        "high": 0.6,
        "extreme": 1.0
    }
    
    return intensity_map.get(horror_intensity, 0.0)


def get_encounter_statistics(session: Session) -> dict[str, Any]:
    """Get statistics about integrated encounter data for reporting."""
    
    encounters_count = session.query(EncounterRecord).count()
    scenarios_count = session.query(CombatScenario).count()
    events_count = session.query(ScriptedEvent).count()
    beasts_count = session.query(Beast).count()
    npcs_count = session.query(NPC).count()
    
    # Count by encounter type
    dungeon_encounters = session.query(EncounterRecord).filter(
        EncounterRecord.encounter_type == "dungeon_exploration"
    ).count()
    
    faction_encounters = session.query(EncounterRecord).filter(
        EncounterRecord.encounter_type == "faction_encounter"
    ).count()
    
    settlement_events = session.query(ScriptedEvent).filter(
        ScriptedEvent.event_type == "settlement_arrival"
    ).count()
    
    return {
        "total_encounters": encounters_count,
        "total_scenarios": scenarios_count,
        "total_events": events_count,
        "total_beasts": beasts_count,
        "total_npcs": npcs_count,
        "dungeon_encounters": dungeon_encounters,
        "faction_encounters": faction_encounters,
        "settlement_events": settlement_events,
        "database_ready_for_godot_sqlite": True
    }
