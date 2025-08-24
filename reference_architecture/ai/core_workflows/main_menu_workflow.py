"""
Main Menu Workflow - AI-Generated Core UI.

This workflow generates the main menu screen using:
- Trophy case (horizontal row of scaled trophies from save data)
- Professor intro video (scaled down, plays once then pauses)
- Image map for interactive areas (parsed from descriptive filename)
- Save slot management (XDG state dir, PressStart2P font, up/down navigation)

This serves as "dogfooding" to test our template/pattern/IR system on predictable UI.
"""

from typing import Literal, Any
from datetime import datetime
from pathlib import Path

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.models import (
    CoreAssetRecord, ImageMapMetadata, InteractiveArea,
    UIRectangle, UICoordinate, PatternSuggestion, CoreSpecification
)
from professor_pixel.schemas.core_rules import parse_core_asset
from professor_pixel.settings import get_settings
from professor_pixel.types import EducationalPhase, GameLibrary


class MainMenuWorkflowState(BaseModel):
    """State for main menu generation workflow."""
    
    # Core assets discovered
    academy_background: CoreAssetRecord | None = Field(default=None, description="Academy backdrop asset")
    main_menu_image_map: CoreAssetRecord | None = Field(default=None, description="Interactive image map")
    professor_intro_video: CoreAssetRecord | None = Field(default=None, description="Professor intro video")
    trophy_asset: CoreAssetRecord | None = Field(default=None, description="Trophy icon for trophy case")
    press_start_font: CoreAssetRecord | None = Field(default=None, description="PressStart2P font")
    
    # Generated layout specifications
    trophy_case_layout: dict[str, Any] = Field(default_factory=dict, description="Trophy display layout")
    professor_video_layout: dict[str, Any] = Field(default_factory=dict, description="Video section layout")
    save_slot_layout: dict[str, Any] = Field(default_factory=dict, description="Save slot management layout")
    
    # Interactive elements
    interactive_areas: list[InteractiveArea] = Field(default_factory=list, description="Clickable areas")
    
    # Generated specification (not raw code - uses our pattern system!)
    main_menu_specification: CoreSpecification | None = Field(default=None, description="Pattern-based specification for compilation")
    
    # Workflow metadata
    workflow_id: str = Field(description="Workflow identifier")
    started_at: datetime = Field(default_factory=datetime.now)
    completed_at: datetime | None = Field(default=None)
    step_count: int = Field(default=0)


