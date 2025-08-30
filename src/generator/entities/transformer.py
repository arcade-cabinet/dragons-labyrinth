"""
Entity Transformer - Routes HBF entities to specialized processors.

Implements clustering logic similar to scripts/extract_hbf_worldbuilding.sh
but with ML-enhanced categorization. Routes entities to specialized processors
based on known regions, settlements, factions, and dungeons.
"""

from __future__ import annotations

import json
import re
import sqlite3
from pathlib import Path
from typing import Any

from generator.constants import REGIONS, SETTLEMENTS, FACTIONS, DUNGEONS, BIOMES
from generator.entities.processors.regions import process_region_cluster
from generator.entities.processors.settlements import process_settlement_cluster
from generator.entities.processors.factions import process_faction_cluster
from generator.entities.processors.dungeons import process_dungeon_cluster


class EntityCluster:
    """Container for entities clustered by category."""
    
    def __init__(self, category: str, name: str):
        self.category = category
        self.name = name
        self.entities: list[dict[str, Any]] = []
        self.processor_type = self._determine_processor_type()
    
    def _determine_processor_type(self) -> str:
        """Determine which specialized processor to use."""
        if self.category == "regions":
            return "regions"
        elif self.category == "settlements":
            return "settlements"
        elif self.category == "factions":
            return "factions"
        elif self.category == "dungeons":
            return "dungeons"
        elif self.category == "biomes":
            return "regions"  # Biomes processed by regions processor
        else:
            return "base"
    
    def add_entity(self, entity: dict[str, Any]) -> None:
        """Add entity to this cluster."""
        self.entities.append(entity)
    
    def get_entity_count(self) -> int:
        """Get count of entities in this cluster."""
        return len(self.entities)


