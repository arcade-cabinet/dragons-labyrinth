"""
Organized ML Training System - Extract content using organized HBF examples.

Replaces generic ML discovery with organized extraction approach using 
breakthrough HBF worldbuilding data. Coordinates category-specific modules
for regions, settlements, factions, and dungeons.
"""

from __future__ import annotations

import logging
from typing import Any

from .meta import get_training_metadata
from .regions import run as run_regions
from .settlements import run as run_settlements  
from .factions import run as run_factions
from .dungeons import run as run_dungeons


def run(engine, logger: logging.Logger, console) -> dict[str, Any]:
    """
    Run complete organized ML training system.
    
    Uses organized HBF worldbuilding data from memory-bank/world-building/
    to train category-specific ML extractors. Focuses on content extraction
    rather than category detection.
    
    Args:
        engine: Database engine (passed to modules but not used for file ops)
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Complete training results from all categories
    """
    
    logger.info("Starting organized ML training system")
    console.print("🎯 [bold magenta]Organized ML Training System[/bold magenta]")
    console.print("Using organized HBF worldbuilding breakthrough data")
    
    # Get training metadata overview
    metadata = get_training_metadata()
    
    console.print(f"📋 Training Overview:")
    console.print(f"  • World: {metadata['world_context']['name']}")
    console.print(f"  • Categories: {metadata['world_context']['total_categories']}")
    console.print(f"  • Total Examples: {metadata['world_context']['total_examples']}")
    console.print(f"  • Data Quality: {metadata['data_quality']}")
    
    results = {
        "training_approach": "organized_extraction",
        "data_source": "memory-bank/world-building/",
        "metadata": metadata,
        "category_results": {},
        "overall_success": True,
        "error_summary": []
    }
    
    # Run category-specific training
    try:
        # 1. Region training (27 examples)
        console.print("\n🏔️ Running region training...")
        region_results = run_regions(engine, logger, console)
        results["category_results"]["regions"] = region_results
        
        if "error" in region_results:
            results["error_summary"].append("regions: " + region_results["error"])
            logger.warning("Region training failed")
        else:
            logger.info(f"Region training complete: {region_results['examples_analyzed']} examples")
            
    except Exception as e:
        error_msg = f"Region training error: {e}"
        results["error_summary"].append(error_msg)
        logger.error(error_msg)
    
    try:
        # 2. Settlement training (10 examples)
        console.print("\n🏘️ Running settlement training...")
        settlement_results = run_settlements(engine, logger, console)
        results["category_results"]["settlements"] = settlement_results
        
        if "error" in settlement_results:
            results["error_summary"].append("settlements: " + settlement_results["error"])
            logger.warning("Settlement training failed")
        else:
            logger.info(f"Settlement training complete: {settlement_results['examples_analyzed']} examples")
            
    except Exception as e:
        error_msg = f"Settlement training error: {e}"
        results["error_summary"].append(error_msg)
        logger.error(error_msg)
    
    try:
        # 3. Faction training (5 examples)
        console.print("\n⚔️ Running faction training...")
        faction_results = run_factions(engine, logger, console)
        results["category_results"]["factions"] = faction_results
        
        if "error" in faction_results:
            results["error_summary"].append("factions: " + faction_results["error"])
            logger.warning("Faction training failed")
        else:
            logger.info(f"Faction training complete: {faction_results['examples_analyzed']} examples")
            
    except Exception as e:
        error_msg = f"Faction training error: {e}"
        results["error_summary"].append(error_msg)
        logger.error(error_msg)
    
    try:
        # 4. Dungeon training (18 examples)  
        console.print("\n🏰 Running dungeon training...")
        dungeon_results = run_dungeons(engine, logger, console)
        results["category_results"]["dungeons"] = dungeon_results
        
        if "error" in dungeon_results:
            results["error_summary"].append("dungeons: " + dungeon_results["error"])
            logger.warning("Dungeon training failed")
        else:
            logger.info(f"Dungeon training complete: {dungeon_results['examples_analyzed']} examples")
            
    except Exception as e:
        error_msg = f"Dungeon training error: {e}"
        results["error_summary"].append(error_msg)
        logger.error(error_msg)
    
    # Calculate overall success
    successful_categories = len([
        category for category, result in results["category_results"].items()
        if "error" not in result
    ])
    
    total_categories = 4
    results["overall_success"] = successful_categories == total_categories
    results["success_rate"] = successful_categories / total_categories
    
    # Summary output
    console.print(f"\n📊 [bold cyan]Training Summary[/bold cyan]")
    console.print(f"  • Successful Categories: {successful_categories}/{total_categories}")
    console.print(f"  • Success Rate: {results['success_rate']:.1%}")
    
    if results["error_summary"]:
        console.print(f"  • Errors: {len(results['error_summary'])}")
        for error in results["error_summary"]:
            console.print(f"    - {error}")
    
    if results["overall_success"]:
        console.print("✅ [bold green]Organized ML training complete[/bold green] - All categories successful")
        
        # Display training statistics
        total_examples = sum(
            result.get("examples_analyzed", 0) 
            for result in results["category_results"].values()
            if isinstance(result, dict)
        )
        
        console.print(f"\n🎯 [bold blue]Training Statistics[/bold blue]")
        console.print(f"  • Total Examples Processed: {total_examples}")
        console.print(f"  • Regions: {results['category_results'].get('regions', {}).get('examples_analyzed', 0)}")
        console.print(f"  • Settlements: {results['category_results'].get('settlements', {}).get('examples_analyzed', 0)}")
        console.print(f"  • Factions: {results['category_results'].get('factions', {}).get('examples_analyzed', 0)}")
        console.print(f"  • Dungeons: {results['category_results'].get('dungeons', {}).get('examples_analyzed', 0)}")
        
        console.print(f"\n💾 Training patterns saved to:")
        console.print(f"  • training/regions/")
        console.print(f"  • training/settlements/")
        console.print(f"  • training/factions/")
        console.print(f"  • training/dungeons/")
        
    else:
        console.print("⚠️ [bold yellow]Training completed with some errors[/bold yellow]")
    
    logger.info(f"Organized ML training complete: {successful_categories}/{total_categories} categories successful")
    
    return results


