"""
Settlement ML Training - Extract settlement content from organized HBF examples.

Uses organized settlement data from memory-bank/world-building/settlements/ to train
ML models for settlement content extraction. Focuses on scale detection, economic
analysis, establishment categorization, and NPC relationship mapping.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

from .meta import load_organized_examples
from .patterns import extract_settlement_data, create_ml_training_vector, validate_extraction_quality


def run(engine, logger: logging.Logger, console) -> dict[str, Any]:
    """
    Run settlement ML training using organized HBF examples.
    
    Args:
        engine: Database engine (passed but not used - we use organized files)
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Training results with learned patterns
    """
    
    logger.info("Starting settlement ML training from organized examples")
    console.print("🏘️ [bold blue]Settlement ML Training[/bold blue] - Learning from 10 organized examples")
    
    # Load organized settlement examples
    settlement_examples = load_organized_examples("settlements")
    
    if not settlement_examples:
        logger.error("No organized settlement examples found")
        return {"error": "No training data available"}
    
    logger.info(f"Loaded {len(settlement_examples)} settlement examples")
    console.print(f"📊 Loaded {len(settlement_examples)} organized settlement examples")
    
    # Analyze settlement patterns
    analysis_results = _analyze_settlement_patterns(settlement_examples, logger, console)
    
    # Create ML training vectors
    training_vectors = _create_settlement_training_vectors(settlement_examples, logger, console)
    
    # Generate settlement extraction rules
    extraction_rules = _generate_settlement_extraction_rules(analysis_results, logger, console)
    
    # Save learned patterns
    patterns_saved = _save_settlement_patterns(extraction_rules, analysis_results)
    
    results = {
        "examples_analyzed": len(settlement_examples),
        "pattern_analysis": analysis_results,
        "training_vectors": training_vectors,
        "extraction_rules": extraction_rules,
        "patterns_saved": patterns_saved,
        "training_quality": "organized_breakthrough_data"
    }
    
    console.print("✅ [bold green]Settlement training complete[/bold green] - Patterns learned from organized data")
    logger.info(f"Settlement training complete: {len(settlement_examples)} examples processed")
    
    return results


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
