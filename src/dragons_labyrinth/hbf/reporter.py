"""
HBF Report Generation Mixin
Generates HTML and JSON reports from HBF analysis
"""

import json
from pathlib import Path
from datetime import datetime

from jinja2 import Environment, FileSystemLoader

from dragons_labyrinth.models import HBFSummary
from dragons_labyrinth.hbf.base import DataFrameMixin


class HBFReporter(DataFrameMixin):
    """
    Reporter mixin for generating reports from HBF analysis results.
    Uses shared OrchestratorState for configuration and logging.
    """
    
    def __init__(self, state):
        """Initialize with shared orchestrator state"""
        super().__init__(state)
        
        # Setup Jinja2 environment
        template_dir = Path(__file__).parent.parent / "templates"
        self.jinja_env = Environment(
            loader=FileSystemLoader(template_dir),
            autoescape=True
        )
    
    def generate_html_report(
        self,
        summary: HBFSummary | dict,
        output_path: Path | None = None,
        diagnostics: dict | None = None,
        compression_stats: dict | None = None
    ) -> Path:
        """Generate HTML report from analysis results"""
        
        # Convert Pydantic model to dict if needed
        if isinstance(summary, HBFSummary):
            summary_dict = summary.model_dump()
        else:
            summary_dict = summary
        
        # Default output path
        if output_path is None:
            output_path = self.config.output_dir / "report.html"
        else:
            output_path = Path(output_path)
        
        # Prepare template context
        context = {
            'config': self.config,
            'summary': summary_dict,
            'timestamp': datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
            'diagnostics': diagnostics,
            'compression_stats': compression_stats
        }
        
        # Render template
        template = self.jinja_env.get_template("hbf_report.html")
        html_content = template.render(**context)
        
        # Write to file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(html_content)
        
        self.log.info(f"[green]✓ HTML report saved to {output_path}[/green]", extra={"markup": True})
        self.console.print(f"[yellow]Open with: open {output_path}[/yellow]")
        
        return output_path
    
    def generate_json_report(
        self,
        summary: HBFSummary | dict,
        output_path: Path | None = None,
        diagnostics: dict | None = None,
        compression_stats: dict | None = None,
        graph_metrics: dict | None = None
    ) -> Path:
        """Generate JSON report from analysis results"""
        
        # Convert Pydantic model to dict if needed
        if isinstance(summary, HBFSummary):
            summary_dict = summary.model_dump()
        else:
            summary_dict = summary
        
        # Default output path
        if output_path is None:
            output_path = self.config.output_dir / "report.json"
        else:
            output_path = Path(output_path)
        
        # Compile full report
        report = {
            'metadata': {
                'hbf_path': str(self.config.hbf_path),
                'generated': datetime.now().isoformat(),
                'version': '1.0.0'
            },
            'summary': summary_dict
        }
        
        # Add optional sections
        if diagnostics:
            report['diagnostics'] = diagnostics
        if compression_stats:
            report['compression'] = compression_stats
        if graph_metrics:
            report['graph_metrics'] = graph_metrics
        
        # Write to file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2)
        
        self.log.info(f"[green]✓ JSON report saved to {output_path}[/green]", extra={"markup": True})
        
        return output_path
    
    def generate_markdown_report(
        self,
        summary: HBFSummary | dict,
        output_path: Path | None = None,
        diagnostics: dict | None = None,
        compression_stats: dict | None = None
    ) -> Path:
        """Generate Markdown report from analysis results"""
        
        # Convert Pydantic model to dict if needed
        if isinstance(summary, HBFSummary):
            summary_dict = summary.model_dump()
        else:
            summary_dict = summary
        
        # Default output path
        if output_path is None:
            output_path = self.config.output_dir / "report.md"
        else:
            output_path = Path(output_path)
        
        # Build markdown content
        lines = [
            "# Dragon's Labyrinth HBF Analysis Report",
            "",
            f"**File:** {self.config.hbf_path.name}",
            f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
            "",
            "## Summary Statistics",
            "",
            f"- **Total Entities:** {summary_dict['total_entities']:,}",
            f"- **Entities with Content:** {summary_dict['entities_with_content']:,}",
            f"- **Empty Entities:** {summary_dict['empty_entities']:,}",
            f"- **Unique Entity Types:** {summary_dict['unique_entity_types']}",
            f"- **Total References:** {summary_dict['total_references']:,}",
            ""
        ]
        
        # Add type distribution
        if summary_dict.get('type_distribution'):
            lines.extend([
                "## Entity Type Distribution",
                "",
                "| Entity Type | Count | Percentage |",
                "|-------------|-------|------------|"
            ])
            
            total = summary_dict['entities_with_content']
            for entity_type, count in list(summary_dict['type_distribution'].items())[:20]:
                percentage = (count / total * 100) if total > 0 else 0
                lines.append(f"| {entity_type or '(unknown)'} | {count:,} | {percentage:.2f}% |")
            
            lines.append("")
        
        # Add compression stats
        if compression_stats:
            lines.extend([
                "## Compression Statistics",
                "",
                f"- **Original Entities:** {compression_stats.get('original_count', 0):,}",
                f"- **Compressed Entities:** {compression_stats.get('compressed_count', 0):,}",
                f"- **Removed:** {compression_stats.get('empty_removed', 0):,}",
                f"- **Reduction:** {compression_stats.get('reduction_percentage', 0):.1f}%",
                ""
            ])
        
        # Write to file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        self.log.info(f"[green]✓ Markdown report saved to {output_path}[/green]", extra={"markup": True})
        
        return output_path
    
    def generate_all_reports(
        self,
        summary: HBFSummary | dict,
        diagnostics: dict | None = None,
        compression_stats: dict | None = None,
        graph_metrics: dict | None = None
    ) -> dict[str, Path]:
        """Generate all report formats"""
        
        reports = {}
        
        # Generate each format
        reports['html'] = self.generate_html_report(
            summary, 
            diagnostics=diagnostics,
            compression_stats=compression_stats
        )
        
        reports['json'] = self.generate_json_report(
            summary,
            diagnostics=diagnostics,
            compression_stats=compression_stats,
            graph_metrics=graph_metrics
        )
        
        reports['markdown'] = self.generate_markdown_report(
            summary,
            diagnostics=diagnostics,
            compression_stats=compression_stats
        )
        
        self.console.print("\n[bold green]✨ All reports generated successfully![/bold green]")
        
        return reports
