"""
Dungeon ML Training - Extract dungeon content from organized HBF examples.

Uses organized dungeon data from memory-bank/world-building/dungeons/ to train
ML models for dungeon content extraction. Focuses on encounter analysis,
treasure assessment, challenge rating estimation, and horror theme extraction.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

from .meta import load_organized_examples
from .patterns import extract_dungeon_data, create_ml_training_vector, validate_extraction_quality


def run(engine, logger: logging.Logger, console) -> dict[str, Any]:
    """
    Run dungeon ML training using organized HBF examples.
    
    Args:
        engine: Database engine (passed but not used - we use organized files)
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Training results with learned patterns
    """
    
    logger.info("Starting dungeon ML training from organized examples")
    console.print("🏰 [bold blue]Dungeon ML Training[/bold blue] - Learning from 18 organized examples")
    
    # Load organized dungeon examples
    dungeon_examples = load_organized_examples("dungeons")
    
    if not dungeon_examples:
        logger.error("No organized dungeon examples found")
        return {"error": "No training data available"}
    
    logger.info(f"Loaded {len(dungeon_examples)} dungeon examples")
    console.print(f"📊 Loaded {len(dungeon_examples)} organized dungeon examples")
    
    # Analyze dungeon patterns
    analysis_results = _analyze_dungeon_patterns(dungeon_examples, logger, console)
    
    # Create ML training vectors
    training_vectors = _create_dungeon_training_vectors(dungeon_examples, logger, console)
    
    # Generate dungeon extraction rules
    extraction_rules = _generate_dungeon_extraction_rules(analysis_results, logger, console)
    
    # Save learned patterns
    patterns_saved = _save_dungeon_patterns(extraction_rules, analysis_results)
    
    results = {
        "examples_analyzed": len(dungeon_examples),
        "pattern_analysis": analysis_results,
        "training_vectors": training_vectors,
        "extraction_rules": extraction_rules,
        "patterns_saved": patterns_saved,
        "training_quality": "organized_breakthrough_data"
    }
    
    console.print("✅ [bold green]Dungeon training complete[/bold green] - Patterns learned from organized data")
    logger.info(f"Dungeon training complete: {len(dungeon_examples)} examples processed")
    
    return results


def _analyze_dungeon_patterns(dungeon_examples: list[dict[str, Any]], logger: logging.Logger, console) -> dict[str, Any]:
    """Analyze patterns across all dungeon examples."""
    
    console.print("🔍 Analyzing dungeon patterns...")
    
    patterns = {
        "type_patterns": {},
        "encounter_patterns": {},
        "treasure_patterns": {},
        "horror_patterns": {},
        "challenge_patterns": {}
    }
    
    for example in dungeon_examples:
        name = example.get("name", "Unknown")
        training_signals = example.get("training_signals", {})
        content_features = example.get("content_features", {})
        
        # Type pattern analysis
        dungeon_type = training_signals.get("dungeon_type", "unknown")
        if dungeon_type not in patterns["type_patterns"]:
            patterns["type_patterns"][dungeon_type] = {"count": 0, "examples": [], "characteristics": []}
        
        patterns["type_patterns"][dungeon_type]["count"] += 1
        patterns["type_patterns"][dungeon_type]["examples"].append(name)
        
        # Encounter pattern analysis
        encounter_density = content_features.get("encounter_density", 0)
        patterns["encounter_patterns"][name] = {
            "encounter_density": encounter_density,
            "content_complexity": _assess_dungeon_complexity(content_features)
        }
        
        # Treasure pattern analysis
        treasure_density = training_signals.get("treasure_density", "none")
        treasure_complexity = content_features.get("treasure_complexity", 0)
        patterns["treasure_patterns"][name] = {
            "treasure_level": treasure_density,
            "treasure_complexity": treasure_complexity,
            "wealth_indicators": _extract_wealth_indicators(content_features)
        }
        
        # Horror theme analysis
        horror_themes = training_signals.get("horror_themes", [])
        patterns["horror_patterns"][name] = {
            "theme_count": len(horror_themes),
            "themes": horror_themes,
            "horror_intensity": _assess_horror_intensity(horror_themes)
        }
        
        # Challenge pattern analysis
        challenge_rating = training_signals.get("challenge_rating", "unknown")
        patterns["challenge_patterns"][name] = {
            "challenge_rating": challenge_rating,
            "difficulty_level": _classify_difficulty(challenge_rating)
        }
        
        logger.debug(f"Analyzed dungeon pattern for: {name} (type: {dungeon_type}, CR: {challenge_rating})")
    
    # Summarize patterns
    patterns["summary"] = _summarize_dungeon_patterns(patterns)
    
    console.print(f"📈 Dungeon pattern analysis complete: {len(patterns['summary'])} key insights")
    
    return patterns


def _assess_dungeon_complexity(content_features: dict[str, Any]) -> int:
    """Assess overall dungeon content complexity."""
    
    complexity = 0
    
    # Encounter complexity
    if content_features.get("encounter_density", 0) > 5:
        complexity += 2
    elif content_features.get("encounter_density", 0) > 0:
        complexity += 1
    
    # Treasure complexity
    treasure_complexity = content_features.get("treasure_complexity", 0)
    if treasure_complexity > 10:
        complexity += 2
    elif treasure_complexity > 0:
        complexity += 1
    
    # Horror elements
    horror_elements = content_features.get("horror_elements", [])
    if len(horror_elements) > 3:
        complexity += 2
    elif len(horror_elements) > 0:
        complexity += 1
    
    return complexity


def _extract_wealth_indicators(content_features: dict[str, Any]) -> dict[str, Any]:
    """Extract wealth indicators from dungeon content."""
    
    # This would be enhanced with actual content analysis
    return {
        "treasure_mentions": content_features.get("treasure_complexity", 0),
        "wealth_level": "unknown",
        "magical_items": 0
    }


def _assess_horror_intensity(horror_themes: list[str]) -> str:
    """Assess horror theme intensity."""
    
    if not horror_themes:
        return "none"
    
    # Categorize horror intensity based on theme types
    high_intensity_themes = ["infernal", "cursed", "violent", "corrupted", "unholy"]
    medium_intensity_themes = ["mourning", "grey", "foresaken", "unspoken"]
    
    high_count = sum(1 for theme in horror_themes if theme in high_intensity_themes)
    medium_count = sum(1 for theme in horror_themes if theme in medium_intensity_themes)
    
    if high_count >= 2:
        return "extreme"
    elif high_count >= 1 or medium_count >= 3:
        return "high"
    elif medium_count >= 1:
        return "moderate"
    else:
        return "low"


def _classify_difficulty(challenge_rating: str) -> str:
    """Classify dungeon difficulty level."""
    
    difficulty_mapping = {
        "easy": "beginner",
        "medium": "intermediate", 
        "hard": "advanced",
        "deadly": "expert"
    }
    
    return difficulty_mapping.get(challenge_rating, "unknown")


def _create_dungeon_training_vectors(dungeon_examples: list[dict[str, Any]], logger: logging.Logger, console) -> list[dict[str, Any]]:
    """Create ML training vectors for all dungeon examples."""
    
    console.print("🤖 Creating ML training vectors...")
    
    vectors = []
    
    for example in dungeon_examples:
        vector = create_ml_training_vector(example, "dungeons")
        
        if vector:
            vector["source_name"] = example.get("name")
            vector["dungeon_type"] = example.get("training_signals", {}).get("dungeon_type", "unknown")
            vector["horror_intensity"] = _assess_horror_intensity(
                example.get("training_signals", {}).get("horror_themes", [])
            )
            vector["quality_score"] = validate_extraction_quality(example)
            vectors.append(vector)
    
    logger.info(f"Created {len(vectors)} dungeon training vectors")
    console.print(f"🎯 Created {len(vectors)} ML training vectors")
    
    return vectors


def _generate_dungeon_extraction_rules(analysis_results: dict[str, Any], logger: logging.Logger, console) -> dict[str, Any]:
    """Generate extraction rules from dungeon pattern analysis."""
    
    console.print("📝 Generating dungeon extraction rules...")
    
    rules = {
        "type_classification": _create_type_rules(analysis_results.get("type_patterns", {})),
        "encounter_extraction": _create_encounter_rules(analysis_results.get("encounter_patterns", {})),
        "treasure_analysis": _create_treasure_rules(analysis_results.get("treasure_patterns", {})),
        "horror_extraction": _create_horror_rules(analysis_results.get("horror_patterns", {})),
        "challenge_assessment": _create_challenge_rules(analysis_results.get("challenge_patterns", {}))
    }
    
    # Add dungeon-specific quality thresholds
    rules["quality_thresholds"] = {
        "minimum_confidence": 0.75,  # Higher for dungeons due to structured nature
        "high_confidence": 0.9,
        "type_classification_confidence": 0.85,
        "treasure_analysis_confidence": 0.8,
        "horror_extraction_confidence": 0.7
    }
    
    logger.info("Generated extraction rules for all dungeon categories")
    console.print("✅ Extraction rules generated for type, encounters, treasure, horror, challenge")
    
    return rules


def _create_type_rules(type_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create dungeon type classification rules."""
    
    # Analyze type distribution
    type_distribution = {
        dungeon_type: data.get("count", 0)
        for dungeon_type, data in type_patterns.items()
    }
    
    return {
        "dungeon_types": ["crypt", "lair", "temple", "shrine", "tomb", "hideout", "bowel", "caverns"],
        "type_distribution": type_distribution,
        "type_indicators": {
            "crypt": ["burial", "undead", "tomb", "grave", "cemetery"],
            "lair": ["beast", "monster", "creature", "den", "nest"],
            "temple": ["worship", "altar", "shrine", "holy", "sacred"],
            "shrine": ["small temple", "altar", "devotion", "minor"],
            "tomb": ["ancient", "burial", "sarcophagus", "mummy"],
            "hideout": ["bandits", "thieves", "criminals", "secret"],
            "bowel": ["deep", "underground", "pit", "chasm"],
            "caverns": ["cave", "underground", "tunnel", "natural"]
        },
        "extraction_patterns": [
            r'(Crypt|Lair|Temple|Shrine|Tomb|Hideout|Bowel|Caverns?) of',  # Direct type in name
            r'(burial|worship|beast|underground)',  # Content type indicators
            r'(undead|altar|monster|cave)',  # Thematic indicators
        ],
        "name_analysis": {
            "prefix_importance": 0.9,  # Type usually in name prefix
            "content_confirmation": 0.7,  # Content should support type
            "theme_consistency": 0.8  # Theme should match type
        }
    }


