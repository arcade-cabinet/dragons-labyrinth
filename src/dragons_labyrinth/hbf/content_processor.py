"""
HBF Content Processor
Extracts and processes rich narrative content, spatial relationships, 
HTML reference webs, and other patterns from HBF entities.
"""

import json
import re
from typing import Dict, Any, List, Set, Optional, Tuple
from dataclasses import dataclass
from collections import defaultdict, Counter
from urllib.parse import urljoin, urlparse

import pandas as pd
from bs4 import BeautifulSoup

from dragons_labyrinth.hbf.entity_classifier import EntityClassifier


@dataclass
class SpatialRelationship:
    """Represents a spatial relationship between entities"""
    from_entity: str
    to_entity: str
    relationship_type: str  # 'contains', 'connects_to', 'located_in', etc.
    description: str


@dataclass
class NarrativeContent:
    """Extracted narrative content from an entity"""
    entity_id: str
    entity_type: str
    title: str
    description: str
    html_content: str
    references: List[str]
    quest_hooks: List[str]
    stat_blocks: Dict[str, Any]
    environmental_details: List[str]


@dataclass
class ReferenceGraph:
    """Represents the HTML reference web between entities"""
    nodes: Set[str]  # Entity IDs
    edges: List[Tuple[str, str, str]]  # (from, to, relationship)
    orphaned_refs: List[str]  # References that don't resolve


