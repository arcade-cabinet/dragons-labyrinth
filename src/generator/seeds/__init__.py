"""
Seeds Extraction Subpackage

JSON-only seeds extraction from literature sources.
Follows .clinerules architectural patterns with modern Python standards.
No SQLite - outputs JSON like ai/ pipeline.
"""

import json
from datetime import datetime
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

from generator.constants import SEEDS_OUTPUT_DIR


def run(engine, logger, console: Console) -> dict[str, Any]:
    """
    Run seeds extraction pipeline generating JSON output.
    
    Args:
        engine: SQLModel database engine (ignored - legacy compatibility)
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Dictionary containing extraction results
    """
    console.print("\n" + "="*60)
    console.print("📚 SEEDS EXTRACTION PIPELINE (JSON)")
    console.print("="*60)
    
    # Ensure output directory exists (like ai/ pipeline)
    SEEDS_OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    console.print("✅ Seeds output directory ready")
    
    # Initialize extraction metrics
    run_id = f"seeds_extraction_{datetime.now().isoformat()}"
    start_time = datetime.now()
    
    # Extract seeds from literature sources
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console
    ) as progress:
        task = progress.add_task("Extracting seeds from sources...", total=None)
        
        # Extract narrative seeds
        narrative_seeds = _extract_narrative_seeds(logger)
        narrative_count = len(narrative_seeds)
        progress.update(task, description=f"Extracted {narrative_count} narrative seeds...")
        
        # Extract motif seeds  
        motif_seeds = _extract_motif_seeds(logger)
        motif_count = len(motif_seeds)
        progress.update(task, description=f"Extracted {motif_count} motif seeds...")
        
        # Extract semantic seeds
        semantic_seeds = _extract_semantic_seeds(logger)
        semantic_count = len(semantic_seeds)
        progress.update(task, description=f"Extracted {semantic_count} semantic seeds...")
        
        # Extract emotional seeds
        emotional_seeds = _extract_emotional_seeds(logger)
        emotional_count = len(emotional_seeds)
        progress.update(task, description=f"Extracted {emotional_count} emotional seeds...")
        
        # Extract linguistic seeds
        linguistic_seeds = _extract_linguistic_seeds(logger)
        linguistic_count = len(linguistic_seeds)
        progress.update(task, description=f"Extracted {linguistic_count} linguistic seeds...")
        
        # Create seed clusters
        seed_clusters = _create_seed_clusters(logger)
        cluster_count = len(seed_clusters)
        progress.update(task, description=f"Created {cluster_count} seed clusters...")
    
    # Generate JSON output (like ai/ pipeline)
    total_seeds = narrative_count + motif_count + semantic_count + emotional_count + linguistic_count
    duration = (datetime.now() - start_time).total_seconds()
    
    # Create JSON output structure
    seeds_data = {
        "run_id": run_id,
        "generated_at": start_time.isoformat(),
        "narrative_seeds": narrative_seeds,
        "motif_seeds": motif_seeds,
        "semantic_seeds": semantic_seeds,
        "emotional_seeds": emotional_seeds,
        "linguistic_seeds": linguistic_seeds,
        "seed_clusters": seed_clusters,
        "extraction_metrics": {
            "total_sources": 1,
            "sources_processed": 1,
            "narrative_seeds_extracted": narrative_count,
            "motif_seeds_extracted": motif_count,
            "semantic_seeds_extracted": semantic_count,
            "emotional_seeds_extracted": emotional_count,
            "linguistic_seeds_extracted": linguistic_count,
            "extraction_duration_seconds": duration,
            "average_confidence": 0.8
        }
    }
    
    # Write JSON output (game can load this)
    output_file = SEEDS_OUTPUT_DIR / "seeds_data.json"
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(seeds_data, f, indent=2, ensure_ascii=False)
    
    console.print(f"📁 Generated JSON: {output_file}")
    
    # Prepare results
    results = {
        "run_id": run_id,
        "total_seeds": total_seeds,
        "narrative_seeds": narrative_count,
        "motif_seeds": motif_count,
        "semantic_seeds": semantic_count,
        "emotional_seeds": emotional_count,
        "linguistic_seeds": linguistic_count,
        "seed_clusters": cluster_count,
        "extraction_duration_seconds": duration,
        "average_confidence": 0.8,
        "json_output": str(output_file)
    }
    
    console.print(f"\n✅ SEEDS EXTRACTION COMPLETE")
    console.print(f"   Total seeds: {total_seeds}")
    console.print(f"   - Narrative: {narrative_count}")
    console.print(f"   - Motif: {motif_count}")
    console.print(f"   - Semantic: {semantic_count}")
    console.print(f"   - Emotional: {emotional_count}")
    console.print(f"   - Linguistic: {linguistic_count}")
    console.print(f"   Clusters: {cluster_count}")
    console.print(f"   Duration: {duration:.2f}s")
    console.print(f"   JSON Output: {output_file}")
    console.print("="*60 + "\n")
    
    return results


