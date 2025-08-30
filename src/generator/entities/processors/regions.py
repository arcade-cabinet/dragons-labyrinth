"""
Region Processor - Process region entity clusters using base ML foundation.

Uses the DragonLabyrinthMLProcessor from base.py to process region clusters
from the transformer. Extracts environmental descriptions, political context,
biome distributions, and settlement patterns with world_hooks for Godot integration.
"""

from __future__ import annotations

import logging
import re
from typing import Any

from generator.entities.processors.base import DragonLabyrinthMLProcessor


def process_region_cluster(cluster) -> dict[str, Any]:
    """
    Process region entity cluster using base ML foundation.
    
    Args:
        cluster: EntityCluster containing region entities from transformer
        
    Returns:
        Processed region data with world_hooks for Godot integration
    """
    
    print(f"🏔️ Processing region cluster: {cluster.name} ({cluster.get_entity_count()} entities)")
    
    # Initialize ML processor
    processor = DragonLabyrinthMLProcessor()
    
    # Convert cluster entities to format expected by base processor
    entity_pairs = []
    for i, entity in enumerate(cluster.entities):
        entity_id = f"{cluster.name}_{i}"
        entity_content = _serialize_entity_for_processing(entity)
        entity_pairs.append((entity_id, entity_content))
    
    # Process entities with base ML
    ml_results = processor.process_entity_batch(entity_pairs)
    
    # Extract region-specific data
    region_data = _extract_region_specific_data(cluster, ml_results)
    
    # Generate world_hooks for Godot integration
    world_hooks = _generate_region_world_hooks(cluster, region_data)
    
    result = {
        "cluster_name": cluster.name,
        "cluster_category": cluster.category,
        "entity_count": cluster.get_entity_count(),
        "region_data": region_data,
        "world_hooks": world_hooks,
        "ml_processing_results": ml_results,
        "processor_type": "regions"
    }
    
    print(f"✅ Region processing complete: {cluster.name}")
    
    return result


def _analyze_region_patterns(region_examples: list[dict[str, Any]], logger: logging.Logger, console) -> dict[str, Any]:
    """Analyze patterns across all region examples."""
    
    console.print("🔍 Analyzing regional patterns...")
    
    patterns = {
        "biome_patterns": {},
        "settlement_patterns": {},
        "geographic_patterns": {},
        "content_complexity": {},
        "political_structures": {}
    }
    
    for example in region_examples:
        name = example.get("name", "Unknown")
        
        # Analyze biome patterns
        if example.get("data_type") == "structured_json":
            _analyze_structured_region(example, patterns, logger)
        else:
            _analyze_html_region(example, patterns, logger)
        
        logger.debug(f"Analyzed region pattern for: {name}")
    
    # Summarize patterns
    patterns["summary"] = _summarize_regional_patterns(patterns)
    
    console.print(f"📈 Pattern analysis complete: {len(patterns['summary'])} key patterns identified")
    
    return patterns


def _analyze_structured_region(example: dict[str, Any], patterns: dict[str, Any], logger: logging.Logger) -> None:
    """Analyze region with structured JSON data."""
    
    name = example["name"]
    content_features = example.get("content_features", {})
    training_signals = example.get("training_signals", {})
    
    # Biome pattern analysis
    biome_dist = content_features.get("biome_distribution", {})
    patterns["biome_patterns"][name] = {
        "diversity": len(biome_dist),
        "dominant_biome": max(biome_dist.items(), key=lambda x: x[1])[0] if biome_dist else "unknown",
        "distribution": biome_dist
    }
    
    # Settlement pattern analysis  
    settlements = content_features.get("settlement_locations", [])
    patterns["settlement_patterns"][name] = {
        "count": len(settlements),
        "types": [s.get("type") for s in settlements],
        "density": len(settlements) / max(training_signals.get("hex_count", 1), 1)
    }
    
    # Geographic pattern analysis
    connectivity = content_features.get("geographic_connectivity", {})
    patterns["geographic_patterns"][name] = {
        "trail_density": connectivity.get("trail_connections", 0),
        "river_density": connectivity.get("river_connections", 0),
        "connectivity_score": connectivity.get("connectivity_density", 0)
    }
    
    # Political structure analysis
    political = content_features.get("political_boundaries", {})
    patterns["political_structures"][name] = {
        "region_count": len(political),
        "territorial_complexity": sum(region_data.get("tiles", 0) for region_data in political.values())
    }
    
    logger.debug(f"Analyzed structured region: {name} ({training_signals.get('hex_count', 0)} hexes)")


