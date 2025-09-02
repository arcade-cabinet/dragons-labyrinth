"""
Dragon's Labyrinth Generator

Independent JSON-only systems - no SQLite, no orchestration.
Use submodules directly:
- python -m src.generator.entities extract
- python -m src.generator.seeds extract
"""

import typer
from rich.console import Console

app = typer.Typer(
    name="generator",
    help="Dragon's Labyrinth Generator - Independent JSON Systems",
    no_args_is_help=True
)

console = Console()

@app.command("info")
def show_info() -> None:
    """Show information about available independent systems."""
    
    console.print("🐉 [bold cyan]Dragon's Labyrinth Generator[/bold cyan]")
    console.print("\n[bold yellow]Available Independent Systems:[/bold yellow]")
    console.print("📊 [cyan]Entities System[/cyan]: Process 70,801 HBF entities → JSON")
    console.print("   Command: [green]python -m src.generator.entities extract[/green]")
    console.print("\n📚 [cyan]Seeds System[/cyan]: Extract literature narrative patterns → JSON")  
    console.print("   Command: [green]python -m src.generator.seeds extract[/green]")
    console.print("\n[bold green]Architecture:[/bold green] Post-SQLite elimination, clean JSON-only pipeline")

if __name__ == "__main__":
    app()
