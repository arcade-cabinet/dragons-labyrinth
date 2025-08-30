"""
Sprites Integration - Receive character data from entities processors.

Handles character sprites, tokens, and visual assets from entities → sprites tables.
Manages NPC sprites, faction banners, settlement tokens, and dungeon markers.
"""

from __future__ import annotations

import json
from typing import Any

from sqlmodel import Session
from generator.sprites.models import CharacterRecord, NPCRecord, CompanionRecord, MonsterRecord, MercenaryRecord
from generator.statistics import RunStatistics


def integrate_from_entities_processors(
    session: Session, 
    processing_results: dict[str, Any]
) -> RunStatistics:
    """
    Integrate character/sprite data from entities processors into sprites tables.
    
    Args:
        session: SQLModel database session
        processing_results: Results from entities transformer/processors
        
    Returns:
        RunStatistics with integration results
    """
    
    run_stats = RunStatistics(subpackage="sprites")
    
    characters_created = 0
    npcs_created = 0
    monsters_created = 0
    faction_sprites_created = 0
    settlement_tokens_created = 0
    
    # Process each category from entities
    for processor_type, processor_result in processing_results.items():
        if "error" in processor_result:
            run_stats.add_error(f"{processor_type}: {processor_result['error']}")
            continue
            
        try:
            if processor_type == "settlements":
                settlement_stats = _integrate_settlement_sprites(session, processor_result)
                settlement_tokens_created += settlement_stats.get("settlement_tokens_created", 0)
                npcs_created += settlement_stats.get("npcs_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "factions":
                faction_stats = _integrate_faction_sprites(session, processor_result)
                faction_sprites_created += faction_stats.get("faction_sprites_created", 0)
                npcs_created += faction_stats.get("npcs_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "dungeons":
                dungeon_stats = _integrate_dungeon_sprites(session, processor_result)
                monsters_created += dungeon_stats.get("monsters_created", 0)
                run_stats.items_processed += 1
                
            elif processor_type == "regions":
                region_stats = _integrate_region_sprites(session, processor_result)
                npcs_created += region_stats.get("npcs_created", 0)
                run_stats.items_processed += 1
                
            run_stats.items_stored += 1
            
        except Exception as e:
            run_stats.add_error(f"Failed to integrate {processor_type}: {str(e)}")
    
    # Store custom metrics
    run_stats.add_metric("characters_created", characters_created)
    run_stats.add_metric("npcs_created", npcs_created)
    run_stats.add_metric("monsters_created", monsters_created)
    run_stats.add_metric("faction_sprites_created", faction_sprites_created)
    run_stats.add_metric("settlement_tokens_created", settlement_tokens_created)
    run_stats.add_metric("database_ready_for_godot_sqlite", True)
    
    session.commit()
    run_stats.finish()
    return run_stats


def _integrate_settlement_sprites(session: Session, settlement_result: dict[str, Any]) -> dict[str, int]:
    """Integrate settlement sprite data into sprites tables."""
    
    stats = {"settlement_tokens_created": 0, "npcs_created": 0}
    
    settlement_data = settlement_result.get("settlement_data", {})
    world_hooks = settlement_result.get("world_hooks", {})
    
    settlement_name = settlement_data.get("name", "Unknown Settlement")
    scale_hint = settlement_data.get("scale_hint", "village")
    
    # Create settlement token sprite record
    token_record = CharacterRecord(
        character_id=f"token_{settlement_name.lower().replace(' ', '_')}",
        character_name=f"{settlement_name} Token",
        character_type="settlement_token",
        sprite_category="tokens",
        scale_hint=scale_hint,
        visual_style="chess_piece",
        world_hooks=json.dumps({
            "settlement_name": settlement_name,
            "scale_hint": scale_hint,
            "sprite_type": "settlement_token",
            "services": settlement_data.get("service_types", []),
            "corruption_resistance": world_hooks.get("godot_integration", {}).get("corruption_resistance", 0)
        })
    )
    
    session.add(token_record)
    stats["settlement_tokens_created"] += 1
    
    # Create NPCs if settlement has them
    if world_hooks.get("has_npcs", False):
        npc_record = NPCRecord(
            npc_id=f"npc_{settlement_name.lower().replace(' ', '_')}_villager",
            npc_name=f"{settlement_name} Villager",
            settlement_affiliation=settlement_name,
            sprite_style="villager_token",
            companion_potential=True if scale_hint in ["village", "town"] else False,
            world_hooks=json.dumps({
                "settlement": settlement_name,
                "role": "villager",
                "sprite_base": "villager_token"
            })
        )
        session.add(npc_record)
        stats["npcs_created"] += 1
    
    return stats


def _integrate_faction_sprites(session: Session, faction_result: dict[str, Any]) -> dict[str, int]:
    """Integrate faction sprite data into sprites tables."""
    
    stats = {"faction_sprites_created": 0, "npcs_created": 0}
    
    faction_data = faction_result.get("faction_data", {})
    world_hooks = faction_result.get("world_hooks", {})
    
    faction_name = faction_data.get("name", "Unknown Faction")
    political_alignment = faction_data.get("political_alignment", "neutral")
    
    # Create faction banner sprite
    banner_record = CharacterRecord(
        character_id=f"banner_{faction_name.lower().replace(' ', '_')}",
        character_name=f"{faction_name} Banner",
        character_type="faction_banner",
        sprite_category="banners",
        visual_style="heraldic",
        world_hooks=json.dumps({
            "faction_name": faction_name,
            "political_alignment": political_alignment,
            "hostility_level": world_hooks.get("hostility_level", "neutral"),
            "sprite_type": "faction_banner",
            "territorial_reach": world_hooks.get("territorial_reach", "local")
        })
    )
    
    session.add(banner_record)
    stats["faction_sprites_created"] += 1
    
    # Create faction member NPCs
    member_count = faction_data.get("member_count", 0)
    if member_count > 0:
        member_record = NPCRecord(
            npc_id=f"npc_{faction_name.lower().replace(' ', '_')}_member",
            npc_name=f"{faction_name} Member",
            faction_affiliation=faction_name,
            sprite_style="faction_member",
            companion_potential=political_alignment in ["neutral", "lawful"],
            world_hooks=json.dumps({
                "faction": faction_name,
                "role": "member",
                "alignment": political_alignment,
                "sprite_base": "faction_member"
            })
        )
        session.add(member_record)
        stats["npcs_created"] += 1
    
    return stats


def _integrate_dungeon_sprites(session: Session, dungeon_result: dict[str, Any]) -> dict[str, int]:
    """Integrate dungeon sprite data into sprites tables."""
    
    stats = {"monsters_created": 0, "npcs_created": 0}
    
    dungeon_data = dungeon_result.get("dungeon_data", {})
    world_hooks = dungeon_result.get("world_hooks", {})
    
    dungeon_name = dungeon_data.get("name", "Unknown Dungeon")
    dungeon_type = dungeon_data.get("dungeon_type", "crypt")
    horror_intensity = dungeon_data.get("horror_intensity", "none")
    
    # Create entrance marker sprite
    entrance_record = CharacterRecord(
        character_id=f"entrance_{dungeon_name.lower().replace(' ', '_')}",
        character_name=f"{dungeon_name} Entrance",
        character_type="dungeon_entrance",
        sprite_category="dungeon_markers",
        visual_style="environmental",
        world_hooks=json.dumps({
            "dungeon_name": dungeon_name,
            "dungeon_type": dungeon_type,
            "horror_intensity": horror_intensity,
            "sprite_type": "dungeon_entrance",
            "entrance_types": world_hooks.get("entrances", [])
        })
    )
    
    session.add(entrance_record)
    
    # Create monsters if dungeon has encounters
    encounter_data = dungeon_data.get("encounters", [])
    for i, encounter in enumerate(encounter_data[:3]):  # Limit to first 3 encounters
        monster_record = MonsterRecord(
            character_id=f"monster_{dungeon_name.lower().replace(' ', '_')}_{i}",
            monster_id=f"monster_{dungeon_name.lower().replace(' ', '_')}_{i}",
            monster_name=encounter.get("name", f"{dungeon_name} Guardian"),
            monster_category="dungeon_monster",
            size_category="medium",
            threat_level=encounter.get("cr", 1),
            horror_theme=horror_intensity,
            world_hooks=json.dumps({
                "dungeon": dungeon_name,
                "encounter_type": encounter.get("type", "guardian"),
                "sprite_base": "monster_token"
            })
        )
        session.add(monster_record)
        stats["monsters_created"] += 1
    
    return stats


def _integrate_region_sprites(session: Session, region_result: dict[str, Any]) -> dict[str, int]:
    """Integrate region sprite data into sprites tables."""
    
    stats = {"npcs_created": 0}
    
    region_data = region_result.get("region_data", {})
    world_hooks = region_result.get("world_hooks", {})
    
    region_name = region_data.get("name", "Unknown Region")
    
    # Create regional NPC if region has settlements
    settlement_locations = region_data.get("settlement_locations", [])
    if settlement_locations:
        regional_npc = NPCRecord(
            npc_id=f"npc_regional_{region_name.lower().replace(' ', '_')}",
            npc_name=f"{region_name} Traveler",
            region_affiliation=region_name,
            sprite_style="regional_traveler",
            companion_potential=True,
            world_hooks=json.dumps({
                "region": region_name,
                "role": "traveler",
                "knowledge_areas": [region_name] + settlement_locations,
                "sprite_base": "traveler_token"
            })
        )
        session.add(regional_npc)
        stats["npcs_created"] += 1
    
    return stats


def get_sprite_statistics(session: Session) -> dict[str, Any]:
    """Get statistics about integrated sprite data for reporting."""
    
    characters_count = session.query(CharacterRecord).count()
    npcs_count = session.query(NPCRecord).count()
    monsters_count = session.query(MonsterRecord).count()
    
    # Count by category
    settlement_tokens = session.query(CharacterRecord).filter(
        CharacterRecord.character_type == "settlement_token"
    ).count()
    
    faction_banners = session.query(CharacterRecord).filter(
        CharacterRecord.character_type == "faction_banner"
    ).count()
    
    dungeon_entrances = session.query(CharacterRecord).filter(
        CharacterRecord.character_type == "dungeon_entrance"
    ).count()
    
    return {
        "total_characters": characters_count,
        "total_npcs": npcs_count,
        "total_monsters": monsters_count,
        "settlement_tokens": settlement_tokens,
        "faction_banners": faction_banners,
        "dungeon_entrances": dungeon_entrances,
        "database_ready_for_godot_sqlite": True
    }
