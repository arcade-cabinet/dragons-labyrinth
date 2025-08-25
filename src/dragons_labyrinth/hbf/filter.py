"""
HBF Entity Filtering Base Class
Provides filtering capabilities for HBF entities as a mixin
"""

from typing import Any
import pandas as pd
from dragons_labyrinth.types import EntitiesDataFrame
from dragons_labyrinth.hbf.base import DataFrameMixin


class EntityFilter(DataFrameMixin):
    """Filter entities based on various criteria - designed as a base class/mixin"""
    
    def by_type(self, entity_type: str) -> EntitiesDataFrame:
        """Filter entities by type"""
        return self.entities_df[self.entities_df['entity_type'] == entity_type]
    
    def by_types(self, entity_types: list[str]) -> EntitiesDataFrame:
        """Filter entities by multiple types"""
        return self.entities_df[self.entities_df['entity_type'].isin(entity_types)]
    
    def non_empty(self) -> EntitiesDataFrame:
        """Get only entities with content"""
        return self.entities_df[self.entities_df['value'] != '{}']
    
    def empty(self) -> EntitiesDataFrame:
        """Get only empty entities"""
        return self.entities_df[self.entities_df['value'] == '{}']
    
    def by_field_exists(self, field_name: str) -> EntitiesDataFrame:
        """Filter entities that have a specific field in their data"""
        def has_field(data: dict) -> bool:
            return field_name in data if isinstance(data, dict) else False
        
        mask = self.entities_df['data'].apply(has_field)
        return self.entities_df[mask]
    
    def by_field_value(self, field_name: str, value: Any) -> EntitiesDataFrame:
        """Filter entities by specific field value"""
        def matches_value(data: dict) -> bool:
            if not isinstance(data, dict):
                return False
            return data.get(field_name) == value
        
        mask = self.entities_df['data'].apply(matches_value)
        return self.entities_df[mask]
    
    def by_field_contains(self, field_name: str, substring: str) -> EntitiesDataFrame:
        """Filter entities where field contains substring"""
        def contains_substring(data: dict) -> bool:
            if not isinstance(data, dict):
                return False
            field_value = data.get(field_name, "")
            if isinstance(field_value, str):
                return substring.lower() in field_value.lower()
            return False
        
        mask = self.entities_df['data'].apply(contains_substring)
        return self.entities_df[mask]
    
    def by_reference_to(self, target_uuid: str, refs_df: pd.DataFrame) -> EntitiesDataFrame:
        """Filter entities that reference a specific target entity"""
        if refs_df is None or refs_df.empty:
            return pd.DataFrame()
        
        # Get UUIDs that reference the target
        referencing_uuids = refs_df[refs_df['value'] == target_uuid]['uuid'].unique()
        return self.entities_df[self.entities_df['uuid'].isin(referencing_uuids)]
    
    def by_referenced_from(self, source_uuid: str, refs_df: pd.DataFrame) -> EntitiesDataFrame:
        """Filter entities referenced by a specific source entity"""
        if refs_df is None or refs_df.empty:
            return pd.DataFrame()
        
        # Get UUIDs referenced by the source
        referenced_uuids = refs_df[refs_df['uuid'] == source_uuid]['value'].unique()
        return self.entities_df[self.entities_df['uuid'].isin(referenced_uuids)]
    
    def by_size_range(self, min_size: int = None, max_size: int = None) -> EntitiesDataFrame:
        """Filter entities by JSON value size"""
        sizes = self.entities_df['value'].str.len()
        
        mask = pd.Series([True] * len(self.entities_df))
        if min_size is not None:
            mask &= sizes >= min_size
        if max_size is not None:
            mask &= sizes <= max_size
        
        return self.entities_df[mask]
    
    def with_valid_json(self) -> EntitiesDataFrame:
        """Filter entities with valid JSON data (successfully parsed)"""
        def is_valid_json(data: Any) -> bool:
            # If data is a dict and not empty, it was successfully parsed
            return isinstance(data, dict) and bool(data)
        
        mask = self.entities_df['data'].apply(is_valid_json)
        return self.entities_df[mask]
    
    def with_invalid_json(self) -> EntitiesDataFrame:
        """Filter entities with invalid or unparseable JSON"""
        def is_invalid_json(data: Any) -> bool:
            # If data is empty dict or not a dict, parsing failed
            return not isinstance(data, dict) or not bool(data)
        
        mask = self.entities_df['data'].apply(is_invalid_json)
        return self.entities_df[mask]
    
    def sample(self, n: int = 10, random_state: int = None) -> EntitiesDataFrame:
        """Get a random sample of entities"""
        return self.entities_df.sample(n=min(n, len(self.entities_df)), random_state=random_state)
    
    # Convenience methods that were in loader
    def filter_by_type(self, entity_type: str) -> EntitiesDataFrame:
        """Alias for by_type for compatibility"""
        return self.by_type(entity_type)
    
    def get_non_empty_entities(self) -> EntitiesDataFrame:
        """Alias for non_empty for compatibility"""
        return self.non_empty()
