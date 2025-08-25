"""
HBF Data Compression Module
Optimizes and compresses entity data
"""

import gzip
from pathlib import Path

import pandas as pd
from rich.progress import track

from dragons_labyrinth.types import EntitiesDataFrame
from dragons_labyrinth.hbf.base import DataFrameMixin


class EntityCompressor(DataFrameMixin):
    """Compress and optimize entity data"""
    
    def __init__(self, state):
        """Initialize compressor with shared state"""
        super().__init__(state)
        self.compressed_df = None
    
    def compress(self) -> EntitiesDataFrame:
        """Remove empty entities and optimize data"""
        self.log.info("[cyan]Compressing entity data...[/cyan]", extra={"markup": True})
        
        # Remove empty entities
        self.compressed_df = self.entities_df[self.entities_df['value'] != '{}'].copy()
        
        # Clean up data fields
        self.compressed_df['data'] = self.compressed_df['data'].apply(self._clean_entity_data)
        
        # Calculate compression stats
        original_count = len(self.entities_df)
        compressed_count = len(self.compressed_df)
        reduction = ((original_count - compressed_count) / original_count) * 100 if original_count > 0 else 0
        
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
    
    def optimize_types(self, df: EntitiesDataFrame = None) -> EntitiesDataFrame:
        """Optimize DataFrame column types for memory efficiency"""
        if df is None:
            df = self.compressed_df if self.compressed_df is not None else self.entities_df
        
        # Convert string columns to category if they have low cardinality
        for col in df.select_dtypes(include=['object']).columns:
            if col not in ['value', 'data']:  # Don't convert JSON columns
                num_unique = df[col].nunique()
                num_total = len(df[col])
                if num_unique / num_total < 0.5:  # Less than 50% unique values
                    df[col] = df[col].astype('category')
        
        return df
    
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
        """Get detailed compression statistics"""
        if self.compressed_df is None:
            return {}
        
        original_size = self.entities_df.memory_usage(deep=True).sum()
        compressed_size = self.compressed_df.memory_usage(deep=True).sum()
        
        return {
            'original_entities': len(self.entities_df),
            'compressed_entities': len(self.compressed_df),
            'removed_entities': len(self.entities_df) - len(self.compressed_df),
            'reduction_percentage': ((len(self.entities_df) - len(self.compressed_df)) / len(self.entities_df) * 100),
            'original_memory_mb': original_size / 1024 / 1024,
            'compressed_memory_mb': compressed_size / 1024 / 1024,
            'memory_saved_mb': (original_size - compressed_size) / 1024 / 1024
        }