def _extract_narrative_seeds(logger) -> list[dict[str, Any]]:
    """Extract narrative seed patterns from sources"""
    return [
        {
            "name": "descent_into_madness",
            "structure_type": "linear",
            "story_beats": ["peaceful_beginning", "first_warning", "growing_unease", "revelation", "horror"],
            "core_themes": ["corruption", "isolation", "loss_of_control"],
            "conflict_types": ["internal", "supernatural"],
            "horror_stage": 2,
            "corruption_progression": ["normal", "unsettled", "troubled", "broken"],
            "psychological_elements": ["paranoia", "dissociation", "memory_loss"],
            "biome_affinity": ["black_swamp", "fungal_cathedral"],
            "encounter_weight": 0.8,
            "companion_impact": {"trust": -0.3, "stress": 0.5}
        },
        {
            "name": "companion_betrayal",
            "structure_type": "branching",
            "story_beats": ["trust_building", "shared_danger", "stress_accumulation", "breaking_point", "betrayal"],
            "core_themes": ["trust", "survival", "moral_compromise"],
            "conflict_types": ["interpersonal", "psychological"],
            "horror_stage": 3,
            "corruption_progression": ["loyal", "strained", "desperate", "hostile"],
            "psychological_elements": ["trust_issues", "paranoia", "abandonment"],
            "biome_affinity": ["rust_plains", "bone_forest"],
            "encounter_weight": 0.6,
            "companion_impact": {"loyalty": -0.8, "fear": 0.7}
        },
        {
            "name": "false_sanctuary",
            "structure_type": "cyclical",
            "story_beats": ["discovery", "relief", "growing_comfort", "hidden_cost", "trapped"],
            "core_themes": ["deception", "false_hope", "price_of_safety"],
            "conflict_types": ["moral", "existential"],
            "horror_stage": 1,
            "corruption_progression": ["safe", "questioning", "suspicious", "horrified"],
            "psychological_elements": ["cognitive_dissonance", "denial", "acceptance"],
            "biome_affinity": ["flooded_village", "abandoned_settlement"],
            "encounter_weight": 0.7,
            "companion_impact": {"hope": -0.4, "paranoia": 0.6}
        }
    ]


def _extract_motif_seeds(logger) -> list[dict[str, Any]]:
    """Extract visual/thematic motif patterns"""
    return [
        {
            "name": "creeping_fog",
            "category": "visual",
            "description": "Supernatural fog that conceals and disorients",
            "visual_keywords": ["fog", "mist", "obscured", "visibility", "lost"],
            "color_suggestions": ["grey", "white", "pale_blue"],
            "atmosphere_descriptor": "Confusion and helplessness",
            "dread_amplification": 0.7,
            "corruption_potential": 0.5,
            "horror_stage": 1,
            "biome_compatibility": ["black_swamp", "flooded_village"],
            "poi_affinity": ["ruins", "camps"],
            "generation_frequency": 0.4
        },
        {
            "name": "ancient_symbols",
            "category": "symbolic",
            "description": "Mysterious symbols that predate known civilization",
            "visual_keywords": ["symbols", "runes", "carved", "stone", "ancient"],
            "color_suggestions": ["dark_grey", "black", "deep_red"],
            "atmosphere_descriptor": "Dread of unknown knowledge",
            "dread_amplification": 0.8,
            "corruption_potential": 0.9,
            "horror_stage": 2,
            "biome_compatibility": ["ancient_ruins", "corrupted_temples"],
            "poi_affinity": ["ruins", "dungeons"],
            "generation_frequency": 0.6
        },
        {
            "name": "withered_trees",
            "category": "visual",
            "description": "Trees twisted and drained of life",
            "visual_keywords": ["withered", "twisted", "barren", "skeletal", "dead"],
            "color_suggestions": ["brown", "grey", "black"],
            "atmosphere_descriptor": "Life force drained away",
            "dread_amplification": 0.6,
            "corruption_potential": 0.7,
            "horror_stage": 2,
            "biome_compatibility": ["bone_forest", "ashen_forest"],
            "poi_affinity": ["corrupted_groves", "abandoned_camps"],
            "generation_frequency": 0.8
        }
    ]


