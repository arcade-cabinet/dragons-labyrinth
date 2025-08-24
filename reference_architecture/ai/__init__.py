"""
AI-powered schema analysis and curriculum generation.

This package provides LangGraph-based workflows for analyzing game development APIs
and generating educational content with human-in-the-loop review and durable execution.
Uses scalable template generation rules for multi-library support.
"""

from professor_pixel.schemas.ai.analysis import SchemaAIAnalyzer
from professor_pixel.schemas.ai.agent import CurriculumAgent, create_curriculum_agent
from professor_pixel.schemas.ai.template_rules import ScalableTemplateGenerator, get_template_rules

__all__ = [
    "SchemaAIAnalyzer",
    "CurriculumAgent", 
    "create_curriculum_agent",
    "ScalableTemplateGenerator",
    "get_template_rules"
]