class EntityTransformer:
    """Transforms raw HBF entities into categorized clusters for specialized processing."""
    
    def __init__(self, hbf_db_path: str):
        self.hbf_db_path = hbf_db_path
        self.clusters: dict[str, EntityCluster] = {}
        self._initialize_clusters()
    
    def _initialize_clusters(self) -> None:
        """Initialize clusters for all known categories."""
        
        # Create region clusters
        for region in REGIONS:
            safe_name = self._safe_name(region)
            self.clusters[f"region_{safe_name}"] = EntityCluster("regions", region)
        
        # Create settlement clusters
        for settlement in SETTLEMENTS:
            safe_name = self._safe_name(settlement)
            self.clusters[f"settlement_{safe_name}"] = EntityCluster("settlements", settlement)
        
        # Create faction clusters
        for faction in FACTIONS:
            safe_name = self._safe_name(faction)
            self.clusters[f"faction_{safe_name}"] = EntityCluster("factions", faction)
        
        # Create dungeon clusters
        for dungeon in DUNGEONS:
            safe_name = self._safe_name(dungeon)
            self.clusters[f"dungeon_{safe_name}"] = EntityCluster("dungeons", dungeon)
        
        # Create biome clusters
        for biome in BIOMES:
            safe_name = self._safe_name(biome)
            self.clusters[f"biome_{safe_name}"] = EntityCluster("biomes", biome)
    
    def _safe_name(self, name: str) -> str:
        """Convert name to safe identifier."""
        return name.lower().replace(" ", "_").replace("'", "")
    
    def extract_and_cluster_entities(self) -> dict[str, EntityCluster]:
        """Extract entities from HBF SQLite and cluster by category."""
        
        print("🔍 Extracting and clustering entities from HBF database...")
        
        conn = sqlite3.connect(self.hbf_db_path)
        cursor = conn.cursor()
        
        try:
            # Get all entity records
            cursor.execute("SELECT value FROM Entities")
            entity_rows = cursor.fetchall()
            
            print(f"📊 Found {len(entity_rows)} total entities in database")
            
            # Process each entity
            for row in entity_rows:
                try:
                    entity_json = json.loads(row[0])
                    self._route_entity_to_cluster(entity_json)
                except json.JSONDecodeError:
                    # Skip invalid JSON
                    continue
            
            conn.close()
            
            # Print clustering results
            self._print_clustering_summary()
            
            return self.clusters
            
        except Exception as e:
            conn.close()
            print(f"❌ Error extracting entities: {e}")
            return {}
    
    def _route_entity_to_cluster(self, entity: dict[str, Any]) -> None:
        """Route entity to appropriate cluster based on content matching."""
        
        entity_content = json.dumps(entity)
        routed = False
        
        # Check for region matches (highest priority)
        for region in REGIONS:
            if region in entity_content:
                safe_name = self._safe_name(region)
                cluster_key = f"region_{safe_name}"
                if cluster_key in self.clusters:
                    self.clusters[cluster_key].add_entity(entity)
                    routed = True
                    break
        
        if routed:
            return
        
        # Check for settlement matches
        for settlement in SETTLEMENTS:
            if settlement in entity_content:
                safe_name = self._safe_name(settlement)
                cluster_key = f"settlement_{safe_name}"
                if cluster_key in self.clusters:
                    self.clusters[cluster_key].add_entity(entity)
                    routed = True
                    break
        
        if routed:
            return
        
        # Check for faction matches
        for faction in FACTIONS:
            if faction in entity_content:
                safe_name = self._safe_name(faction)
                cluster_key = f"faction_{safe_name}"
                if cluster_key in self.clusters:
                    self.clusters[cluster_key].add_entity(entity)
                    routed = True
                    break
        
        if routed:
            return
        
        # Check for dungeon matches
        for dungeon in DUNGEONS:
            if dungeon in entity_content:
                safe_name = self._safe_name(dungeon)
                cluster_key = f"dungeon_{safe_name}"
                if cluster_key in self.clusters:
                    self.clusters[cluster_key].add_entity(entity)
                    routed = True
                    break
        
        if routed:
            return
        
        # Check for biome hex matches
        entity_type = entity.get("type", "")
        if entity_type.endswith("Hex"):
            biome = entity_type.replace("Hex", "")
            if biome in BIOMES:
                safe_name = self._safe_name(biome)
                cluster_key = f"biome_{safe_name}"
                if cluster_key in self.clusters:
                    self.clusters[cluster_key].add_entity(entity)
                    routed = True
        
        # If no specific match, check for general category indicators
        if not routed:
            self._route_by_category_indicators(entity, entity_content)
    
    def _route_by_category_indicators(self, entity: dict[str, Any], content: str) -> None:
        """Route entity based on general category indicators when no specific match found."""
        
        content_lower = content.lower()
        
        # Settlement indicators
        if any(word in content_lower for word in ["village", "town", "city", "settlement"]):
            # Route to first settlement cluster as fallback
            if self.clusters:
                first_settlement = next((k for k in self.clusters.keys() if k.startswith("settlement_")), None)
                if first_settlement:
                    self.clusters[first_settlement].add_entity(entity)
                    return
        
        # Dungeon indicators
        if any(word in content_lower for word in ["crypt", "lair", "temple", "shrine", "tomb", "hideout", "cavern"]):
            # Route to first dungeon cluster as fallback
            if self.clusters:
                first_dungeon = next((k for k in self.clusters.keys() if k.startswith("dungeon_")), None)
                if first_dungeon:
                    self.clusters[first_dungeon].add_entity(entity)
                    return
        
        # Faction indicators
        if any(word in content_lower for word in ["cult", "militia", "organization", "gang", "guild"]):
            # Route to first faction cluster as fallback
            if self.clusters:
                first_faction = next((k for k in self.clusters.keys() if k.startswith("faction_")), None)
                if first_faction:
                    self.clusters[first_faction].add_entity(entity)
                    return
    
    def _print_clustering_summary(self) -> None:
        """Print summary of clustering results."""
        
        print("\n📊 Entity Clustering Summary:")
        print("=" * 50)
        
        category_totals = {"regions": 0, "settlements": 0, "factions": 0, "dungeons": 0, "biomes": 0}
        
        for cluster_key, cluster in self.clusters.items():
            count = cluster.get_entity_count()
            if count > 0:
                print(f"  {cluster.name}: {count} entities")
                category_totals[cluster.category] += count
        
        print("\nCategory Totals:")
        for category, total in category_totals.items():
            print(f"  {category.title()}: {total} entities")
        
        total_entities = sum(category_totals.values())
        print(f"\nTotal Clustered: {total_entities} entities")
    
    def get_clusters_by_category(self, category: str) -> list[EntityCluster]:
        """Get all clusters for a specific category."""
        
        return [cluster for cluster in self.clusters.values() if cluster.category == category]
    
    def get_non_empty_clusters(self) -> list[EntityCluster]:
        """Get all clusters that contain entities."""
        
        return [cluster for cluster in self.clusters.values() if cluster.get_entity_count() > 0]
    
    def get_entities_for_processor(self, processor_type: str) -> list[dict[str, Any]]:
        """Get all entities that should be processed by a specific processor type."""
        
        entities = []
        for cluster in self.clusters.values():
            if cluster.processor_type == processor_type and cluster.get_entity_count() > 0:
                entities.extend(cluster.entities)
        
        return entities
    
    def export_clusters_to_json(self, output_dir: str) -> dict[str, str]:
        """Export clusters to JSON files for inspection and processing."""
        
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        exported_files = {}
        
        for cluster_key, cluster in self.clusters.items():
            if cluster.get_entity_count() > 0:
                file_path = output_path / f"{cluster_key}.json"
                
                cluster_data = {
                    "category": cluster.category,
                    "name": cluster.name,
                    "processor_type": cluster.processor_type,
                    "entity_count": cluster.get_entity_count(),
                    "entities": cluster.entities
                }
                
                with open(file_path, "w", encoding="utf-8") as f:
                    json.dump(cluster_data, f, indent=2, ensure_ascii=False)
                
                exported_files[cluster_key] = str(file_path)
        
        # Export summary
        summary_path = output_path / "clustering_summary.json"
        summary = {
            "total_clusters": len(self.clusters),
            "non_empty_clusters": len([c for c in self.clusters.values() if c.get_entity_count() > 0]),
            "category_counts": self._get_category_counts(),
            "processor_routing": self._get_processor_routing()
        }
        
        with open(summary_path, "w", encoding="utf-8") as f:
            json.dump(summary, f, indent=2, ensure_ascii=False)
        
        exported_files["summary"] = str(summary_path)
        
        print(f"✅ Exported {len(exported_files)} cluster files to {output_dir}")
        
        return exported_files
    
    def _get_category_counts(self) -> dict[str, int]:
        """Get entity counts by category."""
        
        counts = {}
        for cluster in self.clusters.values():
            category = cluster.category
            counts[category] = counts.get(category, 0) + cluster.get_entity_count()
        
        return counts
    
    def _get_processor_routing(self) -> dict[str, list[str]]:
        """Get routing map of processor types to cluster names."""
        
        routing = {}
        for cluster in self.clusters.values():
            if cluster.get_entity_count() > 0:
                processor = cluster.processor_type
                if processor not in routing:
                    routing[processor] = []
                routing[processor].append(cluster.name)
        
        return routing