class MainMenuWorkflow(BaseComponent):
    """Workflow for generating main menu from core assets."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
    
    def discover_core_assets_node(self, state: MainMenuWorkflowState) -> dict[str, Any]:
        """Node: Discover and parse core assets using intelligent filename analysis."""
        
        self.log_info("Discovering core assets using intelligent filename parsing")
        
        settings = get_settings()
        core_assets_path = settings.paths.library_dir.parent / "core"  # assets/core/
        
        discovered_assets = {}
        
        # Scan each core category and parse filenames intelligently
        for category_dir in core_assets_path.iterdir():
            if category_dir.is_dir():
                category = category_dir.name
                
                for asset_file in category_dir.iterdir():
                    if asset_file.is_file():
                        # Use intelligent parsing from core_rules.py
                        metadata = parse_core_asset(category, asset_file.name)
                        
                        if metadata.get("parsed"):
                            asset_record = CoreAssetRecord(
                                path=f"{category}/{asset_file.name}",
                                category=category,
                                filename=asset_file.name,
                                display_name=metadata["display_name"],
                                description=metadata["description"],
                                metadata=metadata,
                                asset_type=category,
                                media_type=self._classify_media_type(asset_file.suffix)
                            )
                            
                            # Store by category for easy access
                            if category not in discovered_assets:
                                discovered_assets[category] = []
                            discovered_assets[category].append(asset_record)
        
        # Select the best assets for main menu (using intelligent criteria)
        academy_bg = self._select_best_background(discovered_assets.get("backgrounds", []))
        image_map = self._select_main_menu_image_map(discovered_assets.get("image_maps", []))
        prof_video = self._select_intro_video(discovered_assets.get("professors", []))
        trophy = self._select_trophy_asset(discovered_assets.get("resources", []))
        font = self._select_ui_font(discovered_assets.get("typography", []))
        
        self.log_success(f"Intelligently selected core assets from {len(discovered_assets)} categories")
        
        return {
            "academy_background": academy_bg,
            "main_menu_image_map": image_map,
            "professor_intro_video": prof_video,
            "trophy_asset": trophy,
            "press_start_font": font,
            "step_count": state.step_count + 1
        }
    
    def _classify_media_type(self, extension: str) -> str:
        """Classify file extension to media type."""
        ext = extension.lower()
        if ext in ['.png', '.jpg', '.jpeg', '.svg']:
            return "image"
        elif ext in ['.mp4', '.avi', '.mov']:
            return "video"
        elif ext in ['.ttf', '.otf', '.woff', '.woff2']:
            return "font"
        return "unknown"
    
    def _select_best_background(self, backgrounds: list[CoreAssetRecord]) -> CoreAssetRecord | None:
        """Select best background for main menu."""
        # Prefer full screen layouts for main menu
        for bg in backgrounds:
            if bg.metadata.get("layout") == "full_screen":
                return bg
        return backgrounds[0] if backgrounds else None
    
    def _select_main_menu_image_map(self, image_maps: list[CoreAssetRecord]) -> CoreAssetRecord | None:
        """Select image map with start/continue buttons."""
        # Look for image maps with button interactions
        for img_map in image_maps:
            if "start_button" in str(img_map.metadata.get("interactive_areas", [])):
                return img_map
        return image_maps[0] if image_maps else None
    
    def _select_intro_video(self, professors: list[CoreAssetRecord]) -> CoreAssetRecord | None:
        """Select professor intro video."""
        # Prefer intro videos
        for prof in professors:
            if prof.metadata.get("is_intro_video"):
                return prof
        return professors[0] if professors else None
    
    def _select_trophy_asset(self, resources: list[CoreAssetRecord]) -> CoreAssetRecord | None:
        """Select trophy for trophy case."""
        # Look for trophy resources
        for resource in resources:
            if resource.metadata.get("resource_type") == "trophy":
                return resource
        return resources[0] if resources else None
    
    def _select_ui_font(self, fonts: list[CoreAssetRecord]) -> CoreAssetRecord | None:
        """Select UI font."""
        # Prefer retro pixel fonts for game UI
        for font in fonts:
            if font.metadata.get("font_category") == "retro_pixel":
                return font
        return fonts[0] if fonts else None
    
    def generate_layout_specifications_node(self, state: MainMenuWorkflowState) -> dict[str, Any]:
        """Node: Generate detailed layout specifications for all UI components."""
        
        self.log_info("Generating layout specifications for main menu components")
        
        # Extract interactive areas from image map
        interactive_areas = []
        if state.main_menu_image_map and "interactive_areas" in state.main_menu_image_map.metadata:
            for area_data in state.main_menu_image_map.metadata["interactive_areas"]:
                bounds_data = area_data["bounds"]
                bounds = UIRectangle(
                    top_left=UICoordinate(**bounds_data["top_left"]),
                    bottom_right=UICoordinate(**bounds_data["bottom_right"])
                )
                
                area = InteractiveArea(
                    name=area_data["name"],
                    description=area_data["description"],
                    bounds=bounds,
                    action_type=area_data["action_type"],
                    action_data=area_data["action_data"]
                )
                interactive_areas.append(area)
        
        # Trophy case layout (top section, horizontal row)
        trophy_case_layout = {
            "section_type": "trophy_case",
            "position": "top",
            "layout": "horizontal_row",
            "trophy_scale": 0.1,  # Scale down 1024x1024 trophies to ~100px
            "spacing": 20,
            "max_trophies_visible": 10,
            "scroll_arrows": True
        }
        
        # Professor video layout (left center)
        prof_video_layout = {
            "section_type": "professor_video",
            "position": "left_center", 
            "video_scale": 0.4,  # Scale down 512x768 to fit section
            "play_behavior": "play_once_then_pause",
            "click_behavior": "restart_video"
        }
        
        # Save slot layout (bottom section)
        save_slot_layout = {
            "section_type": "save_slots",
            "position": "bottom",
            "font": "PressStart2P",
            "font_size": 16,
            "max_slots_visible": 5,
            "navigation": "up_down_arrows_and_keys",
            "selection_highlight": True,
            "truncated_view": True
        }
        
        self.log_success(f"Generated layout specs with {len(interactive_areas)} interactive areas")
        
        return {
            "interactive_areas": interactive_areas,
            "trophy_case_layout": trophy_case_layout,
            "professor_video_layout": prof_video_layout,
            "save_slot_layout": save_slot_layout,
            "step_count": state.step_count + 1
        }
    
    def generate_main_menu_specification_node(self, state: MainMenuWorkflowState) -> dict[str, Any]:
        """Node: Generate main menu specification using our AI pattern system."""
        
        self.log_info("Creating main menu specification using Professor Pixel pattern system")
        
        # Create a specification using our pattern system, not manual code!
        # This demonstrates the full AI pipeline: assets → patterns → specification → generated code
        
        # Analyze what patterns are needed for this main menu
        required_ui_patterns = []
        
        # Background pattern
        if state.academy_background:
            background_pattern = PatternSuggestion(
                opcode="LOAD_BACKGROUND_TEXTURE",
                title="Load Background Image", 
                description="Load and display a background texture",
                complexity=1,
                category="visual",
                source_function="arcade.load_texture",
                common_parameters=["texture_path", "scaling"],
                template_file="visual/load_background.jinja2",
                suggested_choices=[
                    {"name": "background_asset", "value": state.academy_background.path, "required": True},
                    {"name": "scaling_mode", "options": ["stretch", "fit", "center"], "default": "stretch"}
                ]
            )
            required_ui_patterns.append(background_pattern)
        
        # Interactive areas patterns
        for area in state.interactive_areas:
            area_pattern = PatternSuggestion(
                opcode=f"CREATE_INTERACTIVE_AREA_{area.name.upper().replace(' ', '_')}",
                title=f"Create {area.name.title()} Area",
                description=f"Create clickable area for {area.name}",
                complexity=2,
                category="input",
                source_function="arcade.check_for_collision_with_point",
                common_parameters=["x", "y", "width", "height", "callback"],
                template_file="input/interactive_area.jinja2",
                suggested_choices=[
                    {"name": "area_bounds", "value": area.bounds.to_arcade_rect(), "required": True},
                    {"name": "action_type", "value": area.action_type, "required": True},
                    {"name": "action_data", "value": area.action_data, "required": True}
                ]
            )
            required_ui_patterns.append(area_pattern)
        
        # Trophy display pattern (if trophy asset available)
        if state.trophy_asset:
            trophy_pattern = PatternSuggestion(
                opcode="CREATE_TROPHY_CASE",
                title="Create Trophy Display Case",
                description="Display student achievements as trophy icons",
                complexity=3,
                category="visual",
                source_function="arcade.SpriteList", 
                common_parameters=["trophy_texture", "scale", "spacing"],
                template_file="visual/trophy_case.jinja2",
                suggested_choices=[
                    {"name": "trophy_asset", "value": state.trophy_asset.path, "required": True},
                    {"name": "trophy_scale", "value": 0.1, "required": True},
                    {"name": "layout", "value": "horizontal", "required": True}
                ]
            )
            required_ui_patterns.append(trophy_pattern)
        
        # Save slot management pattern
        save_slot_pattern = PatternSuggestion(
            opcode="CREATE_SAVE_SLOT_MANAGER",
            title="Create Save Slot Manager", 
            description="Manage save slot selection and loading",
            complexity=4,
            category="input",
            source_function="arcade.key",
            common_parameters=["font", "max_visible", "navigation_keys"],
            template_file="input/save_slot_manager.jinja2",
            suggested_choices=[
                {"name": "font_asset", "value": state.press_start_font.path if state.press_start_font else None},
                {"name": "max_slots_visible", "value": 5, "required": True},
                {"name": "navigation", "value": "up_down_keys_and_mouse", "required": True}
            ]
        )
        required_ui_patterns.append(save_slot_pattern)
        
        # Classify patterns by behavior type
        initialization_patterns = []
        event_handling_patterns = []
        rendering_patterns = []
        
        for pattern in required_ui_patterns:
            if "LOAD" in pattern.opcode or "CREATE" in pattern.opcode:
                initialization_patterns.append(pattern.opcode)
            elif "INTERACTIVE" in pattern.opcode or "MANAGER" in pattern.opcode:
                event_handling_patterns.append(pattern.opcode)
            else:
                rendering_patterns.append(pattern.opcode)
        
        # Create a proper CoreSpecification for the main menu UI component
        main_menu_spec = CoreSpecification(
            component_id="core_main_menu",
            component_type="main_menu",
            title="Professor Pixel Main Menu",
            description="AI-generated main menu with trophy case, professor video, and save management",
            patterns_used=[pattern.opcode for pattern in required_ui_patterns],
            template_files=[pattern.template_file for pattern in required_ui_patterns],
            core_assets_used={
                "background": state.academy_background.model_dump() if state.academy_background else None,
                "image_map": state.main_menu_image_map.model_dump() if state.main_menu_image_map else None,
                "professor_video": state.professor_intro_video.model_dump() if state.professor_intro_video else None,
                "trophy": state.trophy_asset.model_dump() if state.trophy_asset else None,
                "font": state.press_start_font.model_dump() if state.press_start_font else None
            },
            layout_sections={
                "trophy_case": state.trophy_case_layout,
                "professor_section": state.professor_video_layout,
                "save_slots": state.save_slot_layout
            },
            interactive_areas=[area.model_dump() for area in state.interactive_areas],
            initialization_patterns=initialization_patterns,
            event_handling_patterns=event_handling_patterns,
            rendering_patterns=rendering_patterns,
            output_files=["main_menu_view.py"],
            complexity_level=3  # Core UI is moderately complex
        )
        
        # Validate the specification
        is_valid, validation_errors = main_menu_spec.validate_specification()
        if not is_valid:
            self.log_warning(f"Specification validation issues: {', '.join(validation_errors)}")
        
        self.log_success(f"Generated main menu specification with {len(required_ui_patterns)} patterns")
        self.log_info("Specification ready for compilation using existing template system")
        
        if not is_valid:
            self.log_error(f"Specification validation failed: {', '.join(validation_errors)}")
        
        return {
            "main_menu_specification": main_menu_spec,  # CoreSpecification object
            "completed_at": datetime.now(),
            "step_count": state.step_count + 1
        }
    
    # Note: We removed manual code generation methods
    # The main menu is now generated through our pattern/template system
    # This workflow creates specifications that get compiled by our existing AI infrastructure
    
    def build_workflow(self) -> StateGraph:
        """Build the main menu generation workflow."""
        
        workflow = StateGraph(MainMenuWorkflowState)
        
        # Add nodes
        workflow.add_node("discover_assets", self.discover_core_assets_node)
        workflow.add_node("generate_layouts", self.generate_layout_specifications_node)
        workflow.add_node("generate_specification", self.generate_main_menu_specification_node)
        
        # Add edges - simple linear flow for core UI generation
        workflow.add_edge(START, "discover_assets")
        workflow.add_edge("discover_assets", "generate_layouts")
        workflow.add_edge("generate_layouts", "generate_specification")
        workflow.add_edge("generate_specification", END)
        
        return workflow
    
    def compile_workflow(self, checkpointer=None, durability: Literal["exit", "async", "sync"] = "async") -> StateGraph:
        """Compile workflow for execution."""
        
        workflow = self.build_workflow()
        
        if checkpointer:
            compiled = workflow.compile(
                checkpointer=checkpointer,
                durability=durability
            )
        else:
            compiled = workflow.compile()
        
        self.log_info("Main menu workflow compiled and ready for code generation")
        return compiled


def create_main_menu_workflow() -> StateGraph:
    """Factory function to create main menu generation workflow."""
    workflow = MainMenuWorkflow()
    return workflow.compile_workflow()