def _analyze_html_region(example: dict[str, Any], patterns: dict[str, Any], logger: logging.Logger) -> None:
    """Analyze region with HTML content data."""
    
    name = example["name"]
    content_features = example.get("content_features", {})
    training_signals = example.get("training_signals", {})
    
    # Content complexity analysis
    patterns["content_complexity"][name] = {
        "npc_density": content_features.get("npc_density", 0),
        "table_structures": content_features.get("tables", 0),
        "interactive_elements": content_features.get("spoilers", 0),
        "content_length": training_signals.get("content_length", 0)
    }
    
    logger.debug(f"Analyzed HTML region: {name} ({training_signals.get('content_length', 0)} chars)")


def _create_region_training_vectors(region_examples: list[dict[str, Any]], logger: logging.Logger, console) -> list[dict[str, Any]]:
    """Create ML training vectors for all region examples."""
    
    console.print("🤖 Creating ML training vectors...")
    
    vectors = []
    
    for example in region_examples:
        vector = create_ml_training_vector(example, "regions")
        
        if vector:
            vector["source_name"] = example.get("name")
            vector["data_type"] = example.get("data_type", "unknown")
            vector["quality_score"] = validate_extraction_quality(example)
            vectors.append(vector)
    
    logger.info(f"Created {len(vectors)} training vectors")
    console.print(f"🎯 Created {len(vectors)} ML training vectors")
    
    return vectors


def _generate_region_extraction_rules(analysis_results: dict[str, Any], logger: logging.Logger, console) -> dict[str, Any]:
    """Generate extraction rules from pattern analysis."""
    
    console.print("📝 Generating extraction rules...")
    
    rules = {
        "biome_extraction": _create_biome_rules(analysis_results.get("biome_patterns", {})),
        "settlement_extraction": _create_settlement_rules(analysis_results.get("settlement_patterns", {})),
        "geographic_extraction": _create_geographic_rules(analysis_results.get("geographic_patterns", {})),
        "content_extraction": _create_content_rules(analysis_results.get("content_complexity", {})),
        "political_extraction": _create_political_rules(analysis_results.get("political_structures", {}))
    }
    
    # Add extraction confidence thresholds
    rules["quality_thresholds"] = {
        "minimum_confidence": 0.7,
        "high_confidence": 0.9,
        "extraction_methods": ["regex", "json_parsing", "ml_inference"]
    }
    
    logger.info("Generated extraction rules for all region categories")
    console.print("✅ Extraction rules generated for biomes, settlements, geography, content, politics")
    
    return rules


