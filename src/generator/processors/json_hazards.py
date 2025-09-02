"""
JSON Hazards Processor - Specialized processor for JSON hazards entities.

Processes structured hazards JSON entities with coordinate-based hazard placement
and room-specific danger mapping. Complements dungeon layout data.
"""

from __future__ import annotations

from typing import Any

from generator.entities.processors.base import BaseProcessor


class JSONHazardsProcessor(BaseProcessor):
    """
    Specialized processor for JSON hazards entities.
    
    Extracts:
    - Hazard count and distribution patterns
    - Room-specific hazard mapping
    - Coordinate-based placement data
    - Danger density analysis
    - Cross-reference with dungeon room layouts
    """
    
    def __init__(self):
        super().__init__("json_hazards")
    
    def _extract_specific_data(self, json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
        """Extract hazards-specific data from JSON entity."""
        
        # Extract hazards data
        hazards_data = json_entity.get("hazards", [])
        hazard_analysis = self._analyze_hazards(hazards_data)
        
        # Analyze spatial distribution
        spatial_analysis = self._analyze_spatial_distribution(hazards_data)
        
        # Analyze room mapping
        room_mapping = self._analyze_room_mapping(hazards_data)
        
        # Calculate danger assessment
        danger_assessment = self._calculate_danger_assessment(hazard_analysis, spatial_analysis)
        
        logger.info(f"Hazards analysis - Count: {hazard_analysis['total_hazards']}, Rooms: {room_mapping['rooms_with_hazards']}, Danger: {danger_assessment['danger_level']}")
        
        return {
            "uuid": uuid,
            "hazard_analysis": hazard_analysis,
            "spatial_analysis": spatial_analysis,
            "room_mapping": room_mapping,
            "danger_assessment": danger_assessment,
            "coordinate_system": "grid",
            "has_room_references": bool(room_mapping["rooms_with_hazards"])
        }
    
    def _analyze_hazards(self, hazards_data: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze hazard count and basic characteristics."""
        
        total_hazards = len(hazards_data)
        
        # Extract coordinate ranges
        x_coords = [h.get("x", 0) for h in hazards_data]
        y_coords = [h.get("y", 0) for h in hazards_data]
        
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
        
        return {
            "total_hazards": total_hazards,
            "coordinate_bounds": coordinate_bounds,
            "has_coordinates": bool(x_coords and y_coords)
        }
    
    def _analyze_spatial_distribution(self, hazards_data: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze spatial distribution patterns of hazards."""
        
        if not hazards_data:
            return {
                "distribution_type": "none",
                "clustering": "none",
                "density": 0.0
            }
        
        # Calculate hazard density
        x_coords = [h.get("x", 0) for h in hazards_data]
        y_coords = [h.get("y", 0) for h in hazards_data]
        
        if not x_coords or not y_coords:
            return {
                "distribution_type": "unknown",
                "clustering": "unknown", 
                "density": 0.0
            }
        
        # Calculate area coverage
        width = max(x_coords) - min(x_coords)
        height = max(y_coords) - min(y_coords)
        area = max(width * height, 1)
        density = len(hazards_data) / area * 1000  # Hazards per 1000 sq units
        
        # Analyze clustering using simple distance analysis
        clustering = self._analyze_hazard_clustering(hazards_data)
        
        # Determine distribution type
        if clustering["cluster_coefficient"] >= 0.7:
            distribution_type = "clustered"
        elif clustering["cluster_coefficient"] >= 0.3:
            distribution_type = "mixed"
        else:
            distribution_type = "scattered"
        
        return {
            "distribution_type": distribution_type,
            "clustering": clustering,
            "density": density,
            "area_coverage": area,
            "hazards_per_sq_unit": density / 1000
        }
    
    def _analyze_room_mapping(self, hazards_data: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze which rooms contain hazards."""
        
        room_references = set()
        room_hazard_counts = {}
        
        for hazard in hazards_data:
            room_n = hazard.get("n")
            if room_n is not None:
                room_references.add(room_n)
                room_hazard_counts[room_n] = room_hazard_counts.get(room_n, 0) + 1
        
        # Analyze hazard distribution across rooms
        rooms_with_hazards = len(room_references)
        
        if room_hazard_counts:
            max_hazards_per_room = max(room_hazard_counts.values())
            avg_hazards_per_room = sum(room_hazard_counts.values()) / len(room_hazard_counts)
        else:
            max_hazards_per_room = avg_hazards_per_room = 0
        
        return {
            "rooms_with_hazards": rooms_with_hazards,
            "room_references": list(room_references),
            "room_hazard_counts": room_hazard_counts,
            "max_hazards_per_room": max_hazards_per_room,
            "avg_hazards_per_room": avg_hazards_per_room,
            "hazard_coverage": rooms_with_hazards  # Without knowing total rooms, this is partial
        }
    
    def _analyze_hazard_clustering(self, hazards_data: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze hazard clustering patterns."""
        
        if len(hazards_data) < 2:
            return {
                "cluster_coefficient": 0.0,
                "avg_distance": 0.0,
                "max_distance": 0.0
            }
        
        # Calculate pairwise distances
        distances = []
        
        for i, h1 in enumerate(hazards_data):
            for h2 in hazards_data[i+1:]:
                x1, y1 = h1.get("x", 0), h1.get("y", 0)
                x2, y2 = h2.get("x", 0), h2.get("y", 0)
                
                distance = ((x2 - x1) ** 2 + (y2 - y1) ** 2) ** 0.5
                distances.append(distance)
        
        if distances:
            avg_distance = sum(distances) / len(distances)
            max_distance = max(distances)
            min_distance = min(distances)
            
            # Cluster coefficient: how many hazards are close together
            close_threshold = avg_distance * 0.5
            close_pairs = len([d for d in distances if d <= close_threshold])
            cluster_coefficient = close_pairs / len(distances)
        else:
            avg_distance = max_distance = min_distance = cluster_coefficient = 0.0
        
        return {
            "cluster_coefficient": cluster_coefficient,
            "avg_distance": avg_distance,
            "max_distance": max_distance,
            "min_distance": min_distance,
            "total_distance_pairs": len(distances)
        }
    
    def _calculate_danger_assessment(self, hazard_analysis: dict[str, Any], 
                                   spatial_analysis: dict[str, Any]) -> dict[str, Any]:
        """Calculate overall danger assessment."""
        
        hazard_count = hazard_analysis["total_hazards"]
        density = spatial_analysis["density"]
        distribution = spatial_analysis["distribution_type"]
        
        # Calculate danger score
        danger_score = 0
        
        # Hazard count contribution
        if hazard_count >= 20:
            danger_score += 3
        elif hazard_count >= 10:
            danger_score += 2
        elif hazard_count >= 5:
            danger_score += 1
        
        # Density contribution
        if density >= 5.0:  # High density
            danger_score += 2
        elif density >= 2.0:  # Medium density
            danger_score += 1
        
        # Distribution contribution
        if distribution == "clustered":
            danger_score += 1  # Clustered hazards create danger zones
        
        # Classify danger level
        if danger_score >= 6:
            danger_level = "extreme"
        elif danger_score >= 4:
            danger_level = "high"
        elif danger_score >= 2:
            danger_level = "moderate"
        elif danger_score >= 1:
            danger_level = "low"
        else:
            danger_level = "minimal"
        
        return {
            "danger_level": danger_level,
            "danger_score": danger_score,
            "hazard_density_category": self._categorize_density(density),
            "requires_caution": danger_score >= 3,
            "has_danger_zones": distribution == "clustered"
        }
    
    def _categorize_density(self, density: float) -> str:
        """Categorize hazard density."""
        
        if density >= 5.0:
            return "very_high"
        elif density >= 2.0:
            return "high"
        elif density >= 1.0:
            return "medium"
        elif density >= 0.5:
            return "low"
        else:
            return "sparse"
    
    def _generate_world_hooks(self, json_entity: dict[str, Any], specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate hazards-specific world_hooks for game integration."""
        
        return {
            "hazards_uuid": specific_data["uuid"],
            "total_hazards": specific_data["hazard_analysis"]["total_hazards"],
            "danger_level": specific_data["danger_assessment"]["danger_level"],
            "distribution_type": specific_data["spatial_analysis"]["distribution_type"],
            "rooms_affected": specific_data["room_mapping"]["rooms_with_hazards"],
            "requires_caution": specific_data["danger_assessment"]["requires_caution"],
            "game_integration": {
                "hazard_spawn_data": self._generate_hazard_spawn_data(json_entity),
                "danger_zones": self._generate_danger_zones(specific_data),
                "room_hazard_mapping": specific_data["room_mapping"]["room_hazard_counts"],
                "caution_level": specific_data["danger_assessment"]["danger_level"],
                "hazard_density": specific_data["spatial_analysis"]["density"]
            }
        }
    
    def _generate_hazard_spawn_data(self, json_entity: dict[str, Any]) -> list[dict[str, Any]]:
        """Generate game-ready hazard spawn data."""
        
        hazards_data = json_entity.get("hazards", [])
        spawn_data = []
        
        for hazard in hazards_data:
            spawn_data.append({
                "x": hazard.get("x", 0),
                "y": hazard.get("y", 0),
                "room_number": hazard.get("n"),
                "hazard_type": "trap",  # Default type, could be enhanced
                "spawn_probability": 1.0  # All hazards from JSON are confirmed
            })
        
        return spawn_data
    
    def _generate_danger_zones(self, specific_data: dict[str, Any]) -> list[dict[str, Any]]:
        """Generate danger zone data for game."""
        
        danger_zones = []
        
        # If hazards are clustered, create danger zones
        if specific_data["spatial_analysis"]["distribution_type"] == "clustered":
            # Simple implementation: create zones around high-hazard rooms
            room_counts = specific_data["room_mapping"]["room_hazard_counts"]
            
            for room_n, hazard_count in room_counts.items():
                if hazard_count >= 3:  # High hazard rooms become danger zones
                    danger_zones.append({
                        "room_number": room_n,
                        "hazard_count": hazard_count,
                        "zone_type": "high_danger",
                        "warning_required": True
                    })
        
        return danger_zones


def process_json_hazards_entity(json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
    """
    Process JSON hazards entity using JSONHazardsProcessor.
    
    Args:
        json_entity: Parsed JSON entity data
        uuid: Entity UUID
        logger: Logger instance
        console: Rich console
        
    Returns:
        Processed hazards data with world_hooks for game integration
    """
    
    processor = JSONHazardsProcessor()
    
    # Extract specific data
    specific_data = processor._extract_specific_data(json_entity, uuid, logger, console)
    
    # Generate world hooks
    world_hooks = processor._generate_world_hooks(json_entity, specific_data)
    
    return {
        "uuid": uuid,
        "entity_type": "json_hazards",
        "specific_data": specific_data,
        "world_hooks": world_hooks,
        "processor_type": "json_hazards"
    }
