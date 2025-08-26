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
from dragons_labyrinth.workflows.asset_generation.workflow import AssetGenerationWorkflow

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


@app.command()
def generate_assets(
    toml_spec: Path = typer.Argument(
        ...,
        help="Path to TOML asset specification file"
    ),
    base_dir: Path = typer.Option(
        Path("crates/game-engine"),
        "--base-dir", "-b", 
        help="Base directory (assets will go in base_dir/assets/)"
    ),
    batch_size: int = typer.Option(
        5,
        "--batch-size", "-b",
        help="Number of assets to generate per batch"
    ),
    autonomous: bool = typer.Option(
        False,
        "--autonomous/--interactive",
        help="Skip human review checkpoints"
    ),
    skip_existing: bool = typer.Option(
        True,
        "--skip-existing/--overwrite",
        help="Skip assets that already exist"
    )
):
    """
    Generate game assets using DALL-E via LangChain workflow
    """
    console.print(Panel.fit(
        "[bold cyan]🎨 Dragon's Labyrinth Asset Generator[/bold cyan]",
        subtitle="LangGraph + DALL-E workflow"
    ))
    
    try:
        # Import modular workflow
        from dragons_labyrinth.workflows.asset_generation.workflow import AssetGenerationWorkflow
        
        # Validate TOML spec exists
        if not toml_spec.exists():
            log.error(f"[red]TOML specification not found: {toml_spec}[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        
        # Parse basic info from TOML filename
        try:
            import tomllib
        except ImportError:
            import tomli as tomllib
            
        with open(toml_spec, 'rb') as f:
            spec_data = tomllib.load(f)
        
        batch_info = spec_data.get('batch', {})
        asset_category = batch_info.get('category', 'unknown')
        level_range = batch_info.get('level_range', 'unknown')
        
        log.info(f"[cyan]📋 Generating {asset_category} assets for levels {level_range}[/cyan]", extra={"markup": True})
        log.info(f"[cyan]📝 Specification: {toml_spec}[/cyan]", extra={"markup": True})
        # Calculate output directory from base_dir and category
        output_dir = base_dir / "assets" / asset_category
        
        log.info(f"[cyan]📁 Output: {output_dir}[/cyan]", extra={"markup": True})
        log.info(f"[cyan]🤖 Mode: {'Autonomous' if autonomous else 'Interactive'}[/cyan]", extra={"markup": True})
        
        # Create workflow
        workflow = AssetGenerationWorkflow()
        
        # Execute asset generation
        console.rule("[cyan]Starting Asset Generation Workflow[/cyan]")
        
        result = workflow.generate_assets(
            asset_category=asset_category,
            level_range=level_range,
            toml_spec_path=toml_spec,
            output_dir=output_dir,
            batch_size=batch_size,
            autonomous_mode=autonomous,
            skip_existing=skip_existing
        )
        
        # Display results
        console.rule("[cyan]Generation Complete[/cyan]")
        
        if result.status == "SUCCESS":
            log.info(f"[bold green]✨ Asset generation successful![/bold green]", extra={"markup": True})
            log.info(f"[green]📊 Generated: {result.assets_generated}/{result.assets_requested} assets[/green]", extra={"markup": True})
            
            if result.assets_failed > 0:
                log.warning(f"[yellow]⚠️  Failed: {result.assets_failed} assets[/yellow]", extra={"markup": True})
            
            log.info(f"[green]⏱️  Processing time: {result.processing_time_seconds:.1f}s[/green]", extra={"markup": True})
            log.info(f"[green]📁 Output directory: {output_dir}[/green]", extra={"markup": True})
            
            # List generated files
            if result.asset_files:
                console.print("\n[bold]Generated Assets:[/bold]")
                for asset_name, file_path in result.asset_files.items():
                    console.print(f"  [cyan]{asset_name}:[/cyan] {file_path}")
            
        else:
            log.error(f"[bold red]❌ Asset generation failed[/bold red]", extra={"markup": True})
            log.error(f"[red]Generated: {result.assets_generated}, Failed: {result.assets_failed}[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        
    except ImportError as e:
        log.error(f"[red]Missing dependencies for asset generation: {e}[/red]", extra={"markup": True})
        log.error("[red]Install with: pip install langchain langchain-community[/red]", extra={"markup": True})
        raise typer.Exit(code=1)
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        import traceback
        traceback.print_exc()
        raise typer.Exit(code=1)


@app.command()
def generate_all_assets(
    base_dir: Path = typer.Option(
        Path("crates/game-engine"),
        "--base-dir", "-b", 
        help="Base directory (assets will go in base_dir/assets/)"
    ),
    specs_dir: Path = typer.Option(
        Path("crates/game-engine/prompts"),
        "--specs-dir", "-s",
        help="Directory containing TOML asset specifications"
    ),
    batch_size: int = typer.Option(
        10,
        "--batch-size", 
        help="Number of assets to generate per batch"
    ),
    max_variants: int = typer.Option(
        50,
        "--max-variants",
        help="Maximum variants per archetype (limits combinatorial explosion)"
    ),
    skip_existing: bool = typer.Option(
        True,
        "--skip-existing/--overwrite",
        help="Skip assets that already exist"
    )
):
    """
    Auto-discover and generate ALL assets from prompts directory with idempotency
    """
    console.print(Panel.fit(
        "[bold cyan]🚀 Dragon's Labyrinth Bulk Asset Generator[/bold cyan]",
        subtitle="Auto-discovery + Idempotent Generation"
    ))
    
    try:
        try:
            import tomllib
        except ImportError:
            import tomli as tomllib
        
        if not specs_dir.exists():
            log.error(f"[red]Specifications directory not found: {specs_dir}[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        
        # Auto-discover all TOML files recursively
        toml_files = list(specs_dir.glob("**/*.toml"))
        
        # Filter out non-asset TOMLs 
        asset_specs = []
        for toml_file in toml_files:
            try:
                with open(toml_file, 'rb') as f:
                    spec_data = tomllib.load(f)
                
                batch_info = spec_data.get('batch', {})
                assets_section = spec_data.get('assets', {})
                
                # Skip if no assets or no category
                if not assets_section or not batch_info.get('category'):
                    continue
                    
                # Skip GLOBAL_STYLE_GUIDE
                if toml_file.name == 'GLOBAL_STYLE_GUIDE.toml':
                    continue
                
                category = batch_info.get('category', 'unknown')
                asset_count = len(assets_section)
                variants_section = spec_data.get('variants', {})
                
                # Calculate potential variants
                total_variants = 0
                if variants_section:
                    # This is a universal variant system
                    for asset_name, asset_data in assets_section.items():
                        asset_variants = asset_data.get('variants', [])
                        if asset_variants:
                            variant_combinations = 1
                            for variant_dim in asset_variants:
                                variant_values = variants_section.get(variant_dim, [])
                                variant_combinations *= len(variant_values)
                            
                            # Limit combinatorial explosion
                            variant_combinations = min(variant_combinations, max_variants)
                            total_variants += variant_combinations
                        else:
                            total_variants += 1
                else:
                    # Traditional asset system
                    total_variants = asset_count
                
                asset_specs.append({
                    'file': toml_file,
                    'category': category,
                    'base_assets': asset_count,
                    'total_variants': total_variants,
                    'spec_data': spec_data
                })
                
            except Exception as e:
                log.warning(f"[yellow]Skipping {toml_file.name}: {e}[/yellow]", extra={"markup": True})
                
        if not asset_specs:
            log.warning(f"[yellow]No valid asset specifications found in {specs_dir}[/yellow]", extra={"markup": True})
            return
        
        # Display discovery results
        console.rule("[cyan]Asset Discovery Results[/cyan]")
        total_expected_assets = sum(spec['total_variants'] for spec in asset_specs)
        console.print(f"📋 Found [bold green]{len(asset_specs)}[/bold green] asset specifications")
        console.print(f"🎨 Expected total assets: [bold magenta]{total_expected_assets:,}[/bold magenta]")
        console.print()
        
        for spec in sorted(asset_specs, key=lambda x: x['category']):
            console.print(f"[cyan]{spec['category']}[/cyan]: {spec['base_assets']} base → {spec['total_variants']:,} variants")
        
        # Idempotency check - scan existing assets
        console.rule("[cyan]Idempotency Check[/cyan]")
        existing_assets = {}
        assets_base = base_dir / "assets"
        
        if assets_base.exists():
            for category_dir in assets_base.iterdir():
                if category_dir.is_dir():
                    category = category_dir.name
                    existing_files = list(category_dir.glob("**/*.png"))
                    existing_assets[category] = len(existing_files)
                    if existing_files:
                        console.print(f"📁 {category}: [green]{len(existing_files)} existing assets[/green]")
        
        # Generate missing assets
        console.rule("[cyan]Starting Bulk Generation[/cyan]")
        
        total_generated = 0
        total_skipped = 0
        failed_specs = []
        
        for spec in asset_specs:
            category = spec['category']
            toml_file = spec['file']
            expected_variants = spec['total_variants']
            
            console.print(f"\n[bold]Processing {category} category[/bold]")
            console.print(f"📝 Spec: {toml_file.relative_to(specs_dir)}")
            console.print(f"🎯 Expected variants: {expected_variants:,}")
            
            # Check existing assets for this category
            category_output_dir = base_dir / "assets" / category
            existing_count = 0
            if category_output_dir.exists():
                existing_count = len(list(category_output_dir.glob("**/*.png")))
            
            if skip_existing and existing_count >= expected_variants:
                console.print(f"✅ [green]Skipping {category} - {existing_count} assets already exist[/green]")
                total_skipped += existing_count
                continue
            
            try:
                # Create workflow using modular system
                workflow = AssetGenerationWorkflow()
                
                # Generate assets for this category using modular workflow
                result = workflow.generate_assets(
                    asset_category=category,
                    toml_spec_path=toml_file,
                    output_dir=category_output_dir,
                    batch_size=batch_size,
                    autonomous_mode=True
                )
                
                variants_generated = len(result.get('generated_variants', {}))
                if variants_generated > 0:
                    console.print(f"✅ [green]{category}: Generated {variants_generated} assets[/green]")
                    total_generated += variants_generated
                else:
                    console.print(f"❌ [red]{category}: Generation failed - STOPPING bulk generation[/red]") 
                    failed_specs.append(category)
                    # FAIL-FAST: Stop entire bulk generation on API issues
                    console.print(f"🚨 [red]STOPPING BULK GENERATION: API issues detected in {category}[/red]")
                    break
                    
            except ImportError as e:
                log.error(f"[red]Missing dependencies for {category}: {e}[/red]", extra={"markup": True})
                failed_specs.append(category)
                console.print(f"🚨 [red]STOPPING BULK GENERATION: Dependency issues[/red]")
                break
            except Exception as e:
                log.error(f"[red]Error generating {category}: {e}[/red]", extra={"markup": True})
                failed_specs.append(category)
                console.print(f"🚨 [red]STOPPING BULK GENERATION: Critical error in {category}[/red]")
                break
        
        # Final summary
        console.rule("[cyan]Bulk Generation Complete[/cyan]")
        console.print(f"✨ [bold green]Generated: {total_generated:,} new assets[/bold green]")
        if total_skipped > 0:
            console.print(f"⏭️  [yellow]Skipped: {total_skipped:,} existing assets[/yellow]")
        if failed_specs:
            console.print(f"❌ [red]Failed categories: {', '.join(failed_specs)}[/red]")
        
        console.print(f"\n🎯 [bold]Asset Directory: {base_dir / 'assets'}[/bold]")
        console.print(f"📊 [bold]Categories: {len(asset_specs)}[/bold]")
        console.print(f"🎨 [bold]Total Coverage: {total_generated + total_skipped:,} assets[/bold]")
        
    except Exception as e:
        log.error(f"[bold red]Bulk generation error: {e}[/bold red]", extra={"markup": True})
        import traceback
        traceback.print_exc()
        raise typer.Exit(code=1)


@app.command()
def list_asset_specs(
    specs_dir: Path = typer.Option(
        Path("crates/game-engine/prompts"),
        "--specs-dir", "-s",
        help="Directory containing TOML asset specifications"
    )
):
    """
    List available TOML asset specifications
    """
    console.print(Panel.fit(
        "[bold cyan]📋 Available Asset Specifications[/bold cyan]"
    ))
    
    try:
        try:
            import tomllib
        except ImportError:
            import tomli as tomllib
        
        if not specs_dir.exists():
            log.error(f"[red]Specifications directory not found: {specs_dir}[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        
        # Find all TOML files recursively
        toml_files = list(specs_dir.glob("**/*.toml"))
        
        if not toml_files:
            log.warning(f"[yellow]No TOML specifications found in {specs_dir}[/yellow]", extra={"markup": True})
            return
        
        console.print(f"\n[bold]Found {len(toml_files)} specifications:[/bold]\n")
        
        for toml_file in sorted(toml_files):
            try:
                with open(toml_file, 'rb') as f:
                    spec_data = tomllib.load(f)
                
                batch_info = spec_data.get('batch', {})
                assets_section = spec_data.get('assets', {})
                
                name = batch_info.get('name', toml_file.stem)
                description = batch_info.get('description', 'No description')
                category = batch_info.get('category', 'unknown')
                level_range = batch_info.get('level_range', 'unknown')
                asset_count = len(assets_section)
                
                console.print(f"[bold cyan]{toml_file.name}[/bold cyan]")
                console.print(f"  [green]Name:[/green] {name}")
                console.print(f"  [green]Category:[/green] {category}")
                console.print(f"  [green]Levels:[/green] {level_range}")
                console.print(f"  [green]Assets:[/green] {asset_count}")
                console.print(f"  [green]Description:[/green] {description}")
                console.print()
                
            except Exception as e:
                console.print(f"[red]❌ Error reading {toml_file.name}: {e}[/red]")
                
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        raise typer.Exit(code=1)


def main():
    """Main entry point"""
    app()
