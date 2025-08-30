"""
DungeonsProcessor - Specialized processor for dungeon entity clusters.

Processes dungeon clusters from the transformer, extracting structural
analysis, threat assessment, treasure evaluation, and entrance characteristics
with world_hooks for Godot integration.
"""

from __future__ import annotations

import re
from typing import Any

from generator.constants import (
    DUNGEONS, DUNGEON_TYPE_PATTERNS, ENTRANCE_MAPPINGS, 
    THREAT_THRESHOLDS, TREASURE_THRESHOLDS, COMPLEXITY_THRESHOLDS
)
from generator.entities.processors.base import BaseProcessor


class DungeonsProcessor(BaseProcessor):
    """
    Specialized processor for dungeon entity clusters.
    
    Extracts:
    - Dungeon type and structural characteristics
    - Threat level and enemy analysis
    - Treasure and reward assessment
    - Entrance type and accessibility
    - Depth estimation and complexity level
    """
    
    def __init__(self):
        super().__init__("dungeons")
        
        # Load dungeon-specific configuration from constants
        self.known_dungeons = DUNGEONS
        self.dungeon_type_patterns = DUNGEON_TYPE_PATTERNS
        self.entrance_mappings = ENTRANCE_MAPPINGS
        self.threat_thresholds = THREAT_THRESHOLDS
        self.treasure_thresholds = TREASURE_THRESHOLDS
        self.complexity_thresholds = COMPLEXITY_THRESHOLDS
        
        # Configure corruption influence patterns
        self.corruption_patterns = {
            3: ["corrupted", "defiled", "cursed", "infernal"],
            2: ["dark", "shadow", "unholy", "violent"],
            1: ["burning", "raging", "mourning"],
            0: ["blessed", "holy", "pure", "light"]
        }
    
    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger, console) -> dict[str, Any]:
        """Extract dungeon-specific data from cluster entities and ML results."""
        
        entities = ml_results.get("entities", [])
        
        # Analyze dungeon characteristics
        dungeon_type = self._determine_dungeon_type(cluster.name)
        entrance_analysis = self._analyze_entrance(cluster.name, entities)
        threat_assessment = self._assess_threat_level(cluster, entities)
        treasure_analysis = self._analyze_treasure(cluster, entities)
        structural_analysis = self._analyze_structure(cluster, entities)
        
        # Use ML results for enhanced analysis
        ml_confidence = self._calculate_ml_confidence(entities)
        relationships = ml_results.get("relationships", [])
        
        logger.info(f"Dungeon analysis - Type: {dungeon_type}, Threat: {threat_assessment['threat_level']}, Rooms: {structural_analysis['estimated_rooms']}")
        
        return {
            "name": cluster.name,
            "dungeon_type": dungeon_type,
            "entrance_analysis": entrance_analysis,
            "threat_assessment": threat_assessment,
            "treasure_analysis": treasure_analysis,
            "structural_analysis": structural_analysis,
            "accessibility": self._assess_accessibility(cluster, entities),
            "corruption_influence": self._assess_corruption_influence(cluster.name),
            "exploration_difficulty": self._calculate_exploration_difficulty(threat_assessment, structural_analysis),
            "ml_confidence": ml_confidence,
            "entity_relationships": len(relationships),
            "anomaly_count": ml_results.get("anomaly_count", 0)
        }
    
    def _generate_world_hooks(self, cluster, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate dungeon-specific world_hooks for Godot integration."""
        
        return {
            "dungeon_name": cluster.name,
            "dungeon_type": specific_data.get("dungeon_type", "unknown"),
            "entrance_type": specific_data.get("entrance_analysis", {}).get("entrance_type", "unknown"),
            "threat_level": specific_data.get("threat_assessment", {}).get("threat_level", 1),
            "estimated_rooms": specific_data.get("structural_analysis", {}).get("estimated_rooms", 1),
            "has_treasure": specific_data.get("treasure_analysis", {}).get("has_major_treasure", False),
            "accessibility": specific_data.get("accessibility", "unknown"),
            "corruption_influence": specific_data.get("corruption_influence", 0),
            "exploration_difficulty": specific_data.get("exploration_difficulty", "moderate"),
            "has_boss": specific_data.get("threat_assessment", {}).get("has_boss_encounter", False),
            "is_multi_level": specific_data.get("structural_analysis", {}).get("is_multi_level", False),
            "godot_integration": {
                "dungeon_tileset": f"res://art/dungeons/{specific_data.get('dungeon_type', 'generic')}_tiles.png",
                "entrance_sprite": f"res://art/entrances/{specific_data.get('entrance_analysis', {}).get('entrance_type', 'cave')}.png",
                "room_generation_seed": self._generate_room_seed(cluster.name),
                "enemy_spawn_rate": self._calculate_spawn_rate(specific_data.get("threat_assessment", {})),
                "treasure_spawn_rate": self._calculate_treasure_spawn_rate(specific_data.get("treasure_analysis", {})),
                "ambient_corruption": specific_data.get("corruption_influence", 0) / 5.0,
                "exploration_rewards": self._calculate_exploration_rewards(specific_data)
            }
        }
    
    def _determine_dungeon_type(self, dungeon_name: str) -> str:
        """Determine dungeon type from name."""
        
        name_lower = dungeon_name.lower()
        
        if any(word in name_lower for word in ["crypt", "tomb", "mausoleum"]):
            return "crypt"
        elif any(word in name_lower for word in ["cave", "cavern", "grotto"]):
            return "cave"
        elif any(word in name_lower for word in ["temple", "shrine", "sanctuary"]):
            return "temple"
        elif any(word in name_lower for word in ["lair", "den", "nest"]):
            return "lair"
        elif any(word in name_lower for word in ["hideout", "base", "stronghold"]):
            return "hideout"
        elif any(word in name_lower for word in ["bowel", "pit", "abyss"]):
            return "pit"
        else:
            return "dungeon"
    
    def _analyze_entrance(self, dungeon_name: str, ml_entities: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze dungeon entrance characteristics."""
        
        name_lower = dungeon_name.lower()
        
        # Determine entrance type from name
        if any(word in name_lower for word in ["crypt", "tomb"]):
            entrance_type = "tomb_entrance"
        elif any(word in name_lower for word in ["cave", "cavern"]):
            entrance_type = "cave_mouth"
        elif any(word in name_lower for word in ["temple", "shrine"]):
            entrance_type = "temple_entrance"
        elif any(word in name_lower for word in ["lair"]):
            entrance_type = "lair_entrance"
        elif any(word in name_lower for word in ["pit", "bowel"]):
            entrance_type = "pit_entrance"
        else:
            entrance_type = "generic_entrance"
        
        # Analyze entrance features from entities
        is_hidden = False
        is_guarded = False
        requires_key = False
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            extracted_data = entity.get("extracted_data", {})
            
            if ml_features.get("trap_mentions", 0) > 0:
                is_hidden = True
            if ml_features.get("has_stat_blocks", False):
                is_guarded = True
            
            content = str(extracted_data).lower()
            if any(word in content for word in ["key", "lock", "sealed"]):
                requires_key = True
        
        return {
            "entrance_type": entrance_type,
            "is_hidden": is_hidden,
            "is_guarded": is_guarded,
            "requires_key": requires_key,
            "accessibility_rating": self._calculate_entrance_accessibility(is_hidden, is_guarded, requires_key)
        }
    
    def _assess_threat_level(self, cluster, ml_entities: list[dict[str, Any]]) -> dict[str, Any]:
        """Assess dungeon threat level and enemy composition."""
        
        enemy_count = 0
        boss_encounters = 0
        trap_count = 0
        undead_count = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            extracted_data = entity.get("extracted_data", {})
            
            # Count enemies (entities with stats)
            if ml_features.get("has_stat_blocks", False):
                enemy_count += 1
                
                # Check for boss indicators
                content = str(extracted_data).lower()
                if any(word in content for word in ["boss", "guardian", "ancient", "powerful", "champion"]):
                    boss_encounters += 1
                if any(word in content for word in ["undead", "skeleton", "zombie", "ghost", "wraith"]):
                    undead_count += 1
            
            # Count traps
            trap_count += ml_features.get("trap_mentions", 0)
        
        # Calculate threat level (1-5 scale)
        threat_level = 1
        if enemy_count >= self.threat_thresholds["enemy_high"]:
            threat_level += 2
        elif enemy_count >= self.threat_thresholds["enemy_moderate"]:
            threat_level += 1
        
        if boss_encounters > 0:
            threat_level += 1
        
        if trap_count >= self.threat_thresholds["trap_high"]:
            threat_level += 1
        
        threat_level = min(threat_level, self.threat_thresholds["max_threat"])
        
        return {
            "threat_level": threat_level,
            "enemy_count": enemy_count,
            "has_boss_encounter": boss_encounters > 0,
            "trap_count": trap_count,
            "undead_presence": undead_count > 0,
            "undead_ratio": undead_count / max(enemy_count, 1),
            "trap_density": trap_count / max(len(cluster.entities), 1)
        }
    
    def _analyze_treasure(self, cluster, ml_entities: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze treasure and reward potential."""
        
        treasure_mentions = 0
        gold_mentions = 0
        magic_item_mentions = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            
            treasure_mentions += ml_features.get("treasure_mentions", 0)
            gold_mentions += ml_features.get("currency_mentions", 0)
        
        # Check for magic item indicators
        for entity in cluster.entities:
            content = str(entity).lower()
            if any(word in content for word in ["magic", "enchanted", "artifact", "relic"]):
                magic_item_mentions += 1
        
        # Assess treasure level
        treasure_level = "low"
        if treasure_mentions >= self.treasure_thresholds["mentions_high"] or gold_mentions >= self.treasure_thresholds["currency_high"]:
            treasure_level = "high"
        elif treasure_mentions >= self.treasure_thresholds["mentions_moderate"] or gold_mentions >= self.treasure_thresholds["currency_moderate"]:
            treasure_level = "moderate"
        
        return {
            "treasure_level": treasure_level,
            "has_major_treasure": treasure_mentions >= 3,
            "has_currency": gold_mentions > 0,
            "has_magic_items": magic_item_mentions > 0,
            "treasure_density": treasure_mentions / max(len(cluster.entities), 1),
            "reward_variety": len([x for x in [treasure_mentions > 0, gold_mentions > 0, magic_item_mentions > 0] if x])
        }
    
    def _analyze_structure(self, cluster, ml_entities: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze dungeon structural characteristics."""
        
        # Estimate room count from entity count and content
        entity_count = len(cluster.entities)
        
        # Basic room estimation (entities per room varies by dungeon type)
        estimated_rooms = max(1, entity_count // 3)  # Rough estimate
        
        # Check for multi-level indicators
        is_multi_level = False
        for entity in cluster.entities:
            content = str(entity).lower()
            if any(word in content for word in ["level", "floor", "deep", "below", "above"]):
                is_multi_level = True
                break
        
        # Analyze complexity indicators
        complexity_score = 0
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            complexity_score += ml_features.get("trap_mentions", 0)
            complexity_score += ml_features.get("treasure_mentions", 0)
        
        # Determine complexity level
        if complexity_score >= self.complexity_thresholds["very_high"]:
            complexity_level = "very_high"
        elif complexity_score >= self.complexity_thresholds["high"]:
            complexity_level = "high"
        elif complexity_score >= self.complexity_thresholds["moderate"]:
            complexity_level = "moderate"
        else:
            complexity_level = "simple"
        
        return {
            "estimated_rooms": estimated_rooms,
            "is_multi_level": is_multi_level,
            "complexity_level": complexity_level,
            "complexity_score": complexity_score,
            "layout_density": entity_count / max(estimated_rooms, 1)
        }
    
    def _assess_accessibility(self, cluster, ml_entities: list[dict[str, Any]]) -> str:
        """Assess dungeon accessibility level."""
        
        # Check for accessibility barriers
        barrier_count = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            
            # Barriers: traps, locks, guards
            barrier_count += ml_features.get("trap_mentions", 0)
            if ml_features.get("has_stat_blocks", False):
                barrier_count += 1  # Guards count as barriers
        
        # Check name for accessibility hints
        name_lower = cluster.name.lower()
        if any(word in name_lower for word in ["hidden", "secret", "lost"]):
            barrier_count += 2
        
        # Classify accessibility
        if barrier_count >= 8:
            return "very_difficult"
        elif barrier_count >= 5:
            return "difficult"
        elif barrier_count >= 2:
            return "moderate"
        else:
            return "easy"
    
    def _assess_corruption_influence(self, dungeon_name: str) -> int:
        """Assess corruption influence level (0-5 scale)."""
        
        name_lower = dungeon_name.lower()
        
        corruption_level = 0
        
        # High corruption themes
        if any(word in name_lower for word in ["corrupted", "defiled", "cursed", "infernal"]):
            corruption_level += 3
        
        # Medium corruption themes
        if any(word in name_lower for word in ["dark", "shadow", "unholy", "violent"]):
            corruption_level += 2
        
        # Moderate corruption themes
        if any(word in name_lower for word in ["burning", "raging", "mourning"]):
            corruption_level += 1
        
        # Dungeon type influences corruption
        if any(word in name_lower for word in ["crypt", "tomb"]):
            corruption_level += 1  # Undead presence
        
        return min(corruption_level, 5)
    
    def _calculate_exploration_difficulty(self, threat_assessment: dict[str, Any], structural_analysis: dict[str, Any]) -> str:
        """Calculate overall exploration difficulty."""
        
        difficulty_score = 0
        
        # Threat contributes to difficulty
        difficulty_score += threat_assessment.get("threat_level", 1)
        
        # Structure complexity contributes
        complexity = structural_analysis.get("complexity_level", "simple")
        if complexity == "very_high":
            difficulty_score += 3
        elif complexity == "high":
            difficulty_score += 2
        elif complexity == "moderate":
            difficulty_score += 1
        
        # Multi-level dungeons are harder
        if structural_analysis.get("is_multi_level", False):
            difficulty_score += 1
        
        # Classify difficulty
        if difficulty_score >= 8:
            return "extreme"
        elif difficulty_score >= 6:
            return "hard"
        elif difficulty_score >= 4:
            return "moderate"
        elif difficulty_score >= 2:
            return "easy"
        else:
            return "trivial"
    
    def _calculate_entrance_accessibility(self, is_hidden: bool, is_guarded: bool, requires_key: bool) -> float:
        """Calculate entrance accessibility rating (0.0-1.0, higher = more accessible)."""
        
        accessibility = 1.0
        
        if is_hidden:
            accessibility -= 0.4
        if is_guarded:
            accessibility -= 0.3
        if requires_key:
            accessibility -= 0.3
        
        return max(0.0, accessibility)
    
    def _generate_room_seed(self, dungeon_name: str) -> int:
        """Generate a consistent seed for room generation based on dungeon name."""
        
        # Simple hash of dungeon name for consistent generation
        return hash(dungeon_name) % 1000000
    
    def _calculate_spawn_rate(self, threat_assessment: dict[str, Any]) -> float:
        """Calculate enemy spawn rate for Godot integration (0.0-1.0)."""
        
        threat_level = threat_assessment.get("threat_level", 1)
        enemy_count = threat_assessment.get("enemy_count", 0)
        
        base_rate = 0.1
        threat_multiplier = threat_level * 0.15
        count_multiplier = min(0.3, enemy_count / 20.0)
        
        return min(1.0, base_rate + threat_multiplier + count_multiplier)
    
    def _calculate_treasure_spawn_rate(self, treasure_analysis: dict[str, Any]) -> float:
        """Calculate treasure spawn rate for Godot integration (0.0-1.0)."""
        
        treasure_level = treasure_analysis.get("treasure_level", "low")
        treasure_density = treasure_analysis.get("treasure_density", 0.0)
        
        base_rate = 0.05
        
        if treasure_level == "high":
            base_rate += 0.3
        elif treasure_level == "moderate":
            base_rate += 0.15
        
        density_bonus = min(0.2, treasure_density)
        
        return min(1.0, base_rate + density_bonus)
    
    def _calculate_exploration_rewards(self, specific_data: dict[str, Any]) -> dict[str, float]:
        """Calculate exploration reward multipliers for Godot integration."""
        
        threat_level = specific_data.get("threat_assessment", {}).get("threat_level", 1)
        treasure_level = specific_data.get("treasure_analysis", {}).get("treasure_level", "low")
        complexity = specific_data.get("structural_analysis", {}).get("complexity_level", "simple")
        
        # Base rewards
        experience_multiplier = 1.0 + (threat_level - 1) * 0.2
        loot_multiplier = 1.0
        
        # Treasure level affects loot
        if treasure_level == "high":
            loot_multiplier += 0.5
        elif treasure_level == "moderate":
            loot_multiplier += 0.25
        
        # Complexity affects both
        complexity_bonus = 0.0
        if complexity == "very_high":
            complexity_bonus = 0.3
        elif complexity == "high":
            complexity_bonus = 0.2
        elif complexity == "moderate":
            complexity_bonus = 0.1
        
        experience_multiplier += complexity_bonus
        loot_multiplier += complexity_bonus
        
        return {
            "experience_multiplier": min(3.0, experience_multiplier),
            "loot_multiplier": min(2.5, loot_multiplier),
            "reputation_bonus": complexity_bonus
        }


def process_dungeon_cluster(cluster, logger, console) -> dict[str, Any]:
    """
    Process dungeon entity cluster using DungeonsProcessor.
    
    Args:
        cluster: EntityCluster containing dungeon entities from transformer
        logger: Logger instance from orchestrator
        console: Rich console from orchestrator
        
    Returns:
        Processed dungeon data with world_hooks for Godot integration
    """
    
    processor = DungeonsProcessor()
    return processor.process_cluster(cluster, logger, console)
