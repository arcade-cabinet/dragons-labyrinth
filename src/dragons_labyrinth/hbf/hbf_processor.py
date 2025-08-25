"""
Specialized HBF Processor
Handles the actual HBF structure: a single map entity containing hex tiles,
plus referenced entities for features, NPCs, locations, etc.
"""

import json
import sqlite3
from typing import Dict, Any, List, Optional
from dataclasses import dataclass
from collections import defaultdict, Counter

import pandas as pd
from rich.console import Console
from rich.progress import track

console = Console()


@dataclass
class HexTile:
    """Represents a hex tile from the HBF map"""
    x: int
    y: int
    type: str  # JungleHex, DesertHex, etc.
    uuid: str
    feature: str
    feature_uuid: Optional[str]
    rivers: List[int]
    trails: List[int]
    region: str
    realm: str


@dataclass
class ProcessedHBF:
    """Processed HBF world data"""
    hex_tiles: List[HexTile]
    realms: Dict[str, Dict[str, Any]]
    regions: Dict[str, str]
    borders: Dict[str, List[Dict[str, Any]]]
    features: Dict[str, Any]  # Feature entities with content
    total_entities: int
    processed_entities: int


class HBFProcessor:
    """Processes HBF database with proper structure understanding"""
    
    def __init__(self, hbf_path: str):
        self.hbf_path = hbf_path
        self.conn = None
    
    def __enter__(self):
        self.conn = sqlite3.connect(self.hbf_path)
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.conn:
            self.conn.close()
    
    def process_hbf(self) -> ProcessedHBF:
        """Process the complete HBF database"""
        console.print("🗺️ Processing HBF world data...")
        
        # Get the main map entity
        cursor = self.conn.cursor()
        cursor.execute('SELECT uuid, value FROM Entities WHERE uuid = "map"')
        result = cursor.fetchone()
        
        if not result:
            raise ValueError("No 'map' entity found in HBF database")
        
        _, map_value = result
        map_data = json.loads(map_value)
        
        console.print(f"📍 Found map with {len(map_data['map'])} hex tiles")
        
        # Process hex tiles
        hex_tiles = []
        feature_uuids = set()
        
        for hex_data in track(map_data['map'], description="Processing hex tiles"):
            hex_tile = HexTile(
                x=hex_data['x'],
                y=hex_data['y'],
                type=hex_data['type'],
                uuid=hex_data['uuid'],
                feature=hex_data.get('feature', 'None'),
                feature_uuid=hex_data.get('feature_uuid'),
                rivers=hex_data.get('rivers', []),
                trails=hex_data.get('trails', []),
                region=hex_data.get('region', ''),
                realm=hex_data.get('realm', '')
            )
            hex_tiles.append(hex_tile)
            
            if hex_tile.feature_uuid:
                feature_uuids.add(hex_tile.feature_uuid)
        
        # Process referenced features
        features = self._load_feature_entities(feature_uuids)
        
        # Get total entity count
        cursor.execute('SELECT COUNT(*) FROM Entities')
        total_entities = cursor.fetchone()[0]
        
        return ProcessedHBF(
            hex_tiles=hex_tiles,
            realms=map_data.get('realms', {}),
            regions=map_data.get('regions', {}),
            borders=map_data.get('borders', {}),
            features=features,
            total_entities=total_entities,
            processed_entities=len(hex_tiles) + len(features)
        )
    
    def _load_feature_entities(self, feature_uuids: set) -> Dict[str, Any]:
        """Load and parse feature entities"""
        features = {}
        cursor = self.conn.cursor()
        
        console.print(f"🔍 Loading {len(feature_uuids)} feature entities...")
        
        for uuid in track(feature_uuids, description="Loading features"):
            cursor.execute('SELECT uuid, value FROM Entities WHERE uuid = ?', (uuid,))
            result = cursor.fetchone()
            
            if result and result[1]:  # Has content
                _, value = result
                try:
                    if value.strip():  # Not empty
                        feature_data = json.loads(value)
                        features[uuid] = feature_data
                except json.JSONDecodeError:
                    # Try as plain text
                    features[uuid] = {'raw_content': value}
        
        console.print(f"✅ Found {len(features)} features with content")
        return features
    
    def get_statistics(self, processed: ProcessedHBF) -> Dict[str, Any]:
        """Generate statistics about the processed world"""
        hex_type_counts = Counter(hex_tile.type for hex_tile in processed.hex_tiles)
        feature_counts = Counter(hex_tile.feature for hex_tile in processed.hex_tiles)
        
        stats = {
            'total_hex_tiles': len(processed.hex_tiles),
            'hex_types': dict(hex_type_counts),
            'features': dict(feature_counts),
            'realms_count': len(processed.realms),
            'regions_count': len(processed.regions),
            'features_with_content': len(processed.features),
            'total_entities': processed.total_entities,
            'processed_entities': processed.processed_entities,
            'processing_ratio': processed.processed_entities / processed.total_entities * 100
        }
        
        return stats
    
    def export_world_data(self, processed: ProcessedHBF, output_dir: str) -> Dict[str, str]:
        """Export processed world data to files"""
        from pathlib import Path
        
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        generated_files = {}
        
        # Export hex tiles
        hex_tiles_file = output_path / "hex_tiles.json"
        with open(hex_tiles_file, 'w') as f:
            hex_data = [
                {
                    'x': h.x, 'y': h.y, 'type': h.type, 'uuid': h.uuid,
                    'feature': h.feature, 'feature_uuid': h.feature_uuid,
                    'rivers': h.rivers, 'trails': h.trails,
                    'region': h.region, 'realm': h.realm
                }
                for h in processed.hex_tiles
            ]
            json.dump(hex_data, f, indent=2)
        generated_files['hex_tiles'] = str(hex_tiles_file)
        
        # Export realms
        realms_file = output_path / "realms.json"
        with open(realms_file, 'w') as f:
            json.dump(processed.realms, f, indent=2)
        generated_files['realms'] = str(realms_file)
        
        # Export regions
        regions_file = output_path / "regions.json"
        with open(regions_file, 'w') as f:
            json.dump(processed.regions, f, indent=2)
        generated_files['regions'] = str(regions_file)
        
        # Export features
        features_file = output_path / "features.json"
        with open(features_file, 'w') as f:
            json.dump(processed.features, f, indent=2)
        generated_files['features'] = str(features_file)
        
        # Export statistics
        stats = self.get_statistics(processed)
        stats_file = output_path / "world_statistics.json"
        with open(stats_file, 'w') as f:
            json.dump(stats, f, indent=2)
        generated_files['statistics'] = str(stats_file)
        
        return generated_files
    
    def generate_bevy_world_code(self, processed: ProcessedHBF) -> str:
        """Generate Bevy/Rust code for the world"""
        stats = self.get_statistics(processed)
        
        rust_code = f"""// Auto-generated world data from HBF
// Generated from {processed.total_entities} entities, processed {processed.processed_entities}
use bevy::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HexTile {{
    pub x: i32,
    pub y: i32,
    pub hex_type: HexType,
    pub feature: Feature,
    pub rivers: Vec<u8>,
    pub trails: Vec<u8>,
    pub region: String,
    pub realm: String,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HexType {{
"""
        
        # Add hex types from data
        for hex_type in stats['hex_types'].keys():
            clean_type = hex_type.replace('Hex', '')
            rust_code += f"    {clean_type},\n"
        
        rust_code += """
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Feature {
"""
        
        # Add features from data
        for feature in stats['features'].keys():
            if feature != 'Other' and feature != 'None':
                clean_feature = feature.replace(' ', '').replace("'", "")
                rust_code += f"    {clean_feature},\n"
        
        rust_code += """    Other,
    None,
}

pub fn spawn_world_hexes(mut commands: Commands) {
    // Spawn all hex tiles
"""
        
        # Add sample hex spawning (first 5 hexes)
        for hex_tile in processed.hex_tiles[:5]:
            hex_type = hex_tile.type.replace('Hex', '')
            rust_code += f"""    commands.spawn(HexTile {{
        x: {hex_tile.x},
        y: {hex_tile.y},
        hex_type: HexType::{hex_type},
        feature: Feature::Other, // TODO: Map features properly
        rivers: vec!{hex_tile.rivers},
        trails: vec!{hex_tile.trails},
        region: "{hex_tile.region}".to_string(),
        realm: "{hex_tile.realm}".to_string(),
    }});
"""
        
        rust_code += f"""    
    // ... and {len(processed.hex_tiles) - 5} more hex tiles
    println!("Spawned {{}} hex tiles", {len(processed.hex_tiles)});
}}

// World statistics: {{
//   Total hex tiles: {stats['total_hex_tiles']}
//   Hex types: {len(stats['hex_types'])}
//   Regions: {stats['regions_count']} 
//   Realms: {stats['realms_count']}
//   Features with content: {stats['features_with_content']}
// }}
"""
        
        return rust_code


