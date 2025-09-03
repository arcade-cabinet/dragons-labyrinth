"""
Seeds System - Literature-based Narrative Patterns

Independent system for extracting narrative seeds from literature sources.
No SQLite, no database - pure JSON output for atmospheric game enhancement.
"""

import typer
from pathlib import Path
from rich.console import Console

from src.generator.constants import SEEDS_OUTPUT_DIR
from src.generator.seeds import run

app = typer.Typer(
    name="seeds", 
    help="Dragon's Labyrinth Literature Seeds Processor",
    no_args_is_help=True
)

console = Console()

@app.command("extract")
def extract_seeds(
    output_dir: str = typer.Option(str(SEEDS_OUTPUT_DIR), help="Output directory for seeds JSON")
) -> None:
    """Extract narrative seeds from literature sources into JSON for game integration."""
    
    console.print("📚 [bold cyan]Dragon's Labyrinth - Literature Seeds Processor[/bold cyan]")
    console.print(f"📁 Output: {output_dir}")
    
    # Create output directory
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # Run seeds extraction (compatible with existing interface)
    console.print("🔄 [yellow]Extracting narrative seeds from literature sources...[/yellow]")
    
    # Create a simple logger for compatibility
    import logging
    logging.basicConfig(level=logging.INFO, format="%(message)s")
    logger = logging.getLogger("seeds")
    
    # Run the seeds extraction
    results = run(
        engine=None,  # Not used in JSON-only version
        logger=logger,
        console=console
    )
    
    # Success summary
    console.print("\n✅ [bold green]Literature Seeds Extraction Complete![/bold green]")
    console.print(f"📊 Total seeds: {results.get('total_seeds', 0)}")
    console.print(f"   - Narrative: {results.get('narrative_seeds', 0)}")
    console.print(f"   - Motif: {results.get('motif_seeds', 0)}")
    console.print(f"   - Semantic: {results.get('semantic_seeds', 0)}")
    console.print(f"   - Emotional: {results.get('emotional_seeds', 0)}")
    console.print(f"   - Linguistic: {results.get('linguistic_seeds', 0)}")
    console.print(f"🎯 JSON Output: {results.get('json_output', 'N/A')}")

@app.command("test")
def test_seeds() -> None:
    """Test the seeds extraction system."""
    
    console.print("🧪 [bold cyan]Testing Seeds Extraction[/bold cyan]")
    
    # Test imports
    try:
        from src.generator.seeds import get_emotional_seeds_data, get_horror_progression_data
        console.print("✅ Seeds functions imported successfully")
        
        # Test horror progression data
        progression_data = get_horror_progression_data()
        console.print(f"✅ Horror progression stages: {len(progression_data.get('stages', []))}")
        
        console.print("🎯 Seeds system ready for extraction!")
        
    except Exception as e:
        console.print(f"❌ Seeds test error: {e}")

if __name__ == "__main__":
    app()
