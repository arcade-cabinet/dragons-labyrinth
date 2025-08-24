"""
Library-specific scanning rules for different game development frameworks.

This module contains configuration for scanning each library's API to generate
appropriate patterns for different educational phases.
"""

from __future__ import annotations

from professor_pixel.types import GameLibrary, EducationalPhase, PatternCategory, AssetFileType


class LibraryScanRules:
    """Base class for library-specific scanning configuration (NO EXCLUSIONS)."""
    
    def __init__(self, library: GameLibrary):
        self.library = library
    
    @property
    def core_modules(self) -> set[str]:
        """ALL modules to scan for this library (comprehensive)."""
        raise NotImplementedError
    
    @property
    def exclude_modules(self) -> set[str]:
        """Only exclude truly internal/broken modules."""
        raise NotImplementedError
    
    @property 
    def category_mappings(self) -> dict[str, str]:
        """Map function/module names to educational categories."""
        raise NotImplementedError
    
    @property
    def complexity_indicators(self) -> dict[str, int]:
        """Keywords that indicate complexity levels (for metadata only)."""
        raise NotImplementedError


class ArcadeRules(LibraryScanRules):
    """Arcade scanning rules - scan EVERYTHING, let database queries filter by skill."""
    
    def __init__(self):
        super().__init__(GameLibrary.ARCADE)
    
    @property
    def core_modules(self) -> set[str]:
        """ALL Arcade modules (comprehensive scan for complete API coverage)."""
        return {
            "arcade",               # Root module with aliases
            "arcade.draw",          # Drawing primitives
            "arcade.sprite",        # Sprite management  
            "arcade.sound",         # Audio
            "arcade.texture",       # Texture loading
            "arcade.camera",        # Camera system (advanced but scannable)
            "arcade.gui",           # GUI widgets (complex but scannable)
            "arcade.scene",         # Scene management
            "arcade.math",          # Math utilities
            "arcade.color",         # Color constants
            "arcade.key",           # Key constants
            "arcade.physics_engines", # Physics (advanced)
            "arcade.tilemap",       # Tile maps (advanced)
            "arcade.particles",     # Particle effects (advanced)
            "arcade.isometric",     # Isometric rendering (advanced)
            "arcade.sections",      # Section management (advanced)
        }
    
    @property
    def exclude_modules(self) -> set[str]:
        """Only exclude truly broken/internal modules."""
        return {
            "arcade.examples",        # Example code, not API
            "arcade.experimental",    # Unstable/broken features
            "arcade.future",         # Deprecated/migration code
            "arcade._internal",      # Internal implementation
            "arcade.gl.backends",    # Backend-specific code
            "arcade.__pyinstaller",  # Build system code
        }
    
    @property
    def category_mappings(self) -> list[tuple[str, str]]:
        """Map function names to educational categories (ordered by specificity)."""
        return [
            # Most specific patterns first (to avoid over-matching)
            
            # Collision detection (most specific)
            ("check_for_collision", "collision"),
            ("collision", "collision"),
            ("hit_test", "collision"),
            
            # Audio functions (specific)
            ("play_sound", "audio"),
            ("load_sound", "audio"),
            ("stop_sound", "audio"),
            ("sound", "audio"),
            ("audio", "audio"),
            ("music", "audio"),
            ("mixer", "audio"),
            
            # Sprite operations (specific)
            ("sprite_list", "sprites"),
            ("spritesheet", "sprites"),
            ("animated", "sprites"),
            ("sprite", "sprites"),
            
            # Input handling (specific)
            ("get_pressed", "input"),
            ("controller", "input"),
            ("mouse", "input"),
            ("key", "input"),
            ("event", "input"),
            ("input", "input"),
            
            # Game/level systems (specific)
            ("tilemap", "game"),
            ("tile_map", "game"),
            ("scene", "game"),
            ("view", "game"),
            ("window", "game"),
            ("application", "game"),
            
            # Physics/movement (specific)
            ("physics_engine", "motion"),
            ("pymunk", "motion"),
            ("velocity", "motion"),
            ("acceleration", "motion"),
            ("physics", "motion"),
            
            # Texture management (specific but before general drawing)
            ("load_texture", "sprites"),
            ("texture", "sprites"),
            
            # Drawing functions (less specific - put last)
            ("draw_circle", "visual"),
            ("draw_rectangle", "visual"),
            ("draw_line", "visual"),
            ("draw_text", "visual"),
            ("draw_point", "visual"),
            ("draw_polygon", "visual"),
            ("draw_arc", "visual"),
            ("draw_ellipse", "visual"),
            ("draw_triangle", "visual"),
            ("draw", "visual"),  # Most general - put at end
            
            # Advanced graphics (specific)
            ("camera", "visual"),
            ("light", "visual"),
            ("particle", "visual"),
            ("shader", "visual"),
            ("gl", "visual"),
            
            # UI/Interface (specific)
            ("gui", "visual"),
            ("widget", "visual"),
            ("button", "visual"),
            ("label", "visual"),
        ]
    
    @property
    def complexity_indicators(self) -> dict[str, int]:
        """Keywords that add complexity points (metadata only, not exclusions)."""
        return {
            # Simple operations (complexity boost: +0)
            "draw_": 0,
            "load_": 0,
            "play_": 0,
            "stop_": 0,
            "get_": 0,
            
            # Moderate operations (complexity boost: +1)  
            "create_": 1,
            "update_": 1,
            "check_": 1,
            "set_": 1,
            
            # Advanced operations (complexity boost: +2)
            "manager": 2,
            "engine": 2,
            "context": 2,
            "buffer": 2,
            "shader": 2,
            "advanced": 2,
            
            # Expert operations (complexity boost: +3)
            "opengl": 3,
            "compute": 3,
            "async": 3,
            "thread": 3,
        }


