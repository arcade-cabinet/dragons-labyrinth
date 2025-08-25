"""
HBF Orchestrator - Main interface for HBF data management
Inherits from all mixin classes for complete functionality
"""

import json
import logging
import sqlite3
from pathlib import Path
from typing import Any

import pandas as pd
import networkx as nx
from sqlalchemy import create_engine
from rich.console import Console
from rich.logging import RichHandler
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.table import Table
from rich import box
from rich.panel import Panel

from dragons_labyrinth.models import HBFConfig, HBFSummary, OrchestratorState
from dragons_labyrinth.types import (
    EntitiesDataFrame, 
    ReferencesDataFrame, 
    ClusterDataFrame,
    EntityGraph,
    SQLiteEngine
)
from dragons_labyrinth.hbf.filter import EntityFilter
from dragons_labyrinth.hbf.compressor import EntityCompressor
from dragons_labyrinth.hbf.analysis import HBFAnalysis
from dragons_labyrinth.hbf.diagnostics import HBFDiagnostics
from dragons_labyrinth.hbf.reporter import HBFReporter
from dragons_labyrinth.hbf.entity_classifier import classify_entity_smart

# Setup logging
FORMAT = "%(message)s"
logging.basicConfig(
    level=logging.INFO,
    format=FORMAT,
    datefmt="[%X]",
    handlers=[RichHandler(rich_tracebacks=True)]
)

log = logging.getLogger("hbf_orchestrator")
console = Console()


