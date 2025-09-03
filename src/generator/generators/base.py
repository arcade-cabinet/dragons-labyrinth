"""
Base Generator for layer cake ECS generation with proper image and Rust generation capabilities.

Enhanced base class providing image generation via OpenAI gpt-image-1, template processing, 
and Rust ECS component generation for bevy_ecs_tilemap integration.
"""

from __future__ import annotations

import base64
import json
from pathlib import Path
from typing import Any

from jinja2 import Environment, FileSystemLoader
from openai import OpenAI

from generator.entities.models import ProcessingResult


class BaseGenerator:
    """
    Enhanced base class for layer cake generators.
    
    Provides:
    - Template rendering with Jinja2 (Rust ECS templates)
    - Image generation via DALL-E MCP server (proper gpt-image-1 model)
    - Rust component generation for bevy_ecs_tilemap
    - File utilities and idempotent operations
    
    Specialized generators inherit and implement specific logic.
    """
    
    def __init__(self, generator_type: str):
        self.generator_type = generator_type
        
        # Template environment pointing to generators/templates
        template_dir = Path(__file__).parent / "templates"
        self.jinja_env = Environment(
            loader=FileSystemLoader(template_dir),
            trim_blocks=True,
            lstrip_blocks=True
        )
    
    def render_template(self, template_name: str, context: dict[str, Any]) -> str:
        """Render Jinja2 template with context data for Rust ECS generation."""
        template = self.jinja_env.get_template(template_name)
        return template.render(**context)
    
    def generate_image(self, prompt: str, output_path: Path, 
                      size: str = "1024x1024", model: str = "gpt-image-1") -> Path:
        """
        Generate image using OpenAI Image API with gpt-image-1 model.
        
        This method creates:
        - Hex biome tiles for bevy_ecs_tilemap
        - Entity sprites and icons
        - UI components
        
        Returns Path to generated image.
        Raises Exception if generation fails.
        """
        # Idempotent check
        if output_path.exists():
            return output_path
        
        client = OpenAI()
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Use gpt-image-1 via Image API for high quality generation
        response = client.images.generate(
            model=model,
            prompt=prompt,
            size=size,
            quality="high",
            response_format="b64_json",
            n=1
        )
        
        # Decode and save image
        image_data = base64.b64decode(response.data[0].b64_json)
        
        with open(output_path, "wb") as f:
            f.write(image_data)
        
        return output_path
    
    def generate_rust_component(self, processing_result: ProcessingResult, 
                               template_name: str) -> str:
        """
        Generate Rust ECS component code from processing result.
        
        Converts processed entity data into Bevy ECS components compatible
        with bevy_ecs_tilemap and avian physics systems.
        """
        context = {
            'cluster_name': processing_result.cluster_name,
            'cluster_category': processing_result.cluster_category,
            'entity_count': processing_result.entity_count,
            'specific_data': processing_result.specific_data,
            'bevy_hooks': processing_result.bevy_hooks,
            'ml_results': processing_result.ml_results,
            'generator_type': self.generator_type,
            'rust_module_name': self._to_rust_module_name(processing_result.cluster_name),
            'rust_struct_name': self._to_rust_struct_name(processing_result.cluster_name)
        }
        
        return self.render_template(template_name, context)
    
    def generate_bevy_tilemap_layer(self, layer_data: dict[str, Any]) -> str:
        """
        Generate Bevy ECS tilemap layer configuration for bevy_ecs_tilemap.
        
        Creates proper layer definitions for:
        - Background terrain (hex tiles)
        - Foreground objects (entities, POIs)
        - UI overlay elements
        """
        context = {
            'layer_name': layer_data.get('name', 'default_layer'),
            'layer_z_index': layer_data.get('z_index', 0),
            'tile_size': layer_data.get('tile_size', 64),
            'chunk_size': layer_data.get('chunk_size', 64),
            'layer_type': layer_data.get('type', 'hexagonal'),
            'entities': layer_data.get('entities', []),
            'textures': layer_data.get('textures', [])
        }
        
        return self.render_template('bevy_tilemap_layer.rs.j2', context)
    
    def generate_avian_physics_setup(self, physics_data: dict[str, Any]) -> str:
        """
        Generate Avian physics system setup for entities.
        
        Creates proper physics components for:
        - Player collision and movement
        - Environmental barriers (walls, water, etc.)
        - Interactive objects (shops, dungeons, etc.)
        """
        context = {
            'physics_layers': physics_data.get('layers', {}),
            'collision_groups': physics_data.get('collision_groups', {}),
            'rigid_bodies': physics_data.get('rigid_bodies', []),
            'colliders': physics_data.get('colliders', [])
        }
        
        return self.render_template('avian_physics.rs.j2', context)
    
    def write_rust_module(self, content: str, output_path: Path) -> Path:
        """Write Rust module content to file with proper formatting."""
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(content)
        
        return output_path
    
    def write_json_data(self, data: dict[str, Any], output_path: Path) -> Path:
        """Write JSON data file for runtime loading."""
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output_path, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2)
        
        return output_path
    
    def _to_rust_module_name(self, name: str) -> str:
        """Convert entity name to valid Rust module name."""
        return name.lower().replace(" ", "_").replace("'", "").replace("-", "_").replace(".", "")
    
    def _to_rust_struct_name(self, name: str) -> str:
        """Convert entity name to valid Rust struct name (PascalCase)."""
        # Convert to snake_case first, then to PascalCase
        snake_case = self._to_rust_module_name(name)
        return ''.join(word.capitalize() for word in snake_case.split('_'))
    
    def _extract_hex_coordinates(self, entity_data: dict[str, Any]) -> tuple[int, int] | None:
        """Extract hex coordinates (q, r) from entity data if available."""
        # Look for common coordinate field names
        for coord_field in ['coordinates', 'position', 'location', 'hex']:
            if coord_field in entity_data:
                coords = entity_data[coord_field]
                if isinstance(coords, (list, tuple)) and len(coords) >= 2:
                    return (int(coords[0]), int(coords[1]))
                elif isinstance(coords, dict) and 'q' in coords and 'r' in coords:
                    return (int(coords['q']), int(coords['r']))
        
        return None
    
    def _generate_texture_atlas_entry(self, image_path: Path, atlas_index: int) -> dict[str, Any]:
        """Generate texture atlas entry for bevy_ecs_tilemap."""
        return {
            'path': str(image_path),
            'index': atlas_index,
            'size': [64, 64],  # Standard hex tile size
            'offset': [0, 0]
        }