class PygameRules(LibraryScanRules):
    """Pygame scanning rules - comprehensive API coverage."""
    
    def __init__(self):
        super().__init__(GameLibrary.PYGAME_CE)
    
    @property
    def core_modules(self) -> set[str]:
        return {
            "pygame",
            "pygame.sprite",
            "pygame.surface", 
            "pygame.rect",
            "pygame.mixer",
            "pygame.event",
            "pygame.key",
            "pygame.mouse",
        }
    
    @property
    def exclude_modules(self) -> set[str]:
        return {
            "pygame.examples",
            "pygame._internal",
            "pygame.freetype",  # Advanced text
            "pygame.gfxdraw",   # Advanced graphics
        }
    
    @property
    def validation_rules(self) -> dict[str, object]:
        return {
            "forbidden_functions": {"quit", "init", "_internal"},
            "complex_classes": {"Clock", "Group"},
            "beginner_safe": {
                "Surface", "Rect", "draw.circle", "draw.rect",
                "mixer.Sound", "event.get", "key.get_pressed"
            },
            "parameter_limits": {
                "max_parameters": 6,
                "required_parameters": 3,
            }
        }
    
    @property
    def category_mappings(self) -> dict[str, str]:
        return {
            "draw": "visual",
            "sprite": "sprites", 
            "mixer": "audio",
            "event": "input",
            "key": "input",
            "mouse": "input",
            "surface": "visual",
            "rect": "collision",
        }


class PySDL2Rules(LibraryScanRules):
    """PySDL2 scanning rules - comprehensive low-level API coverage."""
    
    def __init__(self):
        super().__init__(GameLibrary.PYSDL2)
    
    @property
    def core_modules(self) -> set[str]:
        return {
            "sdl2",
            "sdl2.ext",
            "sdl2.sdlgfx",
            "sdl2.sdlmixer",
        }
    
    @property
    def exclude_modules(self) -> set[str]:
        return {
            "sdl2.test",
            "sdl2.examples",
        }
    
    @property
    def validation_rules(self) -> dict[str, object]:
        return {
            "forbidden_functions": {"SDL_Quit", "SDL_Init"},
            "complex_classes": set(),
            "beginner_safe": {
                "SDL_CreateWindow", "SDL_CreateRenderer",
                "SDL_SetRenderDrawColor", "SDL_RenderDrawRect",
                "SDL_LoadBMP", "SDL_CreateTextureFromSurface"
            },
            "parameter_limits": {
                "max_parameters": 10,  # SDL functions can be very detailed
                "required_parameters": 5,
            }
        }
    
    @property
    def category_mappings(self) -> dict[str, str]:
        return {
            "render": "visual",
            "draw": "visual",
            "texture": "sprites",
            "surface": "sprites", 
            "mixer": "audio",
            "event": "input",
            "window": "game",
        }


# Factory function to get rules for a library
def get_library_rules(library: GameLibrary) -> LibraryScanRules:
    """Get comprehensive scanning rules for a specific library."""
    if library == GameLibrary.ARCADE:
        return ArcadeRules()
    elif library in [GameLibrary.PYGAME_CE, GameLibrary.PYGAME]:
        return PygameRules()
    elif library == GameLibrary.PYSDL2:
        return PySDL2Rules()
    else:
        raise ValueError(f"No scanning rules defined for library: {library}")
