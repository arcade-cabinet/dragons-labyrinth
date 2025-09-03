"""
Analysis package main entry point.

Extracts HBF entities, clusters them, generates AI-powered models, and writes analysis files.
Run with: python -m generator.analysis
"""

import logging
import sqlite3
import shutil
from pathlib import Path

from rich.logging import RichHandler

from generator.analysis.models import RawEntities, AnalysisSummary
from generator.analysis.constants import (
    ANALYSIS_OUTPUT_DIR, HBF_RAW_FILE, PROCESSOR_MODELS_DIR
)
from generator.utils import get_git_root_dir, render_template_to_file


def setup_logging() -> logging.Logger:
    """Setup logging with Rich handler."""
    logging.basicConfig(
        level="INFO",
        format="%(message)s",
        datefmt="[%X]",
        handlers=[RichHandler(rich_tracebacks=True)]
    )
    return logging.getLogger(__name__)


def ensure_analysis_dirs(analysis_dir: Path, log: logging.Logger):
    """Ensure analysis subdirectories exist (idempotent)."""
    
    log.info("Ensuring analysis directories exist...")
    
    # Create directories if they don't exist (idempotent)
    for category in ["regions", "settlements", "factions", "dungeons"]:
        category_dir = analysis_dir / category
        if not category_dir.exists():
            category_dir.mkdir(parents=True, exist_ok=True)
            log.debug(f"Created {category_dir}")
        else:
            log.debug(f"Directory already exists: {category_dir}")


def extract_all_entities(hbf_path: Path, log: logging.Logger) -> RawEntities:
    """Extract all entities from HBF database using RawEntities model."""
    
    log.info(f"Extracting entities from {hbf_path}")
    entities_collection = RawEntities()
    
    with sqlite3.connect(hbf_path) as conn:
        cursor = conn.cursor()
        
        # Extract all entities with uuid and value
        cursor.execute("SELECT uuid, value FROM Entities")
        
        for uuid, value in cursor.fetchall():
            entities_collection.add_entity(uuid, value)
    
    log.info(f"Extracted {entities_collection.total_entities} total entities")
    return entities_collection


def generate_init_module(models_dir: Path, templates_dir: Path, log: logging.Logger):
    """Generate __init__.py module using Jinja template."""
    
    log.info("Generating __init__.py module...")
    
    # Find all generated Python model files
    model_files = []
    for py_file in models_dir.glob("*.py"):
        if py_file.name != "__init__.py":
            module_name = py_file.stem
            model_files.append(module_name)
    
    # Render template to __init__.py
    template_path = templates_dir / "init_module.j2"
    output_path = models_dir / "__init__.py"
    
    context = {
        "model_names": sorted(model_files)
    }
    
    render_template_to_file(template_path, output_path, context, log)
    log.info(f"✓ Generated __init__.py with {len(model_files)} model imports")


def print_analysis_summary(entities: RawEntities, all_results: dict[str, any], log: logging.Logger):
    """Print comprehensive summary of analysis results."""
    
    summary = entities.get_summary()
    
    log.info("\n=== ANALYSIS SUMMARY ===")
    
    for category, category_entities in summary.items():
        if category == "uncategorized":
            if category_entities > 0:
                log.info(f"\nUNCATEGORIZED: {category_entities} entities")
            continue
            
        log.info(f"\n{category.upper()}:")
        
        total_entities = 0
        for entity_name, count in category_entities.items():
            if count > 0:
                log.info(f"  {entity_name}: {count} entities")
                total_entities += count
        
        log.info(f"  TOTAL {category}: {total_entities} entities")
        
        # Show AI model generation results
        if category in all_results:
            results = all_results[category]
            if results.success:
                log.info(f"  ✓ AI models generated: {len(results.models_generated)} files")
            else:
                log.warning(f"  ✗ AI model generation failed")
    
    log.info(f"\nTotal entities processed: {entities.total_entities}")
    log.info("Analysis directories created in analysis/")
    log.info("AI-generated processor models available for review")
    log.info("Ready for processor phase!")


def main():
    """Run the complete analysis pipeline with clean model architecture."""
    log = setup_logging()
    
    try:
        # Setup paths
        root_dir = get_git_root_dir(logger=log)
        hbf_raw_file = root_dir / HBF_RAW_FILE
        analysis_output_dir = root_dir / ANALYSIS_OUTPUT_DIR
        processor_models_dir = root_dir / PROCESSOR_MODELS_DIR
        templates_dir = Path(__file__).parent / "templates"
        
        log.info("Starting Dragons Labyrinth HBF analysis with refactored model architecture...")
        
        # Ensure analysis directories exist (idempotent)
        ensure_analysis_dirs(analysis_output_dir, log)

        # Ensure processor models directory exists
        processor_models_dir.mkdir(parents=True, exist_ok=True)
        
        # Extract all entities using RawEntities model
        entities = extract_all_entities(hbf_raw_file, log)
        
        # Write all entities to disk in cluster directories
        entities.write_all_entities(analysis_output_dir, log)
        
        # PHASE 1: Generate individual category models
        phase1_results = entities.generate_all_individual_models(
            processor_models_dir, templates_dir, log
        )
        
        # PHASE 2 & 3: Generate container models
        container_results = entities.generate_container_models(
            processor_models_dir, templates_dir, phase1_results, log
        )
        
        # Generate __init__.py module using template
        generate_init_module(processor_models_dir, templates_dir, log)
        
        # Combine all generation results
        all_results = {**phase1_results, **container_results}
        
        # Print comprehensive summary
        print_analysis_summary(entities, all_results, log)
        
        log.info("Complete 3-phase analysis orchestration finished!")
        
    except Exception as e:
        log.error(f"Analysis failed: {e}")
        raise


if __name__ == "__main__":
    main()
