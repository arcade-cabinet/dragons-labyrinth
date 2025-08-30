"""
FactionsProcessor - Specialized processor for faction entity clusters.

Processes faction clusters from the transformer, extracting organizational
structure, territorial reach, member analysis, and political alignment
with world_hooks for Godot integration.
"""

from __future__ import annotations

import re
from typing import Any

from generator.constants import (
    SETTLEMENTS, REGIONS, FACTIONS, FACTION_TYPE_PATTERNS, 
    HOSTILITY_PATTERNS, ALIGNMENT_PATTERNS, THREAT_WEIGHTS
)
from generator.entities.processors.base import BaseProcessor


class FactionsProcessor(BaseProcessor):
    """
    Specialized processor for faction entity clusters.
    
    Extracts:
    - Organizational structure and hierarchy
    - Territorial reach and operating regions
    - Member composition and capabilities
    - Political alignment and hostility assessment
    - Faction resources and influence level
    """
    
    def __init__(self):
        super().__init__("factions")
        
        # Load faction-specific configuration from constants
        self.known_settlements = SETTLEMENTS
        self.known_regions = REGIONS
        self.known_factions = FACTIONS
        self.faction_type_patterns = FACTION_TYPE_PATTERNS
        self.hostility_patterns = HOSTILITY_PATTERNS
        self.alignment_patterns = ALIGNMENT_PATTERNS
        self.threat_weights = THREAT_WEIGHTS
    
    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger, console) -> dict[str, Any]:
        """Extract faction-specific data from cluster entities and ML results."""
        
        entities = ml_results.get("entities", [])
        
        # Analyze faction characteristics
        operating_regions = self._extract_operating_regions(cluster)
        member_analysis = self._analyze_members(cluster, entities)
        hostility_assessment = self._assess_hostility(cluster.name, entities)
        territorial_reach = self._calculate_territorial_reach(operating_regions)
        influence_level = self._assess_influence_level(cluster, entities)
        
        # Use ML results for enhanced analysis
        ml_confidence = self._calculate_ml_confidence(entities)
        relationships = ml_results.get("relationships", [])
        
        logger.info(f"Faction analysis - Members: {member_analysis['member_count']}, Regions: {len(operating_regions)}, Hostility: {hostility_assessment}")
        
        return {
            "name": cluster.name,
            "faction_type": self._determine_faction_type(cluster.name),
            "operating_regions": operating_regions,
            "member_analysis": member_analysis,
            "hostility_level": hostility_assessment,
            "territorial_reach": territorial_reach,
            "influence_level": influence_level,
            "political_alignment": self._determine_political_alignment(cluster.name),
            "resource_level": self._assess_resource_level(cluster, entities),
            "ml_confidence": ml_confidence,
            "entity_relationships": len(relationships),
            "anomaly_count": ml_results.get("anomaly_count", 0)
        }
    
    def _generate_world_hooks(self, cluster, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate faction-specific world_hooks for Godot integration."""
        
        return {
            "faction_name": cluster.name,
            "faction_type": specific_data.get("faction_type", "unknown"),
            "operating_regions": specific_data.get("operating_regions", []),
            "member_count_estimate": specific_data.get("member_analysis", {}).get("member_count", 0),
            "hostility_level": specific_data.get("hostility_level", "neutral"),
            "territorial_reach": specific_data.get("territorial_reach", "local"),
            "political_alignment": specific_data.get("political_alignment", "neutral"),
            "influence_level": specific_data.get("influence_level", 1),
            "has_settlements": len(specific_data.get("operating_regions", [])) > 0,
            "is_hostile": specific_data.get("hostility_level") in ["hostile", "aggressive"],
            "is_lawful": specific_data.get("political_alignment") == "lawful",
            "godot_integration": {
                "faction_banner": f"res://art/factions/{cluster.name.lower().replace(' ', '_').replace('the_', '')}.png",
                "member_spawn_rate": min(1.0, specific_data.get("member_analysis", {}).get("member_count", 0) / 50.0),
                "territory_control": len(specific_data.get("operating_regions", [])),
                "threat_level": self._calculate_threat_level(specific_data),
                "encounter_frequency": self._calculate_encounter_frequency(specific_data),
                "reputation_modifier": self._calculate_reputation_modifier(specific_data.get("political_alignment", "neutral"))
            }
        }
    
    def _determine_faction_type(self, faction_name: str) -> str:
        """Determine faction type from name characteristics."""
        
        name_lower = faction_name.lower()
        
        if any(word in name_lower for word in ["defiled", "corrupted", "dark", "shadow"]):
            return "cult"
        elif any(word in name_lower for word in ["justice", "fists", "swords", "order"]):
            return "militia"
        elif any(word in name_lower for word in ["snakes", "wolves", "wyverns", "gang"]):
            return "syndicate"
        elif any(word in name_lower for word in ["guild", "company", "merchants"]):
            return "guild"
        else:
            return "organization"
    
    def _extract_operating_regions(self, cluster) -> list[str]:
        """Extract regions where the faction operates."""
        
        operating_regions = []
        
        for entity in cluster.entities:
            content = str(entity)
            
            # Check for settlement operations
            for settlement in self.known_settlements:
                if settlement in content:
                    operating_regions.append(settlement)
            
            # Check for regional operations
            for region in self.known_regions:
                if region in content:
                    operating_regions.append(region)
        
        return list(set(operating_regions))  # Remove duplicates
    
    def _analyze_members(self, cluster, ml_entities: list[dict[str, Any]]) -> dict[str, Any]:
        """Analyze faction member composition."""
        
        member_count = 0
        member_types = []
        leadership_count = 0
        combat_capable = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            extracted_data = entity.get("extracted_data", {})
            
            # Count members (entities with stats or class mentions)
            if ml_features.get("has_stat_blocks", False):
                member_count += 1
                combat_capable += 1
            elif ml_features.get("class_mentions", 0) > 0:
                member_count += 1
                combat_capable += 1
            elif ml_features.get("title_mentions", 0) > 0:
                member_count += 1
                # Leaders are those with titles
                if any(title in str(extracted_data).lower() for title in ["leader", "captain", "boss", "chief"]):
                    leadership_count += 1
        
        # Analyze member roles from cluster content
        for entity in cluster.entities:
            content = str(entity).lower()
            if any(role in content for role in ["fighter", "cleric", "wizard", "rogue"]):
                if content not in member_types:
                    member_types.append("combat_class")
            if any(role in content for role in ["merchant", "trader", "smuggler"]):
                if "economic" not in member_types:
                    member_types.append("economic")
            if any(role in content for role in ["spy", "assassin", "thief"]):
                if "covert" not in member_types:
                    member_types.append("covert")
        
        return {
            "member_count": member_count,
            "member_types": member_types,
            "leadership_count": leadership_count,
            "combat_capable": combat_capable,
            "combat_ratio": combat_capable / max(member_count, 1),
            "leadership_ratio": leadership_count / max(member_count, 1)
        }
    
    def _assess_hostility(self, faction_name: str, ml_entities: list[dict[str, Any]]) -> str:
        """Assess faction hostility level."""
        
        name_lower = faction_name.lower()
        
        # Check name for hostility indicators
        if any(word in name_lower for word in ["defiled", "corrupted", "dark", "evil"]):
            return "hostile"
        elif any(word in name_lower for word in ["justice", "protection", "guard"]):
            return "lawful"
        elif any(word in name_lower for word in ["snakes", "wolves", "raiders"]):
            return "aggressive"
        
        # Check entity content for hostility indicators
        hostile_indicators = 0
        lawful_indicators = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            
            hostile_indicators += ml_features.get("violence_indicators", 0)
            hostile_indicators += ml_features.get("corruption_words", 0)
            
            # Check for lawful indicators (would need to be added to ML features)
            if ml_features.get("organization_words", 0) > ml_features.get("conflict_words", 0):
                lawful_indicators += 1
        
        if hostile_indicators > lawful_indicators * 2:
            return "hostile"
        elif lawful_indicators > hostile_indicators:
            return "lawful"
        else:
            return "neutral"
    
    def _calculate_territorial_reach(self, operating_regions: list[str]) -> str:
        """Calculate territorial reach based on operating regions."""
        
        region_count = len(operating_regions)
        
        if region_count >= 5:
            return "widespread"
        elif region_count >= 3:
            return "regional"
        elif region_count >= 1:
            return "local"
        else:
            return "unknown"
    
    def _assess_influence_level(self, cluster, ml_entities: list[dict[str, Any]]) -> int:
        """Assess faction influence level (1-5 scale)."""
        
        influence = 1  # Base influence
        
        # Member count contributes to influence
        member_count = len(ml_entities)
        if member_count >= 20:
            influence += 2
        elif member_count >= 10:
            influence += 1
        
        # Territorial reach contributes
        operating_regions = len(self._extract_operating_regions(cluster))
        if operating_regions >= 3:
            influence += 1
        
        # Economic indicators contribute
        economic_score = 0
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            economic_score += ml_features.get("currency_mentions", 0)
            economic_score += ml_features.get("trade_indicators", 0)
        
        if economic_score >= 10:
            influence += 1
        
        return min(influence, 5)  # Cap at 5
    
    def _determine_political_alignment(self, faction_name: str) -> str:
        """Determine political alignment from faction name."""
        
        name_lower = faction_name.lower()
        
        if any(word in name_lower for word in ["justice", "order", "protection", "guard"]):
            return "lawful"
        elif any(word in name_lower for word in ["defiled", "corrupted", "chaos", "shadow"]):
            return "chaotic"
        elif any(word in name_lower for word in ["neutral", "balance", "trade"]):
            return "neutral"
        else:
            return "neutral"  # Default
    
    def _assess_resource_level(self, cluster, ml_entities: list[dict[str, Any]]) -> int:
        """Assess faction resource level (1-5 scale)."""
        
        resources = 1  # Base resources
        
        # Economic indicators
        total_economic = 0
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            total_economic += ml_features.get("currency_mentions", 0)
            total_economic += ml_features.get("trade_indicators", 0)
        
        if total_economic >= 15:
            resources += 2
        elif total_economic >= 5:
            resources += 1
        
        # Equipment/infrastructure indicators
        for entity in cluster.entities:
            content = str(entity).lower()
            if any(word in content for word in ["stronghold", "base", "headquarters"]):
                resources += 1
                break
            if any(word in content for word in ["weapons", "armor", "equipment"]):
                resources += 1
                break
        
        return min(resources, 5)  # Cap at 5
    
    def _calculate_threat_level(self, specific_data: dict[str, Any]) -> float:
        """Calculate threat level for Godot integration (0.0-1.0)."""
        
        threat = 0.0
        
        # Hostility contributes to threat
        hostility = specific_data.get("hostility_level", "neutral")
        if hostility in self.threat_weights:
            threat += self.threat_weights[hostility]
        
        # Member count contributes
        member_count = specific_data.get("member_analysis", {}).get("member_count", 0)
        threat += min(0.3, member_count * self.threat_weights["member_factor"])
        
        # Territorial reach contributes
        reach = specific_data.get("territorial_reach", "local")
        if reach == "widespread":
            threat += self.threat_weights["territorial_widespread"]
        elif reach == "regional":
            threat += self.threat_weights["territorial_regional"]
        
        # Influence level contributes
        influence = specific_data.get("influence_level", 1)
        threat += influence / 10.0
        
        return max(0.0, min(1.0, threat))
    
    def _calculate_encounter_frequency(self, specific_data: dict[str, Any]) -> float:
        """Calculate encounter frequency for Godot integration (0.0-1.0)."""
        
        frequency = 0.1  # Base frequency
        
        # Territorial reach affects frequency
        reach = specific_data.get("territorial_reach", "local")
        if reach == "widespread":
            frequency += 0.4
        elif reach == "regional":
            frequency += 0.2
        elif reach == "local":
            frequency += 0.1
        
        # Member count affects frequency
        member_count = specific_data.get("member_analysis", {}).get("member_count", 0)
        frequency += min(0.3, member_count / 50.0)
        
        # Hostility affects frequency
        hostility = specific_data.get("hostility_level", "neutral")
        if hostility in ["hostile", "aggressive"]:
            frequency += 0.2
        
        return min(1.0, frequency)
    
    def _calculate_reputation_modifier(self, political_alignment: str) -> float:
        """Calculate reputation modifier for Godot integration (-1.0 to 1.0)."""
        
        if political_alignment == "lawful":
            return 0.5  # Positive reputation boost
        elif political_alignment == "chaotic":
            return -0.5  # Negative reputation penalty
        else:
            return 0.0  # Neutral


def process_faction_cluster(cluster, logger, console) -> dict[str, Any]:
    """
    Process faction entity cluster using FactionsProcessor.
    
    Args:
        cluster: EntityCluster containing faction entities from transformer
        logger: Logger instance from orchestrator
        console: Rich console from orchestrator
        
    Returns:
        Processed faction data with world_hooks for Godot integration
    """
    
    processor = FactionsProcessor()
    return processor.process_cluster(cluster, logger, console)