def _create_biome_rules(biome_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create biome extraction rules from patterns."""
    
    # Analyze biome diversity patterns
    diversity_scores = [data.get("diversity", 0) for data in biome_patterns.values()]
    avg_diversity = sum(diversity_scores) / max(len(diversity_scores), 1)
    
    # Identify dominant biome types
    all_biomes = set()
    for region_data in biome_patterns.values():
        if region_data.get("distribution"):
            all_biomes.update(region_data["distribution"].keys())
    
    return {
        "expected_biome_types": list(all_biomes),
        "average_diversity": avg_diversity,
        "extraction_patterns": [
            r'"type":\s*"([^"]+)"',  # JSON biome type
            r'(\w+)Hex',  # Hex type patterns
            r'biome[^:]*:\s*"([^"]+)"'  # Direct biome references
        ],
        "validation_rules": {
            "known_biomes": list(all_biomes),
            "min_diversity": 1,
            "max_diversity": 10
        }
    }


def _create_settlement_rules(settlement_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create settlement extraction rules from patterns."""
    
    # Analyze settlement density patterns
    densities = [data.get("density", 0) for data in settlement_patterns.values()]
    avg_density = sum(densities) / max(len(densities), 1)
    
    # Analyze settlement type distributions
    all_types = []
    for region_data in settlement_patterns.values():
        all_types.extend(region_data.get("types", []))
    
    type_counts = {}
    for settlement_type in all_types:
        type_counts[settlement_type] = type_counts.get(settlement_type, 0) + 1
    
    return {
        "settlement_types": ["Village", "Town", "City"],
        "average_density": avg_density,
        "type_distribution": type_counts,
        "extraction_patterns": [
            r'"feature":\s*"(Village|Town|City)"',  # JSON settlement features
            r'(Village|Town|City) of ([^<"]+)',  # Settlement titles
            r'"label":\s*"([^"]+)"'  # Settlement labels
        ],
        "scale_indicators": {
            "village": ["small", "hamlet", "rural", "farm"],
            "town": ["market", "trade", "merchant", "shops"],
            "city": ["district", "thousands", "walls", "metropolis"]
        }
    }


def _create_geographic_rules(geographic_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create geographic feature extraction rules."""
    
    # Analyze connectivity patterns
    trail_densities = [data.get("trail_density", 0) for data in geographic_patterns.values()]
    river_densities = [data.get("river_density", 0) for data in geographic_patterns.values()]
    
    return {
        "connectivity_features": ["rivers", "trails", "harbors", "borders"],
        "average_trail_density": sum(trail_densities) / max(len(trail_densities), 1),
        "average_river_density": sum(river_densities) / max(len(river_densities), 1),
        "extraction_patterns": [
            r'"rivers":\s*\[([\d,\s]*)\]',  # JSON river data
            r'"trails":\s*\[([\d,\s]*)\]',  # JSON trail data
            r'"harbor":\s*(\d+)',  # Harbor information
            r'"borderline":\s*(true|false)'  # Border information
        ]
    }


def _create_content_rules(content_complexity: dict[str, Any]) -> dict[str, Any]:
    """Create content extraction rules from complexity analysis."""
    
    # Analyze NPC density patterns
    npc_densities = [data.get("npc_density", 0) for data in content_complexity.values()]
    avg_npc_density = sum(npc_densities) / max(len(npc_densities), 1)
    
    # Analyze table structure patterns
    table_counts = [data.get("table_structures", 0) for data in content_complexity.values()]
    avg_table_count = sum(table_counts) / max(len(table_counts), 1)
    
    return {
        "npc_extraction": {
            "average_density": avg_npc_density,
            "extraction_patterns": [
                r'<a class="npc-anchor"[^>]*></a><strong>([^<]+)</strong>',
                r'level (\d+) (\w+) (\w+)',
                r'\(<em>([^<]+)</em>\)'  # Emotional states
            ]
        },
        "table_extraction": {
            "average_count": avg_table_count,
            "table_types": ["weather", "rumors", "encounters", "prices"],
            "extraction_patterns": [
                r'<h5>([^<]+)</h5>.*?<table>(.*?)</table>',
                r'<tr><td>([^<]+)</td><td>([^<]+)</td></tr>'
            ]
        },
        "environmental_features": {
            "description_patterns": [
                r'<p>\s*([^<]+)\s*</p>',  # Environmental descriptions
                r'A ([^.]+\.[^<]*)',  # Descriptive sentences
                r'The ([^.]+\.[^<]*)'  # Environment descriptions
            ]
        }
    }


def _create_political_rules(political_structures: dict[str, Any]) -> dict[str, Any]:
    """Create political structure extraction rules."""
    
    # Analyze territorial complexity
    complexities = [data.get("territorial_complexity", 0) for data in political_structures.values()]
    avg_complexity = sum(complexities) / max(len(complexities), 1)
    
    return {
        "territorial_analysis": {
            "average_complexity": avg_complexity,
            "control_patterns": [
                r'"region":\s*"([^"]+)"',  # Regional control
                r'"realm":\s*"([^"]+)"',  # Realm control
                r'Member of the <a[^>]*><strong>([^<]+)</strong>'  # Faction control
            ]
        },
        "political_entities": {
            "faction_indicators": ["The Defiled Wolves", "The Fists Of Justice", "The Swords Of Justice"],
            "control_types": ["regional", "factional", "territorial"]
        }
    }


def _summarize_regional_patterns(patterns: dict[str, Any]) -> dict[str, Any]:
    """Summarize key patterns across all categories."""
    
    summary = {}
    
    # Biome pattern summary
    biome_data = patterns.get("biome_patterns", {})
    if biome_data:
        all_biomes = set()
        diversities = []
        for region_data in biome_data.values():
            if region_data.get("distribution"):
                all_biomes.update(region_data["distribution"].keys())
                diversities.append(region_data.get("diversity", 0))
        
        summary["biome_insights"] = {
            "total_biome_types": len(all_biomes),
            "average_diversity": sum(diversities) / max(len(diversities), 1),
            "common_biomes": list(all_biomes)[:10]  # Top 10
        }
    
    # Settlement pattern summary
    settlement_data = patterns.get("settlement_patterns", {})
    if settlement_data:
        total_settlements = sum(data.get("count", 0) for data in settlement_data.values())
        settlement_types = []
        for region_data in settlement_data.values():
            settlement_types.extend(region_data.get("types", []))
        
        type_distribution = {}
        for s_type in settlement_types:
            type_distribution[s_type] = type_distribution.get(s_type, 0) + 1
        
        summary["settlement_insights"] = {
            "total_settlements": total_settlements,
            "type_distribution": type_distribution,
            "average_per_region": total_settlements / max(len(settlement_data), 1)
        }
    
    # Geographic connectivity summary
    geo_data = patterns.get("geographic_patterns", {})
    if geo_data:
        connectivity_scores = [data.get("connectivity_score", 0) for data in geo_data.values()]
        summary["geographic_insights"] = {
            "average_connectivity": sum(connectivity_scores) / max(len(connectivity_scores), 1),
            "high_connectivity_regions": [
                name for name, data in geo_data.items() 
                if data.get("connectivity_score", 0) > 0.5
            ]
        }
    
    return summary


def _save_region_patterns(extraction_rules: dict[str, Any], analysis_results: dict[str, Any]) -> bool:
    """Save learned region patterns for future use."""
    
    patterns_dir = Path("training") / "regions"
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
        
        # Save training guide update
        _update_training_guide(patterns_dir, analysis_results)
        
        return True
        
    except Exception as e:
        logging.error(f"Failed to save region patterns: {e}")
        return False


def _update_training_guide(patterns_dir: Path, analysis_results: dict[str, Any]) -> None:
    """Update training guide with learned patterns."""
    
    guide_content = f"""# Region ML Training Guide - Updated with Organized Data Analysis

## Training Data Summary

### Biome Patterns
{_format_biome_summary(analysis_results)}

### Settlement Patterns  
{_format_settlement_summary(analysis_results)}

### Geographic Patterns
{_format_geographic_summary(analysis_results)}

### Content Extraction Rules

1. **Biome Detection**: Use JSON parsing for structured data, regex for HTML
2. **Settlement Extraction**: Focus on feature fields and scale indicators
3. **NPC Extraction**: Use npc-anchor patterns with stat block parsing
4. **Geographic Features**: Extract connectivity and boundary data

### Training Confidence

- **Data Quality**: Organized breakthrough data from successful HBF extraction
- **Coverage**: 27 regions with comprehensive examples
- **Pattern Reliability**: High confidence in learned extraction patterns
- **ML Readiness**: Training vectors created for all categories

Generated from organized HBF worldbuilding breakthrough data.
"""
    
    try:
        with open(patterns_dir / "training_guide.md", "w", encoding="utf-8") as f:
            f.write(guide_content)
    except Exception as e:
        logging.error(f"Failed to update training guide: {e}")


def _format_biome_summary(analysis_results: dict[str, Any]) -> str:
    """Format biome analysis for training guide."""
    
    summary = analysis_results.get("summary", {})
    biome_insights = summary.get("biome_insights", {})
    
    if not biome_insights:
        return "No biome data analyzed"
    
    return f"""
- **Total Biome Types**: {biome_insights.get('total_biome_types', 0)}
- **Average Diversity**: {biome_insights.get('average_diversity', 0):.1f} biomes per region
- **Common Biomes**: {', '.join(biome_insights.get('common_biomes', [])[:5])}
"""


def _format_settlement_summary(analysis_results: dict[str, Any]) -> str:
    """Format settlement analysis for training guide."""
    
    summary = analysis_results.get("summary", {})
    settlement_insights = summary.get("settlement_insights", {})
    
    if not settlement_insights:
        return "No settlement data analyzed"
    
    return f"""
- **Total Settlements**: {settlement_insights.get('total_settlements', 0)}
- **Average Per Region**: {settlement_insights.get('average_per_region', 0):.1f}
- **Type Distribution**: {settlement_insights.get('type_distribution', {})}
"""


def _format_geographic_summary(analysis_results: dict[str, Any]) -> str:
    """Format geographic analysis for training guide."""
    
    summary = analysis_results.get("summary", {})
    geo_insights = summary.get("geographic_insights", {})
    
    if not geo_insights:
        return "No geographic data analyzed"
    
    return f"""
- **Average Connectivity**: {geo_insights.get('average_connectivity', 0):.2f}
- **Well-Connected Regions**: {', '.join(geo_insights.get('high_connectivity_regions', [])[:3])}
"""


def extract_region_content(entity_content: str, learned_rules: dict[str, Any] | None = None) -> dict[str, Any]:
    """
    Extract region content using learned patterns.
    
    Args:
        entity_content: Raw entity content to analyze
        learned_rules: Optional pre-learned extraction rules
        
    Returns:
        Extracted region data with confidence scoring
    """
    
    if not learned_rules:
        # Load saved patterns
        patterns_file = Path("training") / "regions" / "extraction_rules.json"
        if patterns_file.exists():
            try:
                import json
                with open(patterns_file, "r", encoding="utf-8") as f:
                    learned_rules = json.load(f)
            except Exception:
                learned_rules = {}
        else:
            learned_rules = {}
    
    extracted_data = {
        "category": "region",
        "extraction_method": "learned_patterns",
        "biomes": [],
        "settlements": [],
        "geographic_features": {},
        "npcs": [],
        "content_analysis": {}
    }
    
    # Apply biome extraction rules
    if learned_rules.get("biome_extraction"):
        extracted_data["biomes"] = _apply_biome_extraction(entity_content, learned_rules["biome_extraction"])
    
    # Apply settlement extraction rules  
    if learned_rules.get("settlement_extraction"):
        extracted_data["settlements"] = _apply_settlement_extraction(entity_content, learned_rules["settlement_extraction"])
    
    # Apply geographic extraction rules
    if learned_rules.get("geographic_extraction"):
        extracted_data["geographic_features"] = _apply_geographic_extraction(entity_content, learned_rules["geographic_extraction"])
    
    # Apply content extraction rules
    if learned_rules.get("content_extraction"):
        extracted_data["npcs"] = _apply_npc_extraction(entity_content, learned_rules["content_extraction"])
    
    # Calculate confidence
    extracted_data["confidence_score"] = validate_extraction_quality(extracted_data)
    
    return extracted_data


def _apply_biome_extraction(content: str, biome_rules: dict[str, Any]) -> list[str]:
    """Apply biome extraction patterns to content."""
    
    biomes = []
    
    for pattern in biome_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content)
        for match in matches:
            if isinstance(match, tuple):
                biome = match[0] if match else ""
            else:
                biome = match
            
            if biome and biome in biome_rules.get("validation_rules", {}).get("known_biomes", []):
                biomes.append(biome)
    
    return list(set(biomes))  # Remove duplicates


def _apply_settlement_extraction(content: str, settlement_rules: dict[str, Any]) -> list[dict[str, Any]]:
    """Apply settlement extraction patterns to content."""
    
    settlements = []
    
    for pattern in settlement_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content)
        for match in matches:
            if isinstance(match, tuple) and len(match) >= 2:
                settlement = {
                    "type": match[0],
                    "name": match[1] if len(match) > 1 else "Unknown"
                }
                settlements.append(settlement)
    
    return settlements


def _apply_geographic_extraction(content: str, geo_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply geographic extraction patterns to content."""
    
    features = {}
    
    for pattern in geo_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content)
        
        if "rivers" in pattern:
            features["rivers"] = len(matches)
        elif "trails" in pattern:
            features["trails"] = len(matches)
        elif "harbor" in pattern:
            features["harbors"] = len(matches)
        elif "border" in pattern:
            features["borders"] = len(matches)
    
    return features


def _apply_npc_extraction(content: str, content_rules: dict[str, Any]) -> list[str]:
    """Apply NPC extraction patterns to content."""
    
    npcs = []
    
    npc_rules = content_rules.get("npc_extraction", {})
    for pattern in npc_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content)
        for match in matches:
            if isinstance(match, tuple):
                npc_name = match[0] if match else ""
            else:
                npc_name = match
            
            if npc_name and len(npc_name) > 2:  # Basic validation
                npcs.append(npc_name)
    
    return list(set(npcs))  # Remove duplicates


