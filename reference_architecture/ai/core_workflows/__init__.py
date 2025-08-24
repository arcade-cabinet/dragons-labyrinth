"""
Core application workflows.

These workflows generate the core UI components of Professor Pixel:
- Main application entrypoint
- Title screen with academy backdrop
- Main menu with trophy case, professor video, image map, save slots

Core workflows compile to actual Python code, not database content.
They serve as "dogfooding" to test our template/pattern/IR system.
"""

from .main_menu_workflow import MainMenuWorkflow, create_main_menu_workflow
from .specification_compiler import CoreSpecificationCompiler, create_core_compiler

__all__ = [
    "MainMenuWorkflow",
    "create_main_menu_workflow", 
    "CoreSpecificationCompiler",
    "create_core_compiler",
    # "MainEntrypointWorkflow",  # Future
    # "TitleScreenWorkflow",     # Future
]
