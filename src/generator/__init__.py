"""
Generator Package

Modern Python generator system with clean separation:
- analysis: Extract and cluster HBF entities
- processors: Process analysis data into Rust ECS structure
"""

from generator.utils import get_git_root_dir


__VERSION__ = "0.1.0"

__all__ = [
    "get_git_root_dir",
    "__VERSION__"
]