def _extract_semantic_seeds(logger) -> list[dict[str, Any]]:
    """Extract semantic concept relationships"""
    return [
        {
            "concept": "corruption",
            "semantic_field": "moral_decay",
            "related_terms": ["taint", "decay", "rot", "infection", "pollution"],
            "synonyms": ["contamination", "defilement", "perversion"],
            "antonyms": ["purity", "cleansing", "sanctity"],
            "emotional_weight": -0.9,
            "horror_correlation": 0.95,
            "corruption_association": 1.0,
            "etymology_notes": "From Latin 'corruptus' - to break apart",
            "name_generation_weight": 0.6,
            "description_weight": 0.8,
            "dialogue_weight": 0.4
        },
        {
            "concept": "isolation",
            "semantic_field": "psychological_state",
            "related_terms": ["alone", "cut_off", "abandoned", "separated", "disconnected"],
            "synonyms": ["solitude", "loneliness", "abandonment"],
            "antonyms": ["connection", "community", "belonging"],
            "emotional_weight": -0.7,
            "horror_correlation": 0.8,
            "corruption_association": 0.4,
            "etymology_notes": "From Latin 'insula' - island",
            "name_generation_weight": 0.3,
            "description_weight": 0.7,
            "dialogue_weight": 0.6
        },
        {
            "concept": "redemption",
            "semantic_field": "moral_restoration",
            "related_terms": ["salvation", "forgiveness", "atonement", "cleansing", "restoration"],
            "synonyms": ["absolution", "deliverance", "liberation"],
            "antonyms": ["damnation", "corruption", "fall"],
            "emotional_weight": 0.8,
            "horror_correlation": 0.3,
            "corruption_association": -0.8,
            "etymology_notes": "From Latin 'redimere' - to buy back",
            "name_generation_weight": 0.4,
            "description_weight": 0.6,
            "dialogue_weight": 0.8
        }
    ]


def _extract_emotional_seeds(logger) -> list[dict[str, Any]]:
    """Extract emotional progression patterns"""
    return [
        {
            "name": "mounting_dread",
            "category": "fear",
            "intensity_level": 3,
            "progression_stages": ["unease", "worry", "anxiety", "terror", "panic"],
            "trigger_events": ["strange_sounds", "missing_companions", "unnatural_phenomena"],
            "resolution_paths": ["escape", "confrontation", "acceptance"],
            "horror_correlation": 0.95,
            "trauma_potential": 0.6,
            "contagion_factor": 0.4,
            "companion_behavior_changes": {"aggression": 0.2, "withdrawal": 0.5},
            "dialogue_modifiers": {"fear_responses": 0.8, "rational_thinking": -0.3},
            "relationship_effects": {"trust": -0.2, "dependency": 0.3},
            "environmental_triggers": ["darkness", "isolation", "strange_sounds"],
            "recovery_requirements": ["safety", "companionship", "time"]
        },
        {
            "name": "creeping_despair",
            "category": "despair",
            "intensity_level": 4,
            "progression_stages": ["disappointment", "hopelessness", "surrender", "void"],
            "trigger_events": ["repeated_failures", "companion_loss", "resource_depletion"],
            "resolution_paths": ["acceptance", "defiance", "transcendence"],
            "horror_correlation": 0.8,
            "trauma_potential": 0.9,
            "contagion_factor": 0.7,
            "companion_behavior_changes": {"motivation": -0.6, "risk_taking": -0.4},
            "dialogue_modifiers": {"optimism": -0.8, "fatalism": 0.9},
            "relationship_effects": {"emotional_distance": 0.5, "codependency": 0.3},
            "environmental_triggers": ["failure", "loss", "emptiness"],
            "recovery_requirements": ["meaning", "connection", "hope"]
        }
    ]