def extract_entity_content(entity_content: str, entity_category: str | None = None) -> dict[str, Any]:
    """
    Extract entity content using trained category-specific patterns.
    
    Args:
        entity_content: Raw entity content to analyze
        entity_category: Optional known category ('regions', 'settlements', 'factions', 'dungeons')
        
    Returns:
        Extracted content with category classification and confidence scoring
    """
    
    if entity_category:
        # Use specific category extractor
        if entity_category == "regions":
            from .regions import extract_region_content
            return extract_region_content(entity_content)
        elif entity_category == "settlements":
            from .settlements import extract_settlement_content
            return extract_settlement_content(entity_content)
        elif entity_category == "factions":
            from .factions import extract_faction_content
            return extract_faction_content(entity_content)
        elif entity_category == "dungeons":
            from .dungeons import extract_dungeon_content
            return extract_dungeon_content(entity_content)
    
    # Auto-categorize then extract
    category = auto_categorize_entity(entity_content)
    
    if category != "unknown":
        return extract_entity_content(entity_content, category)
    else:
        return {
            "category": "unknown",
            "confidence_score": 0.0,
            "extraction_method": "failed_categorization",
            "error": "Could not determine entity category"
        }


def auto_categorize_entity(entity_content: str) -> str:
    """
    Automatically categorize entity using learned patterns.
    
    Args:
        entity_content: Raw entity content
        
    Returns:
        Predicted category: 'regions', 'settlements', 'factions', 'dungeons', or 'unknown'
    """
    
    # Use simple heuristics for now - can be enhanced with ML later
    content_lower = entity_content.lower()
    
    # Check for region indicators (JSON structure, hex coordinates)
    if '"map":' in entity_content and '"regions":' in entity_content:
        return "regions"
    
    # Check for settlement indicators
    if any(phrase in content_lower for phrase in ["city of", "town of", "village of"]):
        return "settlements"
    
    # Check for faction indicators  
    if any(phrase in content_lower for phrase in ["members", "collaborators", "leader is a"]):
        return "factions"
    
    # Check for dungeon indicators
    if any(phrase in content_lower for phrase in ["crypt of", "lair of", "temple of", "shrine of", "tomb of"]):
        return "dungeons"
    
    # Check for encounter/monster content (likely dungeon)
    if "monster-block" in entity_content and "CR:" in entity_content:
        return "dungeons"
    
    # Check for establishment content (likely settlement)
    if any(phrase in content_lower for phrase in ["tavern", "inn", "shop", "market"]):
        return "settlements"
    
    return "unknown"


def get_category_statistics() -> dict[str, Any]:
    """
    Get statistics about trained categories.
    
    Returns:
        Statistics about training data and learned patterns
    """
    
    metadata = get_training_metadata()
    
    stats = {
        "training_data": metadata["category_summary"],
        "world_context": metadata["world_context"],
        "ml_targets": metadata["ml_targets"],
        "data_quality": metadata["data_quality"]
    }
    
    # Load saved pattern statistics if available
    from pathlib import Path
    
    for category in ["regions", "settlements", "factions", "dungeons"]:
        patterns_file = Path("training") / category / "pattern_analysis.json"
        if patterns_file.exists():
            try:
                import json
                with open(patterns_file, "r", encoding="utf-8") as f:
                    pattern_data = json.load(f)
                    stats[f"{category}_patterns"] = pattern_data
            except Exception:
                continue
    
    return stats


def validate_training_system() -> dict[str, Any]:
    """
    Validate that training system is properly configured.
    
    Returns:
        Validation results with recommendations
    """
    
    validation = {
        "system_ready": True,
        "issues": [],
        "recommendations": [],
        "data_availability": {}
    }
    
    # Check organized data availability
    from pathlib import Path
    
    base_dir = Path("memory-bank/world-building")
    categories = ["regions", "settlements", "factions", "dungeons"]
    
    for category in categories:
        category_dir = base_dir / category
        
        if category_dir.exists():
            file_count = len(list(category_dir.glob("*.txt")))
            validation["data_availability"][category] = {
                "available": True,
                "file_count": file_count,
                "directory": str(category_dir)
            }
            
            if file_count == 0:
                validation["issues"].append(f"No training files found in {category_dir}")
                validation["system_ready"] = False
        else:
            validation["data_availability"][category] = {
                "available": False,
                "file_count": 0,
                "directory": str(category_dir)
            }
            validation["issues"].append(f"Missing training data directory: {category_dir}")
            validation["system_ready"] = False
    
    # Check training anchor file
    anchor_file = Path("training/meta/anchors.json")
    if anchor_file.exists():
        validation["anchor_file"] = {"available": True, "path": str(anchor_file)}
    else:
        validation["anchor_file"] = {"available": False, "path": str(anchor_file)}
        validation["recommendations"].append("Create training/meta/anchors.json with world knowledge")
    
    # System readiness assessment
    if validation["system_ready"]:
        available_categories = sum(1 for cat_data in validation["data_availability"].values() if cat_data["available"])
        validation["readiness_percentage"] = available_categories / len(categories)
    else:
        validation["readiness_percentage"] = 0.0
    
    return validation


# Export key functions for external use
__all__ = [
    "run",
    "extract_entity_content", 
    "auto_categorize_entity",
    "get_category_statistics",
    "validate_training_system"
]
