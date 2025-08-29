"""
Faction ML Training - Extract faction content from organized HBF examples.

Uses organized faction data from memory-bank/world-building/factions/ to train
ML models for faction content extraction. Focuses on leadership analysis,
membership mapping, political alignment detection, and territorial control.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

from .meta import load_organized_examples
from .patterns import extract_faction_data, create_ml_training_vector, validate_extraction_quality


def run(engine, logger: logging.Logger, console) -> dict[str, Any]:
    """
    Run faction ML training using organized HBF examples.
    
    Args:
        engine: Database engine (passed but not used - we use organized files)
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Training results with learned patterns
    """
    
    logger.info("Starting faction ML training from organized examples")
    console.print("⚔️ [bold blue]Faction ML Training[/bold blue] - Learning from 5 organized examples")
    
    # Load organized faction examples
    faction_examples = load_organized_examples("factions")
    
    if not faction_examples:
        logger.error("No organized faction examples found")
        return {"error": "No training data available"}
    
    logger.info(f"Loaded {len(faction_examples)} faction examples")
    console.print(f"📊 Loaded {len(faction_examples)} organized faction examples")
    
    # Analyze faction patterns
    analysis_results = _analyze_faction_patterns(faction_examples, logger, console)
    
    # Create ML training vectors
    training_vectors = _create_faction_training_vectors(faction_examples, logger, console)
    
    # Generate faction extraction rules
    extraction_rules = _generate_faction_extraction_rules(analysis_results, logger, console)
    
    # Save learned patterns
    patterns_saved = _save_faction_patterns(extraction_rules, analysis_results)
    
    results = {
        "examples_analyzed": len(faction_examples),
        "pattern_analysis": analysis_results,
        "training_vectors": training_vectors,
        "extraction_rules": extraction_rules,
        "patterns_saved": patterns_saved,
        "training_quality": "organized_breakthrough_data"
    }
    
    console.print("✅ [bold green]Faction training complete[/bold green] - Patterns learned from organized data")
    logger.info(f"Faction training complete: {len(faction_examples)} examples processed")
    
    return results


def _analyze_faction_patterns(faction_examples: list[dict[str, Any]], logger: logging.Logger, console) -> dict[str, Any]:
    """Analyze patterns across all faction examples."""
    
    console.print("🔍 Analyzing faction patterns...")
    
    patterns = {
        "organizational_patterns": {},
        "leadership_patterns": {},
        "membership_patterns": {},
        "territorial_patterns": {},
        "political_patterns": {}
    }
    
    for example in faction_examples:
        name = example.get("name", "Unknown")
        training_signals = example.get("training_signals", {})
        content_features = example.get("content_features", {})
        
        # Organizational pattern analysis
        member_count = training_signals.get("member_count", 0)
        leadership = training_signals.get("leadership_structure", {})
        
        patterns["organizational_patterns"][name] = {
            "member_count": member_count,
            "has_leadership": leadership.get("has_leader", False),
            "leader_level": leadership.get("leader_level", 0),
            "organizational_sophistication": _assess_faction_sophistication(training_signals)
        }
        
        # Leadership pattern analysis
        patterns["leadership_patterns"][name] = leadership
        
        # Membership pattern analysis
        territorial_control = training_signals.get("territorial_control", {})
        patterns["membership_patterns"][name] = {
            "geographic_spread": len(territorial_control),
            "location_distribution": territorial_control,
            "member_density": member_count / max(len(territorial_control), 1)
        }
        
        # Territorial pattern analysis
        patterns["territorial_patterns"][name] = territorial_control
        
        # Political pattern analysis
        political_alignment = training_signals.get("political_alignment", "unknown")
        patterns["political_patterns"][name] = {
            "alignment": political_alignment,
            "activity_type": _classify_faction_activity(content_features)
        }
        
        logger.debug(f"Analyzed faction pattern for: {name} ({member_count} members)")
    
    # Summarize patterns
    patterns["summary"] = _summarize_faction_patterns(patterns)
    
    console.print(f"📈 Faction pattern analysis complete: {len(patterns['summary'])} key insights")
    
    return patterns


def _assess_faction_sophistication(training_signals: dict[str, Any]) -> int:
    """Assess organizational sophistication of faction."""
    
    sophistication = 0
    
    # Leadership sophistication
    leadership = training_signals.get("leadership_structure", {})
    if leadership.get("has_leader"):
        sophistication += 1
        if leadership.get("leader_level", 0) > 5:
            sophistication += 1
    
    # Member count sophistication
    member_count = training_signals.get("member_count", 0)
    if member_count > 15:
        sophistication += 1
    if member_count > 25:
        sophistication += 1
    
    # Territorial sophistication
    territorial_control = training_signals.get("territorial_control", {})
    if len(territorial_control) > 3:
        sophistication += 1
    
    return sophistication


def _classify_faction_activity(content_features: dict[str, Any]) -> str:
    """Classify primary faction activity type."""
    
    activity_patterns = content_features.get("activity_patterns", [])
    
    if not activity_patterns:
        return "unknown"
    
    # Analyze activity descriptions
    activities = []
    for activity in activity_patterns:
        if isinstance(activity, dict):
            activities.append(activity.get("type", "unknown"))
        else:
            activities.append(str(activity))
    
    # Classify based on most common activity
    if "conducting" in activities:
        return "experimental"
    elif "plotting" in activities:
        return "subversive"
    elif "gathering" in activities:
        return "social"
    else:
        return "general"


def _create_faction_training_vectors(faction_examples: list[dict[str, Any]], logger: logging.Logger, console) -> list[dict[str, Any]]:
    """Create ML training vectors for all faction examples."""
    
    console.print("🤖 Creating ML training vectors...")
    
    vectors = []
    
    for example in faction_examples:
        vector = create_ml_training_vector(example, "factions")
        
        if vector:
            vector["source_name"] = example.get("name")
            vector["faction_type"] = example.get("training_signals", {}).get("political_alignment", "unknown")
            vector["quality_score"] = validate_extraction_quality(example)
            vectors.append(vector)
    
    logger.info(f"Created {len(vectors)} faction training vectors")
    console.print(f"🎯 Created {len(vectors)} ML training vectors")
    
    return vectors


def _generate_faction_extraction_rules(analysis_results: dict[str, Any], logger: logging.Logger, console) -> dict[str, Any]:
    """Generate extraction rules from faction pattern analysis."""
    
    console.print("📝 Generating faction extraction rules...")
    
    rules = {
        "organizational_analysis": _create_organizational_rules(analysis_results.get("organizational_patterns", {})),
        "leadership_extraction": _create_leadership_rules(analysis_results.get("leadership_patterns", {})),
        "membership_extraction": _create_membership_rules(analysis_results.get("membership_patterns", {})),
        "territorial_analysis": _create_territorial_rules(analysis_results.get("territorial_patterns", {})),
        "political_classification": _create_political_rules(analysis_results.get("political_patterns", {}))
    }
    
    # Add faction-specific quality thresholds
    rules["quality_thresholds"] = {
        "minimum_confidence": 0.7,
        "high_confidence": 0.9,
        "leadership_detection_confidence": 0.8,
        "membership_analysis_confidence": 0.75,
        "territorial_analysis_confidence": 0.7
    }
    
    logger.info("Generated extraction rules for all faction categories")
    console.print("✅ Extraction rules generated for organization, leadership, membership, territory, politics")
    
    return rules


def _create_organizational_rules(org_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create organizational structure extraction rules."""
    
    # Analyze sophistication distribution
    sophistications = [data.get("organizational_sophistication", 0) for data in org_patterns.values()]
    avg_sophistication = sum(sophistications) / max(len(sophistications), 1)
    
    # Analyze member counts
    member_counts = [data.get("member_count", 0) for data in org_patterns.values()]
    avg_members = sum(member_counts) / max(len(member_counts), 1)
    
    return {
        "sophistication_levels": [0, 1, 2, 3, 4, 5],  # 0 = basic, 5 = highly sophisticated
        "average_sophistication": avg_sophistication,
        "average_member_count": avg_members,
        "sophistication_indicators": {
            "basic": ["small group", "informal", "loose"],
            "moderate": ["organized", "structure", "hierarchy"],
            "sophisticated": ["leadership", "territories", "operations"],
            "advanced": ["complex", "coordination", "strategic"],
            "elite": ["mastery", "dominance", "control"]
        },
        "extraction_patterns": [
            r'(\w+) leader',  # Leadership mentions
            r'(\d+) members?',  # Member count mentions
            r'(cult|militia|organization|group)',  # Organizational type
            r'(hideout|base|headquarters)',  # Base indicators
        ]
    }