class HBFOrchestrator(
    EntityFilter,
    EntityCompressor,
    HBFAnalysis,
    HBFDiagnostics,
    HBFReporter
):
    """
    Main orchestrator for HBF data management.
    
    Inherits from all mixin classes and provides the core loading functionality.
    Manages the shared state that all mixins operate on.
    """
    
    def __init__(self, config: HBFConfig):
        """Initialize orchestrator with configuration"""
        # Create shared state
        state = OrchestratorState(
            config=config,
            log=log,
            console=console
        )
        
        # Initialize all mixins with shared state - call each parent
        EntityFilter.__init__(self, state)
        EntityCompressor.__init__(self, state)
        HBFAnalysis.__init__(self, state)
        HBFDiagnostics.__init__(self, state)
        HBFReporter.__init__(self, state)
        
        # Store state reference for loader methods
        self.state = state
        
        # Additional state for compatibility
        self.engine: SQLiteEngine | None = None
        self.compressed_df: EntitiesDataFrame | None = None
        self.clusters_df: ClusterDataFrame | None = None
        self.graph: EntityGraph | None = None
    
    # Core loader functionality (merged from loader.py)
    
    def _setup_engine(self):
        """Setup SQLAlchemy engine and SQLite connection"""
        if not self.config.hbf_path.exists():
            raise FileNotFoundError(f"HBF file not found: {self.config.hbf_path}")
        
        db_url = f"sqlite:///{self.config.hbf_path}"
        self.engine = create_engine(db_url)
        self.state.sqlalchemy_engine = self.engine
        
        # Also create raw SQLite connection for diagnostics
        self.state.sqlite_conn = sqlite3.connect(str(self.config.hbf_path))
        
        log.info(f"[green]Connected to HBF database: {self.config.hbf_path}[/green]", extra={"markup": True})
    
    def load_data(self) -> tuple[pd.DataFrame, pd.DataFrame]:
        """Load entities and refs into pandas DataFrames"""
        # Setup connections
        self._setup_engine()
        
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console
        ) as progress:
            # Load Entities table
            task = progress.add_task("[cyan]Loading entities...", total=None)
            self.state.entities_df = pd.read_sql_table('Entities', self.engine)
            progress.update(task, completed=True)
            
            # Load Refs table
            task = progress.add_task("[cyan]Loading references...", total=None)
            self.state.refs_df = pd.read_sql_table('Refs', self.engine)
            progress.update(task, completed=True)
        
        # Parse JSON values in entities
        self.state.entities_df['data'] = self.state.entities_df['value'].apply(self._parse_json)
        
        # Extract entity types using smart classifier
        self.state.entities_df['entity_type'] = self.state.entities_df['value'].apply(classify_entity_smart)
        
        log.info(f"[green]✓ Loaded {len(self.state.entities_df):,} entities[/green]", extra={"markup": True})
        log.info(f"[green]✓ Loaded {len(self.state.refs_df):,} references[/green]", extra={"markup": True})
        
        return self.state.entities_df, self.state.refs_df
    
    def _parse_json(self, value: str) -> dict:
        """Safely parse JSON value"""
        if not value or value == '{}':
            return {}
        try:
            return json.loads(value)
        except (json.JSONDecodeError, TypeError):
            log.warning(f"Failed to parse JSON value")
            return {}
    
    def get_summary_stats(self) -> dict:
        """Get summary statistics"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        # Filter entities with content
        with_content = self.state.entities_df[self.state.entities_df['value'] != '{}']
        
        return {
            'total_entities': len(self.state.entities_df),
            'entities_with_content': len(with_content),
            'empty_entities': len(self.state.entities_df) - len(with_content),
            'unique_entity_types': self.state.entities_df['entity_type'].nunique(),
            'total_references': len(self.state.refs_df) if self.state.refs_df is not None else 0,
            'type_distribution': with_content['entity_type'].value_counts().to_dict()
        }
    
    def get_summary_model(self) -> HBFSummary:
        """Get summary statistics as Pydantic model"""
        stats_dict = self.get_summary_stats()
        return HBFSummary(**stats_dict)
    
    def display_summary(self):
        """Display summary statistics using Rich tables"""
        stats = self.get_summary_stats()
        
        # Main stats table
        table = Table(title="HBF Database Summary", box=box.ROUNDED)
        table.add_column("Metric", style="cyan", no_wrap=True)
        table.add_column("Value", style="magenta")
        table.add_column("Percentage", style="green")
        
        total = stats['total_entities']
        with_content = stats['entities_with_content']
        empty = stats['empty_entities']
        
        table.add_row("Total Entities", f"{total:,}", "100%")
        table.add_row("With Content", f"{with_content:,}", f"{(with_content/total)*100:.1f}%")
        table.add_row("Empty Entities", f"{empty:,}", f"{(empty/total)*100:.1f}%")
        table.add_row("Unique Types", f"{stats['unique_entity_types']}", "-")
        table.add_row("Total References", f"{stats['total_references']:,}", "-")
        
        console.print("\n")
        console.print(table)
        
        # Type distribution table
        if stats['type_distribution']:
            type_table = Table(title="Entity Type Distribution (Top 10)", box=box.ROUNDED)
            type_table.add_column("Entity Type", style="cyan")
            type_table.add_column("Count", style="magenta")
            
            for entity_type, count in list(stats['type_distribution'].items())[:10]:
                type_table.add_row(entity_type or "(no type)", f"{count:,}")
            
            console.print("\n")
            console.print(type_table)
    
    def save_to_parquet(self):
        """Save DataFrames to Parquet format for faster loading"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        entities_path = self.config.output_dir / "entities.parquet"
        refs_path = self.config.output_dir / "refs.parquet"
        
        self.state.entities_df.to_parquet(entities_path, index=False)
        log.info(f"[green]✓ Saved entities to {entities_path}[/green]", extra={"markup": True})
        
        if self.state.refs_df is not None:
            self.state.refs_df.to_parquet(refs_path, index=False)
            log.info(f"[green]✓ Saved refs to {refs_path}[/green]", extra={"markup": True})
    
    def save_summary_json(self):
        """Save summary statistics to JSON"""
        stats = self.get_summary_stats()
        output_path = self.config.output_dir / "hbf_summary.json"
        
        with open(output_path, 'w') as f:
            json.dump(stats, f, indent=2)
        
        log.info(f"[green]✓ Saved summary to {output_path}[/green]", extra={"markup": True})
    
    def filter_by_type(self, entity_type: str) -> pd.DataFrame:
        """Filter entities by type"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        return self.state.entities_df[self.state.entities_df['entity_type'] == entity_type]
    
    def get_non_empty_entities(self) -> pd.DataFrame:
        """Get only entities with content"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        return self.state.entities_df[self.state.entities_df['value'] != '{}']
    
    def execute_sql(self, query: str) -> pd.DataFrame:
        """Execute arbitrary SQL query on the HBF database"""
        if self.engine is None:
            raise ValueError("Database not connected. Call load_data() first.")
        
        return pd.read_sql_query(query, self.engine)
    
    def get_entity_by_id(self, entity_id: str) -> dict | None:
        """Get a single entity by UUID"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        entity = self.state.entities_df[self.state.entities_df['uuid'] == entity_id]
        if entity.empty:
            return None
        
        return entity.iloc[0].to_dict()
    
    def get_entity_references(self, entity_id: str) -> pd.DataFrame:
        """Get all references for a specific entity"""
        if self.state.refs_df is None:
            return pd.DataFrame()
        
        return self.state.refs_df[
            (self.state.refs_df['uuid'] == entity_id) | 
            (self.state.refs_df['value'] == entity_id)
        ]
    
    def close(self):
        """Close database connections and cleanup"""
        if self.engine:
            self.engine.dispose()
            self.engine = None
            self.state.sqlalchemy_engine = None
        
        if self.state.sqlite_conn:
            self.state.sqlite_conn.close()
            self.state.sqlite_conn = None
        
        log.info("Orchestrator closed and resources cleaned up")
    
    # Compatibility methods for existing code
    
    def compress_entities(self) -> EntitiesDataFrame:
        """Compress entities by removing empty ones and optimizing data"""
        if self.state.entities_df is None:
            raise ValueError("Data not loaded. Call load_data() first.")
        
        # Use mixin method
        self.compressed_df = self.compress()  # From EntityCompressor mixin
        return self.compressed_df
    
    def run_diagnostics(self) -> dict:
        """Run diagnostics using raw SQLite connection"""
        if self.state.sqlite_conn is None:
            raise ValueError("Database not connected. Call load_data() first.")
        
        # Use mixin method
        return self.diagnose_json_issues()  # From HBFDiagnostics mixin
    
    def run_full_diagnostics(self) -> dict:
        """Run comprehensive diagnostics"""
        if self.state.sqlite_conn is None:
            raise ValueError("Database not connected. Call load_data() first.")
        
        # Use mixin methods
        results = {
            "schema": self.check_schema(),
            "entities": self.analyze_entities(),
            "references": self.analyze_references(),
            "integrity": self.check_integrity() if hasattr(self, 'check_integrity') else {},
            "issues": self.detect_issues() if hasattr(self, 'detect_issues') else {}
        }
        
        if hasattr(self, 'display_results'):
            self.display_results(results)
        
        return results
    
    def generate_html_report(self, **kwargs) -> Path:
        """Generate HTML report using mixin"""
        summary = self.get_summary_model()
        compression_stats = self.get_compression_stats() if hasattr(self, 'get_compression_stats') else None
        diagnostics = kwargs.get('diagnostics')
        
        return super().generate_html_report(
            summary,
            diagnostics=diagnostics,
            compression_stats=compression_stats
        )
    
    def generate_json_report(self, **kwargs) -> Path:
        """Generate JSON report using mixin"""
        summary = self.get_summary_model()
        compression_stats = self.get_compression_stats() if hasattr(self, 'get_compression_stats') else None
        graph_metrics = kwargs.get('graph_metrics')
        diagnostics = kwargs.get('diagnostics')
        
        return super().generate_json_report(
            summary,
            diagnostics=diagnostics,
            compression_stats=compression_stats,
            graph_metrics=graph_metrics
        )