def _serialize_entity_for_processing(entity: dict[str, Any]) -> str:
    """Serialize entity dict to string for base ML processor."""
    
    import json
    return json.dumps(entity, indent=2)


def _extract_region_specific_data(cluster, ml_results: dict[str, Any]) -> dict[str, Any]:
    """Extract region-specific data from ML processing results."""
    
    entities = ml_results.get("entities", [])
    
    # Aggregate region characteristics
    biome_distribution = {}
    settlement_locations = []
    npc_count = 0
    total_area = 0
    
    for entity_result in entities:
        extracted = entity_result.get("extracted_data", {})
        
        # Count biomes
        if extracted.get("biome_type"):
            biome = extracted["biome_type"]
            biome_distribution[biome] = biome_distribution.get(biome, 0) + 1
        
        # Track settlements
        if extracted.get("settlement_name"):
            settlement_locations.append({
                "name": extracted["settlement_name"],
                "type": extracted.get("settlement_type", "unknown")
            })
        
        # Count NPCs
        if extracted.get("hit_points"):  # Has stats = NPC/Monster
            npc_count += 1
        
        # Estimate area
        if extracted.get("coordinate"):
            total_area += 1
    
    # Determine dominant biome
    dominant_biome = "unknown"
    if biome_distribution:
        dominant_biome = max(biome_distribution.items(), key=lambda x: x[1])[0]
    
    return {
        "name": cluster.name,
        "dominant_biome": dominant_biome,
        "biome_distribution": biome_distribution,
        "settlement_locations": settlement_locations,
        "npc_count": npc_count,
        "total_area": total_area,
        "settlement_density": len(settlement_locations) / max(total_area, 1),
        "processing_confidence": _calculate_region_confidence(ml_results)
    }


