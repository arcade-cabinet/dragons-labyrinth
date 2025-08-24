"""
Database seeding workflows.

These workflows populate the database with AI-generated content:
- Template rules generation
- Pattern analysis and suggestions  
- Curriculum structure compilation

All database workflows are idempotent and cache results based on input hashes.
"""

from .analysis_workflow import AnalysisWorkflow, create_analysis_workflow
from .compilation_workflow import CompilationWorkflow, create_compilation_workflow  
from .template_rules_workflow import TemplateRulesWorkflow, create_template_rules_workflow

__all__ = [
    "AnalysisWorkflow",
    "CompilationWorkflow", 
    "TemplateRulesWorkflow",
    "create_analysis_workflow",
    "create_compilation_workflow",
    "create_template_rules_workflow",
]