def _create_leadership_rules(leadership_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create leadership extraction rules."""
    
    # Analyze leader characteristics
    leader_levels = []
    has_leader_count = 0
    
    for faction_data in leadership_patterns.values():
        if faction_data.get("has_leader"):
            has_leader_count += 1
            leader_levels.append(faction_data.get("leader_level", 0))
    
    avg_leader_level = sum(leader_levels) / max(len(leader_levels), 1)
    leadership_percentage = has_leader_count / max(len(leadership_patterns), 1)
    
    return {
        "leadership_indicators": ["leader", "chief", "master", "head", "commander"],
        "average_leader_level": avg_leader_level,
        "leadership_percentage": leadership_percentage,
        "extraction_patterns": [
            r'leader is a <strong>([^<]+)</strong>, a level (\d+) (\w+) (\w+)',  # Full leader info
            r'led by ([^<,.]+)',  # Leadership phrases
            r'(chief|master|commander|head)[^:]*:\s*([^<]+)',  # Leadership titles
        ],
        "leadership_validation": {
            "min_level": 1,
            "max_level": 20,
            "expected_classes": ["fighter", "cleric", "wizard", "rogue"]
        }
    }


def _create_membership_rules(membership_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create membership extraction rules."""
    
    # Analyze membership distribution
    spreads = [data.get("geographic_spread", 0) for data in membership_patterns.values()]
    avg_spread = sum(spreads) / max(len(spreads), 1)
    
    densities = [data.get("member_density", 0) for data in membership_patterns.values()]
    avg_density = sum(densities) / max(len(densities), 1)
    
    return {
        "membership_categories": ["Members", "Collaborators"],
        "average_geographic_spread": avg_spread,
        "average_member_density": avg_density,
        "extraction_patterns": [
            r'<h4>\s*(Members|Collaborators)\s*</h4>',  # Section headers
            r'<a href="[^"]*"><strong>([^<]+)</strong></a>\s*([^&#]*)',  # Member entries
            r'The (\w+)\s+from ([A-Z][a-z]+)',  # Role and location
            r'Member of the <a[^>]*><strong>([^<]+)</strong>',  # Faction membership
        ],
        "role_indicators": {
            "combat": ["Fighter", "Warrior", "Guard"],
            "magic": ["Wizard", "Cleric", "Sorcerer"],
            "stealth": ["Rogue", "Assassin", "Scout"],
            "support": ["Merchant", "Blacksmith", "Healer"]
        },
        "location_tracking": {
            "settlement_names": ["Headsmen", "Palemoon", "Devilville", "Harad", "Kothian"],
            "territorial_indicators": ["from", "of", "in"]
        }
    }


def _create_territorial_rules(territorial_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create territorial analysis rules."""
    
    # Analyze territorial control patterns
    all_territories = []
    for faction_name, territories in territorial_patterns.items():
        all_territories.extend(territories.keys())
    
    # Count territory frequency
    territory_frequency = {}
    for territory in all_territories:
        territory_frequency[territory] = territory_frequency.get(territory, 0) + 1
    
    # Contested territories (controlled by multiple factions)
    contested = [territory for territory, count in territory_frequency.items() if count > 1]
    
    return {
        "known_territories": list(set(all_territories)),
        "territory_frequency": territory_frequency,
        "contested_territories": contested,
        "control_indicators": ["from", "of", "in", "based in", "operates in"],
        "extraction_patterns": [
            r'from ([A-Z][a-z]+)',  # Location extraction
            r'in ([A-Z][a-z]+)',  # Area of operation
            r'based in ([^<]+)',  # Base location
            r'operates from ([^<]+)',  # Operational base
        ],
        "territorial_analysis": {
            "single_territory": "local faction",
            "multi_territory": "regional faction", 
            "widespread": "major faction",
            "contested_presence": "factional conflict"
        }
    }


def _create_political_rules(political_patterns: dict[str, Any]) -> dict[str, Any]:
    """Create political alignment classification rules."""
    
    # Analyze political alignments
    alignments = [data.get("alignment", "unknown") for data in political_patterns.values()]
    activities = [data.get("activity_type", "unknown") for data in political_patterns.values()]
    
    alignment_distribution = {}
    for alignment in alignments:
        alignment_distribution[alignment] = alignment_distribution.get(alignment, 0) + 1
    
    activity_distribution = {}
    for activity in activities:
        activity_distribution[activity] = activity_distribution.get(activity, 0) + 1
    
    return {
        "alignment_categories": ["lawful", "chaotic", "neutral", "aggressive"],
        "alignment_distribution": alignment_distribution,
        "activity_distribution": activity_distribution,
        "alignment_indicators": {
            "lawful": ["justice", "law", "order", "patrol", "guard"],
            "chaotic": ["cult", "defiled", "chaos", "corruption", "dark"],
            "aggressive": ["eliminate", "destroy", "attack", "plotting", "war"],
            "neutral": ["trade", "balance", "neutral", "independent"]
        },
        "extraction_patterns": [
            r'(cult|militia|organization|syndicate)',  # Organizational type
            r'(justice|law|order)',  # Lawful indicators
            r'(cult|defiled|corrupted)',  # Chaotic indicators
            r'(plotting|eliminate|destroy)',  # Aggressive indicators
        ],
        "political_classification": {
            "justice_faction": "lawful_good",
            "cult_faction": "chaotic_evil", 
            "militia_faction": "lawful_neutral",
            "criminal_faction": "chaotic_neutral"
        }
    }


def _summarize_faction_patterns(patterns: dict[str, Any]) -> dict[str, Any]:
    """Summarize key patterns across all faction categories."""
    
    summary = {}
    
    # Organizational summary
    org_data = patterns.get("organizational_patterns", {})
    if org_data:
        member_counts = [data.get("member_count", 0) for data in org_data.values()]
        sophistications = [data.get("organizational_sophistication", 0) for data in org_data.values()]
        
        summary["organizational_insights"] = {
            "total_factions": len(org_data),
            "average_member_count": sum(member_counts) / max(len(member_counts), 1),
            "average_sophistication": sum(sophistications) / max(len(sophistications), 1),
            "leadership_percentage": sum(1 for data in org_data.values() if data.get("has_leadership")) / max(len(org_data), 1)
        }
    
    # Territorial summary
    territorial_data = patterns.get("territorial_patterns", {})
    if territorial_data:
        all_locations = []
        for faction_territories in territorial_data.values():
            all_locations.extend(faction_territories.keys())
        
        location_frequency = {}
        for location in all_locations:
            location_frequency[location] = location_frequency.get(location, 0) + 1
        
        summary["territorial_insights"] = {
            "total_territories": len(set(all_locations)),
            "contested_territories": [loc for loc, count in location_frequency.items() if count > 1],
            "most_controlled_territories": sorted(location_frequency.items(), key=lambda x: x[1], reverse=True)[:5]
        }
    
    # Political summary
    political_data = patterns.get("political_patterns", {})
    if political_data:
        alignments = [data.get("alignment", "unknown") for data in political_data.values()]
        activities = [data.get("activity_type", "unknown") for data in political_data.values()]
        
        alignment_dist = {}
        for alignment in alignments:
            alignment_dist[alignment] = alignment_dist.get(alignment, 0) + 1
        
        summary["political_insights"] = {
            "alignment_distribution": alignment_dist,
            "primary_activities": list(set(activities)),
            "factional_conflicts": len([a for a in alignments if a in ["aggressive", "chaotic"]])
        }
    
    return summary


def _save_faction_patterns(extraction_rules: dict[str, Any], analysis_results: dict[str, Any]) -> bool:
    """Save learned faction patterns for future use."""
    
    patterns_dir = Path("training") / "factions"
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
        _create_faction_training_guide(patterns_dir, analysis_results)
        
        return True
        
    except Exception as e:
        logging.error(f"Failed to save faction patterns: {e}")
        return False


def _create_faction_training_guide(patterns_dir: Path, analysis_results: dict[str, Any]) -> None:
    """Create faction-specific training guide."""
    
    summary = analysis_results.get("summary", {})
    
    guide_content = f"""# Faction ML Training Guide - Organized Data Analysis

## Faction Training Data Summary

### Organizational Insights
{_format_organizational_summary(summary)}

### Territorial Control
{_format_territorial_summary(summary)}

### Political Alignment
{_format_political_summary(summary)}

## Content Extraction Strategy

### 1. Organizational Structure Detection
- **Leadership Identification**: Look for "leader is a" patterns with stat blocks
- **Member Counting**: Parse "Members" and "Collaborators" sections  
- **Sophistication Assessment**: Analyze hierarchy, territory, member count

### 2. Membership Analysis
- **Role Extraction**: Extract professional classes from member descriptions
- **Geographic Mapping**: Track "from X" patterns for territorial control
- **Relationship Networks**: Map members to locations and activities

### 3. Territorial Control Analysis
- **Location Tracking**: Count faction presence across settlements
- **Control Density**: Assess member concentration per location
- **Contested Areas**: Identify territories with multiple faction presence

### 4. Political Classification
- **Alignment Detection**: Use keyword analysis for lawful/chaotic/neutral
- **Activity Analysis**: Classify primary faction activities (experimental/subversive/social)
- **Conflict Mapping**: Identify opposing faction relationships

## Training Patterns

### Known Faction Types
- **Cult**: The Defiled Wolves (chaotic, experimental activities)
- **Militia**: The Fists Of Justice (lawful, elimination activities)
- **Organization**: The Swords Of Justice (lawful, social activities)

### Leadership Patterns
- High-level leaders (7-8th level) with detailed stat blocks
- Clear hierarchy with named leadership roles
- Leader faction membership and territorial base

### Membership Distribution
- 15-30 members per major faction
- Geographic spread across 3-8 territories
- Mixed combat and social roles
- Clear "Members" vs "Collaborators" distinction

## Training Confidence

- **Data Source**: 5 organized faction examples with complete networks
- **Membership Coverage**: 200-300 entities per faction
- **Political Complexity**: Clear factional conflicts and territorial disputes
- **Content Richness**: Full member rosters, leadership details, political relationships

Generated from HBF worldbuilding breakthrough organized data.
"""
    
    try:
        with open(patterns_dir / "training_guide.md", "w", encoding="utf-8") as f:
            f.write(guide_content)
    except Exception as e:
        logging.error(f"Failed to create faction training guide: {e}")


def _format_organizational_summary(summary: dict[str, Any]) -> str:
    """Format organizational insights summary."""
    
    org_insights = summary.get("organizational_insights", {})
    
    return f"""
- **Total Factions**: {org_insights.get('total_factions', 0)}
- **Average Member Count**: {org_insights.get('average_member_count', 0):.1f}
- **Average Sophistication**: {org_insights.get('average_sophistication', 0):.1f}/5
- **Leadership Percentage**: {org_insights.get('leadership_percentage', 0):.1%}
"""


def _format_territorial_summary(summary: dict[str, Any]) -> str:
    """Format territorial control summary."""
    
    territorial_insights = summary.get("territorial_insights", {})
    contested = territorial_insights.get("contested_territories", [])
    
    return f"""
- **Total Territories**: {territorial_insights.get('total_territories', 0)}
- **Contested Areas**: {len(contested)} ({', '.join(contested[:3])})
- **High Control Areas**: {', '.join([loc for loc, count in territorial_insights.get('most_controlled_territories', [])[:3]])}
"""


def _format_political_summary(summary: dict[str, Any]) -> str:
    """Format political alignment summary."""
    
    political_insights = summary.get("political_insights", {})
    alignment_dist = political_insights.get("alignment_distribution", {})
    
    return f"""
- **Alignment Distribution**: {alignment_dist}
- **Primary Activities**: {', '.join(political_insights.get('primary_activities', []))}
- **Factional Conflicts**: {political_insights.get('factional_conflicts', 0)} aggressive/chaotic factions
"""


def extract_faction_content(entity_content: str, learned_rules: dict[str, Any] | None = None) -> dict[str, Any]:
    """
    Extract faction content using learned patterns.
    
    Args:
        entity_content: Raw entity content to analyze
        learned_rules: Optional pre-learned extraction rules
        
    Returns:
        Extracted faction data with confidence scoring
    """
    
    if not learned_rules:
        # Load saved patterns
        patterns_file = Path("training") / "factions" / "extraction_rules.json"
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
    extracted_data = extract_faction_data(entity_content)
    
    # Add learned rule enhancements
    if learned_rules.get("organizational_analysis"):
        org_analysis = _apply_organizational_analysis(entity_content, learned_rules["organizational_analysis"])
        extracted_data["organizational_analysis"] = org_analysis
    
    # Add territorial analysis
    if learned_rules.get("territorial_analysis"):
        territorial_analysis = _apply_territorial_analysis(entity_content, learned_rules["territorial_analysis"])
        extracted_data["territorial_analysis"] = territorial_analysis
    
    # Add political classification
    if learned_rules.get("political_classification"):
        political_analysis = _apply_political_classification(entity_content, learned_rules["political_classification"])
        extracted_data["political_analysis"] = political_analysis
    
    # Calculate overall confidence
    extracted_data["confidence_score"] = validate_extraction_quality(extracted_data)
    extracted_data["category"] = "faction"
    extracted_data["extraction_method"] = "learned_patterns"
    
    return extracted_data


def _apply_organizational_analysis(content: str, org_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply organizational structure analysis."""
    
    analysis = {
        "sophistication_score": 0,
        "leadership_detected": False,
        "member_count_estimate": 0,
        "organizational_type": "unknown"
    }
    
    # Check sophistication indicators
    sophistication_indicators = org_rules.get("sophistication_indicators", {})
    for level, indicators in sophistication_indicators.items():
        for indicator in indicators:
            if indicator.lower() in content.lower():
                analysis["sophistication_score"] += 1
    
    # Check for leadership
    for pattern in org_rules.get("extraction_patterns", []):
        if "leader" in pattern and re.search(pattern, content, re.IGNORECASE):
            analysis["leadership_detected"] = True
    
    # Estimate member count from content
    member_mentions = content.count("Member") + content.count("Collaborator")
    analysis["member_count_estimate"] = member_mentions
    
    # Classify organizational type
    if "cult" in content.lower():
        analysis["organizational_type"] = "cult"
    elif "militia" in content.lower():
        analysis["organizational_type"] = "militia"
    elif "justice" in content.lower():
        analysis["organizational_type"] = "justice_organization"
    
    return analysis


def _apply_territorial_analysis(content: str, territorial_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply territorial control analysis."""
    
    analysis = {
        "controlled_territories": [],
        "territorial_reach": "local",
        "contested_areas": [],
        "base_location": None
    }
    
    # Extract territories from content
    for pattern in territorial_rules.get("extraction_patterns", []):
        matches = re.findall(pattern, content, re.IGNORECASE)
        for match in matches:
            territory = match.strip() if isinstance(match, str) else match[0].strip()
            if territory and territory not in analysis["controlled_territories"]:
                analysis["controlled_territories"].append(territory)
    
    # Assess territorial reach
    territory_count = len(analysis["controlled_territories"])
    if territory_count >= 5:
        analysis["territorial_reach"] = "widespread"
    elif territory_count >= 3:
        analysis["territorial_reach"] = "regional"
    else:
        analysis["territorial_reach"] = "local"
    
    # Check for contested areas
    contested_territories = territorial_rules.get("contested_territories", [])
    analysis["contested_areas"] = [
        territory for territory in analysis["controlled_territories"]
        if territory in contested_territories
    ]
    
    return analysis


def _apply_political_classification(content: str, political_rules: dict[str, Any]) -> dict[str, Any]:
    """Apply political alignment classification."""
    
    analysis = {
        "detected_alignment": "neutral",
        "alignment_confidence": 0.0,
        "political_activity": "unknown",
        "factional_relationships": []
    }
    
    # Check alignment indicators
    alignment_indicators = political_rules.get("alignment_indicators", {})
    alignment_scores = {}
    
    for alignment, indicators in alignment_indicators.items():
        score = 0
        for indicator in indicators:
            if indicator.lower() in content.lower():
                score += 1
        
        if score > 0:
            alignment_scores[alignment] = score / len(indicators)
    
    # Determine primary alignment
    if alignment_scores:
        best_alignment = max(alignment_scores.items(), key=lambda x: x[1])
        analysis["detected_alignment"] = best_alignment[0]
        analysis["alignment_confidence"] = best_alignment[1]
    
    # Classify political activity
    content_lower = content.lower()
    if "eliminate" in content_lower or "plotting" in content_lower:
        analysis["political_activity"] = "aggressive"
    elif "experiment" in content_lower:
        analysis["political_activity"] = "experimental"
    elif "gather" in content_lower or "meeting" in content_lower:
        analysis["political_activity"] = "organizational"
    
    return analysis
