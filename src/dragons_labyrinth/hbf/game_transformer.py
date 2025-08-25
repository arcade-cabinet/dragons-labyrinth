"""
HBF Game Transformer
Transforms processed HBF content into game-ready formats including:
- Rust structs and Bevy components
- JSON manifests for asset loading
- Spatial navigation meshes
- Quest generation data
"""

import json
import re
from typing import Dict, Any, List, Set, Optional, Tuple
from dataclasses import dataclass
from collections import defaultdict
from pathlib import Path

from dragons_labyrinth.hbf.content_processor import (
    NarrativeContent, 
    SpatialRelationship, 
    ReferenceGraph
)


@dataclass
class GameEntity:
    """Represents a game entity with all its components"""
    id: str
    entity_type: str
    name: str
    position: Optional[Tuple[float, float, float]]
    components: Dict[str, Any]
    relationships: List[str]


@dataclass
class AssetRequirement:
    """Represents an asset requirement for the game"""
    asset_type: str  # 'model', 'texture', 'audio', 'sprite'
    name: str
    variants: int
    priority: str  # 'high', 'medium', 'low'
    description: str


class GameTransformer:
    """Transforms HBF content into game-ready format"""
    
    def __init__(self):
        # Mapping from HBF entity types to game components
        self.component_mappings = {
            'hex': ['HexTile', 'TerrainInfo', 'WeatherSystem'],
            'settlement': ['Settlement', 'Population', 'Economy', 'Services'],
            'location': ['Location', 'Navigable', 'Describable'],
            'dungeon': ['Dungeon', 'MultiLevel', 'TreasureContainer'],
            'room': ['Room', 'Navigable', 'Searchable', 'MonsterSpawn'],
            'npc': ['Character', 'Stats', 'Dialogue', 'AI', 'FactionMember'],
            'monster': ['Monster', 'Stats', 'AI', 'CombatBehavior'],
            'faction': ['Faction', 'Members', 'Goals', 'Resources'],
            'item': ['Item', 'Stats', 'Usable', 'Valuable'],
            'quest': ['Quest', 'Objectives', 'Rewards', 'Prerequisites']
        }
        
        # Asset type mappings
        self.asset_mappings = {
            'hex': {'models': ['hex_tile'], 'textures': ['terrain', 'biome']},
            'settlement': {'models': ['buildings', 'walls'], 'textures': ['town']},
            'location': {'models': ['landmark'], 'textures': ['environment']},
            'dungeon': {'models': ['entrance', 'corridor'], 'textures': ['stone', 'dark']},
            'room': {'models': ['chamber'], 'textures': ['dungeon_walls']},
            'npc': {'sprites': ['character'], 'audio': ['voice']},
            'monster': {'models': ['creature'], 'audio': ['growl', 'attack']},
            'item': {'models': ['object'], 'textures': ['item_icon']}
        }
    
    def transform_to_game_format(self, processed_content: Dict[str, Any]) -> Dict[str, Any]:
        """
        Transform processed HBF content into game-ready format
        
        Args:
            processed_content: Output from HBFContentProcessor
            
        Returns:
            Game-ready data structure
        """
        print("🎮 Transforming content to game format...")
        
        game_data = {
            'entities': [],
            'spatial_grid': {},
            'quest_system': {},
            'faction_network': {},
            'asset_requirements': [],
            'rust_components': "",
            'bevy_systems': "",
            'world_manifest': {}
        }
        
        # Transform narrative content to game entities
        entities = self._transform_entities(processed_content['narrative_content'])
        game_data['entities'] = entities
        
        # Build spatial grid system
        game_data['spatial_grid'] = self._build_spatial_grid(
            entities, processed_content['spatial_relationships']
        )
        
        # Build quest system
        game_data['quest_system'] = self._build_quest_system(
            processed_content['quest_network'], entities
        )
        
        # Build faction network
        game_data['faction_network'] = self._build_faction_network(entities)
        
        # Generate asset requirements
        game_data['asset_requirements'] = self._generate_asset_requirements(entities)
        
        # Generate Rust code
        game_data['rust_components'] = self._generate_rust_components(entities)
        game_data['bevy_systems'] = self._generate_bevy_systems(entities)
        
        # Generate world manifest
        game_data['world_manifest'] = self._generate_world_manifest(game_data)
        
        return game_data
    
    def _transform_entities(self, narrative_content: List[NarrativeContent]) -> List[GameEntity]:
        """Transform narrative content into game entities"""
        entities = []
        
        for content in narrative_content:
            # Extract position if available
            position = self._extract_position(content)
            
            # Generate components based on entity type
            components = self._generate_components(content)
            
            # Extract relationships
            relationships = content.references
            
            entity = GameEntity(
                id=content.entity_id,
                entity_type=content.entity_type,
                name=content.title or f"{content.entity_type}_{content.entity_id[:8]}",
                position=position,
                components=components,
                relationships=relationships
            )
            
            entities.append(entity)
        
        return entities
    
    def _extract_position(self, content: NarrativeContent) -> Optional[Tuple[float, float, float]]:
        """Extract 3D position from content"""
        # Look for hex coordinates
        hex_pattern = r'([NSEW])(\d+)([NSEW])(\d+)'
        coord_pattern = r'(\d+),\s*(\d+)'
        
        text = content.description + " " + content.html_content
        
        # Try hex coordinates first
        hex_match = re.search(hex_pattern, text)
        if hex_match:
            # Convert hex coordinates to world position
            # This is simplified - real implementation would use proper hex math
            ns, ns_num, ew, ew_num = hex_match.groups()
            
            x = float(ew_num) * 100  # Each hex is 100 units
            if ew == 'W':
                x = -x
                
            z = float(ns_num) * 100
            if ns == 'S':
                z = -z
            
            return (x, 0.0, z)
        
        # Try regular coordinates
        coord_match = re.search(coord_pattern, text)
        if coord_match:
            x, z = map(float, coord_match.groups())
            return (x * 10, 0.0, z * 10)  # Scale up
        
        return None
    
    def _generate_components(self, content: NarrativeContent) -> Dict[str, Any]:
        """Generate Bevy components for an entity"""
        components = {}
        
        # Get base components for entity type
        base_components = self.component_mappings.get(content.entity_type, ['BasicEntity'])
        
        for component_name in base_components:
            components[component_name] = self._generate_component_data(component_name, content)
        
        return components
    
    def _generate_component_data(self, component_name: str, content: NarrativeContent) -> Dict[str, Any]:
        """Generate specific component data"""
        if component_name == 'Character':
            return {
                'name': content.title,
                'description': content.description,
                'stats': content.stat_blocks,
                'dialogue_tree': content.quest_hooks
            }
        
        elif component_name == 'HexTile':
            return {
                'biome': self._extract_biome(content),
                'weather': self._extract_weather(content),
                'terrain_difficulty': 1.0,
                'resources': []
            }
        
        elif component_name == 'Settlement':
            return {
                'name': content.title,
                'population': self._extract_population(content),
                'services': self._extract_services(content),
                'government': self._extract_government(content)
            }
        
        elif component_name == 'Dungeon':
            return {
                'name': content.title,
                'levels': self._extract_dungeon_levels(content),
                'entrance_type': 'standard',
                'difficulty_rating': 1
            }
        
        elif component_name == 'Room':
            return {
                'area_number': self._extract_area_number(content),
                'connections': content.references,
                'contents': self._extract_room_contents(content),
                'hazards': []
            }
        
        elif component_name == 'Monster':
            return {
                'name': content.title,
                'stats': content.stat_blocks,
                'ai_type': 'hostile',
                'spawn_conditions': {}
            }
        
        elif component_name == 'Faction':
            return {
                'name': content.title,
                'goals': content.quest_hooks,
                'resources': {},
                'reputation': 0
            }
        
        elif component_name == 'Location':
            return {
                'name': content.title,
                'description': content.description,
                'environment': content.environmental_details
            }
        
        else:
            # Generic component
            return {
                'name': content.title,
                'description': content.description
            }
    
    def _extract_biome(self, content: NarrativeContent) -> str:
        """Extract biome type from content"""
        text = (content.description + " " + content.html_content).lower()
        
        biome_keywords = {
            'forest': ['forest', 'trees', 'woodland'],
            'desert': ['desert', 'sand', 'dune'],
            'mountain': ['mountain', 'peak', 'cliff'],
            'plains': ['plains', 'grassland', 'meadow'],
            'swamp': ['swamp', 'marsh', 'bog'],
            'ice': ['ice', 'frozen', 'glacier'],
            'ocean': ['ocean', 'sea', 'water']
        }
        
        for biome, keywords in biome_keywords.items():
            if any(keyword in text for keyword in keywords):
                return biome
        
        return 'unknown'
    
    def _extract_weather(self, content: NarrativeContent) -> str:
        """Extract weather information"""
        weather_patterns = [
            r'weather:\s*([^.\n]+)',
            r'climate:\s*([^.\n]+)',
            r'([^.]*storm[^.]*)',
            r'([^.]*rain[^.]*)',
            r'([^.]*snow[^.]*)'
        ]
        
        text = content.description + " " + content.html_content
        
        for pattern in weather_patterns:
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                return match.group(1).strip()
        
        return 'temperate'
    
    def _extract_population(self, content: NarrativeContent) -> int:
        """Extract population number"""
        pop_pattern = r'population[:\s]*(\d+)'
        text = content.description + " " + content.html_content
        
        match = re.search(pop_pattern, text, re.IGNORECASE)
        if match:
            return int(match.group(1))
        
        # Estimate based on settlement type
        if 'city' in content.title.lower():
            return 5000
        elif 'town' in content.title.lower():
            return 1000
        elif 'village' in content.title.lower():
            return 200
        
        return 100
    
    def _extract_services(self, content: NarrativeContent) -> List[str]:
        """Extract available services"""
        services = []
        text = (content.description + " " + content.html_content).lower()
        
        service_keywords = {
            'inn': ['inn', 'tavern', 'lodging'],
            'shop': ['shop', 'store', 'merchant'],
            'blacksmith': ['blacksmith', 'forge', 'weapons'],
            'temple': ['temple', 'church', 'shrine'],
            'guards': ['guards', 'guard', 'patrol'],
            'healer': ['healer', 'cleric', 'medicine']
        }
        
        for service, keywords in service_keywords.items():
            if any(keyword in text for keyword in keywords):
                services.append(service)
        
        return services
    
    def _extract_government(self, content: NarrativeContent) -> str:
        """Extract government type"""
        text = (content.description + " " + content.html_content).lower()
        
        if any(word in text for word in ['mayor', 'elected']):
            return 'democracy'
        elif any(word in text for word in ['lord', 'baron', 'count']):
            return 'feudalism'
        elif any(word in text for word in ['guild', 'council']):
            return 'oligarchy'
        
        return 'unknown'
    
    def _extract_dungeon_levels(self, content: NarrativeContent) -> int:
        """Extract number of dungeon levels"""
        level_pattern = r'level[s]?\s*(\d+)|(\d+)\s*level[s]?'
        text = content.description + " " + content.html_content
        
        matches = re.findall(level_pattern, text, re.IGNORECASE)
        if matches:
            levels = [int(m[0] or m[1]) for m in matches if m[0] or m[1]]
            return max(levels) if levels else 1
        
        return 1
    
    def _extract_area_number(self, content: NarrativeContent) -> Optional[int]:
        """Extract area/room number"""
        area_pattern = r'area\s*(\d+)|room\s*(\d+)'
        text = content.title + " " + content.description
        
        match = re.search(area_pattern, text, re.IGNORECASE)
        if match:
            return int(match.group(1) or match.group(2))
        
        return None
    
    def _extract_room_contents(self, content: NarrativeContent) -> List[str]:
        """Extract room contents and features"""
        contents = []
        
        # Look for treasure mentions
        if any(word in content.html_content.lower() for word in ['treasure', 'gold', 'coins']):
            contents.append('treasure')
        
        # Look for monster mentions
        if any(word in content.html_content.lower() for word in ['monster', 'creature', 'beast']):
            contents.append('monster')
        
        # Look for furniture/features
        furniture_keywords = ['chest', 'table', 'chair', 'altar', 'statue', 'door', 'stairs']
        for keyword in furniture_keywords:
            if keyword in content.html_content.lower():
                contents.append(keyword)
        
        return contents
    
    def _build_spatial_grid(self, entities: List[GameEntity], 
                          relationships: List[SpatialRelationship]) -> Dict[str, Any]:
        """Build spatial navigation grid"""
        grid = {
            'hex_grid': {},
            'connections': [],
            'regions': {},
            'navigation_mesh': []
        }
        
        # Build hex grid
        hex_entities = [e for e in entities if e.entity_type == 'hex' and e.position]
        for entity in hex_entities:
            x, y, z = entity.position
            hex_key = f"{int(x//100)}_{int(z//100)}"
            grid['hex_grid'][hex_key] = {
                'entity_id': entity.id,
                'position': entity.position,
                'biome': entity.components.get('HexTile', {}).get('biome', 'unknown'),
                'connections': []
            }
        
        # Build connections from relationships
        for rel in relationships:
            grid['connections'].append({
                'from': rel.from_entity,
                'to': rel.to_entity,
                'type': rel.relationship_type,
                'description': rel.description
            })
        
        # Build regions (group nearby hexes)
        regions = defaultdict(list)
        for hex_key, hex_data in grid['hex_grid'].items():
            biome = hex_data['biome']
            regions[biome].append(hex_key)
        
        grid['regions'] = dict(regions)
        
        return grid
    
    def _build_quest_system(self, quest_network: Dict[str, List[str]], 
                          entities: List[GameEntity]) -> Dict[str, Any]:
        """Build quest generation system"""
        quest_system = {
            'quest_givers': [],
            'quest_objectives': [],
            'reward_types': [],
            'quest_chains': []
        }
        
        # Find NPCs that can give quests
        for entity in entities:
            if entity.entity_type == 'npc' and entity.id in quest_network:
                quest_system['quest_givers'].append({
                    'npc_id': entity.id,
                    'name': entity.name,
                    'hooks': quest_network[entity.id],
                    'location': entity.position
                })
        
        # Extract quest objectives from hooks
        objectives = set()
        for hooks in quest_network.values():
            for hook in hooks:
                # Classify quest types
                if any(word in hook.lower() for word in ['find', 'seek', 'locate']):
                    objectives.add('find_item')
                elif any(word in hook.lower() for word in ['kill', 'defeat', 'slay']):
                    objectives.add('eliminate_target')
                elif any(word in hook.lower() for word in ['deliver', 'bring', 'take']):
                    objectives.add('delivery')
                elif any(word in hook.lower() for word in ['rescue', 'save']):
                    objectives.add('rescue')
        
        quest_system['quest_objectives'] = list(objectives)
        
        return quest_system
    
    def _build_faction_network(self, entities: List[GameEntity]) -> Dict[str, Any]:
        """Build faction relationship network"""
        faction_network = {
            'factions': [],
            'members': {},
            'relationships': {},
            'territories': {}
        }
        
        # Find factions
        factions = [e for e in entities if e.entity_type == 'faction']
        for faction in factions:
            faction_network['factions'].append({
                'id': faction.id,
                'name': faction.name,
                'goals': faction.components.get('Faction', {}).get('goals', [])
            })
        
        # Find NPCs and their faction memberships
        for entity in entities:
            if entity.entity_type == 'npc':
                # Check if NPC references any factions
                for ref in entity.relationships:
                    faction_match = next((f for f in factions if f.id == ref), None)
                    if faction_match:
                        if faction_match.id not in faction_network['members']:
                            faction_network['members'][faction_match.id] = []
                        faction_network['members'][faction_match.id].append(entity.id)
        
        return faction_network
    
    def _generate_asset_requirements(self, entities: List[GameEntity]) -> List[AssetRequirement]:
        """Generate asset requirements for the game"""
        asset_counts = defaultdict(int)
        requirements = []
        
        # Count entities by type
        for entity in entities:
            asset_counts[entity.entity_type] += 1
        
        # Generate requirements based on entity types
        for entity_type, count in asset_counts.items():
            if entity_type in self.asset_mappings:
                mapping = self.asset_mappings[entity_type]
                
                for asset_type, asset_names in mapping.items():
                    for asset_name in asset_names:
                        # Determine variants needed
                        variants = min(count // 5 + 1, 10)  # At least 1, max 10
                        
                        # Determine priority
                        priority = 'high' if count > 50 else 'medium' if count > 10 else 'low'
                        
                        requirements.append(AssetRequirement(
                            asset_type=asset_type,
                            name=f"{entity_type}_{asset_name}",
                            variants=variants,
                            priority=priority,
                            description=f"{asset_type.title()} for {entity_type} entities ({count} total)"
                        ))
        
        return requirements
    
    def _generate_rust_components(self, entities: List[GameEntity]) -> str:
        """Generate Rust component definitions"""
        component_types = set()
        for entity in entities:
            component_types.update(entity.components.keys())
        
        rust_code = """// Auto-generated Bevy components from HBF data
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

"""
        
        # Generate component structs
        for component_type in sorted(component_types):
            rust_code += f"""#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct {component_type} {{
    // Component data - implement based on your needs
    pub data: std::collections::HashMap<String, String>,
}}

impl Default for {component_type} {{
    fn default() -> Self {{
        Self {{
            data: std::collections::HashMap::new(),
        }}
    }}
}}

"""
        
        return rust_code
    
    def _generate_bevy_systems(self, entities: List[GameEntity]) -> str:
        """Generate Bevy system code"""
        systems_code = """// Auto-generated Bevy systems from HBF data
use bevy::prelude::*;
use crate::components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_world_entities)
            .add_systems(Update, (
                update_hex_tiles,
                update_npcs,
                update_settlements,
                update_dungeons,
            ));
    }
}

fn spawn_world_entities(mut commands: Commands) {
    // Spawn all entities from HBF data
    // This would be populated with actual entity data
}

fn update_hex_tiles(query: Query<&HexTile>) {
    // Update hex tile systems
}

fn update_npcs(query: Query<&Character>) {
    // Update NPC behavior
}

fn update_settlements(query: Query<&Settlement>) {
    // Update settlement systems
}

fn update_dungeons(query: Query<&Dungeon>) {
    // Update dungeon systems
}
"""
        return systems_code
    
    def _generate_world_manifest(self, game_data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate world manifest for the game engine"""
        return {
            'version': '1.0.0',
            'world_name': 'HBF Generated World',
            'total_entities': len(game_data['entities']),
            'hex_grid_size': len(game_data['spatial_grid'].get('hex_grid', {})),
            'factions_count': len(game_data['faction_network'].get('factions', [])),
            'quest_givers_count': len(game_data['quest_system'].get('quest_givers', [])),
            'asset_requirements_count': len(game_data['asset_requirements']),
            'generation_timestamp': '2025-01-25T14:33:00Z',
            'source': 'HBF Database Transformation'
        }
    
    def export_game_data(self, game_data: Dict[str, Any], output_dir: str) -> Dict[str, str]:
        """Export game data to files"""
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        generated_files = {}
        
        # Export entities as JSON
        entities_file = output_path / "game_entities.json"
        with open(entities_file, 'w') as f:
            entities_data = [
                {
                    'id': entity.id,
                    'entity_type': entity.entity_type,
                    'name': entity.name,
                    'position': entity.position,
                    'components': entity.components,
                    'relationships': entity.relationships
                }
                for entity in game_data['entities']
            ]
            json.dump(entities_data, f, indent=2)
        generated_files['entities'] = str(entities_file)
        
        # Export spatial grid
        spatial_file = output_path / "spatial_grid.json"
        with open(spatial_file, 'w') as f:
            json.dump(game_data['spatial_grid'], f, indent=2)
        generated_files['spatial'] = str(spatial_file)
        
        # Export quest system
        quest_file = output_path / "quest_system.json"
        with open(quest_file, 'w') as f:
            json.dump(game_data['quest_system'], f, indent=2)
        generated_files['quests'] = str(quest_file)
        
        # Export faction network
        faction_file = output_path / "faction_network.json"
        with open(faction_file, 'w') as f:
            json.dump(game_data['faction_network'], f, indent=2)
        generated_files['factions'] = str(faction_file)
        
        # Export asset requirements
        assets_file = output_path / "asset_requirements.json"
        with open(assets_file, 'w') as f:
            requirements_data = [
                {
                    'asset_type': req.asset_type,
                    'name': req.name,
                    'variants': req.variants,
                    'priority': req.priority,
                    'description': req.description
                }
                for req in game_data['asset_requirements']
            ]
            json.dump(requirements_data, f, indent=2)
        generated_files['assets'] = str(assets_file)
        
        # Export Rust components
        rust_file = output_path / "components.rs"
        with open(rust_file, 'w') as f:
            f.write(game_data['rust_components'])
        generated_files['rust_components'] = str(rust_file)
        
        # Export Bevy systems
        systems_file = output_path / "systems.rs"
        with open(systems_file, 'w') as f:
            f.write(game_data['bevy_systems'])
        generated_files['rust_systems'] = str(systems_file)
        
        # Export world manifest
        manifest_file = output_path / "world_manifest.json"
        with open(manifest_file, 'w') as f:
            json.dump(game_data['world_manifest'], f, indent=2)
        generated_files['manifest'] = str(manifest_file)
        
        return generated_files
