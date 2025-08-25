"""
Dragon's Labyrinth HBF Analysis CLI
"""

import logging
from pathlib import Path
from enum import Enum

import typer
from rich.console import Console
from rich.panel import Panel
from rich.logging import RichHandler

from dragons_labyrinth.models import HBFConfig
from dragons_labyrinth.hbf.orchestrator import HBFOrchestrator

# Setup logging and console
console = Console()
app = typer.Typer(
    name="hbf",
    help="Dragon's Labyrinth HBF Analysis Tools",
    no_args_is_help=True,
    pretty_exceptions_enable=False
)

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
    datefmt="[%X]",
    handlers=[RichHandler(rich_tracebacks=True)]
)

log = logging.getLogger("hbf_cli")


class AnalysisMode(str, Enum):
    """Available analysis modes"""
    all = "all"
    summary = "summary"
    compress = "compress"
    cluster = "cluster"
    relationships = "relationships"


@app.command()
def analyze(
    hbf_path: Path = typer.Argument(
        Path("crates/hexroll-transformer/game.hbf"),
        help="Path to HBF database file"
    ),
    output_dir: Path = typer.Option(
        Path("crates/hexroll-transformer/analysis"),
        "--output", "-o",
        help="Output directory for results"
    ),
    mode: AnalysisMode = typer.Option(
        AnalysisMode.all,
        "--mode", "-m",
        help="Analysis mode to run"
    ),
    save_parquet: bool = typer.Option(
        True,
        "--save-parquet/--no-save-parquet",
        help="Save data as Parquet files"
    ),
    save_json: bool = typer.Option(
        True,
        "--save-json/--no-save-json",
        help="Save summary as JSON"
    )
):
    """
    Analyze HBF database and generate reports
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 Dragon's Labyrinth HBF Analyzer[/bold cyan]",
        subtitle="Analyzing world data"
    ))
    
    try:
        # Create orchestrator
        config = HBFConfig(hbf_path=hbf_path, output_dir=output_dir)
        orchestrator = HBFOrchestrator(config)
        
        # Load data
        orchestrator.load_data()
        
        # Run analyses based on mode
        if mode in [AnalysisMode.all, AnalysisMode.summary]:
            console.rule("[cyan]Summary Analysis[/cyan]")
            orchestrator.display_summary()
            
            if save_json:
                orchestrator.save_summary_json()
        
        if mode in [AnalysisMode.all, AnalysisMode.compress]:
            console.rule("[cyan]Compression Analysis[/cyan]")
            compressed_df = orchestrator.compress_entities()
            
            if save_parquet:
                output_path = config.output_dir / "entities_compressed.parquet"
                orchestrator.save_compressed(output_path)
        
        if mode in [AnalysisMode.all, AnalysisMode.cluster]:
            console.rule("[cyan]Clustering Analysis[/cyan]")
            clusters_df = orchestrator.cluster_entities()
            orchestrator.display_cluster_summary()
            
            if save_parquet:
                clusters_path = config.output_dir / "clusters.parquet"
                clusters_df.to_parquet(clusters_path, index=False)
                log.info(f"[green]✓ Saved clusters to {clusters_path}[/green]", extra={"markup": True})
        
        if mode in [AnalysisMode.all, AnalysisMode.relationships]:
            console.rule("[cyan]Relationship Analysis[/cyan]")
            graph = orchestrator.build_relationship_graph()
            orchestrator.display_graph_summary()
            
            if save_json:
                metrics = orchestrator.get_graph_metrics()
                import json
                metrics_path = config.output_dir / "graph_metrics.json"
                with open(metrics_path, 'w') as f:
                    json.dump(metrics, f, indent=2)
                log.info(f"[green]✓ Saved graph metrics to {metrics_path}[/green]", extra={"markup": True})
        
        # Save main dataframes if requested
        if save_parquet and mode == AnalysisMode.all:
            orchestrator.save_to_parquet()
        
        console.print("\n[bold green]✨ Analysis complete![/bold green]")
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        raise typer.Exit(code=1)


@app.command()
def quick(
    hbf_path: Path = typer.Argument(
        Path("crates/hexroll-transformer/game.hbf"),
        help="Path to HBF database file"
    )
):
    """
    Quick summary of HBF database
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 Quick HBF Summary[/bold cyan]"
    ))
    
    try:
        config = HBFConfig(
            hbf_path=hbf_path,
            output_dir=Path.cwd() / "temp"
        )
        orchestrator = HBFOrchestrator(config)
        orchestrator.load_data()
        orchestrator.display_summary()
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        raise typer.Exit(code=1)


