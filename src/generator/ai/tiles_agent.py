"""
Tiles AI Agent for Dragon's Labyrinth
Generates tile descriptions and converts them to BPY scripts for 2.5D/3D asset creation
"""

import json
import os
from pathlib import Path
import sqlite3
from datetime import datetime
from openai import OpenAI

class TilesAgent:
    """AI agent for generating tile assets with horror progression"""
    
    def __init__(self):
        self.client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
        self.db_path = Path("assets/assets.db")
        self.output_dir = Path("assets/generated/tiles")
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        # Import the generic BPY processor
        import sys
        sys.path.insert(0, str(Path(__file__).parent.parent))
        from bpy_processor import BPYProcessor
        self.bpy_processor = BPYProcessor()
        
    def generate_tile_descriptions(
        self,
        tile_types: list[str],
        dread_level: int
    ) -> dict:
        """Generate AI descriptions for tiles at specific dread level"""
        
        prompt = f"""
You are designing hexagonal tile assets for a horror RPG game called Dragon's Labyrinth.
The game has 5 dread levels (0-4) representing emotional progression from Peace to Horror.

Current dread level: {dread_level}
Emotional context: {self._get_dread_context(dread_level)}

Generate detailed 2.5D/3D tile descriptions for the following tile types:
{', '.join(tile_types)}

For each tile type at dread level {dread_level}, provide:
1. Visual appearance (textures, colors, materials)
2. Decorative elements (rocks, plants, corruption effects)
3. Height variations and surface details
4. Lighting/emission properties for horror atmosphere
5. Specific Blender material node setup suggestions

Output as JSON with this structure:
{{
    "tile_type": {{
        "base_color": [r, g, b, a],
        "roughness": float,
        "metallic": float,
        "height_variation": float,
        "decorations": [
            {{
                "type": "rock|plant|corruption|etc",
                "count": int,
                "scale_range": [min, max],
                "description": "details"
            }}
        ],
        "material_nodes": {{
            "noise_scale": float,
            "voronoi_scale": float,
            "emission_strength": float,
            "corruption_factor": float
        }},
        "description": "prose description"
    }}
}}

Remember:
- Dread 0 (Peace): Beautiful, vibrant, welcoming
- Dread 1 (Unease): Subtle wrongness, muted colors
- Dread 2 (Dread): Visible decay, growing darkness
- Dread 3 (Terror): Active corruption, hostile environment
- Dread 4 (Horror): Nightmare realm, reality breaking
"""
        
        response = self.client.chat.completions.create(
            model="gpt-4o",
            messages=[
                {"role": "system", "content": "You are a horror game asset designer specializing in environmental storytelling through tiles."},
                {"role": "user", "content": prompt}
            ],
            temperature=0.7,
            response_format={"type": "json_object"}
        )
        
        descriptions = json.loads(response.choices[0].message.content)
        
        # Track generation
        self._track_generation(tile_types, dread_level, descriptions)
        
        return descriptions
    
    def generate_bpy_scripts_from_descriptions(
        self,
        descriptions: dict,
        dread_level: int,
        batch_name: str = None
    ) -> dict:
        """Convert AI descriptions to BPY scripts"""
        
        configs = []
        
        for tile_type, desc in descriptions.items():
            # Create config from AI description
            config = {
                "tile_id": f"{tile_type}_dread{dread_level}_ai",
                "tile_type": tile_type,
                "dread_level": dread_level,
                "radius": 1.0,
                "height": 0.3 + desc.get("height_variation", 0),
                "export_path": f"{self.output_dir}/{tile_type}_dread{dread_level}.glb",
                "ai_description": desc
            }
            configs.append(config)
            
            # Generate enhanced BPY script with AI parameters
            self._enhance_bpy_script_with_ai(config, desc)
        
        # Generate BPY scripts and process with generic processor
        batch_name = batch_name or f"ai_tiles_dread{dread_level}"
        batch_scripts = []
        
        for config in configs:
            # Generate BPY script content
            script_content = self._generate_hex_tile_bpy_script(config)
            
            batch_scripts.append({
                "script": script_content,
                "filename": f"{config['tile_id']}.glb"
            })
        
        # Process batch using generic processor
        output_dir = str(self.output_dir / batch_name)
        results = self.bpy_processor.process_batch(batch_scripts, output_dir)
        
        return results
    
    def _enhance_bpy_script_with_ai(self, config: dict, description: dict):
        """Enhance BPY script generation with AI-generated parameters"""
        
        # This would modify the BPY script to include AI-generated details
        # For now, we store the AI description for reference
        config["material_overrides"] = {
            "base_color": description.get("base_color", [0.5, 0.5, 0.5, 1.0]),
            "roughness": description.get("roughness", 0.8),
            "metallic": description.get("metallic", 0.0),
            "noise_scale": description.get("material_nodes", {}).get("noise_scale", 10.0),
            "emission_strength": description.get("material_nodes", {}).get("emission_strength", 0.0)
        }
        
        config["decorations"] = description.get("decorations", [])
    
    def generate_complete_tileset(self) -> dict:
        """Generate complete tileset for all dread levels"""
        
        tile_types = ["grassland", "forest", "mountain", "swamp", "desert", "ocean", "corruption"]
        all_results = {}
        
        for dread_level in range(5):
            print(f"\nGenerating tiles for dread level {dread_level}...")
            
            # Generate AI descriptions
            descriptions = self.generate_tile_descriptions(tile_types, dread_level)
            
            # Convert to BPY scripts
            batch_results = self.generate_bpy_scripts_from_descriptions(
                descriptions, 
                dread_level
            )
            
            all_results[f"dread_{dread_level}"] = {
                "descriptions": descriptions,
                "batch_results": batch_results
            }
            
            # Save descriptions for reference
            desc_path = self.output_dir / f"descriptions_dread{dread_level}.json"
            with open(desc_path, 'w') as f:
                json.dump(descriptions, f, indent=2)
        
        return all_results
    
    def generate_prompt_chain(
        self,
        base_tile: str,
        transformations: list[str]
    ) -> list[dict]:
        """Generate a chain of prompts for progressive tile transformation"""
        
        prompt = f"""
Create a prompt chain for progressively transforming a {base_tile} tile through these stages:
{', '.join(transformations)}

Each prompt should build on the previous, showing gradual horror progression.
Focus on material changes, geometry modifications, and atmospheric effects.

Output as JSON array with structure:
[
    {{
        "stage": "stage_name",
        "prompt": "detailed Blender creation prompt",
        "key_changes": ["list of specific changes"],
        "bpy_hints": ["Blender Python API suggestions"]
    }}
]
"""
        
        response = self.client.chat.completions.create(
            model="gpt-4o",
            messages=[
                {"role": "system", "content": "You are an expert in creating progressive 3D asset transformations for horror games."},
                {"role": "user", "content": prompt}
            ],
            temperature=0.7,
            response_format={"type": "json_object"}
        )
        
        chain = json.loads(response.choices[0].message.content)
        return chain.get("prompts", chain) if isinstance(chain, dict) else chain
    
    def _get_dread_context(self, dread_level: int) -> str:
        """Get emotional context for dread level"""
        
        contexts = {
            0: "Peace - A beautiful morning, birds singing, warm sunlight",
            1: "Unease - Something feels off, shadows seem longer, colors muted",
            2: "Dread - Decay creeping in, darkness spreading, hope fading",
            3: "Terror - Active malevolence, reality distorting, companions breaking",
            4: "Horror - Complete nightmare, first-person stalking, reality shattered"
        }
        return contexts.get(dread_level, "Unknown dread level")
    
    def _track_generation(self, tile_types: list[str], dread_level: int, descriptions: dict):
        """Track AI generation in database"""
        
        if not self.db_path.exists():
            return
        
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Ensure table exists
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS tile_ai_generations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tile_types TEXT,
                dread_level INTEGER,
                descriptions TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)
        
        cursor.execute("""
            INSERT INTO tile_ai_generations (tile_types, dread_level, descriptions)
            VALUES (?, ?, ?)
        """, (
            json.dumps(tile_types),
            dread_level,
            json.dumps(descriptions)
        ))
        
        conn.commit()
        conn.close()
    
    def _generate_hex_tile_bpy_script(self, config: dict) -> str:
        """Generate BPY script content for a hex tile based on AI description"""
        
        material_overrides = config.get("material_overrides", {})
        decorations = config.get("decorations", [])
        
        script = f'''# Auto-generated BPY script for {config["tile_id"]}
import bpy
import bmesh
import mathutils
from mathutils import Vector

# Clear existing mesh objects
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)

# Create hexagonal base mesh
bm = bmesh.new()

# Create hexagon vertices (6 sides)
radius = {config["radius"]}
height = {config["height"]}

# Create hexagon vertices
vertices = []
for i in range(6):
    angle = i * 3.14159 * 2 / 6
    x = radius * mathutils.math.cos(angle)
    y = radius * mathutils.math.sin(angle)
    vertices.append(bm.verts.new((x, y, 0)))

# Create top vertices (same positions but elevated)
top_vertices = []
for vert in vertices:
    top_vertices.append(bm.verts.new((vert.co.x, vert.co.y, height)))

# Create hexagon faces
bm.faces.new(vertices)  # Bottom face
bm.faces.new(reversed(top_vertices))  # Top face

# Create side faces
for i in range(6):
    next_i = (i + 1) % 6
    bm.faces.new([
        vertices[i],
        vertices[next_i],
        top_vertices[next_i],
        top_vertices[i]
    ])

# Create mesh object
mesh = bpy.data.meshes.new("{config["tile_id"]}")
bm.to_mesh(mesh)
bm.free()

obj = bpy.data.objects.new("{config["tile_id"]}", mesh)
bpy.context.collection.objects.link(obj)

# Create material
mat = bpy.data.materials.new(name="{config["tile_id"]}_material")
mat.use_nodes = True
obj.data.materials.append(mat)

# Set up material nodes
nodes = mat.node_tree.nodes
links = mat.node_tree.links

# Clear default nodes
nodes.clear()

# Add Principled BSDF
principled = nodes.new(type='ShaderNodeBsdfPrincipled')
principled.location = (0, 0)

# Add Material Output
output = nodes.new(type='ShaderNodeOutputMaterial')
output.location = (300, 0)
links.new(principled.outputs['BSDF'], output.inputs['Surface'])

# Set material properties from AI
base_color = {material_overrides.get("base_color", [0.5, 0.8, 0.3, 1.0])}
principled.inputs['Base Color'].default_value = base_color
principled.inputs['Roughness'].default_value = {material_overrides.get("roughness", 0.8)}
principled.inputs['Metallic'].default_value = {material_overrides.get("metallic", 0.0)}

# Add emission for horror effects
emission_strength = {material_overrides.get("emission_strength", 0.0)}
if emission_strength > 0:
    principled.inputs['Emission Strength'].default_value = emission_strength
    principled.inputs['Emission Color'].default_value = [0.8, 0.2, 0.2, 1.0]  # Red glow

# Add noise texture for variation
noise = nodes.new(type='ShaderNodeTexNoise')
noise.location = (-300, 0)
noise.inputs['Scale'].default_value = {material_overrides.get("noise_scale", 10.0)}
links.new(noise.outputs['Color'], principled.inputs['Base Color'])

# Select the object for export
bpy.context.view_layer.objects.active = obj
obj.select_set(True)
'''
        
        # Add decorations if any
        if decorations:
            script += "\n# Add decorations\n"
            for decoration in decorations:
                decoration_script = self._generate_decoration_script(
                    decoration, 
                    config["radius"], 
                    config["height"]
                )
                script += decoration_script + "\n"
        
        return script
    
    def _generate_decoration_script(self, decoration: dict, tile_radius: float = 1.0, tile_height: float = 0.3) -> str:
        """Generate BPY script for tile decorations"""
        
        decoration_type = decoration.get("type", "rock")
        count = decoration.get("count", 3)
        scale_range = decoration.get("scale_range", [0.1, 0.3])
        
        script = f'''
# Add {decoration_type} decorations
import random
random.seed(42)  # Deterministic placement

for i in range({count}):
    # Random position within hex tile
    angle = random.random() * 6.28
    distance = random.random() * {tile_radius * 0.7}
    x = distance * mathutils.math.cos(angle)
    y = distance * mathutils.math.sin(angle)
    z = {tile_height}
    
    # Create decoration object
    if "{decoration_type}" == "rock":
        bpy.ops.mesh.primitive_ico_sphere_add(
            radius=random.uniform({scale_range[0]}, {scale_range[1]}),
            location=(x, y, z)
        )
        # Make it rocky
        bpy.ops.object.modifier_add(type='SUBSURF')
        bpy.context.object.modifiers["Subdivision Surface"].levels = 1
    elif "{decoration_type}" == "plant" or "{decoration_type}" == "grass":
        bpy.ops.mesh.primitive_plane_add(
            size=random.uniform({scale_range[0]}, {scale_range[1]}),
            location=(x, y, z)
        )
        # Rotate randomly
        bpy.context.object.rotation_euler[2] = random.random() * 6.28
    elif "{decoration_type}" == "corruption":
        # Create corruption tendril
        bpy.ops.mesh.primitive_cylinder_add(
            radius=random.uniform(0.02, 0.05),
            depth=random.uniform({scale_range[0]}, {scale_range[1]}),
            location=(x, y, z)
        )
        # Make it twisted
        bpy.context.object.rotation_euler = (
            random.random() * 3.14,
            random.random() * 3.14,
            random.random() * 6.28
        )
'''
        
        return script