class HBFContentProcessor:
    """Processes HBF content to extract rich narrative and spatial data"""
    
    def __init__(self):
        self.classifier = EntityClassifier()
        
        # Patterns for different content types
        self.quest_patterns = [
            r'seeking\s+([^.]+)',
            r'looking for\s+([^.]+)',
            r'needs?\s+([^.]+)',
            r'wants?\s+([^.]+)',
            r'missing\s+([^.]+)',
            r'lost\s+([^.]+)',
            r'reward\s*:\s*([^.]+)',
            r'will pay\s+([^.]+)',
            r'treasure\s+([^.]+)',
        ]
        
        self.spatial_patterns = [
            r'located in\s+([^.]+)',
            r'north of\s+([^.]+)',
            r'south of\s+([^.]+)',
            r'east of\s+([^.]+)',
            r'west of\s+([^.]+)',
            r'entrance to\s+([^.]+)',
            r'leads to\s+([^.]+)',
            r'connects to\s+([^.]+)',
            r'in the\s+([^.]+)',
            r'area\s+(\d+)',
            r'room\s+(\d+)',
            r'level\s+(\d+)',
        ]
        
        self.environmental_patterns = [
            r'([^.]*weather[^.]*)',
            r'([^.]*climate[^.]*)',
            r'([^.]*temperature[^.]*)',
            r'([^.]*lighting[^.]*)',
            r'([^.]*sounds?[^.]*)',
            r'([^.]*smells?[^.]*)',
            r'([^.]*atmosphere[^.]*)',
        ]
        
    def process_entities(self, entities_df: pd.DataFrame) -> Dict[str, Any]:
        """
        Process all entities to extract rich content and relationships.
        
        Args:
            entities_df: DataFrame with entity data
            
        Returns:
            Dictionary with processed content and analysis
        """
        results = {
            'narrative_content': [],
            'spatial_relationships': [],
            'reference_graph': None,
            'content_statistics': {},
            'entity_clusters': defaultdict(list),
            'quest_network': defaultdict(list)
        }
        
        print("🔍 Processing entity content...")
        
        # Process each entity
        for _, entity in entities_df.iterrows():
            try:
                narrative = self._extract_narrative_content(entity)
                if narrative:
                    results['narrative_content'].append(narrative)
                    results['entity_clusters'][narrative.entity_type].append(narrative)
                
                # Extract spatial relationships
                spatial_rels = self._extract_spatial_relationships(entity)
                results['spatial_relationships'].extend(spatial_rels)
                
                # Extract quest hooks
                if narrative and narrative.quest_hooks:
                    results['quest_network'][entity['uuid']] = narrative.quest_hooks
                    
            except Exception as e:
                print(f"⚠️  Error processing entity {entity['uuid']}: {e}")
                continue
        
        # Build reference graph
        results['reference_graph'] = self._build_reference_graph(results['narrative_content'])
        
        # Generate statistics
        results['content_statistics'] = self._generate_statistics(results)
        
        return results
    
    def _extract_narrative_content(self, entity: pd.Series) -> Optional[NarrativeContent]:
        """Extract narrative content from a single entity"""
        try:
            data = json.loads(entity['value']) if entity['value'] != '{}' else {}
        except (json.JSONDecodeError, TypeError):
            return None
        
        if not data:
            return None
        
        # Extract basic info
        title = self._extract_title(data)
        description = self._extract_description(data)
        html_content = self._extract_html_content(data)
        
        if not any([title, description, html_content]):
            return None
        
        # Extract references from HTML
        references = self._extract_references(html_content) if html_content else []
        
        # Extract quest hooks
        quest_hooks = self._extract_quest_hooks(html_content or description or "")
        
        # Extract stat blocks
        stat_blocks = self._extract_stat_blocks(data, html_content or "")
        
        # Extract environmental details
        environmental_details = self._extract_environmental_details(html_content or description or "")
        
        return NarrativeContent(
            entity_id=entity['uuid'],
            entity_type=entity.get('entity_type', 'unknown'),
            title=title,
            description=description,
            html_content=html_content,
            references=references,
            quest_hooks=quest_hooks,
            stat_blocks=stat_blocks,
            environmental_details=environmental_details
        )
    
    def _extract_title(self, data: Dict[str, Any]) -> str:
        """Extract title/name from entity data"""
        # Look for common title fields
        title_fields = ['title', 'name', 'label', 'heading']
        
        for field in title_fields:
            if field in data and data[field]:
                return str(data[field]).strip()
        
        # Try to extract from HTML if present
        for value in data.values():
            if isinstance(value, str) and '<h' in value.lower():
                soup = BeautifulSoup(value, 'html.parser')
                header = soup.find(['h1', 'h2', 'h3', 'h4', 'h5', 'h6'])
                if header:
                    return header.get_text().strip()
        
        return ""
    
    def _extract_description(self, data: Dict[str, Any]) -> str:
        """Extract description from entity data"""
        # Look for description fields
        desc_fields = ['description', 'desc', 'content', 'text', 'body']
        
        for field in desc_fields:
            if field in data and data[field]:
                text = str(data[field]).strip()
                # Clean HTML if present
                if '<' in text and '>' in text:
                    soup = BeautifulSoup(text, 'html.parser')
                    return soup.get_text().strip()
                return text
        
        return ""
    
    def _extract_html_content(self, data: Dict[str, Any]) -> str:
        """Extract HTML content from entity data"""
        html_parts = []
        
        def find_html_recursive(obj):
            if isinstance(obj, str):
                if '<' in obj and '>' in obj:
                    # Basic HTML validation
                    if any(tag in obj.lower() for tag in ['<p>', '<div>', '<a>', '<h', '<ul>', '<li>']):
                        html_parts.append(obj)
            elif isinstance(obj, dict):
                for value in obj.values():
                    find_html_recursive(value)
            elif isinstance(obj, list):
                for item in obj:
                    find_html_recursive(item)
        
        find_html_recursive(data)
        return '\n'.join(html_parts) if html_parts else ""
    
    def _extract_references(self, html_content: str) -> List[str]:
        """Extract references from HTML content"""
        if not html_content:
            return []
        
        soup = BeautifulSoup(html_content, 'html.parser')
        links = soup.find_all('a', href=True)
        
        references = []
        for link in links:
            href = link['href']
            # Clean and normalize the reference
            if href.startswith('/'):
                references.append(href)
            elif href.startswith('http'):
                # Extract path from full URL
                parsed = urlparse(href)
                if parsed.path:
                    references.append(parsed.path)
        
        return references
    
    def _extract_quest_hooks(self, content: str) -> List[str]:
        """Extract potential quest hooks from content"""
        if not content:
            return []
        
        quest_hooks = []
        content_lower = content.lower()
        
        for pattern in self.quest_patterns:
            matches = re.findall(pattern, content_lower, re.IGNORECASE)
            for match in matches:
                if match.strip() and len(match.strip()) > 3:
                    quest_hooks.append(match.strip())
        
        return quest_hooks
    
    def _extract_stat_blocks(self, data: Dict[str, Any], content: str) -> Dict[str, Any]:
        """Extract RPG stat blocks from entity data"""
        stat_block = {}
        
        # Look for common RPG stats in data
        stat_fields = {
            'level': ['level', 'lvl'],
            'hit_points': ['hit_points', 'hp', 'health'],
            'armor_class': ['armor_class', 'ac'],
            'strength': ['strength', 'str'],
            'dexterity': ['dexterity', 'dex'],
            'constitution': ['constitution', 'con'],
            'intelligence': ['intelligence', 'int'],
            'wisdom': ['wisdom', 'wis'],
            'charisma': ['charisma', 'cha'],
            'challenge_rating': ['challenge_rating', 'cr']
        }
        
        for stat, field_names in stat_fields.items():
            for field_name in field_names:
                if field_name in data and data[field_name]:
                    stat_block[stat] = data[field_name]
                    break
        
        # Extract from text patterns
        if content:
            stat_patterns = {
                'level': r'level\s+(\d+)',
                'hit_points': r'hp\s+(\d+)',
                'armor_class': r'ac\s+(\d+)',
                'challenge_rating': r'cr\s+(\d+(?:/\d+)?)'
            }
            
            for stat, pattern in stat_patterns.items():
                if stat not in stat_block:
                    match = re.search(pattern, content.lower())
                    if match:
                        stat_block[stat] = match.group(1)
        
        return stat_block
    
    def _extract_environmental_details(self, content: str) -> List[str]:
        """Extract environmental details from content"""
        if not content:
            return []
        
        environmental_details = []
        
        for pattern in self.environmental_patterns:
            matches = re.findall(pattern, content, re.IGNORECASE)
            for match in matches:
                if match.strip() and len(match.strip()) > 10:
                    environmental_details.append(match.strip())
        
        return environmental_details
    
    def _extract_spatial_relationships(self, entity: pd.Series) -> List[SpatialRelationship]:
        """Extract spatial relationships from entity content"""
        relationships = []
        
        try:
            data = json.loads(entity['value']) if entity['value'] != '{}' else {}
            content = self._extract_html_content(data)
            if not content:
                content = self._extract_description(data)
        except (json.JSONDecodeError, TypeError):
            return relationships
        
        if not content:
            return relationships
        
        # Extract spatial relationships
        for pattern in self.spatial_patterns:
            matches = re.findall(pattern, content, re.IGNORECASE)
            for match in matches:
                if match.strip():
                    # Determine relationship type based on pattern
                    relationship_type = 'spatial'
                    if 'entrance' in pattern:
                        relationship_type = 'entrance'
                    elif 'leads' in pattern or 'connects' in pattern:
                        relationship_type = 'connects_to'
                    elif 'in' in pattern:
                        relationship_type = 'located_in'
                    elif any(direction in pattern for direction in ['north', 'south', 'east', 'west']):
                        relationship_type = 'directional'
                    
                    relationships.append(SpatialRelationship(
                        from_entity=entity['uuid'],
                        to_entity=match.strip(),
                        relationship_type=relationship_type,
                        description=f"Entity {entity['uuid']} has {relationship_type} relationship with {match}"
                    ))
        
        return relationships
    
    def _build_reference_graph(self, narrative_content: List[NarrativeContent]) -> ReferenceGraph:
        """Build a graph of HTML references between entities"""
        nodes = set()
        edges = []
        all_references = set()
        entity_ids = set()
        
        # Collect all entity IDs and references
        for content in narrative_content:
            nodes.add(content.entity_id)
            entity_ids.add(content.entity_id)
            all_references.update(content.references)
        
        # Build edges from references
        for content in narrative_content:
            for ref in content.references:
                # Try to resolve reference to an entity
                # This is a simplified approach - could be more sophisticated
                edges.append((content.entity_id, ref, 'references'))
        
        # Find orphaned references (references that don't point to known entities)
        orphaned_refs = [ref for ref in all_references if ref not in entity_ids]
        
        return ReferenceGraph(
            nodes=nodes,
            edges=edges,
            orphaned_refs=orphaned_refs
        )
    
    def _generate_statistics(self, results: Dict[str, Any]) -> Dict[str, Any]:
        """Generate comprehensive statistics about the processed content"""
        narrative_content = results['narrative_content']
        
        stats = {
            'total_entities_processed': len(narrative_content),
            'entities_by_type': Counter(content.entity_type for content in narrative_content),
            'entities_with_html': len([c for c in narrative_content if c.html_content]),
            'entities_with_references': len([c for c in narrative_content if c.references]),
            'entities_with_quest_hooks': len([c for c in narrative_content if c.quest_hooks]),
            'entities_with_stat_blocks': len([c for c in narrative_content if c.stat_blocks]),
            'total_references': sum(len(c.references) for c in narrative_content),
            'total_quest_hooks': sum(len(c.quest_hooks) for c in narrative_content),
            'spatial_relationships': len(results['spatial_relationships']),
            'reference_graph_nodes': len(results['reference_graph'].nodes) if results['reference_graph'] else 0,
            'reference_graph_edges': len(results['reference_graph'].edges) if results['reference_graph'] else 0,
            'orphaned_references': len(results['reference_graph'].orphaned_refs) if results['reference_graph'] else 0
        }
        
        # Content quality metrics
        quality_metrics = {
            'avg_title_length': sum(len(c.title) for c in narrative_content) / len(narrative_content) if narrative_content else 0,
            'avg_description_length': sum(len(c.description) for c in narrative_content) / len(narrative_content) if narrative_content else 0,
            'avg_references_per_entity': stats['total_references'] / len(narrative_content) if narrative_content else 0,
            'coverage_with_content': (len(narrative_content) / len(narrative_content)) * 100 if narrative_content else 0
        }
        
        stats.update(quality_metrics)
        return stats
    
    def export_for_game_engine(self, results: Dict[str, Any], output_dir: str) -> Dict[str, str]:
        """
        Export processed content in formats suitable for the game engine.
        
        Returns paths to generated files.
        """
        from pathlib import Path
        import json
        
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        generated_files = {}
        
        # Export narrative content as JSON
        narrative_file = output_path / "narrative_content.json"
        with open(narrative_file, 'w') as f:
            narrative_data = [
                {
                    'entity_id': content.entity_id,
                    'entity_type': content.entity_type,
                    'title': content.title,
                    'description': content.description,
                    'references': content.references,
                    'quest_hooks': content.quest_hooks,
                    'stat_blocks': content.stat_blocks,
                    'environmental_details': content.environmental_details
                }
                for content in results['narrative_content']
            ]
            json.dump(narrative_data, f, indent=2)
        generated_files['narrative'] = str(narrative_file)
        
        # Export spatial relationships
        spatial_file = output_path / "spatial_relationships.json"
        with open(spatial_file, 'w') as f:
            spatial_data = [
                {
                    'from_entity': rel.from_entity,
                    'to_entity': rel.to_entity,
                    'relationship_type': rel.relationship_type,
                    'description': rel.description
                }
                for rel in results['spatial_relationships']
            ]
            json.dump(spatial_data, f, indent=2)
        generated_files['spatial'] = str(spatial_file)
        
        # Export reference graph
        if results['reference_graph']:
            graph_file = output_path / "reference_graph.json"
            with open(graph_file, 'w') as f:
                graph_data = {
                    'nodes': list(results['reference_graph'].nodes),
                    'edges': results['reference_graph'].edges,
                    'orphaned_refs': results['reference_graph'].orphaned_refs
                }
                json.dump(graph_data, f, indent=2)
            generated_files['graph'] = str(graph_file)
        
        # Export statistics
        stats_file = output_path / "content_statistics.json"
        with open(stats_file, 'w') as f:
            json.dump(results['content_statistics'], f, indent=2)
        generated_files['statistics'] = str(stats_file)
        
        # Export entity clusters by type
        clusters_file = output_path / "entity_clusters.json"
        with open(clusters_file, 'w') as f:
            cluster_data = {}
            for entity_type, entities in results['entity_clusters'].items():
                cluster_data[entity_type] = [
                    {
                        'entity_id': entity.entity_id,
                        'title': entity.title,
                        'description': entity.description[:200] + "..." if len(entity.description) > 200 else entity.description
                    }
                    for entity in entities
                ]
            json.dump(cluster_data, f, indent=2)
        generated_files['clusters'] = str(clusters_file)
        
        return generated_files
