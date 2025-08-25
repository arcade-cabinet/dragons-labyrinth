"""
HBF Entity Classifier
Intelligently determines entity types from HBF JSON data based on content patterns,
HTML structure, and field analysis rather than relying on a simple entity_type field.
"""

import json
import re
from typing import Dict, Any, Optional, List, Set
from urllib.parse import urlparse
from collections import Counter

from bs4 import BeautifulSoup


class EntityClassifier:
    """Classifies HBF entities based on content analysis"""
    
    def __init__(self):
        # URL patterns that indicate entity types
        self.url_patterns = {
            'hex': [r'/hex/', r'/hex_tile/', r'hex_\d+', r'H\d+'],
            'settlement': [r'/settlement/', r'/city/', r'/town/', r'/village/'],
            'location': [r'/location/', r'/place/', r'/site/'],
            'dungeon': [r'/dungeon/', r'/crypt/', r'/tomb/', r'/cave/'],
            'room': [r'/room/', r'/area/', r'area_\d+', r'room_\d+'],
            'npc': [r'/npc/', r'/character/', r'/person/'],
            'faction': [r'/faction/', r'/organization/', r'/group/'],
            'item': [r'/item/', r'/artifact/', r'/treasure/'],
            'monster': [r'/monster/', r'/creature/', r'/beast/'],
            'quest': [r'/quest/', r'/mission/', r'/task/'],
            'region': [r'/region/', r'/area/', r'/zone/']
        }
        
        # Content keywords that suggest entity types
        self.content_keywords = {
            'hex': ['hex', 'coordinates', 'biome', 'weather', 'terrain'],
            'settlement': ['population', 'shops', 'tavern', 'inn', 'mayor', 'guards'],
            'location': ['entrance', 'exit', 'door', 'path', 'road'],
            'dungeon': ['level', 'floor', 'stairs', 'treasure', 'trap', 'monster'],
            'room': ['area', 'chamber', 'corridor', 'passage', 'ceiling', 'floor'],
            'npc': ['level', 'hit points', 'armor class', 'strength', 'dexterity', 'constitution'],
            'faction': ['members', 'leader', 'organization', 'conspiracy', 'gathering place'],
            'item': ['damage', 'armor', 'weapon', 'magic', 'enchanted', 'artifact'],
            'monster': ['challenge rating', 'hit points', 'armor class', 'attack'],
            'quest': ['objective', 'reward', 'completion', 'task'],
            'region': ['climate', 'population', 'capital', 'ruler']
        }
        
        # HTML patterns that indicate entity types
        self.html_patterns = {
            'npc': [r'Level \d+', r'AC \d+', r'HP \d+', r'STR \d+', r'DEX \d+'],
            'monster': [r'CR \d+', r'Challenge Rating', r'\d+d\d+\+?\d*\s+damage'],
            'dungeon': [r'area \d+', r'room \d+', r'level \d+', r'floor \d+'],
            'settlement': [r'population:', r'shops:', r'tavern', r'inn'],
            'faction': [r'members:', r'leader:', r'gathering place:']
        }
        
        # JSON field patterns
        self.field_patterns = {
            'npc': ['level', 'hit_points', 'armor_class', 'strength', 'dexterity', 'constitution', 'wisdom', 'intelligence', 'charisma'],
            'monster': ['challenge_rating', 'hit_points', 'armor_class', 'damage', 'attack_bonus'],
            'location': ['coordinates', 'entrance', 'exit', 'connections'],
            'item': ['damage', 'armor_class', 'magical', 'enchantment', 'value'],
            'hex': ['biome', 'weather', 'coordinates', 'terrain'],
            'settlement': ['population', 'shops', 'services', 'government'],
            'faction': ['members', 'leader', 'goals', 'resources']
        }
    
    def classify_entity(self, entity_data: Dict[str, Any]) -> str:
        """
        Classify an entity based on multiple signals.
        
        Args:
            entity_data: Dictionary containing the entity's parsed JSON data
            
        Returns:
            String representing the most likely entity type
        """
        if not entity_data:
            return 'unknown'
        
        scores = Counter()
        
        # Check URL patterns in the data
        self._score_by_urls(entity_data, scores)
        
        # Check content keywords
        self._score_by_content(entity_data, scores)
        
        # Check HTML patterns
        self._score_by_html(entity_data, scores)
        
        # Check JSON field patterns
        self._score_by_fields(entity_data, scores)
        
        # Check specific field values
        self._score_by_field_values(entity_data, scores)
        
        # Return the highest scoring type, or 'unknown' if no clear winner
        if scores:
            return scores.most_common(1)[0][0]
        return 'unknown'
    
    def _score_by_urls(self, data: Dict[str, Any], scores: Counter) -> None:
        """Score entity type based on URL patterns found in the data"""
        text_content = self._extract_all_text(data)
        
        for entity_type, patterns in self.url_patterns.items():
            for pattern in patterns:
                if re.search(pattern, text_content, re.IGNORECASE):
                    scores[entity_type] += 3  # High weight for URL matches
    
    def _score_by_content(self, data: Dict[str, Any], scores: Counter) -> None:
        """Score entity type based on content keywords"""
        text_content = self._extract_all_text(data).lower()
        
        for entity_type, keywords in self.content_keywords.items():
            for keyword in keywords:
                if keyword in text_content:
                    scores[entity_type] += 1
    
    def _score_by_html(self, data: Dict[str, Any], scores: Counter) -> None:
        """Score entity type based on HTML patterns"""
        html_content = self._extract_html_content(data)
        if not html_content:
            return
        
        for entity_type, patterns in self.html_patterns.items():
            for pattern in patterns:
                if re.search(pattern, html_content, re.IGNORECASE):
                    scores[entity_type] += 2  # Medium-high weight for HTML patterns
    
    def _score_by_fields(self, data: Dict[str, Any], scores: Counter) -> None:
        """Score entity type based on JSON field names"""
        field_names = set(self._get_all_field_names(data))
        
        for entity_type, expected_fields in self.field_patterns.items():
            matches = len([field for field in expected_fields if field in field_names])
            if matches > 0:
                scores[entity_type] += matches * 1.5  # Weight by number of matching fields
    
    def _score_by_field_values(self, data: Dict[str, Any], scores: Counter) -> None:
        """Score entity type based on specific field values and patterns"""
        
        # Check for stat blocks (NPCs/Monsters)
        if self._has_stat_block(data):
            if self._looks_like_player_character(data):
                scores['npc'] += 4
            else:
                scores['monster'] += 3
        
        # Check for coordinate patterns (Hex tiles)
        if self._has_coordinates(data):
            scores['hex'] += 3
        
        # Check for room/area numbers (Dungeon rooms)
        if self._has_area_numbers(data):
            scores['room'] += 3
            scores['dungeon'] += 1
        
        # Check for population data (Settlements)
        if self._has_population_data(data):
            scores['settlement'] += 3
        
        # Check for member lists (Factions)
        if self._has_member_list(data):
            scores['faction'] += 3
    
    def _extract_all_text(self, data: Dict[str, Any]) -> str:
        """Extract all text content from the entity data"""
        text_parts = []
        
        def extract_recursive(obj):
            if isinstance(obj, str):
                text_parts.append(obj)
            elif isinstance(obj, dict):
                for value in obj.values():
                    extract_recursive(value)
            elif isinstance(obj, list):
                for item in obj:
                    extract_recursive(item)
        
        extract_recursive(data)
        return ' '.join(text_parts)
    
    def _extract_html_content(self, data: Dict[str, Any]) -> str:
        """Extract HTML content from the entity data"""
        html_parts = []
        
        def find_html_recursive(obj):
            if isinstance(obj, str):
                # Check if this looks like HTML
                if '<' in obj and '>' in obj:
                    html_parts.append(obj)
            elif isinstance(obj, dict):
                for value in obj.values():
                    find_html_recursive(value)
            elif isinstance(obj, list):
                for item in obj:
                    find_html_recursive(item)
        
        find_html_recursive(data)
        return ' '.join(html_parts)
    
    def _get_all_field_names(self, data: Dict[str, Any]) -> Set[str]:
        """Get all field names from the entity data recursively"""
        fields = set()
        
        def collect_fields(obj, prefix=''):
            if isinstance(obj, dict):
                for key, value in obj.items():
                    field_name = f"{prefix}_{key}" if prefix else key
                    fields.add(key.lower())  # Add just the key
                    fields.add(field_name.lower())  # Add prefixed version
                    collect_fields(value, field_name)
            elif isinstance(obj, list) and obj:
                # Check first item if it's a dict
                if isinstance(obj[0], dict):
                    collect_fields(obj[0], prefix)
        
        collect_fields(data)
        return fields
    
    def _has_stat_block(self, data: Dict[str, Any]) -> bool:
        """Check if the entity has RPG stat block data"""
        stat_indicators = ['level', 'hit_points', 'hp', 'armor_class', 'ac', 
                          'strength', 'str', 'dexterity', 'dex', 'constitution', 'con']
        text = self._extract_all_text(data).lower()
        
        # Look for stat patterns in text
        stat_patterns = [r'level\s+\d+', r'hp\s+\d+', r'ac\s+\d+', r'str\s+\d+']
        for pattern in stat_patterns:
            if re.search(pattern, text):
                return True
        
        # Look for stat fields
        fields = self._get_all_field_names(data)
        return len([field for field in stat_indicators if field in fields]) >= 3
    
    def _looks_like_player_character(self, data: Dict[str, Any]) -> bool:
        """Check if stat block looks like a player character rather than monster"""
        text = self._extract_all_text(data).lower()
        pc_indicators = ['rogue', 'cleric', 'fighter', 'wizard', 'ranger', 'paladin',
                        'faction', 'personality', 'background', 'profession']
        return any(indicator in text for indicator in pc_indicators)
    
    def _has_coordinates(self, data: Dict[str, Any]) -> bool:
        """Check if the entity has coordinate data"""
        text = self._extract_all_text(data)
        coordinate_patterns = [r'[NSEW]\d+[NSEW]\d+', r'\d+,\s*\d+', r'hex\s+\d+']
        return any(re.search(pattern, text, re.IGNORECASE) for pattern in coordinate_patterns)
    
    def _has_area_numbers(self, data: Dict[str, Any]) -> bool:
        """Check if the entity references dungeon areas/rooms"""
        text = self._extract_all_text(data)
        area_patterns = [r'area\s+\d+', r'room\s+\d+', r'level\s+\d+', r'floor\s+\d+']
        return any(re.search(pattern, text, re.IGNORECASE) for pattern in area_patterns)
    
    def _has_population_data(self, data: Dict[str, Any]) -> bool:
        """Check if the entity has settlement population data"""
        text = self._extract_all_text(data).lower()
        pop_indicators = ['population', 'inhabitants', 'residents', 'citizens',
                         'shops', 'tavern', 'inn', 'mayor', 'guards']
        return any(indicator in text for indicator in pop_indicators)
    
    def _has_member_list(self, data: Dict[str, Any]) -> bool:
        """Check if the entity has faction member data"""
        text = self._extract_all_text(data).lower()
        faction_indicators = ['members', 'leader', 'organization', 'faction',
                            'conspiracy', 'gathering place', 'collaborators']
        return any(indicator in text for indicator in faction_indicators)


def classify_entity_smart(raw_value: str) -> str:
    """
    Smart entity classification function that can be used as a drop-in replacement
    for the simple entity_type lookup.
    
    Args:
        raw_value: Raw JSON string from the HBF database
        
    Returns:
        Classified entity type string
    """
    if not raw_value or raw_value == '{}':
        return 'unknown'
    
    try:
        data = json.loads(raw_value)
        classifier = EntityClassifier()
        return classifier.classify_entity(data)
    except (json.JSONDecodeError, TypeError):
        return 'unknown'
