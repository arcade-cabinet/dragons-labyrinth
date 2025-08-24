"""
Rich console interface for comprehensive API analysis review.

This module provides beautiful terminal interfaces for reviewing AI-generated
API analysis results, pattern schemas, and lesson progressions.
"""

import json
import os
from pathlib import Path

from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.columns import Columns
from rich.tree import Tree
from rich.syntax import Syntax
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.prompt import Prompt, Confirm
from rich.layout import Layout
from rich.align import Align

from professor_pixel.base import BaseComponent


class AnalysisReviewInterface(BaseComponent):
    """Rich console interface for comprehensive analysis review."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.console = Console()
        self.autonomous_mode = os.getenv("PROFESSOR_PIXEL_AUTONOMOUS_MODE", "false").lower() == "true"
    
    def show_startup_banner(self, session_id: str, library: str = "ARCADE") -> None:
        """Show startup banner with session information."""
        mode_text = "🤖 Autonomous" if self.autonomous_mode else "👤 Interactive"
        
        banner_content = f"""
[bold blue]🚀 Professor Pixel API Analysis[/bold blue]

[cyan]Session ID:[/cyan] {session_id}
[cyan]Mode:[/cyan] {mode_text}
[cyan]Library:[/cyan] Python {library.title()}

[dim]Starting comprehensive analysis of game development API...[/dim]
        """.strip()
        
        self.console.print(Panel(
            banner_content,
            title="[bold]Professor Pixel's Arcade Academy[/bold]",
            border_style="blue"
        ))
    
    def show_analysis_progress(self) -> list[str]:
        """Show progress during AI analysis steps."""
        steps = [
            "🔍 Analyzing API usage patterns...",
            "⚙️ Generating pattern schemas...",
            "📚 Designing lesson progression...",
            "📋 Creating analysis summary..."
        ]
        
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=self.console
        ) as progress:
            
            tasks = []
            for step in steps:
                task = progress.add_task(step, total=None)
                tasks.append(task)
                
        return tasks
    
    def display_comprehensive_summary(self, analysis_data: dict[str, object]) -> None:
        """Display the comprehensive analysis results."""
        
        # Header
        self.console.print("\n" + "="*80, style="bold blue")
        self.console.print(
            "📊 PROFESSOR PIXEL API ANALYSIS RESULTS", 
            style="bold blue", justify="center"
        )
        self.console.print("="*80 + "\n", style="bold blue")
        
        # Overview panel
        overview = self._create_overview_panel(analysis_data["analysis_summary"])
        
        # Pattern schemas table
        patterns_table = self._create_patterns_table(analysis_data["pattern_schemas"][:10])
        
        # Lesson progression tree
        lessons_tree = self._create_lessons_tree(analysis_data["lesson_progression"])
        
        # Key findings panel
        findings_panel = self._create_findings_panel(analysis_data["analysis_summary"]["key_findings"])
        
        # Layout everything
        layout = Layout()
        layout.split_column(
            Layout(overview, size=8),
            Layout().split_row(
                Layout(patterns_table),
                Layout(lessons_tree)
            ),
            Layout(findings_panel, size=12)
        )
        
        self.console.print(layout)
    
    def _create_overview_panel(self, summary: dict[str, object]) -> Panel:
        """Create overview statistics panel."""
        overview = summary["analysis_overview"]
        
        stats_table = Table.grid(padding=1)
        stats_table.add_column(style="cyan", justify="right")
        stats_table.add_column(style="white")
        
        stats_table.add_row("📊 API Functions Analyzed:", str(overview["total_api_functions"]))
        stats_table.add_row("🎯 Patterns Identified:", str(overview["patterns_identified"]))
        stats_table.add_row("📚 Lessons Designed:", str(overview["lessons_designed"]))
        stats_table.add_row("🎚️ Complexity Levels:", ", ".join(overview["complexity_levels"]))
        stats_table.add_row("🎮 Student Choice Points:", str(summary["proposed_changes"]["student_choice_points"]))
        
        return Panel(
            stats_table,
            title="[bold green]Analysis Overview[/bold green]",
            border_style="green"
        )
    
    def _create_patterns_table(self, pattern_schemas: list[dict[str, object]]) -> Table:
        """Create formatted table of pattern schemas."""
        table = Table(title="[bold yellow]Pattern Schemas[/bold yellow]")
        
        table.add_column("Opcode", style="cyan", no_wrap=True)
        table.add_column("Title", style="white")
        table.add_column("Complexity", style="yellow", justify="center")
        table.add_column("Choices", style="green", justify="center")
        table.add_column("Category", style="magenta")
        
        for pattern in pattern_schemas:
            complexity_color = {
                "BASIC": "green",
                "MODERATE": "yellow", 
                "ADVANCED": "red"
            }.get(pattern["complexity"], "white")
            
            title = pattern["title"]
            if len(title) > 40:
                title = title[:40] + "..."
            
            table.add_row(
                pattern["opcode"],
                title,
                f"[{complexity_color}]{pattern['complexity']}[/{complexity_color}]",
                str(len(pattern.get("choices", []))),
                pattern["category"]
            )
        
        return table
    
    def _create_lessons_tree(self, lesson_progression: dict[str, object]) -> Tree:
        """Create tree view of lesson progression."""
        tree = Tree("📚 [bold blue]Lesson Progression[/bold blue]")
        
        for lesson in lesson_progression["lessons"]:
            lesson_node = tree.add(f"[cyan]{lesson['id']}[/cyan]: {lesson['title']}")
            
            if lesson.get("patterns"):
                patterns_node = lesson_node.add("🎯 Patterns")
                patterns = lesson["patterns"]
                for pattern in patterns[:3]:  # Show first 3
                    patterns_node.add(f"[yellow]{pattern}[/yellow]")
                if len(patterns) > 3:
                    patterns_node.add(f"[dim]...and {len(patterns) - 3} more[/dim]")
            
            if lesson.get("student_choices"):
                choices_node = lesson_node.add("🎮 Student Choices")
                choices_node.add(f"[green]{lesson['student_choices']} choice points[/green]")
        
        return tree
    
    def _create_findings_panel(self, findings: dict[str, object]) -> Panel:
        """Create key findings panel."""
        findings_content = []
        
        # Most common patterns
        findings_content.append("[bold cyan]🔥 Most Common Patterns:[/bold cyan]")
        for pattern in findings["most_common_patterns"]:
            findings_content.append(f"  • {pattern}")
        
        findings_content.append("\n[bold green]✅ Beginner-Friendly Functions:[/bold green]")
        for func in findings["beginner_friendly_functions"]:
            findings_content.append(f"  • {func}")
        
        findings_content.append("\n[bold red]🚀 Advanced Concepts:[/bold red]")
        for concept in findings["advanced_concepts"]:
            findings_content.append(f"  • {concept}")
        
        return Panel(
            "\n".join(findings_content),
            title="[bold yellow]Key Findings[/bold yellow]",
            border_style="yellow"
        )
    
    def show_interactive_menu(self) -> str:
        """Show interactive menu for review actions."""
        
        self.console.print("\n" + "-"*60, style="dim")
        self.console.print("[bold]📝 Review Actions:[/bold]")
        
        actions = [
            ("approve_all", "✅ Approve all analysis results"),
            ("edit_usage", "🔍 Edit usage pattern analysis"),
            ("edit_patterns", "⚙️ Edit specific pattern schemas"),  
            ("edit_lessons", "📚 Edit lesson progression"),
            ("view_details", "👁️ View detailed results"),
            ("reject_all", "❌ Reject and restart analysis")
        ]
        
        for i, (key, description) in enumerate(actions, 1):
            self.console.print(f"  {i}. {description}")
        
        while True:
            choice = Prompt.ask(
                "\n[bold cyan]Choose an action[/bold cyan]",
                choices=[str(i) for i in range(1, len(actions) + 1)],
                default="1"
            )
            
            return actions[int(choice) - 1][0]
    
    def show_pattern_editor(self, pattern_schemas: list[dict[str, object]]) -> int:
        """Show pattern schemas for editing selection."""
        
        self.console.print("\n[bold yellow]🎯 Select Pattern to Edit:[/bold yellow]")
        
        table = Table()
        table.add_column("#", style="dim", width=3)
        table.add_column("Opcode", style="cyan")
        table.add_column("Title", style="white")
        table.add_column("Complexity", style="yellow")
        table.add_column("Choices", style="green")
        
        for i, pattern in enumerate(pattern_schemas):
            title = pattern["title"][:50]
            table.add_row(
                str(i),
                pattern["opcode"],
                title,
                pattern["complexity"],
                str(len(pattern.get("choices", [])))
            )
        
        self.console.print(table)
        
        while True:
            try:
                choice = int(Prompt.ask(
                    f"\n[cyan]Pattern number (0-{len(pattern_schemas)-1})[/cyan]",
                    default="0"
                ))
                if 0 <= choice < len(pattern_schemas):
                    return choice
            except ValueError:
                pass
            
            self.console.print("[red]Invalid selection[/red]")
    
    def show_pattern_details(self, pattern: dict[str, object]) -> str | None:
        """Show detailed pattern for editing."""
        
        self.console.print(f"\n[bold cyan]📝 Editing: {pattern['opcode']}[/bold cyan]")
        
        # Show current pattern as formatted JSON
        pattern_json = Syntax(
            json.dumps(pattern, indent=2),
            "json",
            theme="monokai",
            line_numbers=True
        )
        
        panel = Panel(
            pattern_json,
            title=f"[bold]{pattern['title']}[/bold]",
            border_style="cyan"
        )
        
        self.console.print(panel)
        
        # Get edits
        self.console.print("\n[yellow]Enter your edits (JSON format)[/yellow]")
        self.console.print("[dim]Press Ctrl+D when finished, or 'cancel' to abort[/dim]")
        
        lines = []
        try:
            while True:
                line = input()
                if line.strip() == 'cancel':
                    return None
                lines.append(line)
        except EOFError:
            pass
        
        return '\n'.join(lines)
    
    def show_detailed_results(self, detailed_results: dict[str, object]) -> None:
        """Show detailed syntax-highlighted results."""
        details = Syntax(
            json.dumps(detailed_results, indent=2),
            "json",
            theme="monokai", 
            line_numbers=True
        )
        self.console.print(Panel(details, title="[bold]Detailed Results[/bold]"))
    
    def show_success_summary(self, saved_patterns: int, saved_lessons: int) -> None:
        """Show success summary after saving."""
        
        success_content = f"""
[bold green]✅ Analysis Successfully Saved![/bold green]

📊 Results:
  • [cyan]{saved_patterns}[/cyan] pattern schemas saved to database
  • [cyan]{saved_lessons}[/cyan] lesson designs created
  • [green]Build system ready for lesson generation[/green]

🚀 Next Steps:
  • Run lesson compiler with new patterns
  • Generate student game templates
  • Test with autonomous mode: PROFESSOR_PIXEL_AUTONOMOUS_MODE=true
        """.strip()
        
        success_panel = Panel(
            success_content,
            title="[bold green]🎉 Success[/bold green]",
            border_style="green"
        )
        
        self.console.print(success_panel)
    
    def show_error(self, message: str) -> None:
        """Show error message."""
        self.console.print(f"[bold red]❌ {message}[/bold red]")
    
    def show_info(self, message: str) -> None:
        """Show info message.""" 
        self.console.print(f"[cyan]ℹ️ {message}[/cyan]")
    
    def show_success(self, message: str) -> None:
        """Show success message."""
        self.console.print(f"[bold green]✅ {message}[/bold green]")
