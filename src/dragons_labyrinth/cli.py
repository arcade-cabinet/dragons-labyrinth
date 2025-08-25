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
from dragons_labyrinth.hbf.content_processor import HBFContentProcessor
from dragons_labyrinth.hbf.game_transformer import GameTransformer
from dragons_labyrinth.hbf.hbf_processor import process_hbf_to_game_world

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
def transform(
    hbf_path: Path = typer.Argument(
        Path("crates/hexroll-transformer/game.hbf"),
        help="Path to HBF database file"
    ),
    output_dir: Path = typer.Option(
        Path("crates/hexroll-transformer/game-output"),
        "--output", "-o",
        help="Output directory for game files"
    ),
    content_only: bool = typer.Option(
        False,
        "--content-only",
        help="Only process content, skip game transformation"
    )
):
    """
    Complete HBF to Game transformation pipeline
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 HBF to Game Transformer[/bold cyan]",
        subtitle="Complete pipeline transformation"
    ))
    
    try:
        # Step 1: Load HBF data
        console.rule("[cyan]Step 1: Loading HBF Data[/cyan]")
        config = HBFConfig(hbf_path=hbf_path, output_dir=output_dir)
        orchestrator = HBFOrchestrator(config)
        orchestrator.load_data()
        orchestrator.display_summary()
        
        # Step 2: Process content
        console.rule("[cyan]Step 2: Processing Content[/cyan]")
        processor = HBFContentProcessor()
        processed_content = processor.process_entities(orchestrator.state.entities_df)
        
        # Display processing results
        stats = processed_content['content_statistics']
        console.print(f"📊 Processed [bold magenta]{stats['total_entities_processed']:,}[/bold magenta] entities")
        console.print(f"🔗 Found [bold magenta]{stats['total_references']:,}[/bold magenta] references")
        console.print(f"🎯 Extracted [bold magenta]{stats['total_quest_hooks']:,}[/bold magenta] quest hooks")
        console.print(f"🗺️ Built [bold magenta]{stats['spatial_relationships']:,}[/bold magenta] spatial relationships")
        
        # Export processed content
        content_files = processor.export_for_game_engine(processed_content, str(output_dir / "content"))
        console.print(f"💾 Exported content to [bold green]{len(content_files)}[/bold green] files")
        
        if content_only:
            console.print("\n[bold green]✨ Content processing complete![/bold green]")
            return
        
        # Step 3: Transform to game format
        console.rule("[cyan]Step 3: Transforming to Game Format[/cyan]")
        transformer = GameTransformer()
        game_data = transformer.transform_to_game_format(processed_content)
        
        # Display transformation results
        console.print(f"🎮 Generated [bold magenta]{len(game_data['entities']):,}[/bold magenta] game entities")
        console.print(f"🗺️ Built [bold magenta]{len(game_data['spatial_grid']['hex_grid']):,}[/bold magenta] hex grid")
        console.print(f"⚔️ Found [bold magenta]{len(game_data['quest_system']['quest_givers']):,}[/bold magenta] quest givers")
        console.print(f"🏛️ Identified [bold magenta]{len(game_data['faction_network']['factions']):,}[/bold magenta] factions")
        console.print(f"🎨 Generated [bold magenta]{len(game_data['asset_requirements']):,}[/bold magenta] asset requirements")
        
        # Export game data
        game_files = transformer.export_game_data(game_data, str(output_dir / "game"))
        console.print(f"💾 Exported game data to [bold green]{len(game_files)}[/bold green] files")
        
        # List generated files
        console.rule("[cyan]Generated Files[/cyan]")
        all_files = {**content_files, **game_files}
        for file_type, file_path in all_files.items():
            console.print(f"  [cyan]{file_type}:[/cyan] {file_path}")
        
        console.print("\n[bold green]✨ Complete transformation pipeline finished![/bold green]")
        console.print(f"[yellow]🎯 Output directory: {output_dir}[/yellow]")
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        import traceback
        traceback.print_exc()
        raise typer.Exit(code=1)


@app.command()
def world(
    hbf_path: Path = typer.Argument(
        Path("crates/hexroll-transformer/game.hbf"),
        help="Path to HBF database file"
    ),
    output_dir: Path = typer.Option(
        Path("crates/hexroll-transformer/world-output"),
        "--output", "-o",
        help="Output directory for world files"
    )
):
    """
    Process HBF world data with proper structure understanding
    """
    console.print(Panel.fit(
        "[bold cyan]🐲 HBF World Processor[/bold cyan]",
        subtitle="Understanding HBF structure properly"
    ))
    
    try:
        # Use the specialized processor that understands HBF structure
        result = process_hbf_to_game_world(str(hbf_path), str(output_dir))
        
        # Display generated files
        console.rule("[cyan]Generated Files[/cyan]")
        for file_type, file_path in result['generated_files'].items():
            console.print(f"  [cyan]{file_type}:[/cyan] {file_path}")
        
        console.print(f"\n[bold green]✨ World processing complete![/bold green]")
        console.print(f"[yellow]🎯 Output directory: {output_dir}[/yellow]")
        
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        import traceback
        traceback.print_exc()
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
