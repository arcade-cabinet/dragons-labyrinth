"""
Analysis package - extracts and clusters HBF entities for analysis.

Contains:
- AnalysisTransformer: Extracts entities from HBF and clusters them by type
- Constants: Entity lists and configuration
"""

from generator.analysis.transformer import AnalysisTransformer
from generator.analysis.constants import REGIONS, SETTLEMENTS, FACTIONS, DUNGEONS

__all__ = ["AnalysisTransformer", "REGIONS", "SETTLEMENTS", "FACTIONS", "DUNGEONS"]