def _create_encounter_rules(encounter_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create encounter extraction rules."""
    
    # Analyze encounter density distribution
    densities = [data.get("encounter_density", 0) for data in encounter_patterns.values()]
    avg_density = sum(densities) / max(len(densities), 1)
    
    complexities = [data.get("content_complexity", 0) for data in encounter_patterns.values()]
    avg_complexity = sum(complexities) / max(len(complexities), 1)
    
    return {
        "encounter_types": ["boss", "minion", "trap", "environmental"],
        "average_density": avg_density,
        "average_complexity": avg_complexity,
        "density_thresholds": {
            "sparse": 0,
            "light": avg_density * 0.5,
            "moderate": avg_density,
            "heavy": avg_density * 1.5,
            "dense": avg_density * 2.0
        },
        "extraction_patterns": [
            r'<div id="block-[^"]*" class="monster-block">',  # Monster blocks
            r'CR:\s*(\d+)',  # Challenge ratings
            r'<strong>([^<]+)</strong>.*?CR:',  # Monster names
            r'(\d+)\s*(XP)',  # Experience values
        ],
        "boss_indicators": ["boss", "lord", "master", "chief", "king", "queen"],
        "minion_indicators": ["swarm", "group", "pack", "horde"]
    }


def _create_treasure_rules(treasure_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create treasure analysis rules."""
    
    # Analyze treasure levels
    treasure_levels = [data.get("treasure_level", "none") for data in treasure_patterns.values()]
    level_distribution = {}
    for level in treasure_levels:
        level_distribution[level] = level_distribution.get(level, 0) + 1
    
    # Analyze treasure complexity
    complexities = [data.get("treasure_complexity", 0) for data in treasure_patterns.values()]
    avg_complexity = sum(complexities) / max(len(complexities), 1)
    
    return {
        "treasure_levels": ["none", "basic", "moderate", "rich"],
        "level_distribution": level_distribution,
        "average_complexity": avg_complexity,
        "wealth_categories": {
            "poor": 0,
            "modest": 500,
            "comfortable": 2000,
            "wealthy": 5000,
            "rich": 10000
        },
        "extraction_patterns": [
            r'<strong>([0-9,]+)\s*gp</strong>',  # Coin amounts
            r'Magic Items:</strong>([^<]+)',  # Magic items
            r'gemstones[^:]*:\s*([^<]+)',  # Gemstone lists
            r'artifacts[^:]*:\s*([^<]+)',  # Artifact lists
            r'hoard[^<]*<ul>(.*?)</ul>'  # Hoard contents
        ],
        "treasure_indicators": {
            "coins": ["gp", "gold", "silver", "copper"],
            "gems": ["gemstones", "ruby", "diamond", "emerald", "sapphire"],
            "magic": ["Magic Items", "magical", "enchanted", "artifact"],
            "art": ["artifacts", "statue", "painting", "jewelry"]
        }
    }


def _create_horror_rules(horror_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create horror theme extraction rules."""
    
    # Analyze horror theme distribution
    all_themes = []
    theme_intensities = []
    
    for dungeon_data in horror_patterns.values():
        themes = dungeon_data.get("themes", [])
        all_themes.extend(themes)
        theme_intensities.append(dungeon_data.get("horror_intensity", "none"))
    
    # Count theme frequency
    theme_frequency = {}
    for theme in all_themes:
        theme_frequency[theme] = theme_frequency.get(theme, 0) + 1
    
    # Analyze intensity distribution
    intensity_distribution = {}
    for intensity in theme_intensities:
        intensity_distribution[intensity] = intensity_distribution.get(intensity, 0) + 1
    
    return {
        "horror_themes": [
            "corrupted", "infernal", "cursed", "mourning", "violent",
            "unholy", "burning", "bleeding", "foresaken", "unspoken",
            "raging", "grey", "defiled"
        ],
        "theme_frequency": theme_frequency,
        "intensity_levels": ["none", "low", "moderate", "high", "extreme"],
        "intensity_distribution": intensity_distribution,
        "extraction_patterns": [
            r'(Crypt|Lair|Temple|Shrine|Tomb|Hideout|Bowel|Caverns?) of the ([^<]+)',  # Horror descriptor
            r'(corrupted|infernal|cursed|violent|unholy)',  # Direct theme words
            r'(mourning|burning|bleeding|foresaken)',  # Emotional themes
            r'(raging|grey|defiled|unspoken)',  # Abstract themes
        ],
        "theme_classification": {
            "corruption": ["corrupted", "defiled", "cursed"],
            "violence": ["violent", "raging", "bleeding"],
            "supernatural": ["infernal", "unholy", "unspoken"],
            "emotional": ["mourning", "foresaken", "grey"]
        }
    }


def _create_challenge_rules(challenge_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create challenge rating assessment rules."""
    
    # Analyze challenge distribution
    challenge_ratings = [data.get("challenge_rating", "unknown") for data in challenge_patterns.values()]
    difficulty_levels = [data.get("difficulty_level", "unknown") for data in challenge_patterns.values()]
    
    cr_distribution = {}
    for cr in challenge_ratings:
        cr_distribution[cr] = cr_distribution.get(cr, 0) + 1
    
    difficulty_distribution = {}
    for difficulty in difficulty_levels:
        difficulty_distribution[difficulty] = difficulty_distribution.get(difficulty, 0) + 1
    
    return {
        "challenge_levels": ["easy", "medium", "hard", "deadly"],
        "difficulty_levels": ["beginner", "intermediate", "advanced", "expert"],
        "cr_distribution": cr_distribution,
        "difficulty_distribution": difficulty_distribution,
        "cr_indicators": {
            "easy": ["CR: 1", "CR: 2", "low level"],
            "medium": ["CR: 3", "CR: 4", "CR: 5"],
            "hard": ["CR: 6", "CR: 7", "CR: 8", "CR: 9"],
            "deadly": ["CR: 10", "CR: 1[1-9]", "CR: 2[0-9]"]  # CR 10+
        },
        "extraction_patterns": [
            r'CR:\s*(\d+)',  # Direct CR mentions
            r'(\d+)\s*XP',  # Experience point values
            r'level (\d+)',  # Character level requirements
        ]
    }


def _summarize_dungeon_patterns(patterns: dict[str, Any]) -> dict[str, Any]:
    """Summarize key patterns across all dungeon categories."""
    
    summary = {}
    
    # Type distribution summary
    type_data = patterns.get("type_patterns", {})
    summary["type_insights"] = {
        "type_distribution": {dtype: data.get("count", 0) for dtype, data in type_data.items()},
        "most_common_type": max(type_data.items(), key=lambda x: x[1].get("count", 0))[0] if type_data else "unknown",
        "type_variety": len(type_data)
    }
    
    # Encounter complexity summary
    encounter_data = patterns.get("encounter_patterns", {})
    if encounter_data:
        densities = [data.get("encounter_density", 0) for data in encounter_data.values()]
        complexities = [data.get("content_complexity", 0) for data in encounter_data.values()]
        
        summary["encounter_insights"] = {
            "average_encounter_density": sum(densities) / max(len(densities), 1),
            "average_content_complexity": sum(complexities) / max(len(complexities), 1),
            "high_density_dungeons": [
                name for name, data in encounter_data.items()
                if data.get("encounter_density", 0) > 5
            ]
        }
    
    # Treasure analysis summary
    treasure_data = patterns.get("treasure_patterns", {})
    if treasure_data:
        wealth_levels = [data.get("treasure_level", "none") for data in treasure_data.values()]
        wealth_distribution = {}
        for level in wealth_levels:
            wealth_distribution[level] = wealth_distribution.get(level, 0) + 1
        
        summary["treasure_insights"] = {
            "wealth_distribution": wealth_distribution,
            "wealthy_dungeons": [
                name for name, data in treasure_data.items()
                if data.get("treasure_level") in ["moderate", "rich"]
            ]
        }
    
    # Horror theme summary
    horror_data = patterns.get("horror_patterns", {})
    if horror_data:
        all_themes = []
        intensities = []
        
        for dungeon_data in horror_data.values():
            all_themes.extend(dungeon_data.get("themes", []))
            intensities.append(dungeon_data.get("horror_intensity", "none"))
        
        theme_frequency = {}
        for theme in all_themes:
            theme_frequency[theme] = theme_frequency.get(theme, 0) + 1
        
        intensity_dist = {}
        for intensity in intensities:
            intensity_dist[intensity] = intensity_dist.get(intensity, 0) + 1
        
        summary["horror_insights"] = {
            "total_horror_themes": len(set(all_themes)),
            "most_common_themes": sorted(theme_frequency.items(), key=lambda x: x[1], reverse=True)[:5],
            "intensity_distribution": intensity_dist,
            "average_theme_count": len(all_themes) / max(len(horror_data), 1)
        }
    
    return summary


def _save_dungeon_patterns(extraction_rules: dict[str, Any], analysis_results: dict[str, Any]) -> bool:
    """Save learned dungeon patterns for future use."""
    
    patterns_dir = Path("training") / "dungeons"
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
        _create_dungeon_training_guide(patterns_dir, analysis_results)
        
        return True
        
    except Exception as e:
        logging.error(f"Failed to save dungeon patterns: {e}")
        return False


def _create_dungeon_training_guide(patterns_dir: Path, analysis_results: dict[str, Any]) -> None:
    """Create dungeon-specific training guide."""
    
    summary = analysis_results.get("summary", {})
    
    guide_content = f"""# Dungeon ML Training Guide - Organized Data Analysis

## Dungeon Training Data Summary

### Type Distribution
{_format_type_summary(summary)}

### Encounter Patterns
{_format_encounter_summary(summary)}

### Treasure Analysis
{_format_treasure_summary(summary)}

### Horror Themes
{_format_horror_summary(summary)}

## Content Extraction Strategy

### 1. Type Classification
- **Name Analysis**: Extract type from "X of the Y" patterns in names
- **Content Validation**: Verify type through content themes and encounters
- **Thematic Consistency**: Ensure type matches horror themes and encounters

### 2. Encounter Analysis
- **Monster Block Detection**: Parse structured monster stat blocks
- **Challenge Rating Extraction**: Extract CR values and XP rewards
- **Boss vs Minion Classification**: Identify primary threats vs support creatures

### 3. Treasure Assessment
- **Wealth Quantification**: Extract coin amounts, gem values, item counts
- **Magic Item Detection**: Identify magical rewards and artifacts
- **Treasure Tier Classification**: Assess overall wealth level (none/basic/moderate/rich)

### 4. Horror Theme Extraction
- **Name Theme Analysis**: Extract horror descriptors from dungeon names
- **Content Theme Mining**: Find horror elements in descriptions and encounters
- **Intensity Assessment**: Classify horror intensity from theme types and frequency

## Training Patterns

### Dungeon Type Examples
- **Crypt**: Burial sites with undead, "Crypt of [Horror Theme]"
- **Lair**: Monster lairs with beast encounters, treasure hoards
- **Temple**: Religious sites with altar themes, faction connections

### Horror Theme Taxonomy
- **Corruption**: "Corrupted", "Defiled", "Cursed" 
- **Violence**: "Violent", "Raging", "Bleeding"
- **Supernatural**: "Infernal", "Unholy", "Unspoken"
- **Emotional**: "Mourning", "Foresaken", "Grey"

### Challenge Rating Patterns
- Dungeons range from CR 1 (easy) to CR 10+ (deadly)
- Boss encounters typically CR 5-8 for major content
- Multiple encounter types per dungeon (boss + minions + environmental)

## Training Confidence

- **Data Source**: 18 organized dungeon examples with massive content (2,780+ entities for largest)
- **Type Coverage**: All major dungeon types represented with clear examples
- **Content Richness**: Full encounter tables, treasure hoards, horror themes
- **Pattern Reliability**: Strong naming conventions and thematic consistency

Generated from HBF worldbuilding breakthrough organized data.
"""
    
    try:
        with open(patterns_dir / "training_guide.md", "w", encoding="utf-8") as f:
            f.write(guide_content)
    except Exception as e:
        logging.error(f"Failed to create dungeon training guide: {e}")


def _format_type_summary(summary: dict[str, Any]) -> str:
    """Format dungeon type distribution summary."""
    
    type_insights = summary.get("type_insights", {})
    distribution = type_insights.get("type_distribution", {})
    
    return f"""
- **Type Distribution**: {distribution}
- **Most Common Type**: {type_insights.get('most_common_type', 'unknown')}
- **Type Variety**: {type_insights.get('type_variety', 0)} different dungeon types
"""


def _format_encounter_summary(summary: dict[str, Any]) -> str:
    """Format encounter patterns summary."""
    
    encounter_insights = summary.get("encounter_insights", {})
    
    return f"""
- **Average Encounter Density**: {encounter_insights.get('average_encounter_density', 0):.1f}
- **Average Content Complexity**: {encounter_insights.get('average_content_complexity', 0):.1f}
- **High Density Dungeons**: {', '.join(encounter_insights.get('high_density_dungeons', [])[:3])}
"""


def _format_treasure_summary(summary: dict[str, Any]) -> str:
    """Format treasure analysis summary."""
    
    treasure_insights = summary.get("treasure_insights", {})
    distribution = treasure_insights.get("wealth_distribution", {})
    
    return f"""
- **Wealth Distribution**: {distribution}
- **Wealthy Dungeons**: {', '.join(treasure_insights.get('wealthy_dungeons', [])[:3])}
- **Treasure Complexity**: Coins + gems + magic items + artifacts
"""


def _format_horror_summary(summary: dict[str, Any]) -> str:
    """Format horror theme analysis summary."""
    
    horror_insights = summary.get("horror_insights", {})
    most_common = horror_insights.get("most_common_themes", [])
    
    return f"""
- **Total Horror Themes**: {horror_insights.get('total_horror_themes', 0)}
- **Most Common Themes**: {', '.join([theme for theme, count in most_common[:3]])}
- **Intensity Distribution**: {horror_insights.get('intensity_distribution', {})}
- **Average Theme Count**: {horror_insights.get('average_theme_count', 0):.1f}
"""


def extract_dungeon_content(entity_content: str, learned_rules: dict[str, Any] | None = None) -> dict[str, Any]:
    """
    Extract dungeon content using learned patterns.
    
    Args:
        entity_content: Raw entity content to analyze
        learned_rules: Optional pre-learned extraction rules
        
    Returns:
        Extracted dungeon data with confidence scoring
    """
    
    if not learned_rules:
        # Load saved patterns
        patterns_file = Path("training") / "dungeons" / "extraction_rules.json"
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
    extracted_data = extract_dungeon_data(entity_content)
    
    # Add learned rule enhancements
    if learned_rules.get("type_classification"):
        type_analysis = _apply_type_classification(entity_content, learned_rules["type_classification"])
        extracted_data["type_analysis"] = type_analysis
    
    # Add encounter analysis
    if learned_rules.get("encounter_extraction"):
        encounter_analysis = _apply_encounter_analysis(entity_content, learned_rules["encounter_extraction"])
        extracted_data["encounter_analysis"] = encounter_analysis
    
    # Add treasure analysis
    if learned_rules.get("treasure_analysis"):
        treasure_analysis = _apply_treasure_analysis(entity_content, learned_rules["treasure_analysis"])
        extracted_data["treasure_analysis"] = treasure_analysis
    
    # Add horror analysis
    if learned_rules.get("horror_extraction"):
        horror_analysis = _apply_horror_analysis(entity_content, learned_rules["horror_extraction"])
        extracted_data["horror_analysis"] = horror_analysis
    
    # Calculate overall confidence
    extracted_data["confidence_score"] = validate_extraction_quality(extracted_data)
    extracted_data["category"] = "dungeon"
    extracted_data["extraction_method"] = "learned_patterns"
    
    return extracted_data


def _apply_type_classification(content: str, type_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply dungeon type classification analysis."""
    
    analysis = {
        "predicted_type": "unknown",
        "type_confidence": 0.0,
        "supporting_evidence": [],
        "name_based_type": None
    }
    
    # Analyze name for type indicators
    type_indicators = type_rules.get("type_indicators", {})
    type_scores = {}
    
    for dungeon_type, indicators in type_indicators.items():
        score = 0
        evidence = []
        
        # Check type in name (highest weight)
        if dungeon_type.title() in content:
            score += 0.9
            evidence.append(f"name_contains_{dungeon_type}")
        
        # Check content indicators
        for indicator in indicators:
            if indicator.lower() in content.lower():
                score += 0.1
                evidence.append(f"content_{indicator}")
        
        if score > 0:
            type_scores[dungeon_type] = {"score": min(score, 1.0), "evidence": evidence}
    
    # Determine best type match
    if type_scores:
        best_type = max(type_scores.items(), key=lambda x: x[1]["score"])
        analysis["predicted_type"] = best_type[0]
        analysis["type_confidence"] = best_type[1]["score"]
        analysis["supporting_evidence"] = best_type[1]["evidence"]
    
    return analysis


def _apply_encounter_analysis(content: str, encounter_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply encounter extraction and analysis."""
    
    analysis = {
        "encounter_count": 0,
        "boss_encounters": 0,
        "minion_encounters": 0,
        "max_cr": 0,
        "total_xp": 0,
        "density_classification": "sparse"
    }
    
    # Count monster blocks
    import re
    monster_blocks = re.findall(r'<div id="block-[^"]*" class="monster-block">', content)
    analysis["encounter_count"] = len(monster_blocks)
    
    # Extract challenge ratings
    cr_matches = re.findall(r'CR:\s*(\d+)', content)
    if cr_matches:
        crs = [int(cr) for cr in cr_matches]
        analysis["max_cr"] = max(crs)
    
    # Extract XP values
    xp_matches = re.findall(r'(\d+)\s*XP', content)
    if xp_matches:
        analysis["total_xp"] = sum(int(xp) for xp in xp_matches)
    
    # Classify encounter density
    thresholds = encounter_rules.get("density_thresholds", {})
    encounter_count = analysis["encounter_count"]
    
    if encounter_count >= thresholds.get("dense", 10):
        analysis["density_classification"] = "dense"
    elif encounter_count >= thresholds.get("heavy", 5):
        analysis["density_classification"] = "heavy" 
    elif encounter_count >= thresholds.get("moderate", 2):
        analysis["density_classification"] = "moderate"
    elif encounter_count >= thresholds.get("light", 1):
        analysis["density_classification"] = "light"
    
    return analysis


def _apply_treasure_analysis(content: str, treasure_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply treasure extraction and analysis."""
    
    analysis = {
        "total_gold": 0,
        "magic_item_count": 0,
        "gemstone_count": 0,
        "artifact_count": 0,
        "wealth_classification": "poor"
    }
    
    # Extract coin amounts
    import re
    coin_matches = re.findall(r'<strong>([0-9,]+)\s*gp</strong>', content)
    if coin_matches:
        total_gold = sum(int(match.replace(",", "")) for match in coin_matches)
        analysis["total_gold"] = total_gold
    
    # Count magic items
    magic_mentions = content.count("Magic Items")
    analysis["magic_item_count"] = magic_mentions
    
    # Count gemstones
    gemstone_mentions = content.count("gemstones")
    analysis["gemstone_count"] = gemstone_mentions
    
    # Count artifacts
    artifact_mentions = content.count("artifacts")
    analysis["artifact_count"] = artifact_mentions
    
    # Classify wealth level
    wealth_categories = treasure_rules.get("wealth_categories", {})
    total_gold = analysis["total_gold"]
    
    if total_gold >= wealth_categories.get("rich", 10000):
        analysis["wealth_classification"] = "rich"
    elif total_gold >= wealth_categories.get("wealthy", 5000):
        analysis["wealth_classification"] = "wealthy"
    elif total_gold >= wealth_categories.get("comfortable", 2000):
        analysis["wealth_classification"] = "comfortable"
    elif total_gold >= wealth_categories.get("modest", 500):
        analysis["wealth_classification"] = "modest"
    
    return analysis


def _apply_horror_analysis(content: str, horror_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply horror theme extraction and analysis."""
    
    analysis = {
        "detected_themes": [],
        "horror_intensity": "none",
        "theme_categories": {},
        "horror_confidence": 0.0
    }
    
    # Extract horror themes
    horror_themes = horror_rules.get("horror_themes", [])
    content_lower = content.lower()
    
    for theme in horror_themes:
        if theme in content_lower:
            analysis["detected_themes"].append(theme)
    
    # Categorize themes
    theme_classification = horror_rules.get("theme_classification", {})
    for category, category_themes in theme_classification.items():
        analysis["theme_categories"][category] = [
            theme for theme in analysis["detected_themes"]
            if theme in category_themes
        ]
    
    # Assess horror intensity
    theme_count = len(analysis["detected_themes"])
    if theme_count >= 4:
        analysis["horror_intensity"] = "extreme"
    elif theme_count >= 3:
        analysis["horror_intensity"] = "high"
    elif theme_count >= 2:
        analysis["horror_intensity"] = "moderate"
    elif theme_count >= 1:
        analysis["horror_intensity"] = "low"
    
    # Calculate confidence
    analysis["horror_confidence"] = min(theme_count * 0.25, 1.0)
    
    return analysis
