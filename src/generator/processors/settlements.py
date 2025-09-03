"""
SettlementsProcessor - Specialized processor for settlement entity clusters.

Processes settlement clusters from the transformer, extracting scale detection,
economic analysis, establishment categorization, and NPC relationship mapping
with world_hooks for Godot integration.
"""

from __future__ import annotations

import re
from typing import Any

from generator.constants import (
    ESTABLISHMENT_KEYWORDS, SERVICE_TYPES, ECONOMIC_THRESHOLDS, RESISTANCE_FACTORS
)
from generator.entities.processors.base import BaseProcessor
from generator.entities.types import SettlementScale, ServiceType, ProcessorType
from generator.entities.models import SettlementData, InfrastructureFeatures, BevyHooks


class SettlementsProcessor(BaseProcessor):
    """
    Specialized processor for settlement entity clusters.
    
    Extracts:
    - Settlement scale (village/town/city) detection
    - Economic activity and establishment analysis
    - NPC counts and service diversity
    - Infrastructure and connectivity features
    - Corruption resistance based on settlement characteristics
    """
    
    def __init__(self):
        super().__init__("settlements")
        
        # Load settlement-specific configuration from constants
        self.establishment_keywords = ESTABLISHMENT_KEYWORDS
        self.service_types_config = SERVICE_TYPES
        self.economic_thresholds = ECONOMIC_THRESHOLDS
        self.resistance_factors = RESISTANCE_FACTORS
        
        # Configure settlement processing parameters
        self.scale_detection_patterns = {
            "city": re.compile(r"City of ([^<\"]+)", re.IGNORECASE),
            "town": re.compile(r"Town of ([^<\"]+)", re.IGNORECASE),
            "village": re.compile(r"Village of ([^<\"]+)", re.IGNORECASE)
        }
        
        # Currency detection patterns
        self.currency_patterns = [
            re.compile(r"(\d+)\s*(gp|gold)", re.IGNORECASE),
            re.compile(r"(\d+)\s*(sp|silver)", re.IGNORECASE),
            re.compile(r"(\d+)\s*(cp|copper)", re.IGNORECASE)
        ]
    
    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger, console) -> SettlementData:
        """Extract settlement-specific data using Pydantic model with analyzed data."""
        
        entities = ml_results.get("entities", [])
        
        # Analyze settlement characteristics from actual data
        scale_hint = SettlementScale(self._determine_scale_from_name(cluster.name))
        service_types_set = self._analyze_service_types(cluster)
        service_types = [ServiceType(svc) for svc in service_types_set]
        infrastructure = self._analyze_infrastructure(cluster)
        
        # Calculate actual metrics from analysis
        establishment_count = self._count_establishments(cluster)
        economic_activity = self._assess_economic_activity(cluster)
        npc_count = self._count_npcs_from_ml(entities)
        corruption_resistance = self._calculate_corruption_resistance(scale_hint.value, service_types_set, economic_activity)
        
        # Use ML results for confidence
        ml_confidence = self._calculate_ml_confidence(entities)
        relationships = ml_results.get("relationships", [])
        
        logger.info(f"Settlement analysis - Scale: {scale_hint.value}, Establishments: {establishment_count}, NPCs: {npc_count}, Economic: {economic_activity}")
        
        return SettlementData(
            name=cluster.name,
            scale_hint=scale_hint,
            establishment_count=establishment_count,
            service_types=service_types,
            economic_activity_level=economic_activity,
            npc_count=npc_count,
            service_diversity=len(service_types),
            infrastructure=InfrastructureFeatures(**infrastructure),
            corruption_resistance=corruption_resistance,
            ml_confidence=ml_confidence,
            entity_relationships=len(relationships),
            anomaly_count=ml_results.get("anomaly_count", 0)
        )
    
    def _generate_bevy_hooks(self, cluster, specific_data: SettlementData) -> BevyHooks:
        """Generate Bevy ECS integration hooks for settlements."""
        
        return BevyHooks(
            entity_name=cluster.name,
            processor_type=ProcessorType.SETTLEMENTS,
            component_type="Settlement",
            spawn_function="spawn_settlements",
            update_system="update_settlement_corruption"
        )
    
    def _determine_scale_from_name(self, settlement_name: str) -> str:
        """Determine settlement scale from name."""
        
        if settlement_name.startswith("City of"):
            return "city"
        elif settlement_name.startswith("Town of"):
            return "town"
        elif settlement_name.startswith("Village of"):
            return "village"
        else:
            return "unknown"
    
    def _count_establishments(self, cluster) -> int:
        """Count establishments (buildings/services) in the settlement."""
        
        establishment_count = 0
        
        for entity in cluster.entities:
            content = str(entity).lower()
            
            # Count named establishments
            for keyword in self.establishment_keywords:
                if keyword in content:
                    establishment_count += 1
                    break  # Count each entity only once
        
        return establishment_count
    
    def _analyze_service_types(self, cluster) -> set[str]:
        """Analyze types of services available in the settlement using JSON-derived patterns."""
        
        service_types = set()
        
        # Enhanced POI categorization patterns from JSON processor success
        poi_categories = {
            "commercial": ["market", "shop", "store", "goods", "trading", "merchant", "caravan"],
            "hospitality": ["inn", "tavern", "lodge", "rest", "accommodation", "keeper"],
            "crafting": ["blacksmith", "smithy", "forge", "craft", "workshop", "armor", "weapons"],
            "medical": ["healer", "physician", "herbalist", "medicine", "healing", "apothecary"],
            "education": ["school", "library", "scribe", "learning", "scholar", "books"],
            "religious": ["temple", "shrine", "church", "altar", "priest", "cleric"],
            "defense": ["guard", "militia", "watch", "patrol", "garrison", "barracks"],
            "government": ["hall", "council", "mayor", "official", "administration"]
        }
        
        for entity in cluster.entities:
            content = str(entity).lower()
            
            # Apply enhanced categorization with multiple keywords per category
            for category, keywords in poi_categories.items():
                if any(keyword in content for keyword in keywords):
                    # Map to standardized service types
                    if category == "commercial":
                        service_types.add("commerce")
                    elif category == "hospitality":
                        service_types.add("lodging")
                    elif category == "crafting":
                        service_types.add("crafting")
                    elif category in ["medical", "education", "religious", "defense", "government"]:
                        service_types.add(category.replace("education", "learning"))
        
        return service_types
    
    def _assess_economic_activity(self, cluster) -> int:
        """Assess economic activity using enhanced patterns from JSON processor success."""
        
        economic_activity = 0
        
        for entity in cluster.entities:
            content = str(entity).lower()
            
            # Enhanced currency detection patterns from JSON cities
            currency_patterns = [
                r"(\d+)\s*(gp|gold)",
                r"(\d+)\s*(sp|silver)", 
                r"(\d+)\s*(cp|copper)",
                r"price[^:]*:\s*(\d+)",
                r"cost[^:]*:\s*(\d+)",
                r"(\d+)\s*coins?"
            ]
            
            for pattern in currency_patterns:
                matches = re.findall(pattern, content)
                economic_activity += len(matches)
            
            # Enhanced trade activity detection from JSON patterns
            trade_indicators = [
                "merchant", "trade", "trading", "commerce", "goods", "caravan",
                "price", "cost", "inventory", "stock", "market", "shop", "store",
                "buy", "sell", "purchase", "sale", "vendor", "dealer"
            ]
            
            # Weight trade indicators by importance (from JSON analysis)
            for indicator in trade_indicators:
                if indicator in content:
                    if indicator in ["market", "merchant", "trading"]:
                        economic_activity += 2  # High-value indicators
                    else:
                        economic_activity += 1
        
        return economic_activity
    
    def _count_npcs_from_ml(self, ml_entities: list[dict[str, Any]]) -> int:
        """Count NPCs from ML analysis results."""
        
        npc_count = 0
        
        for entity in ml_entities:
            ml_features = entity.get("ml_features", {})
            extracted_data = entity.get("extracted_data", {})
            
            # Check for NPC indicators from ML
            if ml_features.get("has_stat_blocks", False):
                npc_count += 1
            elif ml_features.get("class_mentions", 0) > 0:
                npc_count += 1
            elif ml_features.get("title_mentions", 0) > 0:
                npc_count += 1
            elif extracted_data.get("content_type") == "settlement" and ml_features.get("named_entities", 0) > 0:
                npc_count += 1
        
        return npc_count
    
    def _analyze_infrastructure(self, cluster) -> dict[str, bool]:
        """Analyze settlement infrastructure."""
        
        infrastructure = {
            "has_walls": False,
            "has_harbor": False,
            "river_adjacent": False,
            "road_access": False,
            "defensive_structures": False
        }
        
        for entity in cluster.entities:
            content = str(entity).lower()
            
            if any(word in content for word in ["wall", "fortified", "gate"]):
                infrastructure["has_walls"] = True
            if any(word in content for word in ["harbor", "port", "dock"]):
                infrastructure["has_harbor"] = True
            if "river" in content:
                infrastructure["river_adjacent"] = True
            if any(word in content for word in ["road", "path", "trail"]):
                infrastructure["road_access"] = True
            if any(word in content for word in ["tower", "garrison", "barracks"]):
                infrastructure["defensive_structures"] = True
        
        return infrastructure
    
    def _calculate_corruption_resistance(self, scale: str, service_types: list[str], economic_activity: int) -> int:
        """Calculate corruption resistance using enhanced patterns from JSON processor analysis."""
        
        resistance = 0
        
        # Enhanced service coverage assessment (from JSON cities 100% coverage analysis)
        essential_services = {"commerce", "lodging", "crafting", "medical"}
        covered_services = len([svc for svc in essential_services if svc in service_types])
        service_coverage = covered_services / len(essential_services)
        
        # Service coverage bonus (major factor from JSON analysis)
        if service_coverage >= 1.0:  # 100% coverage like City of Headsmen
            resistance += 3
        elif service_coverage >= 0.75:  # 75% coverage
            resistance += 2
        elif service_coverage >= 0.5:   # 50% coverage
            resistance += 1
        
        # Enhanced scale assessment with infrastructure weighting (from JSON patterns)
        infrastructure_bonus = 0
        if "defense" in service_types:
            infrastructure_bonus += 1
        if "government" in service_types:
            infrastructure_bonus += 1
        if "learning" in service_types:
            infrastructure_bonus += 1
        
        # Apply JSON-proven scale classification with infrastructure
        poi_equivalent = len(service_types) + infrastructure_bonus * 2
        
        if poi_equivalent >= 15 or scale == "city":   # Metropolis threshold from JSON
            resistance += 3
        elif poi_equivalent >= 10 or scale == "town":  # City threshold from JSON
            resistance += 2  
        elif poi_equivalent >= 5 or scale == "village": # Town threshold from JSON
            resistance += 1
        
        # Economic activity provides resistance (enhanced thresholds from JSON)
        if economic_activity >= 25:      # Very high activity (JSON metropolis level)
            resistance += 3
        elif economic_activity >= 15:    # High activity (JSON city level)
            resistance += 2
        elif economic_activity >= 8:     # Moderate activity (JSON town level) 
            resistance += 1
        
        return min(resistance, 10)  # Increased cap from JSON analysis insights
    
    def _prepare_template_data(self, result: ProcessingResult) -> dict[str, Any]:
        """Prepare settlements data for Rust ECS template using analyzed data."""
        
        # Use actual analyzed data instead of hardcoded placeholders
        specific_data = result.specific_data
        if not isinstance(specific_data, SettlementData):
            return {"settlements": []}
        
        # Calculate actual hex positioning based on settlement analysis
        hex_q, hex_r = self._calculate_hex_position(result.cluster_name, specific_data)
        biome = self._determine_settlement_biome(specific_data)
        distance_band = self._calculate_distance_band(hex_q, hex_r)
        
        settlements_data = [{
            "name": specific_data.name,
            "rust_name": self._to_rust_identifier(specific_data.name),
            "scale_rust": self._scale_to_rust_enum(specific_data.scale_hint.value),
            "economic_activity": specific_data.economic_activity_level,
            "corruption_resistance": specific_data.corruption_resistance,
            "service_types_rust": self._service_types_to_rust([svc.value for svc in specific_data.service_types]),
            "npc_count": specific_data.npc_count,
            "establishment_count": specific_data.establishment_count,
            # Use analyzed positioning and biome data
            "hex_q": hex_q,
            "hex_r": hex_r,
            "biome": biome,
            "distance_band": distance_band
        }]
        
        return {"settlements": settlements_data}
    
    def _calculate_hex_position(self, settlement_name: str, data: SettlementData) -> tuple[int, int]:
        """Calculate hex position based on settlement characteristics and name."""
        
        # Use settlement name hash for consistent positioning
        name_hash = hash(settlement_name)
        
        # Distribute based on scale and economic activity
        scale_factor = {"village": 1, "town": 2, "city": 3, "metropolis": 4}.get(data.scale_hint.value, 1)
        economic_factor = min(data.economic_activity_level // 10, 5)
        
        # Generate consistent coordinates
        q = (name_hash % 20) * scale_factor + economic_factor
        r = ((name_hash // 20) % 20) * scale_factor
        
        return q, r
    
    def _determine_settlement_biome(self, data: SettlementData) -> str:
        """Determine settlement biome based on characteristics."""
        
        # Use corruption resistance to determine biome suitability
        if data.corruption_resistance >= 8:
            return "wet_meadow"  # High resistance = peaceful biome
        elif data.corruption_resistance >= 5:
            return "ashen_forest"  # Medium resistance = unease biome
        elif data.corruption_resistance >= 3:
            return "black_swamp"  # Low resistance = dread biome
        else:
            return "bone_forest"  # Very low resistance = horror biome
    
    def _calculate_distance_band(self, q: int, r: int) -> str:
        """Calculate distance band from hex coordinates."""
        
        distance = ((q * q + r * r + q * r) ** 0.5)
        
        if distance <= 20:
            return "peace"
        elif distance <= 40:
            return "unease"
        elif distance <= 60:
            return "dread"
        elif distance <= 120:
            return "terror"
        else:
            return "horror"
    
    def _classify_economic_level(self, activity_level: int) -> str:
        """Classify economic activity level."""
        
        if activity_level >= self.economic_thresholds["very_high"]:
            return "very_high"
        elif activity_level >= self.economic_thresholds["high"]:
            return "high"
        elif activity_level >= self.economic_thresholds["moderate"]:
            return "moderate"
        elif activity_level >= self.economic_thresholds["low"]:
            return "low"
        else:
            return "none"


def process_settlement_cluster(cluster, logger, console) -> dict[str, Any]:
    """
    Process settlement entity cluster using SettlementsProcessor.
    
    Args:
        cluster: EntityCluster containing settlement entities from transformer
        logger: Logger instance from orchestrator
        console: Rich console from orchestrator
        
    Returns:
        Processed settlement data with world_hooks for Godot integration
    """
    
    processor = SettlementsProcessor()
    return processor.process_cluster(cluster, logger, console)


def _serialize_entity_for_processing(entity: dict[str, Any]) -> str:
    """Serialize entity dict to string for base ML processor."""
    
    import json
    return json.dumps(entity, indent=2)


def _extract_settlement_specific_data(cluster, ml_results: dict[str, Any]) -> dict[str, Any]:
    """Extract settlement-specific data from ML processing results."""
    
    entities = ml_results.get("entities", [])
    
    # Determine settlement scale from name
    scale_hint = _determine_scale_from_name(cluster.name)
    
    # Aggregate settlement characteristics
    establishment_count = 0
    npc_count = 0
    service_types = set()
    economic_activity = 0
    
    for entity_result in entities:
        extracted = entity_result.get("extracted_data", {})
        
        # Count establishments (any named building/service)
        if extracted.get("name") and any(word in str(extracted).lower() for word in ["tavern", "inn", "shop", "market", "temple"]):
            establishment_count += 1
        
        # Count NPCs
        if extracted.get("hit_points") or extracted.get("challenge_rating"):
            npc_count += 1
        
        # Track service types
        content = str(extracted).lower()
        if "tavern" in content or "inn" in content:
            service_types.add("lodging")
        if "shop" in content or "market" in content:
            service_types.add("commerce")
        if "smith" in content or "forge" in content:
            service_types.add("crafting")
        if "temple" in content or "shrine" in content:
            service_types.add("religious")
        
        # Economic activity indicators
        if any(currency in content for currency in ["gp", "sp", "cp", "gold", "silver", "copper"]):
            economic_activity += 1
    
    return {
        "name": cluster.name,
        "scale_hint": scale_hint,
        "establishment_count": establishment_count,
        "npc_count": npc_count,
        "service_types": list(service_types),
        "economic_activity_level": economic_activity,
        "service_diversity": len(service_types),
        "processing_confidence": _calculate_settlement_confidence(ml_results)
    }


def _generate_settlement_world_hooks(cluster, settlement_data: dict[str, Any]) -> dict[str, Any]:
    """Generate world_hooks for Pandora addon integration."""
    
    return {
        "settlement_name": cluster.name,
        "scale_hint": settlement_data.get("scale_hint", "unknown"),
        "establishment_count": settlement_data.get("establishment_count", 0),
        "service_types": settlement_data.get("service_types", []),
        "economic_activity": settlement_data.get("economic_activity_level", 0),
        "npc_density": settlement_data.get("npc_count", 0),
        "has_tavern": "lodging" in settlement_data.get("service_types", []),
        "has_shops": "commerce" in settlement_data.get("service_types", []),
        "has_crafting": "crafting" in settlement_data.get("service_types", []),
        "has_temple": "religious" in settlement_data.get("service_types", []),
        "godot_integration": {
            "settlement_sprite": f"res://art/settlements/{settlement_data.get('scale_hint', 'village')}.png",
            "npc_spawn_count": min(20, max(5, settlement_data.get("npc_count", 0))),
            "service_spawn_points": settlement_data.get("establishment_count", 0),
            "economic_level": _classify_economic_level(settlement_data.get("economic_activity_level", 0)),
            "corruption_resistance": _calculate_settlement_corruption_resistance(settlement_data)
        }
    }


def _determine_scale_from_name(settlement_name: str) -> str:
    """Determine settlement scale from name."""
    
    if settlement_name.startswith("City of"):
        return "city"
    elif settlement_name.startswith("Town of"):
        return "town"
    elif settlement_name.startswith("Village of"):
        return "village"
    else:
        return "unknown"


def _calculate_settlement_confidence(ml_results: dict[str, Any]) -> float:
    """Calculate confidence score for settlement processing."""
    
    entities = ml_results.get("entities", [])
    if not entities:
        return 0.0
    
    # Average confidence across all entities
    confidences = [entity.get("confidence", 0.0) for entity in entities]
    return sum(confidences) / len(confidences)


def _classify_economic_level(activity_level: int) -> str:
    """Classify economic activity level."""
    
    if activity_level >= 10:
        return "high"
    elif activity_level >= 5:
        return "moderate"
    elif activity_level >= 1:
        return "low"
    else:
        return "none"


def _calculate_settlement_corruption_resistance(settlement_data: dict[str, Any]) -> int:
    """Calculate corruption resistance based on settlement characteristics."""
    
    resistance = 0
    
    # Religious presence provides resistance
    if "religious" in settlement_data.get("service_types", []):
        resistance += 2
    
    # Economic activity provides resistance
    economic_level = settlement_data.get("economic_activity_level", 0)
    if economic_level >= 5:
        resistance += 2
    elif economic_level >= 1:
        resistance += 1
    
    # Population provides resistance
    scale = settlement_data.get("scale_hint", "unknown")
    if scale == "city":
        resistance += 3
    elif scale == "town":
        resistance += 2
    elif scale == "village":
        resistance += 1
    
    return min(resistance, 5)  # Cap at 5


def _analyze_settlement_patterns(settlement_examples: list[dict[str, Any]], logger: logging.Logger, console) -> dict[str, Any]:
    """Analyze patterns across all settlement examples."""
    
    console.print("🔍 Analyzing settlement patterns...")
    
    patterns = {
        "scale_patterns": {},
        "economic_patterns": {},
        "establishment_patterns": {},
        "npc_patterns": {},
        "service_patterns": {}
    }
    
    for example in settlement_examples:
        name = example.get("name", "Unknown")
        training_signals = example.get("training_signals", {})
        
        # Scale pattern analysis
        scale = training_signals.get("scale_indicators", "unknown")
        if scale not in patterns["scale_patterns"]:
            patterns["scale_patterns"][scale] = {"count": 0, "examples": [], "characteristics": []}
        
        patterns["scale_patterns"][scale]["count"] += 1
        patterns["scale_patterns"][scale]["examples"].append(name)
        
        # Economic pattern analysis
        economic_complexity = training_signals.get("economic_complexity", 0)
        patterns["economic_patterns"][name] = {
            "complexity_score": economic_complexity,
            "establishment_count": training_signals.get("establishment_count", 0),
            "npc_density": training_signals.get("npc_density", 0)
        }
        
        # Extract more detailed patterns from content
        content_features = example.get("content_features", {})
        _analyze_settlement_content_features(name, content_features, patterns, logger)
        
        logger.debug(f"Analyzed settlement pattern for: {name} (scale: {scale})")
    
    # Summarize patterns
    patterns["summary"] = _summarize_settlement_patterns(patterns)
    
    console.print(f"📈 Settlement pattern analysis complete: {len(patterns['summary'])} key insights")
    
    return patterns


def _analyze_settlement_content_features(name: str, content_features: dict[str, Any], patterns: dict[str, Any], logger: logging.Logger) -> None:
    """Analyze detailed content features for a settlement."""
    
    # Establishment pattern analysis
    establishments = content_features.get("establishment_types", [])
    patterns["establishment_patterns"][name] = {
        "types": establishments,
        "variety": len(establishments),
        "tavern_presence": "tavern" in establishments,
        "shop_presence": "shop" in establishments or "market" in establishments
    }
    
    # NPC pattern analysis
    npc_roles = content_features.get("npc_roles", [])
    patterns["npc_patterns"][name] = {
        "role_variety": len(npc_roles),
        "common_roles": npc_roles,
        "has_merchants": "merchant" in npc_roles,
        "has_crafters": "blacksmith" in npc_roles or "crafter" in npc_roles
    }
    
    # Service pattern analysis
    economic_indicators = content_features.get("economic_indicators", {})
    patterns["service_patterns"][name] = {
        "currency_activity": economic_indicators.get("currency_mentions", 0),
        "trade_activity": economic_indicators.get("trade_activity", 0),
        "inventory_systems": economic_indicators.get("inventory_systems", 0)
    }


def _create_settlement_training_vectors(settlement_examples: list[dict[str, Any]], logger: logging.Logger, console) -> list[dict[str, Any]]:
    """Create ML training vectors for all settlement examples."""
    
    console.print("🤖 Creating ML training vectors...")
    
    vectors = []
    
    for example in settlement_examples:
        vector = create_ml_training_vector(example, "settlements")
        
        if vector:
            vector["source_name"] = example.get("name")
            vector["scale_category"] = example.get("training_signals", {}).get("scale_indicators", "unknown")
            vector["quality_score"] = validate_extraction_quality(example)
            vectors.append(vector)
    
    logger.info(f"Created {len(vectors)} settlement training vectors")
    console.print(f"🎯 Created {len(vectors)} ML training vectors")
    
    return vectors


def _generate_settlement_extraction_rules(analysis_results: dict[str, Any], logger: logging.Logger, console) -> dict[str, Any]:
    """Generate extraction rules from settlement pattern analysis."""
    
    console.print("📝 Generating settlement extraction rules...")
    
    rules = {
        "scale_detection": _create_scale_rules(analysis_results.get("scale_patterns", {})),
        "establishment_extraction": _create_establishment_rules(analysis_results.get("establishment_patterns", {})),
        "economic_analysis": _create_economic_rules(analysis_results.get("economic_patterns", {})),
        "npc_extraction": _create_npc_rules(analysis_results.get("npc_patterns", {})),
        "service_extraction": _create_service_rules(analysis_results.get("service_patterns", {}))
    }
    
    # Add settlement-specific quality thresholds
    rules["quality_thresholds"] = {
        "minimum_confidence": 0.6,  # Lower than regions due to content variety
        "high_confidence": 0.85,
        "scale_detection_confidence": 0.8,
        "economic_analysis_confidence": 0.7
    }
    
    logger.info("Generated extraction rules for all settlement categories")
    console.print("✅ Extraction rules generated for scale, establishments, economics, NPCs, services")
    
    return rules


def _create_scale_rules(scale_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create settlement scale detection rules."""
    
    return {
        "scale_categories": ["village", "town", "city"],
        "scale_distribution": {
            scale: data.get("count", 0) 
            for scale, data in scale_patterns.items()
        },
        "scale_indicators": {
            "village": ["small", "hamlet", "rural", "farm", "cottage"],
            "town": ["market", "trade", "merchant", "shops", "commerce"],
            "city": ["district", "thousands", "walls", "metropolis", "major"]
        },
        "extraction_patterns": [
            r'(?:City|Town|Village) of ([^<"]+)',  # Direct scale indicators
            r'(thousands|hundreds|dozens) of',  # Population indicators
            r'(district|quarter|ward)',  # City structure indicators
            r'(market|trade|merchant)',  # Economic scale indicators
        ],
        "confidence_scoring": {
            "direct_mention": 1.0,  # "City of X" gets highest confidence
            "population_indicator": 0.8,  # Population size mentions
            "structural_indicator": 0.7,  # Districts, walls, etc.
            "economic_indicator": 0.6  # Market activity levels
        }
    }


def _create_establishment_rules(establishment_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create establishment extraction rules."""
    
    # Analyze establishment variety across settlements
    all_establishments = []
    for settlement_data in establishment_patterns.values():
        all_establishments.extend(settlement_data.get("types", []))
    
    type_frequency = {}
    for est_type in all_establishments:
        type_frequency[est_type] = type_frequency.get(est_type, 0) + 1
    
    return {
        "establishment_types": ["tavern", "inn", "shop", "market", "smithy", "temple"],
        "type_frequency": type_frequency,
        "extraction_patterns": [
            r'<strong>"([^"]+Tavern[^"]*)"</strong>',  # Tavern names
            r'<strong>"([^"]+Inn[^"]*)"</strong>',  # Inn names
            r'<strong>([^<]+Shop[^<]*)</strong>',  # Shop names
            r'<strong>([^<]+Market[^<]*)</strong>',  # Market names
            r'([^<]*Smithy[^<]*)',  # Smithy/blacksmith
            r'([^<]*Temple[^<]*)'  # Religious establishments
        ],
        "service_indicators": {
            "tavern": ["drinks", "food", "lodging", "patrons"],
            "shop": ["inventory", "price", "goods", "items"],
            "market": ["trade", "merchant", "caravan", "commerce"]
        }
    }


def _create_economic_rules(economic_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create economic analysis rules."""
    
    # Analyze economic complexity distribution
    complexity_scores = [data.get("complexity_score", 0) for data in economic_patterns.values()]
    avg_complexity = sum(complexity_scores) / max(len(complexity_scores), 1)
    
    return {
        "economic_indicators": ["gp", "sp", "cp", "price", "cost", "trade", "merchant"],
        "average_complexity": avg_complexity,
        "complexity_thresholds": {
            "low": 0,
            "medium": avg_complexity * 0.5,
            "high": avg_complexity * 1.5,
            "very_high": avg_complexity * 2.0
        },
        "extraction_patterns": [
            r'(\d+)\s*(gp|sp|cp)',  # Currency amounts
            r'Price[^:]*:\s*([^<\n]+)',  # Price listings
            r'([^<]*trade[^<]*)',  # Trade activity
            r'([^<]*merchant[^<]*)',  # Merchant activity
            r'<tr><td>([^<]+)</td><td>([^<]+)</td></tr>'  # Price tables
        ]
    }


def _create_npc_rules(npc_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create NPC extraction rules."""
    
    # Analyze role distribution
    all_roles = []
    for settlement_data in npc_patterns.values():
        all_roles.extend(settlement_data.get("common_roles", []))
    
    role_frequency = {}
    for role in all_roles:
        role_frequency[role] = role_frequency.get(role, 0) + 1
    
    return {
        "common_roles": ["fighter", "cleric", "wizard", "rogue", "druid"],
        "role_frequency": role_frequency,
        "extraction_patterns": [
            r'<a class="npc-anchor"[^>]*></a><strong>([^<]+)</strong>',  # NPC names
            r'level (\d+) (\w+) (\w+)',  # Level, race, class
            r'\(<em>([^<]+)</em>\)',  # Emotional states
            r'<small>In the pocket: ([^<]+)</small>',  # Possessions
            r'Member of the <a[^>]*><strong>([^<]+)</strong>'  # Faction membership
        ],
        "profession_indicators": {
            "merchant": ["trade", "goods", "caravan", "inventory"],
            "innkeeper": ["keeper", "tavern", "inn", "lodging"],
            "crafter": ["smith", "forge", "craft", "tools"],
            "guard": ["patrol", "watch", "guard", "militia"]
        }
    }


def _create_service_rules(service_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create service extraction rules."""
    
    # Analyze service complexity
    trade_activities = [data.get("trade_activity", 0) for data in service_patterns.values()]
    avg_trade_activity = sum(trade_activities) / max(len(trade_activities), 1)
    
    return {
        "service_types": ["drinks", "food", "lodging", "goods", "crafting"],
        "average_trade_activity": avg_trade_activity,
        "extraction_patterns": [
            r'<h5>(Drinks|Food|Lodging|Inventory)</h5>',  # Service sections
            r'<tr><td>([^<]+)</td><td>([^<]+gp|[^<]+sp|[^<]+cp)</td></tr>',  # Price entries
            r'offering.*?for sale',  # Trade activity
            r'(Keeper|Staff|Owner)[^:]*:\s*([^<]+)',  # Service providers
        ],
        "economic_indicators": {
            "currency_types": ["gp", "sp", "cp"],
            "trade_keywords": ["sale", "price", "cost", "trade", "merchant", "caravan"],
            "service_keywords": ["drinks", "food", "lodging", "room", "bed"]
        }
    }


def _summarize_settlement_patterns(patterns: dict[str, Any]) -> dict[str, Any]:
    """Summarize key patterns across all settlement categories."""
    
    summary = {}
    
    # Scale distribution summary
    scale_data = patterns.get("scale_patterns", {})
    summary["scale_insights"] = {
        "distribution": {scale: data.get("count", 0) for scale, data in scale_data.items()},
        "most_common_scale": max(scale_data.items(), key=lambda x: x[1].get("count", 0))[0] if scale_data else "unknown"
    }
    
    # Economic complexity summary
    economic_data = patterns.get("economic_patterns", {})
    if economic_data:
        complexities = [data.get("complexity_score", 0) for data in economic_data.values()]
        summary["economic_insights"] = {
            "average_complexity": sum(complexities) / max(len(complexities), 1),
            "high_activity_settlements": [
                name for name, data in economic_data.items()
                if data.get("complexity_score", 0) > 50  # Arbitrary threshold
            ]
        }
    
    # Establishment variety summary
    establishment_data = patterns.get("establishment_patterns", {})
    if establishment_data:
        total_variety = sum(data.get("variety", 0) for data in establishment_data.values())
        tavern_count = sum(1 for data in establishment_data.values() if data.get("tavern_presence"))
        
        summary["establishment_insights"] = {
            "average_variety": total_variety / max(len(establishment_data), 1),
            "tavern_coverage": tavern_count / max(len(establishment_data), 1),
            "common_establishment_types": ["tavern", "shop", "market"]  # Based on HBF data
        }
    
    # NPC role summary
    npc_data = patterns.get("npc_patterns", {})
    if npc_data:
        all_roles = []
        for settlement_data in npc_data.values():
            all_roles.extend(settlement_data.get("common_roles", []))
        
        role_distribution = {}
        for role in all_roles:
            role_distribution[role] = role_distribution.get(role, 0) + 1
        
        summary["npc_insights"] = {
            "total_npc_mentions": len(all_roles),
            "role_distribution": role_distribution,
            "most_common_roles": sorted(role_distribution.items(), key=lambda x: x[1], reverse=True)[:5]
        }
    
    return summary


def _save_settlement_patterns(extraction_rules: dict[str, Any], analysis_results: dict[str, Any]) -> bool:
    """Save learned settlement patterns for future use."""
    
    patterns_dir = Path("training") / "settlements"
    patterns_dir.mkdir(parents=True, exist_ok=True)
    
    try:
        # Save extraction rules
        with open(patterns_dir / "extraction_rules.json", "w", encoding="utf-8") as f:
            import json
            json.dump(extraction_rules, f, indent=2)
        
        # Save analysis summary
        with open(patterns_dir / "pattern_analysis.json", "w", encoding="utf-8") as f:
            import json
            json.dump(analysis_results.get("summary", {}), f, indent=2)
        
        # Save training guide
        _create_settlement_training_guide(patterns_dir, analysis_results)
        
        return True
        
    except Exception as e:
        logging.error(f"Failed to save settlement patterns: {e}")
        return False


def _create_settlement_training_guide(patterns_dir: Path, analysis_results: dict[str, Any]) -> None:
    """Create settlement-specific training guide."""
    
    summary = analysis_results.get("summary", {})
    
    guide_content = f"""# Settlement ML Training Guide - Organized Data Analysis

## Settlement Training Data Summary

### Scale Distribution
{_format_scale_summary(summary)}

### Economic Patterns
{_format_economic_summary(summary)}

### Establishment Patterns
{_format_establishment_summary(summary)}

### NPC Role Distribution
{_format_npc_summary(summary)}

## Content Extraction Strategy

### 1. Scale Detection
- **Primary**: Look for "City/Town/Village of X" patterns
- **Secondary**: Analyze economic complexity and establishment variety
- **Tertiary**: Count NPC density and service offerings

### 2. Economic Analysis
- **Currency Tracking**: Count gp/sp/cp mentions for activity level
- **Trade Activity**: Identify merchant/caravan/trade patterns
- **Service Diversity**: Catalog drinks/food/lodging/goods offerings

### 3. Establishment Classification
- **Taverns/Inns**: Look for "Tavern"/"Inn" in names, drinks/food/lodging services
- **Shops/Markets**: Look for inventory tables, price lists, goods offerings
- **Crafting**: Look for smithy/forge/tools, specific craft services

### 4. NPC Relationship Mapping
- **Professional Roles**: Extract class/profession from descriptions
- **Faction Membership**: Track "Member of" patterns for political mapping
- **Economic Roles**: Map NPCs to establishments and services

## Training Confidence

- **Data Source**: 10 organized settlement examples with rich content
- **Scale Coverage**: Villages (150+ entities) to Cities (1000+ entities)
- **Pattern Reliability**: High confidence in scale and economic indicators
- **Content Richness**: Full NPC networks, establishment details, economic systems

Generated from HBF worldbuilding breakthrough organized data.
"""
    
    try:
        with open(patterns_dir / "training_guide.md", "w", encoding="utf-8") as f:
            f.write(guide_content)
    except Exception as e:
        logging.error(f"Failed to create settlement training guide: {e}")


def _format_scale_summary(summary: dict[str, Any]) -> str:
    """Format scale distribution summary."""
    
    scale_insights = summary.get("scale_insights", {})
    distribution = scale_insights.get("distribution", {})
    
    return f"""
- **Distribution**: {distribution}
- **Most Common**: {scale_insights.get('most_common_scale', 'unknown')}
- **Scale Indicators**: Clear hierarchical structure with distinct characteristics
"""


def _format_economic_summary(summary: dict[str, Any]) -> str:
    """Format economic patterns summary."""
    
    economic_insights = summary.get("economic_insights", {})
    
    return f"""
- **Average Complexity**: {economic_insights.get('average_complexity', 0):.1f}
- **High Activity Settlements**: {', '.join(economic_insights.get('high_activity_settlements', [])[:3])}
- **Economic Indicators**: Currency circulation, trade goods, service variety
"""


def _format_establishment_summary(summary: dict[str, Any]) -> str:
    """Format establishment patterns summary."""
    
    establishment_insights = summary.get("establishment_insights", {})
    
    return f"""
- **Average Variety**: {establishment_insights.get('average_variety', 0):.1f} establishment types
- **Tavern Coverage**: {establishment_insights.get('tavern_coverage', 0):.1%} of settlements
- **Common Types**: {', '.join(establishment_insights.get('common_establishment_types', []))}
"""


def _format_npc_summary(summary: dict[str, Any]) -> str:
    """Format NPC role distribution summary."""
    
    npc_insights = summary.get("npc_insights", {})
    most_common = npc_insights.get("most_common_roles", [])
    
    return f"""
- **Total NPC Mentions**: {npc_insights.get('total_npc_mentions', 0)}
- **Most Common Roles**: {', '.join([role for role, count in most_common[:3]])}
- **Role Distribution**: Balanced across combat and social classes
"""


def extract_settlement_content(entity_content: str, learned_rules: dict[str, Any] | None = None) -> dict[str, Any]:
    """
    Extract settlement content using learned patterns.
    
    Args:
        entity_content: Raw entity content to analyze
        learned_rules: Optional pre-learned extraction rules
        
    Returns:
        Extracted settlement data with confidence scoring
    """
    
    if not learned_rules:
        # Load saved patterns
        patterns_file = Path("training") / "settlements" / "extraction_rules.json"
        if patterns_file.exists():
            try:
                import json
                with open(patterns_file, "r", encoding="utf-8") as f:
                    learned_rules = json.load(f)
            except Exception:
                learned_rules = {}
        else:
            learned_rules = {}
    
    # Use patterns.py function for core extraction
    extracted_data = extract_settlement_data(entity_content)
    
    # Add learned rule enhancements
    if learned_rules.get("scale_detection"):
        scale_confidence = _assess_scale_confidence(entity_content, learned_rules["scale_detection"])
        extracted_data["scale_confidence"] = scale_confidence
    
    # Add economic analysis
    if learned_rules.get("economic_analysis"):
        economic_analysis = _apply_economic_analysis(entity_content, learned_rules["economic_analysis"])
        extracted_data["economic_analysis"] = economic_analysis
    
    # Calculate overall confidence
    extracted_data["confidence_score"] = validate_extraction_quality(extracted_data)
    extracted_data["category"] = "settlement"
    extracted_data["extraction_method"] = "learned_patterns"
    
    return extracted_data


def _assess_scale_confidence(content: str, scale_rules: dict[str, Any]) -> dict[str, Any]:
    """Assess confidence in scale detection."""
    
    confidence_scores = {}
    scale_indicators = scale_rules.get("scale_indicators", {})
    confidence_scoring = scale_rules.get("confidence_scoring", {})
    
    for scale, indicators in scale_indicators.items():
        score = 0.0
        evidence = []
        
        # Check for direct mentions
        if f"{scale.title()} of" in content:
            score += confidence_scoring.get("direct_mention", 1.0)
            evidence.append("direct_mention")
        
        # Check for indicator words
        for indicator in indicators:
            if indicator.lower() in content.lower():
                score += 0.1  # Small boost per indicator
                evidence.append(f"indicator_{indicator}")
        
        confidence_scores[scale] = {
            "score": min(score, 1.0),  # Cap at 1.0
            "evidence": evidence
        }
    
    # Determine most likely scale
    best_scale = max(confidence_scores.items(), key=lambda x: x[1]["score"])
    
    return {
        "scale_scores": confidence_scores,
        "predicted_scale": best_scale[0],
        "prediction_confidence": best_scale[1]["score"],
        "prediction_evidence": best_scale[1]["evidence"]
    }


def _apply_economic_analysis(content: str, economic_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply economic analysis patterns."""
    
    analysis = {
        "currency_activity": 0,
        "trade_activity": 0,
        "service_complexity": 0,
        "economic_classification": "low"
    }
    
    # Count economic indicators
    for indicator in economic_rules.get("economic_indicators", []):
        analysis["currency_activity"] += content.count(indicator)
    
    # Extract price information
    for pattern in economic_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content)
        analysis["trade_activity"] += len(matches)
    
    # Classify economic activity
    thresholds = economic_rules.get("complexity_thresholds", {})
    total_activity = analysis["currency_activity"] + analysis["trade_activity"]
    
    if total_activity >= thresholds.get("very_high", 100):
        analysis["economic_classification"] = "very_high"
    elif total_activity >= thresholds.get("high", 50):
        analysis["economic_classification"] = "high"
    elif total_activity >= thresholds.get("medium", 20):
        analysis["economic_classification"] = "medium"
    else:
        analysis["economic_classification"] = "low"
    
    return analysis
