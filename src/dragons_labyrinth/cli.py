"""
Dragon's Labyrinth HBF Analysis CLI
"""

import logging
from pathlib import Path
from enum import Enum

import typer
import yaml
from importlib.resources import files as pkg_files
from xdg_base_dirs import xdg_cache_home
try:  # Python 3.11+
    import tomllib  # type: ignore[attr-defined]
except Exception:  # Fallback for older environments
    import tomli as tomllib  # type: ignore[assignment]

from rich.console import Console
from rich.panel import Panel
from rich.logging import RichHandler

from dragons_labyrinth.workflows.asset_generation.workflow import AssetGenerationWorkflow
from dragons_labyrinth.workflows.code_generation import generate_biome_rules

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
def generate_assets(
    toml_spec: Path = typer.Argument(
        ...,
        help="Path to TOML asset specification file"
    ),
    batch_size: int = typer.Option(
        5,
        "--batch-size", "-n",
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
        # Enforce fixed, safe output directory in repo
        output_dir = Path("crates/game-engine") / "assets" / asset_category
        
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
        
        # Treat success based on generated variants count to support new workflow state
        generated_count = len(result.get('generated_variants', {})) if isinstance(result, dict) else getattr(result, 'assets_generated', 0)
        failed_count = len(result.get('failed_generations', [])) if isinstance(result, dict) else getattr(result, 'assets_failed', 0)

        if generated_count > 0:
            log.info(f"[bold green]✨ Asset generation successful![/bold green]", extra={"markup": True})
            log.info(f"[green]📊 Generated: {generated_count} assets[/green]", extra={"markup": True})
            if failed_count:
                log.warning(f"[yellow]⚠️  Failed: {failed_count} assets[/yellow]", extra={"markup": True})
            log.info(f"[green]📁 Output directory: {output_dir}[/green]", extra={"markup": True})

            # List generated files
            asset_files = result.get('generated_variants', {}) if isinstance(result, dict) else getattr(result, 'asset_files', {})
            if asset_files:
                console.print("\n[bold]Generated Assets:[/bold]")
                for asset_name, file_path in asset_files.items():
                    console.print(f"  [cyan]{asset_name}:[/cyan] {file_path}")
        else:
            log.error(f"[bold red]❌ Asset generation failed[/bold red]", extra={"markup": True})
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
        from importlib.resources import files as pkg_files
        from xdg_base_dirs import xdg_cache_home
        
        # Discover packaged specs via importlib.resources
        prompts_root = pkg_files("dragons_labyrinth.workflows.asset_generation.prompts")
        toml_files = [p for p in prompts_root.rglob("*.toml")]
        
        # Filter out non-asset TOMLs 
        asset_specs = []
        for toml_res in toml_files:
            try:
                spec_data = tomllib.loads(toml_res.read_text(encoding="utf-8"))
                
                batch_info = spec_data.get('batch', {})
                assets_section = spec_data.get('assets', {})
                
                # Skip if no assets or no category
                if not assets_section or not batch_info.get('category'):
                    continue
                    
                # Skip GLOBAL_STYLE_GUIDE
                if str(toml_res.name) == 'GLOBAL_STYLE_GUIDE.toml':
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
                    'resource': toml_res,
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
        assets_base = Path("crates/game-engine") / "assets"
        
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
        
        # Ensure a cache staging area for materialized TOMLs
        spec_cache_root = xdg_cache_home() / "dragons_labyrinth" / "specs"
        spec_cache_root.mkdir(parents=True, exist_ok=True)
        
        for spec in asset_specs:
            category = spec['category']
            toml_res = spec['resource']
            expected_variants = spec['total_variants']
            
            console.print(f"\n[bold]Processing {category} category[/bold]")
            console.print(f"📝 Spec: {toml_res}")
            console.print(f"🎯 Expected variants: {expected_variants:,}")
            
            # Check existing assets for this category
            category_output_dir = Path("crates/game-engine") / "assets" / category
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
                
                # Materialize packaged TOML to XDG cache path to satisfy file-based loader
                # Preserve relative structure under prompts root if possible
                rel_parts = []
                try:
                    # Best-effort to compute a relative path
                    rel_parts = list(toml_res.parents)
                except Exception:
                    pass
                # Fall back to using resource name only
                target_toml = spec_cache_root / (str(toml_res.name))
                try:
                    target_toml.write_text(toml_res.read_text(encoding="utf-8"), encoding="utf-8")
                except Exception as e:
                    log.warning(f"[yellow]Failed to cache spec; continuing in-memory parse: {e}[/yellow]", extra={"markup": True})
                    # Still proceed, but workflow requires a Path; so bail if cannot write
                    raise
                
                # Generate assets for this category using modular workflow
                result = workflow.generate_assets(
                    asset_category=category,
                    toml_spec_path=target_toml,
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


@app.command(name="codegen-biomes")
def codegen_biomes(
    spec_path: Path | None = typer.Option(
        None,
        "--spec", "-s",
        help="Optional path to biome code generation spec (YAML); if omitted, use packaged spec"
    ),
):
    """
    Generate biome rules code from a separate YAML spec (kept separate from asset prompts).
    """
    console.print(Panel.fit(
        "[bold cyan]🦀 Biome Code Generation[/bold cyan]",
        subtitle="Rules → Rust via templates"
    ))
    try:
        import yaml
        from importlib.resources import files as pkg_files
        # Load spec from packaged resources if no path provided
        if spec_path is None:
            resource = pkg_files("dragons_labyrinth.workflows.code_generation.specs").joinpath("biomes.yaml")
            spec_text = resource.read_text(encoding="utf-8")
            spec = yaml.safe_load(spec_text) or {}
        else:
            if not spec_path.exists():
                log.error(f"[red]Spec not found: {spec_path}[/red]", extra={"markup": True})
                raise typer.Exit(code=1)
            with open(spec_path, "r", encoding="utf-8") as f:
                spec = yaml.safe_load(f) or {}
        # Always use the safe, repo-root default for engine src
        engine_src = Path("crates/game-engine/src")
        if not engine_src.exists():
            log.error(f"[red]Engine src not found at expected path: {engine_src}[/red]", extra={"markup": True})
            raise typer.Exit(code=1)
        out_file = generate_biome_rules(spec, engine_src)
        log.info(f"[green]✓ Generated: {out_file}[/green]", extra={"markup": True})
    except Exception as e:
        log.error(f"[bold red]Error: {e}[/bold red]", extra={"markup": True})
        import traceback
        traceback.print_exc()
        raise typer.Exit(code=1)


def main():
    """Main entry point"""
    app()