def process_hbf_to_game_world(hbf_path: str, output_dir: str) -> Dict[str, Any]:
    """Main function to process HBF and generate game world"""
    with HBFProcessor(hbf_path) as processor:
        # Process the HBF data
        processed = processor.process_hbf()
        
        # Generate statistics
        stats = processor.get_statistics(processed)
        
        # Display results
        console.print(f"\n🎯 [bold green]HBF Processing Complete![/bold green]")
        console.print(f"📊 Processed {stats['processed_entities']:,} of {stats['total_entities']:,} entities ({stats['processing_ratio']:.1f}%)")
        console.print(f"🗺️ Found {stats['total_hex_tiles']:,} hex tiles")
        console.print(f"🏰 Found {stats['realms_count']} realms and {stats['regions_count']} regions")
        console.print(f"🎭 Found {stats['features_with_content']} features with content")
        
        # Export files
        generated_files = processor.export_world_data(processed, output_dir)
        
        # Generate Rust code
        from pathlib import Path
        rust_code = processor.generate_bevy_world_code(processed)
        rust_file = Path(output_dir) / "world.rs"
        with open(rust_file, 'w') as f:
            f.write(rust_code)
        generated_files['rust_world'] = str(rust_file)
        
        return {
            'processed': processed,
            'statistics': stats,
            'generated_files': generated_files
        }