def transform_hbf_to_clusters(hbf_db_path: str, output_dir: str = "clusters") -> dict[str, Any]:
    """
    Main transformation function - extract HBF entities and cluster them.
    
    Args:
        hbf_db_path: Path to HBF SQLite database
        output_dir: Output directory for cluster files
        
    Returns:
        Transformation results with cluster summary
    """
    
    transformer = EntityTransformer(hbf_db_path)
    clusters = transformer.extract_and_cluster_entities()
    exported_files = transformer.export_clusters_to_json(output_dir)
    
    return {
        "clusters": clusters,
        "exported_files": exported_files,
        "category_counts": transformer._get_category_counts(),
        "processor_routing": transformer._get_processor_routing()
    }


def route_to_specialized_processor(cluster: EntityCluster, logger, console) -> dict[str, Any]:
    """
    Route cluster to appropriate specialized processor.
    
    Args:
        cluster: Entity cluster to process
        logger: Logger instance from orchestrator
        console: Rich console from orchestrator
        
    Returns:
        Processing results from specialized processor
    """
    
    processor_type = cluster.processor_type
    
    # Route to specialized processors
    if processor_type == "regions":
        return process_region_cluster(cluster, logger, console)
    elif processor_type == "settlements":
        return process_settlement_cluster(cluster, logger, console)
    elif processor_type == "factions":
        return process_faction_cluster(cluster, logger, console)
    elif processor_type == "dungeons":
        return process_dungeon_cluster(cluster, logger, console)
    else:
        # Unknown processor type - return error result
        logger.warning(f"Unknown processor type: {processor_type} for cluster {cluster.name}")
        return {
            "error": f"Unknown processor type: {processor_type}",
            "cluster_name": cluster.name,
            "processor_type": processor_type
        }


def extract_world_hooks_for_pandora(clusters: dict[str, EntityCluster]) -> dict[str, Any]:
    """
    Extract world hooks data for Pandora addon integration.
    
    Args:
        clusters: Dictionary of entity clusters
        
    Returns:
        World hooks data formatted for Pandora
    """
    
    pandora_data = {
        "regions": [],
        "settlements": [],
        "factions": [],
        "dungeons": [],
        "biomes": []
    }
    
    for cluster in clusters.values():
        if cluster.get_entity_count() == 0:
            continue
        
        category_data = {
            "name": cluster.name,
            "category": cluster.category,
            "entity_count": cluster.get_entity_count(),
            "world_hooks": _extract_category_world_hooks(cluster)
        }
        
        if cluster.category in pandora_data:
            pandora_data[cluster.category].append(category_data)
    
    return pandora_data