@app.command()
def convert(
    hbf_path: Path = typer.Argument(
        ...,
        help="Path to HBF database file"
    ),
    output_path: Path = typer.Argument(
        ...,
        help="Output path for Parquet file"
    ),
    compress: bool = typer.Option(
        True,
        "--compress/--no-compress",
        help="Remove empty entities"
    )
):
    """
    Convert HBF to Parquet format
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 HBF to Parquet Converter[/bold cyan]"
    ))
    
    try:
        # Ensure output directory exists
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load HBF
        config = HBFConfig(
            hbf_path=hbf_path,
            output_dir=output_path.parent
        )
        orchestrator = HBFOrchestrator(config)
        orchestrator.load_data()
        
        # Compress if requested
        if compress:
            entities_df = orchestrator.compress_entities()
        else:
            entities_df = orchestrator.entities_df
        
        # Save to parquet
        entities_df.to_parquet(output_path, index=False)
        log.info(f"[green]✓ Saved to {output_path}[/green]", extra={"markup": True})
        
        # Save refs if exists
        if orchestrator.refs_df is not None:
            refs_path = output_path.with_stem(output_path.stem + "_refs")
            orchestrator.refs_df.to_parquet(refs_path, index=False)
            log.info(f"[green]✓ Saved refs to {refs_path}[/green]", extra={"markup": True})
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        raise typer.Exit(code=1)


@app.command()
def report(
    analysis_dir: Path = typer.Argument(
        Path("crates/hexroll-transformer/analysis"),
        help="Directory containing analysis results"
    ),
    output_path: Path = typer.Option(
        None,
        "--output", "-o",
        help="Output path for HTML report"
    )
):
    """
    Generate HTML report from analysis results
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 Report Generator[/bold cyan]"
    ))
    
    try:
        import json
        from pathlib import Path
        
        # Default output path
        if output_path is None:
            output_path = analysis_dir / "report.html"
        
        # Load analysis results
        summary_path = analysis_dir / "hbf_summary.json"
        if not summary_path.exists():
            log.error("[red]No analysis results found. Run 'analyze' first.[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        
        with open(summary_path) as f:
            summary = json.load(f)
        
        # Generate simple HTML report
        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>Dragon's Labyrinth HBF Analysis</title>
            <style>
                body {{ font-family: system-ui; padding: 2rem; background: #1a1a2e; color: #eee; }}
                h1 {{ color: #667eea; }}
                .stat {{ background: #16213e; padding: 1rem; margin: 1rem 0; border-radius: 8px; }}
                .value {{ font-size: 2em; color: #764ba2; }}
            </style>
        </head>
        <body>
            <h1>🐲 Dragon's Labyrinth HBF Analysis</h1>
            <div class="stat">
                <h3>Total Entities</h3>
                <div class="value">{summary.get('total_entities', 0):,}</div>
            </div>
            <div class="stat">
                <h3>Entities with Content</h3>
                <div class="value">{summary.get('entities_with_content', 0):,}</div>
            </div>
            <div class="stat">
                <h3>Empty Entities</h3>
                <div class="value">{summary.get('empty_entities', 0):,}</div>
            </div>
            <div class="stat">
                <h3>Unique Entity Types</h3>
                <div class="value">{summary.get('unique_entity_types', 0)}</div>
            </div>
        </body>
        </html>
        """
        
        with open(output_path, 'w') as f:
            f.write(html_content)
        
        log.info(f"[green]✓ Report saved to {output_path}[/green]", extra={"markup": True})
        log.info(f"[yellow]Open with: open {output_path}[/yellow]", extra={"markup": True})
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        raise typer.Exit(code=1)


def main():
    """Main entry point"""
    app()
