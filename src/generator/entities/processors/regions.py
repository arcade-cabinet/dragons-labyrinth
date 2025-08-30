"""
RegionsProcessor - Specialized processor for region entity clusters.

Processes region clusters from the transformer, extracting geographical
information, biome distributions, settlement patterns, and connectivity
data with world_hooks for Godot integration.
"""

from __future__ import annotations

import re
from typing import Any

from generator.constants import BIOME_KEYWORDS, CORRUPTION_THEMES, SETTLEMENTS
from generator.entities.processors.base import BaseProcessor


class RegionsProcessor(BaseProcessor):
    """
    Specialized processor for region entity clusters.
    
    Extracts:
    - Biome distributions and dominant terrain types
    - Settlement locations and connectivity
    - Geographic features (rivers, trails, harbors)
    - Corruption levels based on region characteristics
    - Spatial relationships for hex mapping
    """
    
    def __init__(self):
        super().__init__("regions")
        
        # Load region-specific configuration from constants
        self.biome_keywords = BIOME_KEYWORDS
        self.corruption_themes = CORRUPTION_THEMES
        self.known_settlements = SETTLEMENTS
        
        # Configure region processing parameters
        self.connectivity_weights = {
            "rivers": 0.3,
            "trails": 0.4,
            "harbors": 0.2,
            "borders": -0.1  # Negative impact
        }
        
        # Set up biome analysis patterns
        self.biome_patterns = {}
        for biome, keywords in self.biome_keywords.items():
            pattern_str = "|".join(keywords)
            self.biome_patterns[biome] = re.compile(f"({pattern_str})", re.IGNORECASE)
    
    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger, console) -> dict[str, Any]:
        """Extract region-specific data from cluster entities and ML results."""
        
        entities = ml_results.get("entities", [])
        
        # Analyze region characteristics
        biome_distribution = self._analyze_biome_distribution(cluster)
        settlement_locations = self._extract_settlement_locations(cluster)
        geographic_features = self._analyze_geographic_features(cluster)
        corruption_level = self._calculate_corruption_level(cluster.name)
        
        # Use ML results for enhanced analysis
        ml_confidence = self._calculate_ml_confidence(entities)
        relationships = ml_results.get("relationships", [])
        
        logger.info(f"Region analysis - Biomes: {len(biome_distribution)}, Settlements: {len(settlement_locations)}, Corruption: {corruption_level}")
        
        return {
            "name": cluster.name,
            "dominant_biome": self._determine_dominant_biome(biome_distribution),
            "biome_distribution": biome_distribution,
            "settlement_locations": settlement_locations,
            "geographic_features": geographic_features,
            "corruption_level": corruption_level,
            "total_hexes": len(cluster.entities),
            "settlement_density": len(settlement_locations) / max(len(cluster.entities), 1),
            "ml_confidence": ml_confidence,
            "entity_relationships": len(relationships),
            "anomaly_count": ml_results.get("anomaly_count", 0)
        }
    
    def _generate_world_hooks(self, cluster, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate region-specific world_hooks for Godot integration."""
        
        return {
            "region_name": cluster.name,
            "dominant_biome": specific_data.get("dominant_biome", "unknown"),
            "biome_distribution": specific_data.get("biome_distribution", {}),
            "settlement_count": len(specific_data.get("settlement_locations", [])),
            "has_rivers": specific_data.get("geographic_features", {}).get("rivers", 0) > 0,
            "has_trails": specific_data.get("geographic_features", {}).get("trails", 0) > 0,
            "has_harbors": specific_data.get("geographic_features", {}).get("harbors", 0) > 0,
            "has_borders": specific_data.get("geographic_features", {}).get("borders", 0) > 0,
            "corruption_level": specific_data.get("corruption_level", 0),
            "total_hexes": specific_data.get("total_hexes", 0),
            "godot_integration": {
                "biome_sprite_path": f"res://art/biomes/{specific_data.get('dominant_biome', 'unknown').lower()}.png",
                "region_map_path": f"res://art/regions/{cluster.name.lower().replace(' ', '_')}.png",
                "hex_spawn_count": specific_data.get("total_hexes", 0),
                "settlement_spawn_points": len(specific_data.get("settlement_locations", [])),
                "corruption_intensity": specific_data.get("corruption_level", 0) / 5.0,
                "connectivity_score": self._calculate_connectivity_score(specific_data.get("geographic_features", {}))
            }
        }
    
    def _analyze_biome_distribution(self, cluster) -> dict[str, int]:
        """Analyze biome distribution across region entities."""
        
        biome_counts = {}
        
        for entity in cluster.entities:
            # Check entity type for biome information
            entity_type = entity.get("type", "")
            if entity_type.endswith("Hex"):
                biome = entity_type.replace("Hex", "")
                biome_counts[biome] = biome_counts.get(biome, 0) + 1
            
            # Check for biome keywords in content
            content = str(entity).lower()
            biome_keywords = {
                "Forest": ["forest", "tree", "wood", "grove"],
                "Desert": ["desert", "sand", "dune", "arid"],
                "Mountain": ["mountain", "hill", "peak", "crag"],
                "Plains": ["plain", "field", "grass", "meadow"],
                "Swamp": ["swamp", "bog", "marsh", "wetland"],
                "Tundra": ["tundra", "frozen", "ice", "snow"]
            }
            
            for biome, keywords in biome_keywords.items():
                if any(keyword in content for keyword in keywords):
                    biome_counts[biome] = biome_counts.get(biome, 0) + 1
        
        return biome_counts
    
    def _determine_dominant_biome(self, biome_distribution: dict[str, int]) -> str:
        """Determine the dominant biome type for the region."""
        
        if not biome_distribution:
            return "Unknown"
        
        return max(biome_distribution.items(), key=lambda x: x[1])[0]
    
    def _extract_settlement_locations(self, cluster) -> list[dict[str, Any]]:
        """Extract settlement locations within the region."""
        
        settlements = []
        
        for entity in cluster.entities:
            content = str(entity)
            
            # Check for known settlements
            for settlement in self.known_settlements:
                if settlement in content:
                    settlement_type = "village"
                    if settlement.startswith("Town of"):
                        settlement_type = "town"
                    elif settlement.startswith("City of"):
                        settlement_type = "city"
                    
                    settlements.append({
                        "name": settlement,
                        "type": settlement_type,
                        "hex_coordinate": entity.get("coordinate", "unknown")
                    })
        
        return settlements
    
    def _analyze_geographic_features(self, cluster) -> dict[str, int]:
        """Analyze geographic features like rivers, trails, harbors."""
        
        features = {"rivers": 0, "trails": 0, "harbors": 0, "borders": 0}
        
        for entity in cluster.entities:
            if entity.get("rivers"):
                features["rivers"] += 1
            if entity.get("trails"):
                features["trails"] += 1
            if entity.get("harbor"):
                features["harbors"] += 1
            if entity.get("borderline"):
                features["borders"] += 1
        
        return features
    
    def _calculate_corruption_level(self, region_name: str) -> int:
        """Calculate corruption level based on region name and characteristics."""
        
        name_lower = region_name.lower()
        
        # High corruption themes
        if any(word in name_lower for word in ["dark", "blood", "bone", "nightmare", "hell", "vicious"]):
            return 4
        
        # Medium-high corruption themes  
        if any(word in name_lower for word in ["fallen", "grey", "shadow", "fear", "skull"]):
            return 3
        
        # Medium corruption themes
        if any(word in name_lower for word in ["black", "death", "cursed", "haunted"]):
            return 2
        
        # Low corruption themes
        if any(word in name_lower for word in ["heart", "moon", "gold", "thunder"]):
            return 1
        
        # Neutral/positive themes
        if any(word in name_lower for word in ["golden", "silver", "bright", "light", "aurora"]):
            return 0
        
        # Default medium-low corruption
        return 1
    
    def _calculate_connectivity_score(self, geographic_features: dict[str, int]) -> float:
        """Calculate connectivity score based on geographic features."""
        
        score = 0.0
        
        # Use connectivity weights from configuration
        for feature, weight in self.connectivity_weights.items():
            score += geographic_features.get(feature, 0) * weight
        
        return max(0.0, min(1.0, score))


def process_region_cluster(cluster, logger, console) -> dict[str, Any]:
    """
    Process region entity cluster using RegionsProcessor.
    
    Args:
        cluster: EntityCluster containing region entities from transformer
        logger: Logger instance from orchestrator
        console: Rich console from orchestrator
        
    Returns:
        Processed region data with world_hooks for Godot integration
    """
    
    processor = RegionsProcessor()
    return processor.process_cluster(cluster, logger, console)