def _extract_category_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract world hooks for a specific category cluster."""
    
    if cluster.category == "regions":
        return _extract_region_world_hooks(cluster)
    elif cluster.category == "settlements":
        return _extract_settlement_world_hooks(cluster)
    elif cluster.category == "factions":
        return _extract_faction_world_hooks(cluster)
    elif cluster.category == "dungeons":
        return _extract_dungeon_world_hooks(cluster)
    elif cluster.category == "biomes":
        return _extract_biome_world_hooks(cluster)
    
    return {}


def _extract_region_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract region-specific world hooks."""
    
    # Aggregate data from all entities in region
    biome_counts = {}
    has_rivers = False
    has_trails = False
    harbor_count = 0
    has_borders = False
    
    for entity in cluster.entities:
        # Count biome types
        entity_type = entity.get("type", "")
        if entity_type.endswith("Hex"):
            biome = entity_type.replace("Hex", "")
            biome_counts[biome] = biome_counts.get(biome, 0) + 1
        
        # Check for rivers/trails
        if entity.get("rivers"):
            has_rivers = True
        if entity.get("trails"):
            has_trails = True
        
        # Count harbors
        if entity.get("harbor"):
            harbor_count += 1
        
        # Check borders
        if entity.get("borderline"):
            has_borders = True
    
    # Determine dominant biome
    dominant_biome = max(biome_counts.items(), key=lambda x: x[1])[0] if biome_counts else "Unknown"
    
    return {
        "dominant_biome": dominant_biome,
        "biome_distribution": biome_counts,
        "has_rivers": has_rivers,
        "has_trails": has_trails,
        "harbor_count": harbor_count,
        "has_borders": has_borders,
        "total_hexes": len(cluster.entities)
    }


def _extract_settlement_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract settlement-specific world hooks."""
    
    # Determine scale from name
    name = cluster.name
    scale_hint = "unknown"
    if name.startswith("City of"):
        scale_hint = "city"
    elif name.startswith("Town of"):
        scale_hint = "town"
    elif name.startswith("Village of"):
        scale_hint = "village"
    
    # Analyze entities for infrastructure
    has_harbor = False
    river_adjacent = False
    has_walls = False
    market_size = "none"
    
    for entity in cluster.entities:
        content = json.dumps(entity).lower()
        
        if "harbor" in content or "port" in content:
            has_harbor = True
        if "river" in content:
            river_adjacent = True
        if "wall" in content or "fortified" in content:
            has_walls = True
        if "market" in content:
            market_size = "small"
            if "large market" in content:
                market_size = "large"
    
    return {
        "scale_hint": scale_hint,
        "has_harbor": has_harbor,
        "river_adjacent": river_adjacent,
        "has_walls": has_walls,
        "market_size": market_size,
        "entity_count": cluster.get_entity_count()
    }


def _extract_faction_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract faction-specific world hooks."""
    
    # Analyze faction characteristics
    operating_places = []
    hostility = "neutral"
    member_count = 0
    
    for entity in cluster.entities:
        content = json.dumps(entity).lower()
        
        # Check for settlement operations
        for settlement in SETTLEMENTS:
            if settlement.lower() in content:
                operating_places.append(settlement)
        
        # Assess hostility
        if "defiled" in content or "corruption" in content:
            hostility = "hostile"
        elif "justice" in content:
            hostility = "lawful"
        elif "eliminate" in content or "destroy" in content:
            hostility = "aggressive"
        
        # Count potential members (rough estimate)
        if "level" in content and any(cls in content for cls in ["fighter", "cleric", "wizard", "rogue"]):
            member_count += 1
    
    return {
        "operating_places": list(set(operating_places)),
        "hostility": hostility,
        "member_count_estimate": member_count,
        "territorial_reach": "regional" if len(set(operating_places)) > 2 else "local"
    }


def _extract_dungeon_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract dungeon-specific world hooks."""
    
    # Determine dungeon type from name
    name = cluster.name.lower()
    entrance_type = "unknown"
    
    if "crypt" in name:
        entrance_type = "crypt-portal"
    elif "cave" in name or "cavern" in name:
        entrance_type = "cave-mouth"
    elif "temple" in name or "shrine" in name:
        entrance_type = "temple-entrance"
    elif "tomb" in name:
        entrance_type = "tomb-entrance"
    elif "lair" in name:
        entrance_type = "lair-entrance"
    
    # Analyze content for depth and complexity
    total_content = ""
    for entity in cluster.entities:
        total_content += json.dumps(entity) + " "
    
    room_count = total_content.lower().count("room")
    depth_hint = "shallow"
    if "deep" in total_content.lower() or room_count > 10:
        depth_hint = "deep"
    elif "level 2" in total_content.lower() or room_count > 5:
        depth_hint = "mid"
    
    return {
        "entrance_type": entrance_type,
        "depth_hint": depth_hint,
        "room_count_estimate": room_count,
        "has_treasure": "treasure" in total_content.lower() or "hoard" in total_content.lower(),
        "entity_count": cluster.get_entity_count()
    }


def _extract_biome_world_hooks(cluster: EntityCluster) -> dict[str, Any]:
    """Extract biome-specific world hooks."""
    
    # Simple biome data for tile generation
    return {
        "biome_type": cluster.name,
        "tile_count": cluster.get_entity_count(),
        "sprite_sheet_path": f"res://art/atlas_{cluster.name.lower()}.png"
    }