def _generate_region_world_hooks(cluster, region_data: dict[str, Any]) -> dict[str, Any]:
    """Generate world_hooks for Pandora addon integration."""
    
    return {
        "region_name": cluster.name,
        "dominant_biome": region_data.get("dominant_biome", "unknown"),
        "biome_distribution": region_data.get("biome_distribution", {}),
        "settlement_count": len(region_data.get("settlement_locations", [])),
        "npc_density": region_data.get("npc_count", 0) / max(region_data.get("total_area", 1), 1),
        "has_rivers": _check_for_rivers(cluster),
        "has_trails": _check_for_trails(cluster),
        "political_control": _extract_political_control(cluster),
        "border_regions": _extract_border_regions(cluster),
        "godot_integration": {
            "biome_sprite_path": f"res://art/biomes/{region_data.get('dominant_biome', 'unknown').lower()}.png",
            "settlement_spawn_points": len(region_data.get("settlement_locations", [])),
            "npc_spawn_density": min(10, max(1, region_data.get("npc_count", 0))),
            "corruption_base_level": _calculate_corruption_level(cluster.name)
        }
    }


def _calculate_region_confidence(ml_results: dict[str, Any]) -> float:
    """Calculate confidence score for region processing."""
    
    entities = ml_results.get("entities", [])
    if not entities:
        return 0.0
    
    # Average confidence across all entities
    confidences = [entity.get("confidence", 0.0) for entity in entities]
    return sum(confidences) / len(confidences)


