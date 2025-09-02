"""
JSON Map Processor - Specialized processor for JSON map entities.

Processes structured map JSON entities with hex grid coordinates, biome distribution,
regional mapping, and infrastructure data. Core spatial data for world generation.
"""

from __future__ import annotations

from typing import Any

from generator.entities.processors.base import BaseProcessor


class JSONMapProcessor(BaseProcessor):
    """
    Specialized processor for JSON map entities.
    
    Extracts:
    - Hex grid structure and coordinate system
    - Biome distribution and regional mapping
    - Infrastructure analysis (rivers, trails, harbors)
    - Feature placement and connectivity
    - Border and realm management
    """
    
    def __init__(self):
        super().__init__("json_map")
    
    def _extract_specific_data(self, json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
        """Extract map-specific data from JSON entity."""
        
        # Extract hex map data
        hex_map = json_entity.get("map", [])
        map_analysis = self._analyze_hex_map(hex_map)
        
        # Extract regions and realms
        regions = json_entity.get("regions", {})
        realms = json_entity.get("realms", {})
        region_analysis = self._analyze_regions(regions, hex_map)
        
        # Analyze infrastructure
        infrastructure_analysis = self._analyze_infrastructure(hex_map)
        
        # Analyze connectivity
        connectivity_analysis = self._analyze_connectivity(hex_map)
        
        # Calculate world scope
        world_scope = self._calculate_world_scope(map_analysis, region_analysis)
        
        logger.info(f"Map analysis - Hexes: {map_analysis['total_hexes']}, Regions: {len(regions)}, Biomes: {len(map_analysis['biome_distribution'])}")
        
        return {
            "uuid": uuid,
            "map_analysis": map_analysis,
            "region_analysis": region_analysis,
            "realm_data": realms,
            "infrastructure_analysis": infrastructure_analysis,
            "connectivity_analysis": connectivity_analysis,
            "world_scope": world_scope,
            "coordinate_system": "axial_hex",
            "has_complete_world_data": True
        }
    
    def _analyze_hex_map(self, hex_map: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze hex grid structure and distribution."""
        
        total_hexes = len(hex_map)
        
        # Extract coordinate ranges
        x_coords = [hex_tile.get("x", 0) for hex_tile in hex_map]
        y_coords = [hex_tile.get("y", 0) for hex_tile in hex_map]
        
        coordinate_bounds = {}
        if x_coords and y_coords:
            coordinate_bounds = {
                "x_min": min(x_coords),
                "x_max": max(x_coords),
                "y_min": min(y_coords),
                "y_max": max(y_coords),
                "width": max(x_coords) - min(x_coords),
                "height": max(y_coords) - min(y_coords)
            }
        
        # Analyze biome distribution
        biome_distribution = {}
        feature_distribution = {}
        
        for hex_tile in hex_map:
            biome_type = hex_tile.get("type", "Unknown")
            biome_distribution[biome_type] = biome_distribution.get(biome_type, 0) + 1
            
            feature = hex_tile.get("feature")
            if feature:
                feature_distribution[feature] = feature_distribution.get(feature, 0) + 1
        
        return {
            "total_hexes": total_hexes,
            "coordinate_bounds": coordinate_bounds,
            "biome_distribution": biome_distribution,
            "feature_distribution": feature_distribution,
            "unique_biomes": len(biome_distribution),
            "unique_features": len(feature_distribution),
            "hex_density": total_hexes / max(coordinate_bounds.get("width", 1) * coordinate_bounds.get("height", 1), 1) if coordinate_bounds else 0
        }
    
    def _analyze_regions(self, regions: dict[str, str], hex_map: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze regional distribution and characteristics."""
        
        # Count hexes per region
        region_hex_counts = {}
        region_biomes = {}
        region_features = {}
        
        for hex_tile in hex_map:
            region_id = hex_tile.get("region")
            if region_id:
                # Count hexes
                region_hex_counts[region_id] = region_hex_counts.get(region_id, 0) + 1
                
                # Track biomes per region
                biome = hex_tile.get("type", "Unknown")
                if region_id not in region_biomes:
                    region_biomes[region_id] = {}
                region_biomes[region_id][biome] = region_biomes[region_id].get(biome, 0) + 1
                
                # Track features per region
                feature = hex_tile.get("feature")
                if feature:
                    if region_id not in region_features:
                        region_features[region_id] = {}
                    region_features[region_id][feature] = region_features[region_id].get(feature, 0) + 1
        
        # Analyze region characteristics
        region_characteristics = {}
        for region_id, region_name in regions.items():
            hex_count = region_hex_counts.get(region_id, 0)
            biomes = region_biomes.get(region_id, {})
            features = region_features.get(region_id, {})
            
            # Determine dominant biome
            dominant_biome = max(biomes.items(), key=lambda x: x[1])[0] if biomes else "Unknown"
            
            # Calculate region diversity
            biome_diversity = len(biomes)
            feature_diversity = len(features)
            
            region_characteristics[region_id] = {
                "name": region_name,
                "hex_count": hex_count,
                "dominant_biome": dominant_biome,
                "biome_diversity": biome_diversity,
                "feature_diversity": feature_diversity,
                "biome_distribution": biomes,
                "feature_distribution": features
            }
        
        return {
            "total_regions": len(regions),
            "region_hex_counts": region_hex_counts,
            "region_characteristics": region_characteristics,
            "avg_hexes_per_region": sum(region_hex_counts.values()) / max(len(region_hex_counts), 1)
        }
    
    def _analyze_infrastructure(self, hex_map: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze infrastructure distribution across the map."""
        
        total_rivers = 0
        total_trails = 0
        total_harbors = 0
        borderline_hexes = 0
        
        region_infrastructure = {}
        
        for hex_tile in hex_map:
            region_id = hex_tile.get("region", "unknown")
            
            # Count infrastructure
            rivers = hex_tile.get("rivers", [])
            trails = hex_tile.get("trails", [])
            harbor = hex_tile.get("harbor")
            borderline = hex_tile.get("borderline", False)
            
            total_rivers += len(rivers)
            total_trails += len(trails)
            if harbor is not None:
                total_harbors += 1
            if borderline:
                borderline_hexes += 1
            
            # Track per region
            if region_id not in region_infrastructure:
                region_infrastructure[region_id] = {
                    "rivers": 0, "trails": 0, "harbors": 0, "borderline": 0
                }
            
            region_infrastructure[region_id]["rivers"] += len(rivers)
            region_infrastructure[region_id]["trails"] += len(trails)
            if harbor is not None:
                region_infrastructure[region_id]["harbors"] += 1
            if borderline:
                region_infrastructure[region_id]["borderline"] += 1
        
        return {
            "total_rivers": total_rivers,
            "total_trails": total_trails,
            "total_harbors": total_harbors,
            "borderline_hexes": borderline_hexes,
            "region_infrastructure": region_infrastructure,
            "infrastructure_density": (total_rivers + total_trails + total_harbors) / max(len(hex_map), 1)
        }
    
    def _analyze_connectivity(self, hex_map: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze connectivity patterns across the map."""
        
        # Analyze trail networks
        hexes_with_trails = len([h for h in hex_map if h.get("trails")])
        trail_coverage = hexes_with_trails / max(len(hex_map), 1)
        
        # Analyze river networks
        hexes_with_rivers = len([h for h in hex_map if h.get("rivers")])
        river_coverage = hexes_with_rivers / max(len(hex_map), 1)
        
        # Analyze harbor connectivity
        hexes_with_harbors = len([h for h in hex_map if h.get("harbor") is not None])
        harbor_coverage = hexes_with_harbors / max(len(hex_map), 1)
        
        # Overall connectivity assessment
        overall_connectivity = (trail_coverage * 0.5 + river_coverage * 0.3 + harbor_coverage * 0.2)
        
        connectivity_level = "high" if overall_connectivity >= 0.3 else "medium" if overall_connectivity >= 0.15 else "low"
        
        return {
            "trail_coverage": trail_coverage,
            "river_coverage": river_coverage,
            "harbor_coverage": harbor_coverage,
            "overall_connectivity": overall_connectivity,
            "connectivity_level": connectivity_level,
            "connected_hexes": hexes_with_trails + hexes_with_rivers + hexes_with_harbors
        }
    
    def _calculate_world_scope(self, map_analysis: dict[str, Any], region_analysis: dict[str, Any]) -> dict[str, Any]:
        """Calculate overall world scope and characteristics."""
        
        total_hexes = map_analysis["total_hexes"]
        coordinate_bounds = map_analysis["coordinate_bounds"]
        
        # Calculate world size category
        if total_hexes >= 200:
            size_category = "massive"
        elif total_hexes >= 100:
            size_category = "large"
        elif total_hexes >= 50:
            size_category = "medium"
        elif total_hexes >= 20:
            size_category = "small"
        else:
            size_category = "tiny"
        
        # Calculate world diversity
        biome_diversity = map_analysis["unique_biomes"]
        region_diversity = region_analysis["total_regions"]
        
        diversity_score = (biome_diversity * 0.6 + region_diversity * 0.4)
        
        return {
            "size_category": size_category,
            "total_hexes": total_hexes,
            "world_dimensions": coordinate_bounds,
            "biome_diversity": biome_diversity,
            "region_diversity": region_diversity,
            "diversity_score": diversity_score,
            "exploration_potential": self._calculate_exploration_potential(total_hexes, diversity_score)
        }
    
    def _calculate_exploration_potential(self, total_hexes: int, diversity_score: float) -> dict[str, Any]:
        """Calculate exploration potential for game design."""
        
        # Base exploration time (minutes per hex)
        base_time_per_hex = 3
        total_exploration_time = total_hexes * base_time_per_hex
        
        # Diversity affects replay value
        replay_value = min(1.0, diversity_score / 10.0)
        
        return {
            "estimated_exploration_hours": total_exploration_time / 60,
            "replay_value": replay_value,
            "exploration_density": "high" if total_exploration_time >= 600 else "medium" if total_exploration_time >= 300 else "low"
        }
    
    def _generate_world_hooks(self, json_entity: dict[str, Any], specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate map-specific world_hooks for game integration."""
        
        return {
            "map_uuid": specific_data["uuid"],
            "world_size": specific_data["world_scope"]["size_category"],
            "total_hexes": specific_data["map_analysis"]["total_hexes"],
            "coordinate_bounds": specific_data["map_analysis"]["coordinate_bounds"],
            "connectivity_level": specific_data["connectivity_analysis"]["connectivity_level"],
            "biome_diversity": specific_data["map_analysis"]["unique_biomes"],
            "game_integration": {
                "hex_grid_data": self._generate_hex_grid_data(json_entity),
                "biome_mapping": self._generate_biome_mapping(specific_data),
                "region_boundaries": self._generate_region_boundaries(json_entity),
                "infrastructure_overlay": self._generate_infrastructure_overlay(specific_data),
                "world_generation_seed": hash(specific_data["uuid"]) % 1000000,
                "exploration_parameters": specific_data["world_scope"]["exploration_potential"]
            }
        }
    
    def _generate_hex_grid_data(self, json_entity: dict[str, Any]) -> list[dict[str, Any]]:
        """Generate game-ready hex grid data."""
        
        hex_map = json_entity.get("map", [])
        grid_data = []
        
        for hex_tile in hex_map:
            grid_data.append({
                "x": hex_tile.get("x", 0),
                "y": hex_tile.get("y", 0),
                "uuid": hex_tile.get("uuid", ""),
                "biome_type": hex_tile.get("type", "Unknown"),
                "feature": hex_tile.get("feature"),
                "feature_uuid": hex_tile.get("feature_uuid"),
                "label": hex_tile.get("label"),
                "region": hex_tile.get("region"),
                "realm": hex_tile.get("realm"),
                "rivers": hex_tile.get("rivers", []),
                "trails": hex_tile.get("trails", []),
                "harbor": hex_tile.get("harbor"),
                "borderline": hex_tile.get("borderline", False)
            })
        
        return grid_data
    
    def _generate_biome_mapping(self, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate biome mapping data for game."""
        
        biome_distribution = specific_data["map_analysis"]["biome_distribution"]
        
        # Calculate biome percentages
        total_hexes = specific_data["map_analysis"]["total_hexes"]
        biome_percentages = {}
        
        for biome, count in biome_distribution.items():
            biome_percentages[biome] = count / total_hexes
        
        # Find dominant biomes
        sorted_biomes = sorted(biome_distribution.items(), key=lambda x: x[1], reverse=True)
        dominant_biomes = sorted_biomes[:3] if len(sorted_biomes) >= 3 else sorted_biomes
        
        return {
            "biome_distribution": biome_distribution,
            "biome_percentages": biome_percentages,
            "dominant_biomes": dominant_biomes,
            "biome_diversity_index": len(biome_distribution)
        }
    
    def _generate_region_boundaries(self, json_entity: dict[str, Any]) -> dict[str, Any]:
        """Generate region boundary data for game."""
        
        regions = json_entity.get("regions", {})
        borders = json_entity.get("borders", {})
        
        region_data = {}
        for region_id, region_name in regions.items():
            region_data[region_id] = {
                "name": region_name,
                "id": region_id
            }
        
        return {
            "regions": region_data,
            "borders": borders,
            "total_regions": len(regions)
        }
    
    def _generate_infrastructure_overlay(self, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate infrastructure overlay data for game."""
        
        infrastructure = specific_data["infrastructure_analysis"]
        connectivity = specific_data["connectivity_analysis"]
        
        return {
            "river_network": {
                "total_rivers": infrastructure["total_rivers"],
                "coverage": connectivity["river_coverage"]
            },
            "trail_network": {
                "total_trails": infrastructure["total_trails"],
                "coverage": connectivity["trail_coverage"]
            },
            "harbor_network": {
                "total_harbors": infrastructure["total_harbors"],
                "coverage": connectivity["harbor_coverage"]
            },
            "connectivity_rating": connectivity["connectivity_level"],
            "infrastructure_density": infrastructure["infrastructure_density"]
        }


def process_json_map_entity(json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
    """
    Process JSON map entity using JSONMapProcessor.
    
    Args:
        json_entity: Parsed JSON entity data
        uuid: Entity UUID
        logger: Logger instance
        console: Rich console
        
    Returns:
        Processed map data with world_hooks for game integration
    """
    
    processor = JSONMapProcessor()
    
    # Extract specific data
    specific_data = processor._extract_specific_data(json_entity, uuid, logger, console)
    
    # Generate world hooks
    world_hooks = processor._generate_world_hooks(json_entity, specific_data)
    
    return {
        "uuid": uuid,
        "entity_type": "json_map",
        "specific_data": specific_data,
        "world_hooks": world_hooks,
        "processor_type": "json_map"
    }
