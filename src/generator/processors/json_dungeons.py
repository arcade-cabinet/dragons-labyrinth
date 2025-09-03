"""
JSON Dungeons Processor - Specialized processor for JSON dungeon entities.

Processes structured dungeon JSON entities with room layouts, cavern geometry,
and structural data. Much cleaner than HTML processing.
"""

from __future__ import annotations

from typing import Any

from generator.entities.processors.base import BaseProcessor


class JSONDungeonsProcessor(BaseProcessor):
    """
    Specialized processor for JSON dungeon entities.
    
    Extracts:
    - Room count and layout complexity
    - Cavern geometry and connectivity
    - Structural analysis and navigation difficulty
    - Coordinate mapping for room generation
    - Cross-reference with hazards data
    """
    
    def __init__(self):
        super().__init__("json_dungeons")
    
    def _extract_specific_data(self, json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
        """Extract dungeon-specific data from JSON entity."""
        
        # Extract cavern data
        caverns = json_entity.get("caverns", [])
        room_analysis = self._analyze_rooms(caverns)
        
        # Analyze geometric complexity
        geometry_analysis = self._analyze_geometry(caverns)
        
        # Analyze connectivity patterns
        connectivity = self._analyze_connectivity(caverns)
        
        # Calculate navigation difficulty
        navigation_analysis = self._calculate_navigation_difficulty(room_analysis, geometry_analysis, connectivity)
        
        logger.info(f"Dungeon analysis - Rooms: {room_analysis['room_count']}, Complexity: {geometry_analysis['complexity_level']}, Navigation: {navigation_analysis['difficulty_level']}")
        
        return {
            "uuid": uuid,
            "room_analysis": room_analysis,
            "geometry_analysis": geometry_analysis,
            "connectivity": connectivity,
            "navigation_analysis": navigation_analysis,
            "coordinate_system": "grid",
            "has_structural_layout": True
        }
    
    def _analyze_rooms(self, caverns: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze room count and characteristics."""
        
        room_count = len(caverns)
        
        # Analyze room sizes (by polygon vertex count)
        room_sizes = []
        total_vertices = 0
        
        for cavern in caverns:
            polygon = cavern.get("polygon", [])
            vertex_count = len(polygon)
            room_sizes.append(vertex_count)
            total_vertices += vertex_count
        
        avg_room_complexity = total_vertices / max(room_count, 1)
        
        # Classify room distribution
        if room_count >= 40:
            scale_category = "massive"
        elif room_count >= 20:
            scale_category = "large"
        elif room_count >= 10:
            scale_category = "medium"
        elif room_count >= 5:
            scale_category = "small"
        else:
            scale_category = "tiny"
        
        return {
            "room_count": room_count,
            "total_vertices": total_vertices,
            "avg_room_complexity": avg_room_complexity,
            "scale_category": scale_category,
            "room_sizes": room_sizes,
            "complexity_variance": max(room_sizes) - min(room_sizes) if room_sizes else 0
        }
    
    def _analyze_geometry(self, caverns: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze geometric complexity of cavern layouts."""
        
        # Extract coordinate ranges
        all_x_coords = []
        all_y_coords = []
        
        for cavern in caverns:
            polygon = cavern.get("polygon", [])
            for point in polygon:
                all_x_coords.append(point.get("x", 0))
                all_y_coords.append(point.get("y", 0))
        
        # Calculate dungeon dimensions
        if all_x_coords and all_y_coords:
            width = max(all_x_coords) - min(all_x_coords)
            height = max(all_y_coords) - min(all_y_coords)
            area = width * height
        else:
            width = height = area = 0
        
        # Analyze room distribution patterns
        room_density = len(caverns) / max(area / 1000, 1)  # Rooms per 1000 sq units
        
        # Determine complexity level
        total_vertices = sum(len(cavern.get("polygon", [])) for cavern in caverns)
        
        if total_vertices >= 300:
            complexity_level = "very_complex"
        elif total_vertices >= 150:
            complexity_level = "complex"
        elif total_vertices >= 75:
            complexity_level = "moderate"
        else:
            complexity_level = "simple"
        
        return {
            "width": width,
            "height": height,
            "area": area,
            "room_density": room_density,
            "complexity_level": complexity_level,
            "total_vertices": total_vertices,
            "coordinate_bounds": {
                "x_min": min(all_x_coords) if all_x_coords else 0,
                "x_max": max(all_x_coords) if all_x_coords else 0,
                "y_min": min(all_y_coords) if all_y_coords else 0,
                "y_max": max(all_y_coords) if all_y_coords else 0
            }
        }
    
    def _analyze_connectivity(self, caverns: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze room connectivity patterns."""
        
        room_count = len(caverns)
        
        # Analyze room connections by proximity
        connections = self._find_room_connections(caverns)
        total_connections = len(connections)
        
        # Calculate connectivity metrics
        avg_connections = total_connections / max(room_count, 1)
        
        # Determine layout type
        if avg_connections >= 3.0:
            layout_type = "highly_connected"
        elif avg_connections >= 2.0:
            layout_type = "well_connected"
        elif avg_connections >= 1.5:
            layout_type = "moderately_connected"
        else:
            layout_type = "linear"
        
        return {
            "total_connections": total_connections,
            "avg_connections_per_room": avg_connections,
            "layout_type": layout_type,
            "connection_map": connections,
            "is_linear": layout_type == "linear",
            "has_loops": avg_connections >= 2.0
        }
    
    def _find_room_connections(self, caverns: list[dict[str, Any]]) -> list[dict[str, Any]]:
        """Find connections between rooms based on coordinate proximity."""
        
        connections = []
        
        # Calculate center points for each room
        room_centers = []
        for i, cavern in enumerate(caverns):
            polygon = cavern.get("polygon", [])
            if polygon:
                x_coords = [p.get("x", 0) for p in polygon]
                y_coords = [p.get("y", 0) for p in polygon]
                center_x = sum(x_coords) / len(x_coords)
                center_y = sum(y_coords) / len(y_coords)
                room_centers.append((i, center_x, center_y, cavern.get("uuid", f"room_{i}")))
        
        # Find connections based on proximity (within 20 units)
        connection_threshold = 20
        
        for i, (room_i, x1, y1, uuid1) in enumerate(room_centers):
            for j, (room_j, x2, y2, uuid2) in enumerate(room_centers[i+1:], i+1):
                distance = ((x2 - x1) ** 2 + (y2 - y1) ** 2) ** 0.5
                
                if distance <= connection_threshold:
                    connections.append({
                        "room1": uuid1,
                        "room2": uuid2,
                        "distance": distance,
                        "connection_type": "adjacent"
                    })
        
        return connections
    
    def _calculate_navigation_difficulty(self, room_analysis: dict[str, Any], 
                                       geometry_analysis: dict[str, Any], 
                                       connectivity: dict[str, Any]) -> dict[str, Any]:
        """Calculate navigation difficulty based on layout."""
        
        difficulty_score = 0
        
        # Room count affects difficulty
        room_count = room_analysis["room_count"]
        if room_count >= 30:
            difficulty_score += 3
        elif room_count >= 15:
            difficulty_score += 2
        elif room_count >= 8:
            difficulty_score += 1
        
        # Complexity affects difficulty
        complexity = geometry_analysis["complexity_level"]
        if complexity == "very_complex":
            difficulty_score += 3
        elif complexity == "complex":
            difficulty_score += 2
        elif complexity == "moderate":
            difficulty_score += 1
        
        # Connectivity patterns affect difficulty
        layout_type = connectivity["layout_type"]
        if layout_type == "highly_connected":
            difficulty_score += 2  # Many paths = confusion
        elif layout_type == "linear":
            difficulty_score -= 1  # Linear is easier
        
        # Classify difficulty
        if difficulty_score >= 7:
            difficulty_level = "extreme"
        elif difficulty_score >= 5:
            difficulty_level = "hard"
        elif difficulty_score >= 3:
            difficulty_level = "moderate"
        elif difficulty_score >= 1:
            difficulty_level = "easy"
        else:
            difficulty_level = "trivial"
        
        return {
            "difficulty_level": difficulty_level,
            "difficulty_score": difficulty_score,
            "navigation_factors": {
                "room_count_factor": room_count >= 15,
                "high_complexity": complexity in ["complex", "very_complex"],
                "confusing_layout": layout_type == "highly_connected"
            }
        }
    
    def _generate_world_hooks(self, json_entity: dict[str, Any], specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate dungeon-specific world_hooks for game integration."""
        
        return {
            "dungeon_uuid": specific_data["uuid"],
            "room_count": specific_data["room_analysis"]["room_count"],
            "scale_category": specific_data["room_analysis"]["scale_category"],
            "complexity_level": specific_data["geometry_analysis"]["complexity_level"],
            "navigation_difficulty": specific_data["navigation_analysis"]["difficulty_level"],
            "layout_type": specific_data["connectivity"]["layout_type"],
            "game_integration": {
                "dungeon_tileset": f"res://art/dungeons/{specific_data['room_analysis']['scale_category']}_tiles.png",
                "room_generation_data": self._generate_room_data(json_entity),
                "navigation_hints": self._generate_navigation_hints(specific_data),
                "procedural_seed": hash(specific_data["uuid"]) % 1000000,
                "layout_complexity": specific_data["geometry_analysis"]["complexity_level"],
                "expected_exploration_time": self._estimate_exploration_time(specific_data)
            }
        }
    
    def _generate_room_data(self, json_entity: dict[str, Any]) -> list[dict[str, Any]]:
        """Generate game-ready room data."""
        
        caverns = json_entity.get("caverns", [])
        room_data = []
        
        for cavern in caverns:
            polygon = cavern.get("polygon", [])
            
            # Calculate room bounds
            if polygon:
                x_coords = [p.get("x", 0) for p in polygon]
                y_coords = [p.get("y", 0) for p in polygon]
                bounds = {
                    "x_min": min(x_coords),
                    "x_max": max(x_coords),
                    "y_min": min(y_coords),
                    "y_max": max(y_coords)
                }
                center = {
                    "x": (bounds["x_min"] + bounds["x_max"]) / 2,
                    "y": (bounds["y_min"] + bounds["y_max"]) / 2
                }
                area = (bounds["x_max"] - bounds["x_min"]) * (bounds["y_max"] - bounds["y_min"])
            else:
                bounds = center = {"x": 0, "y": 0}
                area = 0
            
            room_data.append({
                "room_uuid": cavern.get("uuid", f"room_{cavern.get('n', 0)}"),
                "room_number": cavern.get("n", 0),
                "polygon": polygon,
                "bounds": bounds,
                "center": center,
                "area": area,
                "vertex_count": len(polygon)
            })
        
        return room_data
    
    def _generate_navigation_hints(self, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate navigation hints for game."""
        
        return {
            "layout_type": specific_data["connectivity"]["layout_type"],
            "has_loops": specific_data["connectivity"]["has_loops"],
            "complexity_warning": specific_data["geometry_analysis"]["complexity_level"] in ["complex", "very_complex"],
            "room_count_warning": specific_data["room_analysis"]["room_count"] >= 20,
            "difficulty_level": specific_data["navigation_analysis"]["difficulty_level"]
        }
    
    def _estimate_exploration_time(self, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Estimate exploration time based on complexity."""
        
        room_count = specific_data["room_analysis"]["room_count"]
        complexity = specific_data["geometry_analysis"]["complexity_level"]
        difficulty = specific_data["navigation_analysis"]["difficulty_level"]
        
        # Base time per room (in minutes)
        base_time_per_room = 2
        
        # Complexity multiplier
        complexity_multipliers = {
            "simple": 1.0,
            "moderate": 1.3,
            "complex": 1.6,
            "very_complex": 2.0
        }
        
        # Difficulty multiplier
        difficulty_multipliers = {
            "trivial": 0.8,
            "easy": 1.0,
            "moderate": 1.2,
            "hard": 1.5,
            "extreme": 2.0
        }
        
        complexity_mult = complexity_multipliers.get(complexity, 1.0)
        difficulty_mult = difficulty_multipliers.get(difficulty, 1.0)
        
        estimated_minutes = room_count * base_time_per_room * complexity_mult * difficulty_mult
        
        return {
            "estimated_minutes": int(estimated_minutes),
            "estimated_hours": estimated_minutes / 60,
            "complexity_factor": complexity_mult,
            "difficulty_factor": difficulty_mult,
            "time_category": self._categorize_exploration_time(estimated_minutes)
        }
    
    def _categorize_exploration_time(self, minutes: float) -> str:
        """Categorize exploration time."""
        
        if minutes >= 120:  # 2+ hours
            return "epic"
        elif minutes >= 60:  # 1+ hour
            return "long"
        elif minutes >= 30:  # 30+ minutes
            return "medium"
        elif minutes >= 15:  # 15+ minutes
            return "short"
        else:
            return "quick"


def process_json_dungeon_entity(json_entity: dict[str, Any], uuid: str, logger, console) -> dict[str, Any]:
    """
    Process JSON dungeon entity using JSONDungeonsProcessor.
    
    Args:
        json_entity: Parsed JSON entity data
        uuid: Entity UUID
        logger: Logger instance
        console: Rich console
        
    Returns:
        Processed dungeon data with world_hooks for game integration
    """
    
    processor = JSONDungeonsProcessor()
    
    # Extract specific data
    specific_data = processor._extract_specific_data(json_entity, uuid, logger, console)
    
    # Generate world hooks
    world_hooks = processor._generate_world_hooks(json_entity, specific_data)
    
    return {
        "uuid": uuid,
        "entity_type": "json_dungeon",
        "specific_data": specific_data,
        "world_hooks": world_hooks,
        "processor_type": "json_dungeons"
    }
