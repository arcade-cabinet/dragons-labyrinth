"""
Library-specific template generation rules for curriculum compilation.

This module provides configurable template generation rules that scale across
different game libraries (Arcade, Pygame, PySDL2) and educational categories.
"""

from abc import ABC, abstractmethod

from professor_pixel.types import GameLibrary, PatternCategory
from professor_pixel.models import PatternSuggestion


class TemplateGenerationRules(ABC):
    """Base class for library-specific template generation rules."""
    
    def __init__(self, library: GameLibrary):
        self.library = library
    
    @abstractmethod
    def get_template_header(self, pattern: PatternSuggestion) -> list[str]:
        """Generate template header with metadata and comments."""
        pass
    
    @abstractmethod
    def get_choice_variables(self, pattern: PatternSuggestion) -> list[str]:
        """Generate Jinja2 variable declarations for student choices."""
        pass
    
    @abstractmethod
    def get_category_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate category-specific template code."""
        pass
    
    @abstractmethod
    def get_function_mapping(self, pattern: PatternSuggestion) -> dict[str, str]:
        """Map pattern to actual library function calls."""
        pass
    
    @property
    @abstractmethod
    def supported_categories(self) -> set[str]:
        """Categories this library supports."""
        pass
    
    @property
    @abstractmethod
    def template_styles(self) -> dict[str, dict[str, object]]:
        """Available template styles and their configuration."""
        pass


class ArcadeTemplateRules(TemplateGenerationRules):
    """Template generation rules for Python Arcade."""
    
    def __init__(self):
        super().__init__(GameLibrary.ARCADE)
    
    def get_template_header(self, pattern: PatternSuggestion) -> list[str]:
        """Generate template header for Arcade patterns."""
        return [
            f"{{# {pattern.title} Template #}}",
            f"{{# Generated for Arcade {pattern.category} category #}}",
            f"{{# Complexity: {pattern.complexity}/5 #}}",
            f"{{# Source: {pattern.source_function} #}}",
            "",
            f"# === {pattern.title} ===",
            f"# {pattern.description}",
            f"# Teaches: {', '.join(pattern.teaches_concepts)}",
            ""
        ]
    
    def get_choice_variables(self, pattern: PatternSuggestion) -> list[str]:
        """Generate Jinja2 variables for student choices."""
        variables = []
        
        for choice in pattern.suggested_choices:
            choice_id = choice["choice_id"]
            default_value = choice.get("default", "default")
            variables.append(
                f"{{%- set {choice_id} = choices.get('{choice_id}', '{default_value}') %}}"
            )
        
        if variables:
            variables.append("")  # Add spacing after variables
        
        return variables
    
    def get_category_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate category-specific template code for Arcade."""
        
        category = pattern.category.lower()
        
        if category == "sprites":
            return self._generate_arcade_sprite_template(pattern, style)
        elif category == "visual":
            return self._generate_arcade_visual_template(pattern, style)
        elif category == "audio":
            return self._generate_arcade_audio_template(pattern, style)
        elif category == "input":
            return self._generate_arcade_input_template(pattern, style)
        elif category == "collision":
            return self._generate_arcade_collision_template(pattern, style)
        elif category == "motion":
            return self._generate_arcade_motion_template(pattern, style)
        else:
            return self._generate_arcade_generic_template(pattern, style)
    
    def get_function_mapping(self, pattern: PatternSuggestion) -> dict[str, str]:
        """Map pattern opcodes to Arcade function calls."""
        
        # Extract function name from source
        func_name = pattern.source_function.replace("arcade.", "")
        
        return {
            "function_call": f"arcade.{func_name}",
            "import_statement": "import arcade",
            "module_reference": "arcade",
            "documentation_url": f"https://arcade.academy/en/latest/api/{func_name.replace('.', '/')}.html"
        }
    
    @property
    def supported_categories(self) -> set[str]:
        """Categories supported by Arcade."""
        return {
            "sprites", "visual", "audio", "input", 
            "collision", "motion", "game", "general"
        }
    
    @property
    def template_styles(self) -> dict[str, dict[str, object]]:
        """Template styles available for Arcade."""
        return {
            "beginner": {
                "complexity_filter": [1, 2],
                "comment_verbosity": "high",
                "error_handling": "minimal",
                "variable_naming": "descriptive",
                "code_structure": "linear"
            },
            "intermediate": {
                "complexity_filter": [1, 2, 3],
                "comment_verbosity": "medium", 
                "error_handling": "basic",
                "variable_naming": "standard",
                "code_structure": "functions"
            },
            "advanced": {
                "complexity_filter": [1, 2, 3, 4, 5],
                "comment_verbosity": "low",
                "error_handling": "comprehensive",
                "variable_naming": "concise",
                "code_structure": "classes"
            }
        }
    
    def _generate_arcade_sprite_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade sprite template code."""
        
        style_config = self.template_styles[style]
        
        if style == "beginner":
            return [
                f"# Create a {pattern.title.lower()}",
                "my_sprite = arcade.Sprite({{ sprite_image }}, {{ sprite_scale | default(1.0) }})",
                "",
                "# Position the sprite",
                "my_sprite.center_x = {{ start_x | default(400) }}",
                "my_sprite.center_y = {{ start_y | default(300) }}",
                "",
                "# Add to sprite list (you'll create this first!)",
                "sprite_list.append(my_sprite)",
                "",
                "# Student customization point:",
                "# You can change the sprite's position, size, or image above!"
            ]
        elif style == "intermediate":
            return [
                f"# {pattern.title} implementation",
                "class {{ sprite_class_name | default('GameSprite') }}(arcade.Sprite):",
                "    def __init__(self, image_path: str, scale: float = 1.0):",
                "        super().__init__(image_path, scale)",
                "        self.setup_properties()",
                "",
                "    def setup_properties(self):",
                "        \"\"\"Configure sprite properties - customize this!\"\"\"",
                "        self.speed = {{ sprite_speed | default(100) }}",
                "        {% if sprite_type == 'player' %}",
                "        self.health = {{ max_health | default(100) }}",
                "        {% endif %}",
                "",
                "# Create sprite instance",
                "{{ sprite_name | default('my_sprite') }} = {{ sprite_class_name | default('GameSprite') }}(",
                "    '{{ sprite_image }}',",
                "    scale={{ sprite_scale | default(1.0) }}",
                ")"
            ]
        else:  # advanced
            return [
                f"# Advanced {pattern.title} with composition pattern",
                "from dataclasses import dataclass",
                "from typing import Protocol",
                "",
                "@dataclass",
                "class SpriteConfig:",
                "    image_path: str",
                "    scale: float = 1.0",
                "    initial_position: tuple[float, float] = (400, 300)",
                "    custom_properties: dict[str, object] = None",
                "",
                "class SpriteFactory:",
                "    @staticmethod",
                "    def create_sprite(config: SpriteConfig) -> arcade.Sprite:",
                "        sprite = arcade.Sprite(config.image_path, config.scale)",
                "        sprite.center_x, sprite.center_y = config.initial_position",
                "        return sprite",
                "",
                "# Usage - student customizes the config",
                "config = SpriteConfig(",
                "    image_path='{{ sprite_image }}',",
                "    scale={{ sprite_scale | default(1.0) }},",
                "    initial_position=({{ start_x | default(400) }}, {{ start_y | default(300) }})",
                ")",
                "{{ sprite_name | default('my_sprite') }} = SpriteFactory.create_sprite(config)"
            ]
    
    def _generate_arcade_visual_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade visual/drawing template code."""
        
        if style == "beginner":
            return [
                f"# {pattern.title} - draw simple shapes",
                "arcade.draw_{{ shape_type | default('circle') }}_filled(",
                "    {{ x_position | default(400) }},  # X coordinate",
                "    {{ y_position | default(300) }},  # Y coordinate", 
                "    {{ size | default(50) }},         # Size",
                "    {{ color | default('arcade.color.WHITE') }}  # Color",
                ")",
                "",
                "# Try changing the numbers above to move or resize your shape!"
            ]
        elif style == "intermediate":
            return [
                f"# {pattern.title} with functions",
                "def draw_{{ shape_name | default('shape') }}(x: float, y: float, size: float, color: tuple[int, int, int]):",
                "    \"\"\"Draw a customizable shape - modify this function!\"\"\"",
                "    arcade.draw_{{ shape_type | default('circle') }}_filled(x, y, size, color)",
                "",
                "# Draw the shape",
                "draw_{{ shape_name | default('shape') }}(",
                "    x={{ x_position | default(400) }},",
                "    y={{ y_position | default(300) }},",
                "    size={{ size | default(50) }},",
                "    color={{ color | default('arcade.color.WHITE') }}",
                ")"
            ]
        else:  # advanced
            return [
                f"# Advanced {pattern.title} with shape system",
                "from abc import ABC, abstractmethod",
                "from dataclasses import dataclass",
                "",
                "@dataclass",
                "class DrawCommand:",
                "    x: float",
                "    y: float", 
                "    size: float",
                "    color: tuple[int, int, int]",
                "",
                "class ShapeRenderer(ABC):",
                "    @abstractmethod",
                "    def render(self, command: DrawCommand) -> None:",
                "        pass",
                "",
                "class {{ shape_renderer_class | default('CircleRenderer') }}(ShapeRenderer):",
                "    def render(self, command: DrawCommand) -> None:",
                "        arcade.draw_{{ shape_type | default('circle') }}_filled(",
                "            command.x, command.y, command.size, command.color",
                "        )",
                "",
                "# Usage - student implements custom renderers",
                "renderer = {{ shape_renderer_class | default('CircleRenderer') }}()",
                "command = DrawCommand(",
                "    x={{ x_position | default(400) }},",
                "    y={{ y_position | default(300) }},",
                "    size={{ size | default(50) }},",
                "    color={{ color | default('arcade.color.WHITE') }}",
                ")",
                "renderer.render(command)"
            ]
    
    def _generate_arcade_audio_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade audio template code."""
        
        if style == "beginner":
            return [
                f"# {pattern.title} - play sounds in your game",
                "# Load the sound file first",
                "sound = arcade.load_sound('{{ sound_file }}', streaming={{ streaming | default('False') }})",
                "",
                "# Play the sound",
                "arcade.play_sound(sound, volume={{ volume | default(1.0) }})",
                "",
                "# Try different sound files and volumes!"
            ]
        else:
            return [
                f"# {pattern.title} - advanced audio management",
                "class AudioManager:",
                "    def __init__(self):",
                "        self.sounds: dict[str, arcade.Sound] = {}",
                "        self.volume = {{ master_volume | default(1.0) }}",
                "",
                "    def load_sound(self, name: str, file_path: str) -> None:",
                "        \"\"\"Load and cache a sound - student customizes this!\"\"\"",
                "        self.sounds[name] = arcade.load_sound(file_path)",
                "",
                "    def play(self, name: str, volume: float = None) -> None:",
                "        \"\"\"Play a loaded sound\"\"\"",
                "        if name in self.sounds:",
                "            actual_volume = (volume or self.volume) * self.volume",
                "            arcade.play_sound(self.sounds[name], volume=actual_volume)",
                "",
                "# Usage",
                "audio = AudioManager()",
                "audio.load_sound('{{ sound_name }}', '{{ sound_file }}')",
                "audio.play('{{ sound_name }}', volume={{ volume | default(1.0) }})"
            ]
    
    def _generate_arcade_input_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade input handling template code."""
        
        if style == "beginner":
            return [
                f"# {pattern.title} - handle player input",
                "def on_key_press(self, key, modifiers):",
                "    \"\"\"Handle key presses - customize the actions!\"\"\"",
                "    if key == {{ key_binding | default('arcade.key.SPACE') }}:",
                "        # Student action here - what should happen?",
                "        {{ action | default('print(\"Key pressed!\")') }}",
                "",
                "def on_key_release(self, key, modifiers):",
                "    \"\"\"Handle key releases\"\"\"",
                "    if key == {{ key_binding | default('arcade.key.SPACE') }}:",
                "        # Student release action here",
                "        {{ release_action | default('print(\"Key released!\")') }}"
            ]
        else:
            return [
                f"# {pattern.title} - input system with state",
                "from enum import Enum",
                "",
                "class InputState(Enum):",
                "    IDLE = 'idle'",
                "    PRESSED = 'pressed'",
                "    HELD = 'held'",
                "",
                "class InputHandler:",
                "    def __init__(self):",
                "        self.key_states: dict[int, InputState] = {}",
                "        self.key_bindings = {{ key_bindings | default('{}') }}",
                "",
                "    def on_key_press(self, key: int, modifiers: int) -> None:",
                "        \"\"\"Student customizes key bindings and actions\"\"\"",
                "        self.key_states[key] = InputState.PRESSED",
                "        if key in self.key_bindings:",
                "            action = self.key_bindings[key]",
                "            self.execute_action(action)",
                "",
                "    def execute_action(self, action: str) -> None:",
                "        \"\"\"Execute bound action - student implements this!\"\"\"",
                "        # Student customization point",
                "        pass",
                "",
                "# Usage",
                "input_handler = InputHandler()",
                "# Student sets up key bindings",
                "input_handler.key_bindings[{{ key_binding | default('arcade.key.SPACE') }}] = '{{ action_name }}'"
            ]
    
    def _generate_arcade_collision_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade collision detection template code."""
        
        if style == "beginner":
            return [
                f"# {pattern.title} - detect when things hit",
                "# Check if two sprites are touching",
                "hit_list = arcade.check_for_collision_with_list(",
                "    {{ sprite_a | default('player_sprite') }},",
                "    {{ sprite_list_b | default('enemy_list') }}",
                ")",
                "",
                "# Do something when they collide",
                "for hit_sprite in hit_list:",
                "    # Student action here - what happens when they collide?",
                "    {{ collision_action | default('hit_sprite.remove_from_sprite_lists()') }}",
                "    {{ player_action | default('print(\"Hit!\")') }}"
            ]
        else:
            return [
                f"# {pattern.title} - collision system with response",
                "from typing import Protocol, Callable",
                "",
                "class CollisionResponse(Protocol):",
                "    def respond(self, sprite_a: arcade.Sprite, sprite_b: arcade.Sprite) -> None: ...",
                "",
                "class CollisionManager:",
                "    def __init__(self):",
                "        self.responses: dict[tuple[str, str], CollisionResponse] = {}",
                "",
                "    def register_response(self, type_a: str, type_b: str, response: CollisionResponse):",
                "        \"\"\"Student registers collision responses\"\"\"",
                "        self.responses[(type_a, type_b)] = response",
                "",
                "    def check_collisions(self, sprite_lists: dict[str, arcade.SpriteList]):",
                "        \"\"\"Check all registered collision pairs\"\"\"",
                "        for (type_a, type_b), response in self.responses.items():",
                "            if type_a in sprite_lists and type_b in sprite_lists:",
                "                for sprite_a in sprite_lists[type_a]:",
                "                    hit_list = arcade.check_for_collision_with_list(sprite_a, sprite_lists[type_b])",
                "                    for sprite_b in hit_list:",
                "                        response.respond(sprite_a, sprite_b)",
                "",
                "# Usage - student creates custom responses",
                "collision_manager = CollisionManager()"
            ]
    
    def _generate_arcade_motion_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate Arcade motion/physics template code."""
        
        return [
            f"# {pattern.title} - make things move",
            "# Apply velocity to sprite",
            "{{ sprite_name | default('my_sprite') }}.change_x = {{ velocity_x | default(0) }}",
            "{{ sprite_name | default('my_sprite') }}.change_y = {{ velocity_y | default(0) }}",
            "",
            "# Update sprite position (call this every frame)",
            "{{ sprite_name | default('my_sprite') }}.update()",
            "",
            "# Student physics customization:",
            "# Add gravity: sprite.change_y -= 0.5",
            "# Add friction: sprite.change_x *= 0.95"
        ]
    
    def _generate_arcade_generic_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate generic Arcade template code."""
        
        return [
            f"# {pattern.title} implementation",
            f"# Function: {pattern.source_function}",
            f"# Parameters: {', '.join(pattern.common_parameters)}",
            "",
            "# Student implementation area:",
            "# TODO: Implement {{ pattern_name }} functionality",
            "# Use the Arcade documentation for guidance:",
            f"# https://arcade.academy/en/latest/",
            "",
            "# Basic structure:",
            "# result = {{ function_call }}({{ parameter_list }})",
            "pass  # Replace with your implementation"
        ]


class PygameTemplateRules(TemplateGenerationRules):
    """Template generation rules for Pygame."""
    
    def __init__(self):
        super().__init__(GameLibrary.PYGAME_CE)
    
    def get_template_header(self, pattern: PatternSuggestion) -> list[str]:
        """Generate template header for Pygame patterns."""
        return [
            f"{{# {pattern.title} Template #}}",
            f"{{# Generated for Pygame {pattern.category} category #}}",
            f"{{# Complexity: {pattern.complexity}/5 #}}",
            "",
            f"# === {pattern.title} ===",
            f"# {pattern.description}",
            "# Pygame implementation",
            ""
        ]
    
    def get_choice_variables(self, pattern: PatternSuggestion) -> list[str]:
        """Generate Jinja2 variables for Pygame student choices."""
        # Same as Arcade for now - could be customized per library
        variables = []
        
        for choice in pattern.suggested_choices:
            choice_id = choice["choice_id"]
            default_value = choice.get("default", "default")
            variables.append(
                f"{{%- set {choice_id} = choices.get('{choice_id}', '{default_value}') %}}"
            )
        
        if variables:
            variables.append("")
        
        return variables
    
    def get_category_template(self, pattern: PatternSuggestion, style: str) -> list[str]:
        """Generate category-specific template code for Pygame."""
        
        category = pattern.category.lower()
        
        if category == "sprites":
            return [
                f"# {pattern.title} - Pygame sprite",
                "class {{ sprite_class_name | default('GameSprite') }}(pygame.sprite.Sprite):",
                "    def __init__(self, image_path: str):",
                "        super().__init__()",
                "        self.image = pygame.image.load(image_path)",
                "        self.rect = self.image.get_rect()",
                "        self.rect.center = ({{ start_x | default(400) }}, {{ start_y | default(300) }})",
                "",
                "# Create sprite",
                "{{ sprite_name | default('my_sprite') }} = {{ sprite_class_name | default('GameSprite') }}('{{ sprite_image }}')"
            ]
        elif category == "visual":
            return [
                f"# {pattern.title} - Pygame drawing",
                "pygame.draw.{{ shape_type | default('circle') }}(",
                "    screen,  # Surface to draw on",
                "    {{ color | default('(255, 255, 255)') }},  # Color (R, G, B)",
                "    ({{ x_position | default(400) }}, {{ y_position | default(300) }}),  # Position",
                "    {{ size | default(50) }}  # Size",
                ")"
            ]
        else:
            return [f"# {pattern.title} - Pygame implementation", "# TODO: Implement Pygame version"]
    
    def get_function_mapping(self, pattern: PatternSuggestion) -> dict[str, str]:
        """Map pattern opcodes to Pygame function calls."""
        
        func_name = pattern.source_function.replace("pygame.", "")
        
        return {
            "function_call": f"pygame.{func_name}",
            "import_statement": "import pygame", 
            "module_reference": "pygame",
            "documentation_url": f"https://www.pygame.org/docs/ref/{func_name.split('.')[0]}.html"
        }
    
    @property
    def supported_categories(self) -> set[str]:
        """Categories supported by Pygame."""
        return {"sprites", "visual", "audio", "input", "collision", "game"}
    
    @property
    def template_styles(self) -> dict[str, dict[str, object]]:
        """Template styles for Pygame."""
        return {
            "beginner": {"comment_verbosity": "high", "code_structure": "simple"},
            "intermediate": {"comment_verbosity": "medium", "code_structure": "functions"},
            "advanced": {"comment_verbosity": "low", "code_structure": "classes"}
        }


def get_template_rules(library: GameLibrary) -> TemplateGenerationRules:
    """Get template generation rules for a specific library."""
    
    if library == GameLibrary.ARCADE:
        return ArcadeTemplateRules()
    elif library in (GameLibrary.PYGAME, GameLibrary.PYGAME_CE):
        return PygameTemplateRules()
    else:
        raise ValueError(f"Template rules not implemented for {library}")


class ScalableTemplateGenerator:
    """Scalable template generator using library-specific rules."""
    
    def __init__(self, library: GameLibrary):
        self.library = library
        self.rules = get_template_rules(library)
    
    def generate_template(self, pattern: PatternSuggestion, style: str = "intermediate") -> str:
        """Generate complete Jinja2 template for a pattern."""
        
        # Validate style
        if style not in self.rules.template_styles:
            raise ValueError(f"Style '{style}' not supported for {self.library.name}")
        
        # Validate category
        if pattern.category.lower() not in self.rules.supported_categories:
            raise ValueError(f"Category '{pattern.category}' not supported for {self.library.name}")
        
        # Build template
        template_lines = []
        
        # Header with metadata
        template_lines.extend(self.rules.get_template_header(pattern))
        
        # Choice variables
        template_lines.extend(self.rules.get_choice_variables(pattern))
        
        # Category-specific implementation
        template_lines.extend(self.rules.get_category_template(pattern, style))
        
        # Function mapping info as comments
        mapping = self.rules.get_function_mapping(pattern)
        template_lines.extend([
            "",
            f"# Function reference: {mapping['function_call']}",
            f"# Documentation: {mapping['documentation_url']}"
        ])
        
        return "\n".join(template_lines)
    
    def generate_multiple_templates(self, patterns: list[PatternSuggestion], style: str = "intermediate") -> dict[str, str]:
        """Generate templates for multiple patterns."""
        
        templates = {}
        
        for pattern in patterns:
            try:
                template_content = self.generate_template(pattern, style)
                templates[pattern.opcode] = template_content
            except ValueError as e:
                # Log warning but continue with other patterns
                print(f"Warning: Skipping pattern {pattern.opcode}: {e}")
                continue
        
        return templates
