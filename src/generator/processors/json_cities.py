"""
JSON Cities Processor - Specialized processor for JSON city entities.

Processes structured city JSON entities with geometric layouts, POI coordinates,
districts, and infrastructure data. Much cleaner than HTML processing.
"""

from __future__ import annotations

import json
from typing import Any

from generator.entities.processors.base import BaseProcessor


class JSONCitiesProcessor(BaseProcessor):
    """
    Specialized processor for JSON city entities.
    
    Extracts:
    - POI categories and business distribution
    - District layout and organization
    - Infrastructure analysis (roads, walls, rivers)
    - Geometric complexity and city scale
    - Coordinate mapping for game integration
    """
    
    def __init__(self):
        super().__init__("json_cities")
    
    def _extract_specific_data(self, json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
        """Extract city-specific data from JSON entity."""
        
        # Extract POI data
        poi_data = json_entity.get("poi", [])
        poi_analysis = self._analyze_pois(poi_data)
        
        # Extract geometric data
        map_data = json_entity.get("map_data", {})
        geometry_analysis = self._analyze_geometry(map_data)
        
        # Extract infrastructure
        infrastructure = self._analyze_infrastructure(map_data)
        
        # Calculate city scale
        scale_analysis = self._calculate_scale(poi_data, map_data)
        
        logger.info(f"City analysis - POIs: {len(poi_data)}, Districts: {geometry_analysis['district_count']}, Scale: {scale_analysis['scale_category']}")
        
        return {
            "uuid": uuid,
            "poi_analysis": poi_analysis,
            "geometry_analysis": geometry_analysis,
            "infrastructure": infrastructure,
            "scale_analysis": scale_analysis,
            "coordinate_system": "cartesian",
            "has_detailed_layout": True
        }
    
    def _analyze_pois(self, poi_data: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze POI distribution and categories."""
        
        poi_categories = {}
        business_types = {}
        
        for poi in poi_data:
            title = poi.get("title", "Unknown")
            
            # Categorize POI
            category = self._categorize_poi_title(title)
            poi_categories[category] = poi_categories.get(category, 0) + 1
            
            # Track specific business types
            business_types[title] = business_types.get(title, 0) + 1
        
        return {
            "total_pois": len(poi_data),
            "poi_categories": poi_categories,
            "business_types": business_types,
            "commercial_density": len([p for p in poi_data if self._is_commercial_poi(p["title"])]) / max(len(poi_data), 1),
            "service_coverage": self._calculate_service_coverage(poi_categories)
        }
    
    def _analyze_geometry(self, map_data: dict[str, Any]) -> dict[str, Any]:
        """Analyze geometric features and layout complexity."""
        
        features = map_data.get("features", [])
        
        districts = []
        buildings = []
        roads = []
        walls = []
        
        for feature in features:
            feature_id = feature.get("id", "")
            if feature_id == "districts":
                districts = feature.get("geometries", [])
            elif feature_id == "buildings":
                buildings = feature.get("coordinates", [])
            elif feature_id == "roads":
                roads = feature.get("geometries", [])
            elif feature_id == "walls":
                walls = feature.get("geometries", [])
        
        return {
            "district_count": len(districts),
            "building_count": len(buildings),
            "road_count": len(roads),
            "wall_count": len(walls),
            "has_fortifications": bool(walls),
            "layout_complexity": self._calculate_layout_complexity(districts, buildings, roads)
        }
    
    def _analyze_infrastructure(self, map_data: dict[str, Any]) -> dict[str, Any]:
        """Analyze city infrastructure."""
        
        features = map_data.get("features", [])
        
        # Extract infrastructure elements
        has_roads = False
        has_walls = False
        has_rivers = False
        has_bridges = False
        
        for feature in features:
            feature_id = feature.get("id", "")
            if feature_id == "roads":
                has_roads = True
            elif feature_id == "walls":
                has_walls = True
            elif feature_id == "rivers":
                has_rivers = True
            elif feature_id == "planks":
                has_bridges = True
        
        return {
            "has_roads": has_roads,
            "has_walls": has_walls,
            "has_rivers": has_rivers,
            "has_bridges": has_bridges,
            "fortification_level": "high" if has_walls else "none",
            "connectivity": "high" if has_roads and has_bridges else "medium" if has_roads else "low"
        }
    
    def _calculate_scale(self, poi_data: list[dict[str, Any]], map_data: dict[str, Any]) -> dict[str, Any]:
        """Calculate city scale and importance."""
        
        poi_count = len(poi_data)
        
        # Get dimensions from map data
        values = map_data.get("features", [{}])[0] if map_data.get("features") else {}
        width = values.get("width", 0) if isinstance(values, dict) else 0
        height = values.get("height", 0) if isinstance(values, dict) else 0
        
        # Classify scale
        if poi_count >= 50:
            scale_category = "metropolis"
        elif poi_count >= 30:
            scale_category = "city"
        elif poi_count >= 15:
            scale_category = "town"
        elif poi_count >= 5:
            scale_category = "village"
        else:
            scale_category = "hamlet"
        
        return {
            "poi_count": poi_count,
            "width": width,
            "height": height,
            "scale_category": scale_category,
            "importance_score": self._calculate_importance_score(poi_count, width, height)
        }
    
    def _categorize_poi_title(self, title: str) -> str:
        """Categorize POI by title."""
        
        title_lower = title.lower()
        
        # Commercial categories
        if any(word in title_lower for word in ["market", "shop", "store", "goods"]):
            return "commercial"
        elif any(word in title_lower for word in ["inn", "tavern", "lodge"]):
            return "hospitality"
        elif any(word in title_lower for word in ["blacksmith", "armor", "weapons"]):
            return "crafting"
        elif any(word in title_lower for word in ["temple", "shrine", "church"]):
            return "religious"
        elif any(word in title_lower for word in ["school", "library", "scribe"]):
            return "education"
        elif any(word in title_lower for word in ["bank", "treasury"]):
            return "financial"
        elif any(word in title_lower for word in ["stables", "carter", "transport"]):
            return "transport"
        elif any(word in title_lower for word in ["healer", "physician", "herbalist"]):
            return "medical"
        else:
            return "other"
    
    def _is_commercial_poi(self, title: str) -> bool:
        """Check if POI is commercial."""
        commercial_categories = {"commercial", "crafting", "hospitality", "financial"}
        return self._categorize_poi_title(title) in commercial_categories
    
    def _calculate_service_coverage(self, poi_categories: dict[str, int]) -> float:
        """Calculate service coverage score (0.0-1.0)."""
        
        essential_services = {"commercial", "hospitality", "crafting", "medical"}
        covered_services = len([cat for cat in essential_services if poi_categories.get(cat, 0) > 0])
        
        return covered_services / len(essential_services)
    
    def _calculate_layout_complexity(self, districts: list, buildings: list, roads: list) -> str:
        """Calculate layout complexity."""
        
        complexity_score = 0
        
        # Districts add complexity
        complexity_score += len(districts) * 2
        
        # Many buildings add complexity
        if len(buildings) >= 100:
            complexity_score += 3
        elif len(buildings) >= 50:
            complexity_score += 2
        elif len(buildings) >= 20:
            complexity_score += 1
        
        # Road network adds complexity
        if len(roads) >= 10:
            complexity_score += 2
        elif len(roads) >= 5:
            complexity_score += 1
        
        if complexity_score >= 8:
            return "very_complex"
        elif complexity_score >= 5:
            return "complex"
        elif complexity_score >= 3:
            return "moderate"
        else:
            return "simple"
    
    def _calculate_importance_score(self, poi_count: int, width: float, height: float) -> float:
        """Calculate city importance score."""
        
        poi_score = min(poi_count / 50.0, 1.0)  # Normalize to 50 POIs = max
        size_score = min((width * height) / 10000000, 1.0)  # Normalize size
        
        return (poi_score * 0.7 + size_score * 0.3)  # POIs matter more
    
    def _generate_world_hooks(self, json_entity: dict[str, Any], specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate city-specific world_hooks for game integration."""
        
        return {
            "city_uuid": specific_data["uuid"],
            "scale_category": specific_data["scale_analysis"]["scale_category"],
            "poi_count": specific_data["poi_analysis"]["total_pois"],
            "has_fortifications": specific_data["geometry_analysis"]["has_fortifications"],
            "service_coverage": specific_data["poi_analysis"]["service_coverage"],
            "importance_score": specific_data["scale_analysis"]["importance_score"],
            "layout_complexity": specific_data["geometry_analysis"]["layout_complexity"],
            "game_integration": {
                "city_tileset": f"res://art/cities/{specific_data['scale_analysis']['scale_category']}_tiles.png",
                "poi_markers": self._generate_poi_markers(json_entity.get("poi", [])),
                "district_boundaries": self._extract_district_data(json_entity.get("map_data", {})),
                "spawn_points": self._calculate_spawn_points(specific_data),
                "economic_level": self._calculate_economic_level(specific_data["poi_analysis"]),
                "security_level": self._calculate_security_level(specific_data["geometry_analysis"])
            }
        }
    
    def _generate_poi_markers(self, poi_data: list[dict[str, Any]]) -> list[dict[str, Any]]:
        """Generate game-ready POI markers."""
        
        markers = []
        for poi in poi_data:
            coords = poi.get("coords", {})
            markers.append({
                "x": coords.get("x", 0),
                "y": coords.get("y", 0),
                "title": poi.get("title", "Unknown"),
                "category": self._categorize_poi_title(poi.get("title", "")),
                "uuid": poi.get("uuid", "")
            })
        
        return markers
    
    def _extract_district_data(self, map_data: dict[str, Any]) -> list[dict[str, Any]]:
        """Extract district boundary data for game."""
        
        features = map_data.get("features", [])
        
        for feature in features:
            if feature.get("id") == "districts":
                geometries = feature.get("geometries", [])
                return [{"name": geo.get("name", "Unknown"), "coordinates": geo.get("coordinates", [])} 
                       for geo in geometries]
        
        return []
    
    def _calculate_spawn_points(self, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Calculate NPC spawn configuration."""
        
        poi_count = specific_data["poi_analysis"]["total_pois"]
        
        return {
            "merchant_spawn_rate": min(1.0, poi_count / 20.0),
            "guard_spawn_rate": 0.5 if specific_data["geometry_analysis"]["has_fortifications"] else 0.2,
            "civilian_spawn_rate": min(1.0, poi_count / 30.0),
            "max_npcs": min(poi_count * 2, 100)
        }
    
    def _calculate_economic_level(self, poi_analysis: dict[str, Any]) -> str:
        """Calculate economic activity level."""
        
        commercial_density = poi_analysis["commercial_density"]
        
        if commercial_density >= 0.4:
            return "thriving"
        elif commercial_density >= 0.25:
            return "active"
        elif commercial_density >= 0.1:
            return "modest"
        else:
            return "poor"
    
    def _calculate_security_level(self, geometry_analysis: dict[str, Any]) -> str:
        """Calculate security level."""
        
        if geometry_analysis["has_fortifications"]:
            return "high"
        elif geometry_analysis["wall_count"] > 0:
            return "medium"
        else:
            return "low"


def process_json_city_entity(json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
    """
    Process JSON city entity using JSONCitiesProcessor.
    
    Args:
        json_entity: Parsed JSON entity data
        uuid: Entity UUID
        logger: Logger instance
        console: Rich console
        
    Returns:
        Processed city data with world_hooks for game integration
    """
    
    processor = JSONCitiesProcessor()
    
    # Extract specific data
    specific_data = processor._extract_specific_data(json_entity, uuid, logger, console)
    
    # Generate world hooks
    world_hooks = processor._generate_world_hooks(json_entity, specific_data)
    
    return {
        "uuid": uuid,
        "entity_type": "json_city",
        "specific_data": specific_data,
        "world_hooks": world_hooks,
        "processor_type": "json_cities"
    }
