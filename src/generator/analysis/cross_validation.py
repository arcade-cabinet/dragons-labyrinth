"""
Cross-validation module for comparing JSON and HTML entity processing results.

Validates processing quality by comparing structured JSON analysis with HTML text processing,
providing quality metrics and feedback loops for processor enhancement.
"""

from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Any, Dict, List, Tuple

from generator.entities.processors.settlements import SettlementsProcessor
from generator.entities.processors.dungeons import DungeonsProcessor
from generator.entities.processors.regions import RegionsProcessor


class CrossValidator:
    """
    Cross-validation system for comparing JSON and HTML processing results.
    
    Uses JSON entities as "ground truth" to validate and improve HTML processing quality.
    Provides feedback loops for continuous processor enhancement.
    """
    
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        
        # Load JSON processing results for comparison
        self.json_results = self._load_json_results()
        
        # Initialize processors for comparison
        self.settlements_processor = SettlementsProcessor()
        self.dungeons_processor = DungeonsProcessor()
        self.regions_processor = RegionsProcessor()
        
        # Quality metrics tracking
        self.validation_results = {
            "settlements": {"matches": 0, "total": 0, "accuracy": 0.0},
            "dungeons": {"matches": 0, "total": 0, "accuracy": 0.0},
            "regions": {"matches": 0, "total": 0, "accuracy": 0.0}
        }
    
    def validate_settlements_processing(self, html_results: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Validate HTML settlements processing against JSON cities data.
        
        Compares:
        - Scale classification accuracy
        - Service coverage detection  
        - Economic activity assessment
        - POI categorization quality
        """
        
        validation_report = {
            "total_comparisons": 0,
            "scale_accuracy": 0.0,
            "service_coverage_accuracy": 0.0,
            "economic_accuracy": 0.0,
            "detailed_comparisons": []
        }
        
        json_cities = self.json_results.get("cities", [])
        
        for json_city in json_cities:
            # Find corresponding HTML settlement by name matching
            json_name = json_city.get("name", "")
            html_settlement = self._find_matching_settlement(json_name, html_results)
            
            if html_settlement:
                comparison = self._compare_settlement_data(json_city, html_settlement)
                validation_report["detailed_comparisons"].append(comparison)
                validation_report["total_comparisons"] += 1
                
                # Aggregate accuracy metrics
                if comparison["scale_match"]:
                    validation_report["scale_accuracy"] += 1
                if comparison["service_coverage_match"]:
                    validation_report["service_coverage_accuracy"] += 1
                if comparison["economic_activity_match"]:
                    validation_report["economic_accuracy"] += 1
        
        # Calculate percentage accuracies
        if validation_report["total_comparisons"] > 0:
            total = validation_report["total_comparisons"]
            validation_report["scale_accuracy"] /= total
            validation_report["service_coverage_accuracy"] /= total
            validation_report["economic_accuracy"] /= total
            
            overall_accuracy = (
                validation_report["scale_accuracy"] + 
                validation_report["service_coverage_accuracy"] + 
                validation_report["economic_accuracy"]
            ) / 3
            
            validation_report["overall_accuracy"] = overall_accuracy
            
            self.logger.info(f"Settlement validation complete: {overall_accuracy:.1%} overall accuracy")
        
        return validation_report
    
    def validate_dungeons_processing(self, html_results: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Validate HTML dungeons processing against JSON dungeons data.
        
        Compares:
        - Structural complexity assessment
        - Room count estimation accuracy
        - Threat level classification
        - Navigation difficulty prediction
        """
        
        validation_report = {
            "total_comparisons": 0,
            "complexity_accuracy": 0.0,
            "threat_accuracy": 0.0,
            "navigation_accuracy": 0.0,
            "detailed_comparisons": []
        }
        
        json_dungeons = self.json_results.get("dungeons", [])
        
        for json_dungeon in json_dungeons:
            json_name = json_dungeon.get("name", "")
            html_dungeon = self._find_matching_dungeon(json_name, html_results)
            
            if html_dungeon:
                comparison = self._compare_dungeon_data(json_dungeon, html_dungeon)
                validation_report["detailed_comparisons"].append(comparison)
                validation_report["total_comparisons"] += 1
                
                # Aggregate accuracy metrics
                if comparison["complexity_match"]:
                    validation_report["complexity_accuracy"] += 1
                if comparison["threat_level_match"]:
                    validation_report["threat_accuracy"] += 1
                if comparison["navigation_difficulty_match"]:
                    validation_report["navigation_accuracy"] += 1
        
        # Calculate percentage accuracies
        if validation_report["total_comparisons"] > 0:
            total = validation_report["total_comparisons"]
            validation_report["complexity_accuracy"] /= total
            validation_report["threat_accuracy"] /= total
            validation_report["navigation_accuracy"] /= total
            
            overall_accuracy = (
                validation_report["complexity_accuracy"] + 
                validation_report["threat_accuracy"] + 
                validation_report["navigation_accuracy"]
            ) / 3
            
            validation_report["overall_accuracy"] = overall_accuracy
            
            self.logger.info(f"Dungeon validation complete: {overall_accuracy:.1%} overall accuracy")
        
        return validation_report
    
    def validate_regions_processing(self, html_results: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Validate HTML regions processing against JSON map data.
        
        Compares:
        - Biome distribution accuracy
        - Infrastructure connectivity assessment
        - Corruption level classification
        - Regional feature detection
        """
        
        validation_report = {
            "total_comparisons": 0,
            "biome_accuracy": 0.0,
            "connectivity_accuracy": 0.0,
            "corruption_accuracy": 0.0,
            "detailed_comparisons": []
        }
        
        # JSON map provides regional biome and infrastructure data
        json_map_data = self.json_results.get("map", {})
        
        if json_map_data:
            for html_region in html_results:
                comparison = self._compare_region_data(json_map_data, html_region)
                validation_report["detailed_comparisons"].append(comparison)
                validation_report["total_comparisons"] += 1
                
                # Aggregate accuracy metrics
                if comparison["biome_consistency"]:
                    validation_report["biome_accuracy"] += 1
                if comparison["connectivity_match"]:
                    validation_report["connectivity_accuracy"] += 1
                if comparison["corruption_consistency"]:
                    validation_report["corruption_accuracy"] += 1
        
        # Calculate percentage accuracies
        if validation_report["total_comparisons"] > 0:
            total = validation_report["total_comparisons"]
            validation_report["biome_accuracy"] /= total
            validation_report["connectivity_accuracy"] /= total
            validation_report["corruption_accuracy"] /= total
            
            overall_accuracy = (
                validation_report["biome_accuracy"] + 
                validation_report["connectivity_accuracy"] + 
                validation_report["corruption_accuracy"]
            ) / 3
            
            validation_report["overall_accuracy"] = overall_accuracy
            
            self.logger.info(f"Region validation complete: {overall_accuracy:.1%} overall accuracy")
        
        return validation_report
    
    def generate_improvement_recommendations(self, validation_results: Dict[str, Any]) -> Dict[str, List[str]]:
        """
        Generate specific recommendations for improving HTML processors based on validation results.
        
        Analyzes validation gaps and provides actionable enhancement suggestions.
        """
        
        recommendations = {
            "settlements": [],
            "dungeons": [],
            "regions": [],
            "general": []
        }
        
        # Settlement processor recommendations
        settlement_results = validation_results.get("settlements", {})
        if settlement_results.get("scale_accuracy", 0) < 0.8:
            recommendations["settlements"].append(
                "Improve scale detection by adding more infrastructure weighting patterns from JSON analysis"
            )
        
        if settlement_results.get("service_coverage_accuracy", 0) < 0.75:
            recommendations["settlements"].append(
                "Enhance POI categorization using JSON cities comprehensive keyword patterns"
            )
        
        if settlement_results.get("economic_accuracy", 0) < 0.7:
            recommendations["settlements"].append(
                "Apply JSON currency detection patterns and trade activity weighting"
            )
        
        # Dungeon processor recommendations
        dungeon_results = validation_results.get("dungeons", {})
        if dungeon_results.get("complexity_accuracy", 0) < 0.8:
            recommendations["dungeons"].append(
                "Use JSON room/connection counting formulas for better complexity assessment"
            )
        
        if dungeon_results.get("threat_accuracy", 0) < 0.75:
            recommendations["dungeons"].append(
                "Apply JSON hazard density patterns and threat clustering analysis"
            )
        
        if dungeon_results.get("navigation_accuracy", 0) < 0.7:
            recommendations["dungeons"].append(
                "Implement JSON-proven navigation difficulty thresholds based on room counts"
            )
        
        # Region processor recommendations  
        region_results = validation_results.get("regions", {})
        if region_results.get("biome_accuracy", 0) < 0.8:
            recommendations["regions"].append(
                "Apply JSON map biome distribution patterns with weighted keyword scoring"
            )
        
        if region_results.get("connectivity_accuracy", 0) < 0.75:
            recommendations["regions"].append(
                "Use JSON infrastructure synergy bonuses and enhanced connectivity weights"
            )
        
        # General recommendations
        overall_accuracies = [
            settlement_results.get("overall_accuracy", 0),
            dungeon_results.get("overall_accuracy", 0),
            region_results.get("overall_accuracy", 0)
        ]
        
        avg_accuracy = sum(overall_accuracies) / len(overall_accuracies) if overall_accuracies else 0
        
        if avg_accuracy < 0.8:
            recommendations["general"].append(
                "Overall processing accuracy below 80% - consider implementing JSON pattern feedback loops"
            )
        
        if avg_accuracy < 0.7:
            recommendations["general"].append(
                "Critical processing gaps identified - prioritize JSON-to-HTML pattern enhancement"
            )
        
        return recommendations
    
    def _load_json_results(self) -> Dict[str, Any]:
        """Load JSON processing results from final output files (no intermediate directory)."""
        
        json_results = {"cities": [], "dungeons": [], "hazards": [], "map": {}}
        
        # Load JSON processing results from final output structure
        cities_path = Path("crates/world/entities/json_cities.json")
        if cities_path.exists():
            try:
                with open(cities_path, 'r', encoding='utf-8') as f:
                    cities_data = json.load(f)
                    # Extract processing results from the wrapped structure
                    json_results["cities"] = cities_data.get("processing_results", [])
            except Exception as e:
                self.logger.warning(f"Could not load JSON cities results: {e}")
        
        # Load JSON dungeons results from final output
        dungeons_path = Path("crates/world/entities/json_dungeons.json")
        if dungeons_path.exists():
            try:
                with open(dungeons_path, 'r', encoding='utf-8') as f:
                    dungeons_data = json.load(f)
                    json_results["dungeons"] = dungeons_data.get("processing_results", [])
            except Exception as e:
                self.logger.warning(f"Could not load JSON dungeons results: {e}")
        
        # Load JSON map results from final output
        maps_path = Path("crates/world/entities/json_maps.json")
        if maps_path.exists():
            try:
                with open(maps_path, 'r', encoding='utf-8') as f:
                    maps_data = json.load(f)
                    map_results = maps_data.get("processing_results", [])
                    json_results["map"] = map_results[0] if map_results else {}
            except Exception as e:
                self.logger.warning(f"Could not load JSON map results: {e}")
        
        return json_results
    
    def _find_matching_settlement(self, json_name: str, html_results: List[Dict[str, Any]]) -> Dict[str, Any] | None:
        """Find HTML settlement result matching JSON city name."""
        
        for html_settlement in html_results:
            html_name = html_settlement.get("name", "")
            if json_name.lower() in html_name.lower() or html_name.lower() in json_name.lower():
                return html_settlement
        
        return None
    
    def _find_matching_dungeon(self, json_name: str, html_results: List[Dict[str, Any]]) -> Dict[str, Any] | None:
        """Find HTML dungeon result matching JSON dungeon name."""
        
        for html_dungeon in html_results:
            html_name = html_dungeon.get("name", "")
            if json_name.lower() in html_name.lower() or html_name.lower() in json_name.lower():
                return html_dungeon
        
        return None
    
    def _compare_settlement_data(self, json_city: Dict[str, Any], html_settlement: Dict[str, Any]) -> Dict[str, Any]:
        """Compare JSON city data with HTML settlement processing results."""
        
        comparison = {
            "json_name": json_city.get("name", ""),
            "html_name": html_settlement.get("name", ""),
            "scale_match": False,
            "service_coverage_match": False,
            "economic_activity_match": False,
            "details": {}
        }
        
        # Compare scale classification
        json_scale = json_city.get("scale", "").lower()
        html_scale = html_settlement.get("scale_hint", "").lower()
        
        # Allow for scale category equivalence (metropolis = city)
        scale_equivalents = {
            "metropolis": "city",
            "city": "city", 
            "town": "town",
            "village": "village"
        }
        
        json_scale_normalized = scale_equivalents.get(json_scale, json_scale)
        html_scale_normalized = scale_equivalents.get(html_scale, html_scale)
        
        comparison["scale_match"] = json_scale_normalized == html_scale_normalized
        comparison["details"]["scale"] = {
            "json": json_scale,
            "html": html_scale,
            "normalized_match": comparison["scale_match"]
        }
        
        # Compare service coverage
        json_services = set(json_city.get("service_types", []))
        html_services = set(html_settlement.get("service_types", []))
        
        # Calculate service overlap percentage
        if json_services or html_services:
            overlap = len(json_services & html_services)
            total_unique = len(json_services | html_services)
            service_similarity = overlap / total_unique if total_unique > 0 else 0
            comparison["service_coverage_match"] = service_similarity >= 0.6  # 60% similarity threshold
        else:
            comparison["service_coverage_match"] = True  # Both empty
        
        comparison["details"]["services"] = {
            "json": list(json_services),
            "html": list(html_services),
            "similarity": service_similarity if 'service_similarity' in locals() else 0
        }
        
        # Compare economic activity
        json_economic = json_city.get("economic_activity_level", 0)
        html_economic = html_settlement.get("economic_activity_level", 0)
        
        # Allow for reasonable variance in economic assessment
        economic_threshold = max(5, json_economic * 0.3)  # 30% variance allowed
        comparison["economic_activity_match"] = abs(json_economic - html_economic) <= economic_threshold
        
        comparison["details"]["economic"] = {
            "json": json_economic,
            "html": html_economic,
            "variance": abs(json_economic - html_economic),
            "threshold": economic_threshold
        }
        
        return comparison
    
    def _compare_dungeon_data(self, json_dungeon: Dict[str, Any], html_dungeon: Dict[str, Any]) -> Dict[str, Any]:
        """Compare JSON dungeon data with HTML dungeon processing results."""
        
        comparison = {
            "json_name": json_dungeon.get("name", ""),
            "html_name": html_dungeon.get("name", ""),
            "complexity_match": False,
            "threat_level_match": False,
            "navigation_difficulty_match": False,
            "details": {}
        }
        
        # Compare structural complexity
        json_complexity = json_dungeon.get("structural_analysis", {}).get("complexity_level", "").lower()
        html_complexity = html_dungeon.get("structural_analysis", {}).get("complexity_level", "").lower()
        
        comparison["complexity_match"] = json_complexity == html_complexity
        comparison["details"]["complexity"] = {
            "json": json_complexity,
            "html": html_complexity
        }
        
        # Compare threat level (allow ±1 level variance)
        json_threat = json_dungeon.get("threat_assessment", {}).get("threat_level", 1)
        html_threat = html_dungeon.get("threat_assessment", {}).get("threat_level", 1)
        
        comparison["threat_level_match"] = abs(json_threat - html_threat) <= 1
        comparison["details"]["threat"] = {
            "json": json_threat,
            "html": html_threat,
            "variance": abs(json_threat - html_threat)
        }
        
        # Compare navigation difficulty
        json_nav = json_dungeon.get("structural_analysis", {}).get("navigation_difficulty", "").lower()
        html_nav = html_dungeon.get("exploration_difficulty", "").lower()  # HTML uses different key
        
        # Map difficulty levels for comparison
        difficulty_mapping = {
            "trivial": 0, "easy": 1, "moderate": 2, "hard": 3, "extreme": 4, "nightmare": 5
        }
        
        json_nav_level = difficulty_mapping.get(json_nav, 2)
        html_nav_level = difficulty_mapping.get(html_nav, 2)
        
        # Allow ±1 difficulty level variance
        comparison["navigation_difficulty_match"] = abs(json_nav_level - html_nav_level) <= 1
        comparison["details"]["navigation"] = {
            "json": json_nav,
            "html": html_nav,
            "json_level": json_nav_level,
            "html_level": html_nav_level
        }
        
        return comparison
    
    def _compare_region_data(self, json_map: Dict[str, Any], html_region: Dict[str, Any]) -> Dict[str, Any]:
        """Compare JSON map data with HTML region processing results."""
        
        comparison = {
            "region_name": html_region.get("name", ""),
            "biome_consistency": False,
            "connectivity_match": False,
            "corruption_consistency": False,
            "details": {}
        }
        
        # Compare biome distribution consistency
        json_biomes = set(json_map.get("biome_distribution", {}).keys())
        html_biomes = set(html_region.get("biome_distribution", {}).keys())
        
        if json_biomes or html_biomes:
            biome_overlap = len(json_biomes & html_biomes)
            total_biomes = len(json_biomes | html_biomes)
            biome_similarity = biome_overlap / total_biomes if total_biomes > 0 else 0
            comparison["biome_consistency"] = biome_similarity >= 0.4  # 40% biome consistency threshold
        else:
            comparison["biome_consistency"] = True
        
        comparison["details"]["biomes"] = {
            "json": list(json_biomes),
            "html": list(html_biomes),
            "similarity": biome_similarity if 'biome_similarity' in locals() else 0
        }
        
        # Compare connectivity patterns
        json_connectivity = json_map.get("connectivity_analysis", {}).get("connectivity_score", 0)
        html_connectivity = html_region.get("geographic_features", {})
        
        # Calculate HTML connectivity score for comparison
        html_connectivity_score = self._calculate_html_connectivity_score(html_connectivity)
        
        # Allow reasonable variance in connectivity assessment
        connectivity_threshold = 0.3  # 30% variance allowed
        comparison["connectivity_match"] = abs(json_connectivity - html_connectivity_score) <= connectivity_threshold
        
        comparison["details"]["connectivity"] = {
            "json": json_connectivity,
            "html": html_connectivity_score,
            "variance": abs(json_connectivity - html_connectivity_score)
        }
        
        # Compare corruption levels (from region name analysis)
        region_name = html_region.get("name", "").lower()
        json_corruption = self._estimate_json_corruption_level(region_name)
        html_corruption = html_region.get("corruption_level", 0)
        
        # Allow ±1 corruption level variance
        comparison["corruption_consistency"] = abs(json_corruption - html_corruption) <= 1
        comparison["details"]["corruption"] = {
            "estimated_json": json_corruption,
            "html": html_corruption,
            "variance": abs(json_corruption - html_corruption)
        }
        
        return comparison
    
    def _calculate_html_connectivity_score(self, geographic_features: Dict[str, int]) -> float:
        """Calculate connectivity score from HTML region geographic features."""
        
        # Use same logic as enhanced regions processor
        weights = {
            "rivers": 0.25,
            "trails": 0.35, 
            "roads": 0.4,
            "harbors": 0.2,
            "bridges": 0.15,
            "borders": -0.05
        }
        
        score = 0.0
        for feature, weight in weights.items():
            feature_count = geographic_features.get(feature, 0)
            score += feature_count * weight
        
        # Apply synergy bonuses
        total_infrastructure = sum(geographic_features.get(f, 0) for f in ["rivers", "trails", "roads"])
        if total_infrastructure >= 10:
            score += 0.3
        elif total_infrastructure >= 6:
            score += 0.2
        elif total_infrastructure >= 3:
            score += 0.1
        
        return max(0.0, min(1.0, score))
    
    def _estimate_json_corruption_level(self, region_name: str) -> int:
        """Estimate corruption level from region name for JSON comparison."""
        
        # Use same logic as enhanced regions processor
        name_lower = region_name.lower()
        
        if any(word in name_lower for word in ["dragon", "scar", "abyssal", "chasm", "final"]):
            return 5
        elif any(word in name_lower for word in ["nightmare", "hell", "vicious", "void"]):
            return 4
        elif any(word in name_lower for word in ["blood", "bone", "death", "decay", "rust", "famine"]):
            return 3
        elif any(word in name_lower for word in ["black", "dark", "cursed", "haunted", "fear", "dread"]):
            return 2
        elif any(word in name_lower for word in ["grey", "ash", "ashen", "shadow"]):
            return 1
        else:
            return 0
    
    def save_validation_report(self, validation_results: Dict[str, Any], output_path: str = "validation_report.json") -> None:
        """Save validation results to file for analysis with idempotent writing."""
        
        try:
            # Ensure output directory exists
            output_file = Path(output_path)
            output_file.parent.mkdir(parents=True, exist_ok=True)
            
            # Idempotent write - complete file overwrite with timestamp
            validation_results["file_generation_timestamp"] = str(Path(__file__).stat().st_mtime)
            validation_results["idempotent_write"] = True
            
            with open(output_file, 'w', encoding='utf-8') as f:
                json.dump(validation_results, f, indent=2, ensure_ascii=False)
            
            self.logger.info(f"Validation report saved idempotently to {output_path}")
        except Exception as e:
            self.logger.error(f"Failed to save validation report: {e}")


def validate_all_processors(html_settlements: List[Dict[str, Any]], 
                          html_dungeons: List[Dict[str, Any]], 
                          html_regions: List[Dict[str, Any]]) -> Dict[str, Any]:
    """
    Validate all HTML processors against JSON ground truth data idempotently.
    
    Args:
        html_settlements: HTML settlements processing results
        html_dungeons: HTML dungeons processing results  
        html_regions: HTML regions processing results
        
    Returns:
        Comprehensive validation report with accuracy metrics and recommendations
    """
    
    validator = CrossValidator()
    
    # Generate consistent timestamp for this validation run
    from datetime import datetime
    validation_timestamp = datetime.now().isoformat()
    
    validation_report = {
        "validation_timestamp": validation_timestamp,
        "validation_run_id": f"validation_{len(html_settlements)}_{len(html_dungeons)}_{len(html_regions)}",
        "summary": {},
        "settlements": validator.validate_settlements_processing(html_settlements),
        "dungeons": validator.validate_dungeons_processing(html_dungeons),
        "regions": validator.validate_regions_processing(html_regions),
        "recommendations": {}
    }
    
    # Generate summary metrics
    validation_report["summary"] = {
        "overall_accuracy": 0.0,
        "total_validations": 0,
        "successful_validations": 0
    }
    
    accuracies = []
    total_validations = 0
    
    for processor_name in ["settlements", "dungeons", "regions"]:
        processor_results = validation_report.get(processor_name, {})
        if processor_results.get("total_comparisons", 0) > 0:
            accuracy = processor_results.get("overall_accuracy", 0)
            accuracies.append(accuracy)
            total_validations += processor_results.get("total_comparisons", 0)
    
    if accuracies:
        validation_report["summary"]["overall_accuracy"] = sum(accuracies) / len(accuracies)
        validation_report["summary"]["total_validations"] = total_validations
        validation_report["summary"]["successful_validations"] = len(accuracies)
    
    # Generate improvement recommendations
    validation_report["recommendations"] = validator.generate_improvement_recommendations(validation_report)
    
    # Save validation report idempotently 
    output_path = Path("crates/world/entities/validation_report.json")
    validator.save_validation_report(validation_report, str(output_path))
    
    return validation_report