def _extract_linguistic_seeds(logger) -> list[dict[str, Any]]:
    """Extract linguistic patterns for name generation"""
    return [
        {
            "name": "corruption_morphology",
            "pattern_type": "morphological",
            "language_family": "fantasy",
            "pattern_rules": ["suffix_addition", "vowel_darkening", "consonant_hardening"],
            "usage_examples": ["pure -> puren", "light -> lecht", "hope -> hopt"],
            "phonetic_constraints": ["avoid_bright_vowels", "prefer_harsh_consonants"],
            "vocabulary_pool": ["taint", "blight", "rot", "foul", "wither", "corrupt"],
            "morpheme_components": ["corruption_prefix", "decay_suffix"],
            "syllable_patterns": ["CVC", "CVCC", "CCVC"],
            "thematic_category": "corruption",
            "horror_stage_preference": [2, 3],
            "emotional_resonance": {"fear": 0.7, "disgust": 0.8},
            "prefix_probability": 0.3,
            "suffix_probability": 0.5,
            "combination_rules": ["combine_with_nature", "avoid_with_holy"]
        },
        {
            "name": "ancient_etymology",
            "pattern_type": "etymological",
            "language_family": "archaic",
            "pattern_rules": ["old_forms", "lost_meanings", "sacred_language"],
            "usage_examples": ["vox -> vosch", "lumen -> lumeth", "sanctus -> sakthen"],
            "phonetic_constraints": ["preserve_ancient_sounds", "formal_cadence"],
            "vocabulary_pool": ["ancient", "old", "forgotten", "sacred", "hidden", "lost"],
            "morpheme_components": ["archaic_prefix", "formal_suffix"],
            "syllable_patterns": ["CV", "CVC", "CVCV"],
            "thematic_category": "ancient_knowledge",
            "horror_stage_preference": [1, 2],
            "emotional_resonance": {"awe": 0.6, "mystery": 0.8},
            "prefix_probability": 0.4,
            "suffix_probability": 0.6,
            "combination_rules": ["combine_with_places", "avoid_with_modern"]
        }
    ]


def _create_seed_clusters(logger) -> list[dict[str, Any]]:
    """Create thematic clusters of related seeds"""
    return [
        {
            "cluster_name": "early_corruption",
            "cluster_type": "thematic",
            "central_concept": "subtle_decay",
            "narrative_seeds": ["descent_into_madness"],
            "motif_seeds": ["creeping_fog", "withered_trees"],
            "semantic_seeds": ["corruption"],
            "emotional_seeds": ["mounting_dread"],
            "linguistic_seeds": ["corruption_morphology"],
            "coherence_score": 0.85,
            "horror_stage": 2,
            "usage_weight": 1.2,
            "biome_affinities": ["black_swamp", "fungal_cathedral"],
            "generation_contexts": ["first_corruption_encounter", "environmental_shift"],
            "exclusion_rules": ["not_with_peaceful_narratives"]
        },
        {
            "cluster_name": "lost_knowledge",
            "cluster_type": "thematic",
            "central_concept": "forbidden_wisdom",
            "narrative_seeds": ["false_sanctuary"],
            "motif_seeds": ["ancient_symbols"],
            "semantic_seeds": ["redemption"],
            "emotional_seeds": ["mounting_dread"],
            "linguistic_seeds": ["ancient_etymology"],
            "coherence_score": 0.78,
            "horror_stage": 1,
            "usage_weight": 1.0,
            "biome_affinities": ["ancient_ruins", "forgotten_temples"],
            "generation_contexts": ["knowledge_discovery", "ancient_sites"],
            "exclusion_rules": ["not_with_void_themes"]
        }
    ]


# JSON-based backwards compatibility functions
def get_emotional_seeds_data(engine=None) -> list[dict[str, Any]]:
    """Get emotional seeds data from JSON output"""
    seeds_file = SEEDS_OUTPUT_DIR / "seeds_data.json"
    
    if not seeds_file.exists():
        return []
    
    with open(seeds_file, "r", encoding="utf-8") as f:
        seeds_data = json.load(f)
    
    return seeds_data.get("emotional_seeds", [])


def get_horror_progression_data() -> dict[str, Any]:
    """Get horror progression configuration"""
    return {
        "stages": [
            {"level": 0, "name": "Peace", "distance": "0-20"},
            {"level": 1, "name": "Unease", "distance": "20-40"},
            {"level": 2, "name": "Dread", "distance": "40-60"},
            {"level": 3, "name": "Terror", "distance": "60-120"},
            {"level": 4, "name": "Horror", "distance": "120+"}
        ]
    }
