"""
Seeds Extraction Subpackage

Simple run() function for seeds extraction from literature sources.
Follows .clinerules architectural patterns with modern Python standards.
"""

from datetime import datetime
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn
from sqlmodel import Session, SQLModel, create_engine, select

from .models import (
    Sources,
    NarrativeSeeds,
    MotifSeeds,
    SemanticSeeds,
    EmotionalSeeds,
    LinguisticSeeds,
    SeedClusters,
    ExtractionMetrics
)


def run(engine, logger, console: Console) -> dict[str, Any]:
    """
    Run seeds extraction pipeline.
    
    Args:
        engine: SQLModel database engine
        logger: Logger instance
        console: Rich console for output
        
    Returns:
        Dictionary containing extraction results
    """
    console.print("\n" + "="*60)
    console.print("📚 SEEDS EXTRACTION PIPELINE")
    console.print("="*60)
    
    with Session(engine) as session:
        # Create tables
        SQLModel.metadata.create_all(engine, checkfirst=True)
        console.print("✅ Seeds tables created/verified")
        
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
            narrative_count = _extract_narrative_seeds(session, logger)
            progress.update(task, description=f"Extracted {narrative_count} narrative seeds...")
            
            # Extract motif seeds  
            motif_count = _extract_motif_seeds(session, logger)
            progress.update(task, description=f"Extracted {motif_count} motif seeds...")
            
            # Extract semantic seeds
            semantic_count = _extract_semantic_seeds(session, logger)
            progress.update(task, description=f"Extracted {semantic_count} semantic seeds...")
            
            # Extract emotional seeds
            emotional_count = _extract_emotional_seeds(session, logger)
            progress.update(task, description=f"Extracted {emotional_count} emotional seeds...")
            
            # Extract linguistic seeds
            linguistic_count = _extract_linguistic_seeds(session, logger)
            progress.update(task, description=f"Extracted {linguistic_count} linguistic seeds...")
            
            # Create seed clusters
            cluster_count = _create_seed_clusters(session, logger)
            progress.update(task, description=f"Created {cluster_count} seed clusters...")
        
        # Record extraction metrics
        total_seeds = narrative_count + motif_count + semantic_count + emotional_count + linguistic_count
        duration = (datetime.now() - start_time).total_seconds()
        
        metrics = ExtractionMetrics(
            run_id=run_id,
            run_timestamp=start_time,
            total_sources=1,  # Placeholder
            sources_processed=1,
            narrative_seeds_extracted=narrative_count,
            motif_seeds_extracted=motif_count,
            semantic_seeds_extracted=semantic_count,
            emotional_seeds_extracted=emotional_count,
            linguistic_seeds_extracted=linguistic_count,
            extraction_duration_seconds=duration,
            average_confidence=0.8  # Placeholder
        )
        
        session.add(metrics)
        session.commit()
        
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
            "average_confidence": 0.8
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
        console.print("="*60 + "\n")
        
        return results


def _extract_narrative_seeds(session: Session, logger) -> int:
    """Extract narrative seed patterns from sources"""
    # Create sample narrative seeds for demonstration
    sample_narratives = [
        {
            "structure_name": "descent_into_madness",
            "structure_type": "linear",
            "story_beats": '["peaceful_beginning", "first_warning", "growing_unease", "revelation", "horror"]',
            "core_themes": '["corruption", "isolation", "loss_of_control"]',
            "conflict_types": '["internal", "supernatural"]',
            "horror_stage": 2,
            "corruption_arc": '["normal", "unsettled", "troubled", "broken"]'
        },
        {
            "structure_name": "companion_betrayal",
            "structure_type": "branching",
            "story_beats": '["trust_building", "shared_danger", "stress_accumulation", "breaking_point", "betrayal"]',
            "core_themes": '["trust", "survival", "moral_compromise"]',
            "conflict_types": '["interpersonal", "psychological"]',
            "horror_stage": 3,
            "corruption_arc": '["loyal", "strained", "desperate", "hostile"]'
        }
    ]
    
    # Create source entry if needed
    source = session.get(Sources, 1)
    if not source:
        source = Sources(
            id=1,
            source_type="literature",
            source_name="horror_literature_corpus",
            content_type="text",
            processing_status="processed"
        )
        session.add(source)
        session.commit()
    
    # Add narrative seeds
    count = 0
    for narrative_data in sample_narratives:
        narrative = NarrativeSeeds(
            source_id=source.id,
            **narrative_data,
            confidence_score=0.85
        )
        session.add(narrative)
        count += 1
    
    session.commit()
    return count


