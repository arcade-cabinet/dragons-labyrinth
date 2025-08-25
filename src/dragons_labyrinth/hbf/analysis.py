"""
HBF Analysis mixin that operates on shared OrchestratorState
"""

import json
import gzip
from pathlib import Path

import pandas as pd
import networkx as nx
import tiktoken
from bs4 import BeautifulSoup
from rich.table import Table
from rich import box
from rich.progress import track

from dragons_labyrinth.hbf.base import DataFrameMixin


class HBFAnalysis(DataFrameMixin):
    """
    Analysis mixin for HBF data processing.
    Includes compression, clustering, and relationship mapping functionality.
    """
    
    def __init__(self, state):
        """Initialize with shared orchestrator state"""
        super().__init__(state)
        
        # Analysis-specific state
        self.compressed_df = None
        self.clusters_df = None
        self.graph = None
        self.tokenizer = tiktoken.get_encoding("cl100k_base")
        self.max_tokens = 8000
    
    # Compression methods (from EntityCompressor)
    
    def compress(self) -> pd.DataFrame:
        """Remove empty entities and optimize data"""
        self.log.info("[cyan]Compressing entity data...[/cyan]", extra={"markup": True})
        
        # Remove empty entities
        self.compressed_df = self.entities_df[self.entities_df['value'] != '{}'].copy()
        
        # Clean up data fields
        self.compressed_df['data'] = self.compressed_df['data'].apply(self._clean_entity_data)
        
        # Calculate compression stats
        original_count = len(self.entities_df)
        compressed_count = len(self.compressed_df)
        reduction = ((original_count - compressed_count) / original_count) * 100
        
        self.log.info(f"[green]✓ Compressed from {original_count:,} to {compressed_count:,} entities[/green]", 
                extra={"markup": True})
        self.log.info(f"[green]✓ Reduction: {reduction:.1f}%[/green]", extra={"markup": True})
        
        return self.compressed_df
    
    def _clean_entity_data(self, data: dict) -> dict:
        """Remove null/empty fields from entity data"""
        if not data:
            return {}
        
        cleaned = {}
        for key, value in data.items():
            # Skip null values
            if value is None:
                continue
            # Skip empty strings, lists, and dicts
            if value == "" or value == [] or value == {}:
                continue
            cleaned[key] = value
        
        return cleaned
    
    def save_compressed(self, output_path: Path):
        """Save compressed data to parquet"""
        if self.compressed_df is None:
            raise ValueError("No compressed data. Call compress() first.")
        
        output_path = Path(output_path)
        self.compressed_df.to_parquet(output_path, index=False)
        self.log.info(f"[green]✓ Saved compressed data to {output_path}[/green]", extra={"markup": True})
        
        # Also create gzipped version
        gzip_path = output_path.with_suffix('.parquet.gz')
        with open(output_path, 'rb') as f_in:
            with gzip.open(gzip_path, 'wb', compresslevel=9) as f_out:
                f_out.write(f_in.read())
        
        self.log.info(f"[green]✓ Created gzipped version: {gzip_path}[/green]", extra={"markup": True})
    
    def get_compression_stats(self) -> dict:
        """Get compression statistics"""
        if self.compressed_df is None:
            return {}
        
        original_count = len(self.entities_df) if self.entities_df is not None else 0
        compressed_count = len(self.compressed_df)
        
        return {
            'original_count': original_count,
            'compressed_count': compressed_count,
            'reduction_percentage': ((original_count - compressed_count) / original_count * 100) if original_count > 0 else 0,
            'empty_removed': original_count - compressed_count
        }
    
    # Clustering methods (from EntityClusterer)
    
    def count_tokens(self, text: str) -> int:
        """Count tokens in text"""
        return len(self.tokenizer.encode(text))
    
    def cluster_by_type(self) -> pd.DataFrame:
        """Group entities by type and create token-aware clusters"""
        self.log.info("[cyan]Clustering entities by type...[/cyan]", extra={"markup": True})
        
        # Use compressed data if available, otherwise use entities
        data_to_cluster = self.compressed_df if self.compressed_df is not None else self.entities_df
        
        # Group by entity type
        grouped = data_to_cluster.groupby('entity_type')
        
        cluster_data = []
        cluster_id = 0
        
        for entity_type, group in grouped:
            current_cluster = {
                'cluster_id': f"type_{entity_type}_{cluster_id}",
                'entity_type': entity_type,
                'entity_ids': [],
                'total_tokens': 0,
                'entity_count': 0
            }
            
            for _, entity in group.iterrows():
                # Calculate tokens for this entity
                entity_json = json.dumps(entity['data'] if entity['data'] else {})
                tokens = self.count_tokens(entity_json)
                
                # Check if adding this entity would exceed token limit
                if current_cluster['total_tokens'] + tokens > self.max_tokens:
                    # Save current cluster and start new one
                    if current_cluster['entity_ids']:
                        cluster_data.append(current_cluster)
                        cluster_id += 1
                    
                    current_cluster = {
                        'cluster_id': f"type_{entity_type}_{cluster_id}",
                        'entity_type': entity_type,
                        'entity_ids': [],
                        'total_tokens': 0,
                        'entity_count': 0
                    }
                
                # Add entity to cluster
                current_cluster['entity_ids'].append(entity['uuid'])
                current_cluster['total_tokens'] += tokens
                current_cluster['entity_count'] += 1
            
            # Add final cluster for this type
            if current_cluster['entity_ids']:
                cluster_data.append(current_cluster)
                cluster_id += 1
        
        self.clusters_df = pd.DataFrame(cluster_data)
        self.log.info(f"[green]✓ Created {len(self.clusters_df)} clusters[/green]", extra={"markup": True})
        
        return self.clusters_df
    
    def display_cluster_summary(self):
        """Display cluster summary statistics"""
        if self.clusters_df is None:
            raise ValueError("Clusters not created. Call cluster_by_type() first.")
        
        table = Table(title="Clustering Summary", box=box.ROUNDED)
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="magenta")
        
        table.add_row("Total Clusters", str(len(self.clusters_df)))
        table.add_row("Total Entities", f"{self.clusters_df['entity_count'].sum():,}")
        table.add_row("Avg Entities/Cluster", f"{self.clusters_df['entity_count'].mean():.1f}")
        table.add_row("Avg Tokens/Cluster", f"{self.clusters_df['total_tokens'].mean():.0f}")
        table.add_row("Max Tokens in Cluster", f"{self.clusters_df['total_tokens'].max():,}")
        
        self.console.print("\n")
        self.console.print(table)
    
    # Relationship mapping methods (from RelationshipMapper)
    
    def build_graph(self) -> nx.DiGraph:
        """Build a directed graph of entity relationships"""
        self.log.info("[cyan]Building relationship graph...[/cyan]", extra={"markup": True})
        
        self.graph = nx.DiGraph()
        
        # Add nodes
        for _, entity in track(self.entities_df.iterrows(), 
                              description="Adding nodes...", 
                              total=len(self.entities_df)):
            self.graph.add_node(
                entity['uuid'],
                entity_type=entity.get('entity_type', 'unknown'),
                has_content=(entity['value'] != '{}')
            )
        
        # Add edges from refs if available
        if self.refs_df is not None:
            for _, ref in track(self.refs_df.iterrows(),
                               description="Adding edges...",
                               total=len(self.refs_df)):
                if ref['uuid'] in self.graph and ref['value'] in self.graph:
                    self.graph.add_edge(
                        ref['uuid'],
                        ref['value'],
                        ref_type=ref.get('type', 'unknown')
                    )
        
        # Extract relationships from entity data
        for _, entity in self.entities_df.iterrows():
            if entity['data']:
                self._extract_relationships(entity['uuid'], entity['data'])
        
        self.log.info(f"[green]✓ Graph built with {self.graph.number_of_nodes():,} nodes and {self.graph.number_of_edges():,} edges[/green]", 
                extra={"markup": True})
        
        return self.graph
    
    def _extract_relationships(self, entity_id: str, data: dict):
        """Extract relationships from entity data"""
        # Direct field references
        direct_fields = ['faction', 'leader', 'location', 'parent', 'dungeon', 'region', 'hex']
        
        for field in direct_fields:
            if field in data and data[field]:
                ref = data[field]
                if isinstance(ref, str) and ref != entity_id and ref in self.graph:
                    self.graph.add_edge(entity_id, ref, ref_type=field)
        
        # List field references
        list_fields = ['members', 'collaborators', 'connections', 'rooms', 'items']
        
        for field in list_fields:
            if field in data and isinstance(data[field], list):
                for ref in data[field]:
                    if isinstance(ref, str) and ref != entity_id and ref in self.graph:
                        self.graph.add_edge(entity_id, ref, ref_type=field)
        
        # HTML content references
        html_fields = ['content', 'description', 'notes']
        for field in html_fields:
            if field in data and data[field]:
                refs = self._extract_html_references(data[field])
                for ref in refs:
                    if ref != entity_id and ref in self.graph:
                        self.graph.add_edge(entity_id, ref, ref_type=f'html_{field}')
    
    def _extract_html_references(self, html_content: str) -> list:
        """Extract entity references from HTML content"""
        if not html_content or not isinstance(html_content, str):
            return []
        
        try:
            soup = BeautifulSoup(html_content, 'lxml')
            references = []
            
            # Find all anchor tags
            for link in soup.find_all('a'):
                href = link.get('href', '')
                if href and not href.startswith('http'):
                    ref = href.strip('#/').replace('.html', '')
                    if ref:
                        references.append(ref)
            
            return list(set(references))
        except:
            return []
    
    def get_graph_metrics(self) -> dict:
        """Get graph analysis metrics"""
        if self.graph is None:
            raise ValueError("Graph not built. Call build_graph() first.")
        
        metrics = {
            'node_count': self.graph.number_of_nodes(),
            'edge_count': self.graph.number_of_edges(),
            'density': nx.density(self.graph),
            'components': nx.number_weakly_connected_components(self.graph)
        }
        
        # Degree statistics
        in_degrees = dict(self.graph.in_degree())
        out_degrees = dict(self.graph.out_degree())
        
        if in_degrees:
            metrics['avg_in_degree'] = sum(in_degrees.values()) / len(in_degrees)
            metrics['max_in_degree'] = max(in_degrees.values())
        
        if out_degrees:
            metrics['avg_out_degree'] = sum(out_degrees.values()) / len(out_degrees)
            metrics['max_out_degree'] = max(out_degrees.values())
        
        return metrics
    
    def display_graph_summary(self):
        """Display graph summary statistics"""
        metrics = self.get_graph_metrics()
        
        table = Table(title="Graph Metrics", box=box.ROUNDED)
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="magenta")
        
        table.add_row("Total Nodes", f"{metrics['node_count']:,}")
        table.add_row("Total Edges", f"{metrics['edge_count']:,}")
        table.add_row("Graph Density", f"{metrics['density']:.6f}")
        table.add_row("Weakly Connected Components", str(metrics['components']))
        
        if 'avg_in_degree' in metrics:
            table.add_row("Avg In-Degree", f"{metrics['avg_in_degree']:.2f}")
            table.add_row("Max In-Degree", str(metrics['max_in_degree']))
        
        if 'avg_out_degree' in metrics:
            table.add_row("Avg Out-Degree", f"{metrics['avg_out_degree']:.2f}")
            table.add_row("Max Out-Degree", str(metrics['max_out_degree']))
        
        self.console.print("\n")
        self.console.print(table)