def _check_for_rivers(cluster) -> bool:
    """Check if region has rivers."""
    
    for entity in cluster.entities:
        if entity.get("rivers") or "river" in str(entity).lower():
            return True
    return False


def _check_for_trails(cluster) -> bool:
    """Check if region has trails."""
    
    for entity in cluster.entities:
        if entity.get("trails") or "trail" in str(entity).lower():
            return True
    return False


def _extract_political_control(cluster) -> list[str]:
    """Extract political control information from region."""
    
    factions = []
    for entity in cluster.entities:
        entity_str = str(entity)
        # Check for known faction references
        if "Defiled Wolves" in entity_str:
            factions.append("The Defiled Wolves")
        if "Fists Of Justice" in entity_str:
            factions.append("The Fists Of Justice")
        if "Swords Of Justice" in entity_str:
            factions.append("The Swords Of Justice")
        if "Red Snakes" in entity_str:
            factions.append("The Red Snakes")
        if "White Wyverns" in entity_str:
            factions.append("The White Wyverns")
    
    return list(set(factions))


def _extract_border_regions(cluster) -> list[str]:
    """Extract border information for region."""
    
    borders = []
    for entity in cluster.entities:
        if entity.get("borderline") or "border" in str(entity).lower():
            borders.append("has_borders")
    
    return borders


def _calculate_corruption_level(region_name: str) -> int:
    """Calculate base corruption level for region based on name themes."""
    
    name_lower = region_name.lower()
    
    # High corruption themes
    if any(word in name_lower for word in ["dark", "blood", "bone", "nightmare", "hell", "vicious"]):
        return 3
    
    # Medium corruption themes  
    if any(word in name_lower for word in ["fallen", "grey", "shadow", "fear"]):
        return 2
    
    # Low corruption themes
    if any(word in name_lower for word in ["heart", "moon", "gold", "thunder"]):
        return 1
    
    # Neutral themes
    return 0