def _extract_motif_seeds(session: Session, logger) -> int:
    """Extract visual/thematic motif patterns"""
    sample_motifs = [
        {
            "name": "creeping_fog",
            "category": "visual",
            "description": "Supernatural fog that conceals and disorients",
            "keywords": '["fog", "mist", "obscured", "visibility", "lost"]',
            "atmosphere": "Confusion and helplessness",
            "dread_amplification": 0.7,
            "corruption_potential": 0.5
        },
        {
            "name": "ancient_symbols",
            "category": "symbolic",
            "description": "Mysterious symbols that predate known civilization",
            "keywords": '["symbols", "runes", "ancient", "carved", "stone"]',
            "atmosphere": "Dread of unknown knowledge",
            "dread_amplification": 0.8,
            "corruption_potential": 0.9
        }
    ]
    
    source = session.get(Sources, 1)
    count = 0
    for motif_data in sample_motifs:
        motif = MotifSeeds(
            source_id=source.id,
            **motif_data,
            confidence_score=0.80,
            frequency=5
        )
        session.add(motif)
        count += 1
    
    session.commit()
    return count


def _extract_semantic_seeds(session: Session, logger) -> int:
    """Extract semantic concept relationships"""
    sample_semantics = [
        {
            "concept": "corruption",
            "semantic_field": "moral_decay",
            "related_terms": '["taint", "decay", "rot", "infection", "pollution"]',
            "emotional_weight": -0.9,
            "horror_correlation": 0.95
        },
        {
            "concept": "isolation",
            "semantic_field": "psychological_state",
            "related_terms": '["alone", "cut_off", "abandoned", "separated", "disconnected"]',
            "emotional_weight": -0.7,
            "horror_correlation": 0.8
        }
    ]
    
    source = session.get(Sources, 1)
    count = 0
    for semantic_data in sample_semantics:
        semantic = SemanticSeeds(
            source_id=source.id,
            **semantic_data,
            confidence_score=0.75
        )
        session.add(semantic)
        count += 1
    
    session.commit()
    return count


def _extract_emotional_seeds(session: Session, logger) -> int:
    """Extract emotional progression patterns"""
    sample_emotions = [
        {
            "category": "fear",
            "intensity_level": 4,
            "progression_stages": '["unease", "worry", "anxiety", "terror", "panic"]',
            "trigger_events": '["strange_sounds", "missing_companions", "unnatural_phenomena"]',
            "horror_correlation": 0.95,
            "trauma_potential": 0.8,
            "companion_impact": '{"trust_loss": 0.3, "stress_increase": 0.5}'
        },
        {
            "category": "despair",
            "intensity_level": 3,
            "progression_stages": '["doubt", "pessimism", "hopelessness", "surrender"]',
            "trigger_events": '["repeated_failures", "companion_loss", "resource_depletion"]',
            "horror_correlation": 0.7,
            "trauma_potential": 0.9,
            "companion_impact": '{"morale_loss": 0.6, "cooperation_decrease": 0.4}'
        }
    ]
    
    source = session.get(Sources, 1)
    count = 0
    for emotion_data in sample_emotions:
        emotion = EmotionalSeeds(
            source_id=source.id,
            **emotion_data,
            confidence_score=0.82
        )
        session.add(emotion)
        count += 1
    
    session.commit()
    return count


def _extract_linguistic_seeds(session: Session, logger) -> int:
    """Extract linguistic patterns for name generation"""
    sample_linguistic = [
        {
            "pattern_type": "morphological",
            "language": "en",
            "pattern": "corruption_suffix",
            "description": "Suffixes that imply corruption or decay",
            "usage_examples": '["blight", "rot", "foul", "tainted"]',
            "thematic_category": "corruption",
            "name_generation_rules": '{"suffix_probability": 0.3, "combines_with": ["place", "creature"]}'
        }
    ]
    
    source = session.get(Sources, 1)
    count = 0
    for linguistic_data in sample_linguistic:
        linguistic = LinguisticSeeds(
            source_id=source.id,
            **linguistic_data,
            confidence_score=0.70
        )
        session.add(linguistic)
        count += 1
    
    session.commit()
    return count


def _create_seed_clusters(session: Session, logger) -> int:
    """Create thematic clusters of related seeds"""
    # Create sample cluster
    cluster = SeedClusters(
        cluster_name="horror_progression",
        cluster_type="thematic",
        centroid_concept="mounting_dread",
        narrative_seed_ids='[1, 2]',
        motif_seed_ids='[1, 2]',
        emotional_seed_ids='[1, 2]',
        member_count=6,
        coherence_score=0.85,
        horror_stage=2,
        recommended_usage='{"early_game": 0.3, "mid_game": 0.7, "late_game": 0.4}'
    )
    
    session.add(cluster)
    session.commit()
    return 1


# Backwards compatibility
def get_emotional_seeds_data(engine) -> list[dict[str, Any]]:
    """Get emotional seeds data for psychology integration"""
    with Session(engine) as session:
        emotional_seeds = session.exec(select(EmotionalSeeds)).all()
        return [
            {
                "category": seed.category,
                "intensity_level": seed.intensity_level,
                "horror_correlation": seed.horror_correlation,
                "progression_stages": seed.progression_stages,
                "companion_impact": seed.companion_impact
            }
            for seed in emotional_seeds
        ]


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
