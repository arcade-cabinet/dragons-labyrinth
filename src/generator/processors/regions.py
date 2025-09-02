"""
RegionsProcessor - HTML and JSON processor for region entity clusters.

Processes region clusters from the analysis phase, extracting geographical
information from HTML files and JSON map data to create Rust ECS components.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any, Dict, List, Optional

from bs4 import BeautifulSoup

from generator.processors.base import BaseProcessor
from generator.processors.models import RegionData, HexTileData, ProcessingResult


class RegionsProcessor(BaseProcessor):
    """
    Specialized processor for region entity clusters.
    
    Extracts data from:
    - HTML files containing region descriptions and features
    - JSON map files with hex coordinate and biome data
    - Settlement, infrastructure, and geographic information
    
    Generates UUID-based Rust ECS structure:
    - processed/regions/{region_uuid}/mod.rs
    - processed/regions/{region_uuid}/{hex_uuid}.json
    """
    
    def __init__(self):
        super().__init__("regions")
        
        # Biome type mappings from JSON analysis
        self.biome_mappings = {
            "JungleHex": "jungle",
            "ForestHex": "forest", 
            "MountainsHex": "mountains",
            "PlainsHex": "plains",
            "DesertHex": "desert",
            "SwampsHex": "swamps",
            "TundraHex": "tundra"
        }
        
        # Feature type mappings
        self.feature_mappings = {
            "Village": "village",
            "Town": "town", 
            "City": "city",
            "Dungeon": "dungeon",
            "Inn": "inn",
            "Residency": "residency",
            "Other": "other"
        }
    
    def process_cluster_directory(self, analysis_dir: Path, output_dir: Path, logger, console) -> ProcessingResult:
        """Process a region cluster directory containing HTML and JSON files."""
        
        region_name = analysis_dir.name
        console.print(f"Processing region: [bold cyan]{region_name}[/bold cyan]")
        
        # Find JSON map file
        json_files = list(analysis_dir.glob("entity_map.json"))
        html_files = list(analysis_dir.glob("entity_*.html"))
        
        if not json_files:
            logger.warning(f"No entity_map.json found in {analysis_dir}")
            return self._create_empty_result(region_name)
        
        # Process JSON map data
        with open(json_files[0], 'r', encoding='utf-8') as f:
            map_data = json.load(f)
        
        # Process hex tiles from map
        hex_tiles = self._process_hex_tiles(map_data.get("map", []))
        
        # Process HTML files for additional context
        html_descriptions = {}
        for html_file in html_files:
            entity_uuid = html_file.stem.replace("entity_", "")
            description = self._extract_html_description(html_file)
            if description:
                html_descriptions[entity_uuid] = description
        
        # Extract region metadata
        region_data = RegionData(
            name=region_name,
            uuid=self._generate_region_uuid(region_name),
            total_hexes=len(hex_tiles),
            biome_distribution=self._calculate_biome_distribution(hex_tiles),
            settlement_count=self._count_settlements(hex_tiles),
            geographic_features=self._analyze_geographic_features(hex_tiles),
            hex_tiles=hex_tiles
        )
        
        # Create output structure
        region_output_dir = output_dir / "regions" / region_data.uuid
        region_output_dir.mkdir(parents=True, exist_ok=True)
        
        # Generate hex tile JSON files
        for hex_tile in hex_tiles:
            hex_file = region_output_dir / f"{hex_tile.uuid}.json"
            with open(hex_file, 'w', encoding='utf-8') as f:
                json.dump(hex_tile.to_dict(), f, indent=2)
        
        # Generate mod.rs file
        mod_file = region_output_dir / "mod.rs"
        self._generate_rust_module(mod_file, region_data)
        
        logger.info(f"Processed region {region_name}: {len(hex_tiles)} hexes, {region_data.settlement_count} settlements")
        
        return ProcessingResult(
            entity_type="region",
            entity_name=region_name,
            entity_uuid=region_data.uuid,
            success=True,
            output_files=[str(f) for f in region_output_dir.glob("*")],
            entity_count=len(hex_tiles),
            data=region_data.to_dict()
        )
    
    def _process_hex_tiles(self, map_tiles: List[Dict]) -> List[HexTileData]:
        """Process hex tiles from JSON map data."""
        
        hex_tiles = []
        
        for tile in map_tiles:
            # Extract coordinate and biome information
            hex_data = HexTileData(
                uuid=tile.get("uuid", self._generate_uuid()),
                x=tile.get("x", 0),
                y=tile.get("y", 0),
                biome_type=self.biome_mappings.get(tile.get("type", ""), "unknown"),
                feature_type=self.feature_mappings.get(tile.get("feature", "Other"), "other"),
                feature_uuid=tile.get("feature_uuid"),
                feature_label=tile.get("label"),
                rivers=tile.get("rivers", []),
                trails=tile.get("trails", []),
                harbor=tile.get("harbor"),
                borderline=tile.get("borderline", False),
                region_uuid=tile.get("region"),
                realm_uuid=tile.get("realm")
            )
            
            hex_tiles.append(hex_data)
        
        return hex_tiles
    
    def _extract_html_description(self, html_file: Path) -> Optional[str]:
        """Extract description text from HTML file."""
        
        try:
            with open(html_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            soup = BeautifulSoup(content, 'html.parser')
            
            # Extract main description text
            description_parts = []
            
            # Look for blockquotes (descriptions)
            for blockquote in soup.find_all('blockquote'):
                text = blockquote.get_text(strip=True)
                if text:
                    description_parts.append(text)
            
            # Look for paragraph content
            for p in soup.find_all('p'):
                text = p.get_text(strip=True)
                if text and len(text) > 20:  # Filter out short fragments
                    description_parts.append(text)
            
            return " ".join(description_parts) if description_parts else None
            
        except Exception as e:
            return None
    
    def _calculate_biome_distribution(self, hex_tiles: List[HexTileData]) -> Dict[str, int]:
        """Calculate distribution of biome types."""
        
        distribution = {}
        for tile in hex_tiles:
            biome = tile.biome_type
            distribution[biome] = distribution.get(biome, 0) + 1
        
        return distribution
    
    def _count_settlements(self, hex_tiles: List[HexTileData]) -> int:
        """Count settlements in the region."""
        
        settlements = {"village", "town", "city"}
        return sum(1 for tile in hex_tiles if tile.feature_type in settlements)
    
    def _analyze_geographic_features(self, hex_tiles: List[HexTileData]) -> Dict[str, int]:
        """Analyze geographic features across all hex tiles."""
        
        features = {
            "rivers": 0,
            "trails": 0, 
            "harbors": 0,
            "borders": 0
        }
        
        for tile in hex_tiles:
            if tile.rivers:
                features["rivers"] += len(tile.rivers)
            if tile.trails:
                features["trails"] += len(tile.trails)
            if tile.harbor is not None:
                features["harbors"] += 1
            if tile.borderline:
                features["borders"] += 1
        
        return features
    
    def _generate_region_uuid(self, region_name: str) -> str:
        """Generate consistent UUID for region name."""
        import hashlib
        return hashlib.md5(region_name.encode()).hexdigest()[:8]
    
    def _generate_uuid(self) -> str:
        """Generate a random UUID."""
        import uuid
        return str(uuid.uuid4())[:8]
    
    def _generate_rust_module(self, mod_file: Path, region_data: RegionData):
        """Generate Rust module file for the region."""
        
        rust_content = f'''//! {region_data.name} - Generated region module
//!
//! Total hexes: {region_data.total_hexes}
//! Settlements: {region_data.settlement_count}
//! Dominant biome: {self._get_dominant_biome(region_data.biome_distribution)}

use bevy::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {self._to_rust_identifier(region_data.name)} {{
    pub uuid: String,
    pub name: String,
    pub total_hexes: u32,
    pub settlement_count: u32,
    pub biome_distribution: std::collections::HashMap<String, u32>,
    pub geographic_features: std::collections::HashMap<String, u32>,
}}

impl Default for {self._to_rust_identifier(region_data.name)} {{
    fn default() -> Self {{
        Self {{
            uuid: "{region_data.uuid}".to_string(),
            name: "{region_data.name}".to_string(),
            total_hexes: {region_data.total_hexes},
            settlement_count: {region_data.settlement_count},
            biome_distribution: [
{self._format_biome_distribution_rust(region_data.biome_distribution)}
            ].into(),
            geographic_features: [
{self._format_geographic_features_rust(region_data.geographic_features)}
            ].into(),
        }}
    }}
}}

pub fn spawn_region(mut commands: Commands) {{
    commands.spawn({self._to_rust_identifier(region_data.name)}::default());
}}
'''
        
        with open(mod_file, 'w', encoding='utf-8') as f:
            f.write(rust_content)
    
    def _get_dominant_biome(self, biome_distribution: Dict[str, int]) -> str:
        """Get the dominant biome type."""
        if not biome_distribution:
            return "unknown"
        return max(biome_distribution.items(), key=lambda x: x[1])[0]
    
    def _to_rust_identifier(self, name: str) -> str:
        """Convert name to valid Rust identifier."""
        # Remove special characters and convert to PascalCase
        words = re.findall(r'\w+', name)
        return ''.join(word.capitalize() for word in words)
    
    def _format_biome_distribution_rust(self, distribution: Dict[str, int]) -> str:
        """Format biome distribution for Rust HashMap."""
        items = []
        for biome, count in distribution.items():
            items.append(f'                ("{biome}".to_string(), {count}),')
        return '\n'.join(items)
    
    def _format_geographic_features_rust(self, features: Dict[str, int]) -> str:
        """Format geographic features for Rust HashMap."""
        items = []
        for feature, count in features.items():
            items.append(f'                ("{feature}".to_string(), {count}),')
        return '\n'.join(items)
    
    def _create_empty_result(self, region_name: str) -> ProcessingResult:
        """Create empty result for failed processing."""
        return ProcessingResult(
            entity_type="region",
            entity_name=region_name,
            entity_uuid=self._generate_region_uuid(region_name),
            success=False,
            output_files=[],
            entity_count=0,
            data={}
        )
