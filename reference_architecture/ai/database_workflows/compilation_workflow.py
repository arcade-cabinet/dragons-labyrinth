"""
LangGraph workflow for lesson compilation and template generation.

This module implements a durable workflow for compiling AI-generated patterns
into lesson structures and Jinja2 templates with human review capabilities.
"""

from typing import Literal, Any
from datetime import datetime
from pathlib import Path

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.models import PatternSuggestion, LessonProgressionSuggestion
from professor_pixel.schemas.ai.template_rules import ScalableTemplateGenerator
from professor_pixel.types import GameLibrary


class CompilationWorkflowState(BaseModel):
    """State schema for the lesson compilation workflow."""
    
    # Input from analysis workflow
    patterns: list[PatternSuggestion] = Field(description="Pattern suggestions to compile")
    lessons: list[LessonProgressionSuggestion] = Field(description="Lesson progression to compile")
    library_name: str = Field(description="Library name")
    
    # Compilation configuration
    template_style: str = Field(description="Template complexity style") 
    output_directory: str = Field(description="Output directory for files")
    autonomous_mode: bool = Field(description="Skip human review")
    
    # Template generation
    generated_templates: dict[str, str] = Field(default_factory=dict, description="Generated templates")
    template_manifest: dict[str, Any] = Field(default_factory=dict, description="Template metadata")
    
    # Lesson structure generation
    lesson_specifications: list[dict[str, Any]] = Field(default_factory=list, description="Lesson specifications")
    curriculum_structure: dict[str, Any] = Field(default_factory=dict, description="Curriculum structure")
    
    # Human review
    template_approval: str | None = Field(default=None, description="Template approval status")
    template_feedback: dict[str, str] | None = Field(default=None, description="Template feedback")
    
    # Output
    final_templates: dict[str, str] = Field(default_factory=dict, description="Final approved templates")
    final_curriculum: dict[str, Any] = Field(default_factory=dict, description="Final curriculum structure")
    output_paths: list[str] = Field(default_factory=list, description="Generated file paths")
    
    # Metadata
    workflow_id: str = Field(description="Workflow identifier")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Number of completed steps")


class CompilationWorkflow(BaseComponent):
    """LangGraph workflow for lesson compilation with durable execution."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        # Will be initialized when we know the library
        self.template_generator: ScalableTemplateGenerator | None = None
    
    def generate_templates_node(self, state: CompilationWorkflowState) -> dict[str, object]:
        """Node: Generate Jinja2 templates for each pattern using scalable rules."""
        
        self.log_info(f"Generating templates for {len(state.patterns)} patterns")
        
        # Initialize template generator for this library
        library = GameLibrary[state.library_name.upper()]
        if not self.template_generator:
            self.template_generator = ScalableTemplateGenerator(library)
        
        # Generate all templates using scalable rules
        generated_templates = self.template_generator.generate_multiple_templates(
            patterns=state.patterns,
            style=state.template_style
        )
        
        # Create manifest
        template_manifest = {
            "generated_at": datetime.now().isoformat(),
            "library": state.library_name,
            "style": state.template_style,
            "supported_categories": list(self.template_generator.rules.supported_categories),
            "templates": {}
        }
        
        for pattern in state.patterns:
            if pattern.opcode in generated_templates:
                template_manifest["templates"][pattern.opcode] = {
                    "title": pattern.title,
                    "complexity": pattern.complexity,
                    "category": pattern.category,
                    "choice_count": len(pattern.suggested_choices),
                    "template_file": f"{pattern.category.lower()}/{pattern.opcode.lower()}.jinja2",
                    "source_function": pattern.source_function,
                    "teaches_concepts": pattern.teaches_concepts
                }
        
        self.log_success(f"Generated {len(generated_templates)} templates using {library.name} rules")
        
        return {
            "generated_templates": generated_templates,
            "template_manifest": template_manifest,
            "step_count": state.step_count + 1
        }
    
    def generate_lesson_specs_node(self, state: CompilationWorkflowState) -> dict[str, object]:
        """Node: Generate detailed lesson specifications."""
        
        self.log_info(f"Generating specifications for {len(state.lessons)} lessons")
        
        lesson_specifications = []
        
        for lesson in state.lessons:
            spec = {
                "lesson_id": lesson.lesson_id,
                "title": lesson.title,
                "description": lesson.description,
                "estimated_duration": lesson.estimated_duration_minutes,
                "complexity_level": lesson.complexity_level,
                
                # Pattern integration
                "patterns_used": lesson.patterns_used,
                "new_patterns": lesson.new_patterns,
                "reinforced_patterns": lesson.reinforced_patterns,
                
                # Student experience
                "student_choice_points": lesson.student_choice_points,
                "prerequisites": lesson.prerequisites,
                
                # Generated content
                "template_files": [
                    state.template_manifest["templates"][pattern]["template_file"]
                    for pattern in lesson.patterns_used
                    if pattern in state.template_manifest["templates"]
                ],
                
                # Lesson structure
                "intro_dialogue": f"Welcome to {lesson.title}! Today we'll learn {lesson.description.lower()}",
                "success_dialogue": f"Great job! You've mastered {lesson.title}.",
                "hint_dialogue": "If you're stuck, try reviewing the pattern examples.",
                
                # Output files that will be generated
                "output_files": [
                    f"lesson_{lesson.lesson_id}_main.py",
                    f"lesson_{lesson.lesson_id}_student.py"
                ]
            }
            
            lesson_specifications.append(spec)
        
        # Create curriculum structure
        curriculum_structure = {
            "curriculum_id": f"{state.library_name.lower()}_fundamentals",
            "title": f"{state.library_name} Game Development Fundamentals",
            "description": f"Learn game development using {state.library_name}",
            "total_lessons": len(lesson_specifications),
            "estimated_total_hours": sum(spec["estimated_duration"] for spec in lesson_specifications) / 60,
            "lessons": [spec["lesson_id"] for spec in lesson_specifications],
            "progression_type": "linear",
            "prerequisites": [],
            "learning_objectives": [
                "Master fundamental game development concepts",
                "Write clean, readable game code",
                "Understand game development patterns",
                "Build complete playable games"
            ]
        }
        
        self.log_success(f"Generated curriculum structure with {len(lesson_specifications)} lessons")
        
        return {
            "lesson_specifications": lesson_specifications,
            "curriculum_structure": curriculum_structure,
            "step_count": state.step_count + 1
        }
    
    def human_template_review_node(self, state: CompilationWorkflowState) -> dict[str, object]:
        """Node: Human review of generated templates."""
        
        if state.autonomous_mode:
            self.log_info("Autonomous mode: Skipping template review")
            return {
                "template_approval": "approved",
                "step_count": state.step_count + 1
            }
        
        # Prepare review data with template samples
        sample_templates = {}
        for pattern_id, template_content in list(state.generated_templates.items())[:3]:
            sample_templates[pattern_id] = {
                "content_preview": template_content[:500] + "..." if len(template_content) > 500 else template_content,
                "full_content": template_content,
                "metadata": state.template_manifest["templates"][pattern_id]
            }
        
        review_data = {
            "workflow_id": state.workflow_id,
            "library": state.library_name,
            "total_templates": len(state.generated_templates),
            "template_style": state.template_style,
            "sample_templates": sample_templates,
            "curriculum_overview": {
                "total_lessons": state.curriculum_structure["total_lessons"],
                "estimated_hours": state.curriculum_structure["estimated_total_hours"],
                "lesson_titles": [spec["title"] for spec in state.lesson_specifications]
            }
        }
        
        # Interrupt for human review
        human_response = interrupt({
            "type": "template_review",
            "message": f"Review generated templates and curriculum for {state.library_name}",
            "data": review_data,
            "actions": [
                "approve - Accept all templates and curriculum",
                "simplify - Make templates more beginner-friendly",
                "enhance - Add more advanced features",
                "regenerate - Regenerate with different style",
                "reject - Reject and restart compilation"
            ]
        })
        
        # Process human response
        if isinstance(human_response, dict):
            approval = human_response.get("action", "approved")
            feedback = human_response.get("feedback", {})
        else:
            approval = str(human_response) if human_response else "approved"
            feedback = {}
        
        return {
            "template_approval": approval,
            "template_feedback": feedback,
            "step_count": state.step_count + 1
        }
    
    def apply_template_feedback_node(self, state: CompilationWorkflowState) -> dict[str, object]:
        """Node: Apply human feedback to templates."""
        
        if state.template_approval == "reject":
            self.log_info("Templates rejected - will restart compilation")
            return {
                "generated_templates": {},
                "final_templates": {},
                "step_count": state.step_count + 1
            }
        
        elif state.template_approval == "simplify":
            # Regenerate with beginner style using scalable rules
            simplified_templates = self.template_generator.generate_multiple_templates(
                patterns=state.patterns,
                style="beginner"
            )
            
            self.log_info("Regenerated templates with beginner style")
            final_templates = simplified_templates
            
        elif state.template_approval == "enhance":
            # Regenerate with advanced style using scalable rules
            enhanced_templates = self.template_generator.generate_multiple_templates(
                patterns=state.patterns,
                style="advanced"
            )
            
            self.log_info("Regenerated templates with advanced style")
            final_templates = enhanced_templates
            
        elif state.template_approval == "regenerate":
            # Regenerate with different style using scalable rules
            new_style = state.template_feedback.get("new_style", "intermediate")
            regenerated_templates = self.template_generator.generate_multiple_templates(
                patterns=state.patterns,
                style=new_style
            )
            
            self.log_info(f"Regenerated templates with style: {new_style}")
            final_templates = regenerated_templates
            
        else:
            # Approved - use original templates
            final_templates = state.generated_templates
            self.log_info("All templates approved")
        
        return {
            "final_templates": final_templates,
            "final_curriculum": state.curriculum_structure,
            "step_count": state.step_count + 1
        }
    
    def write_output_files_node(self, state: CompilationWorkflowState) -> dict[str, object]:
        """Node: Write templates and curriculum to output directory."""
        
        output_dir = Path(state.output_directory)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        output_paths = []
        
        # Write template files
        templates_dir = output_dir / "templates"
        templates_dir.mkdir(exist_ok=True)
        
        for pattern_id, template_content in state.final_templates.items():
            # Get template metadata
            template_info = state.template_manifest["templates"][pattern_id]
            template_path = templates_dir / template_info["template_file"]
            
            # Create category subdirectory
            template_path.parent.mkdir(parents=True, exist_ok=True)
            
            # Write template file
            template_path.write_text(template_content, encoding="utf-8")
            output_paths.append(str(template_path))
        
        # Write curriculum structure
        curriculum_path = output_dir / "curriculum.json"
        import json
        curriculum_path.write_text(
            json.dumps(state.final_curriculum, indent=2),
            encoding="utf-8"
        )
        output_paths.append(str(curriculum_path))
        
        # Write lesson specifications
        lessons_dir = output_dir / "lessons"
        lessons_dir.mkdir(exist_ok=True)
        
        for spec in state.lesson_specifications:
            spec_path = lessons_dir / f"{spec['lesson_id']}_spec.json"
            spec_path.write_text(
                json.dumps(spec, indent=2),
                encoding="utf-8"
            )
            output_paths.append(str(spec_path))
        
        self.log_success(f"Wrote {len(output_paths)} output files to {output_dir}")
        
        return {
            "output_paths": output_paths,
            "completed_at": datetime.now(),
            "step_count": state.step_count + 1
        }
    
    def should_retry_compilation(self, state: CompilationWorkflowState) -> Literal["retry", "continue"]:
        """Conditional edge: Determine if compilation should be retried."""
        
        if state.template_approval == "reject":
            return "retry"
        else:
            return "continue"
    
    def build_workflow(self) -> StateGraph:
        """Build the complete compilation workflow graph."""
        
        workflow = StateGraph(CompilationWorkflowState)
        
        # Add nodes
        workflow.add_node("generate_templates", self.generate_templates_node)
        workflow.add_node("generate_specs", self.generate_lesson_specs_node)
        workflow.add_node("human_review", self.human_template_review_node)
        workflow.add_node("apply_feedback", self.apply_template_feedback_node)
        workflow.add_node("write_output", self.write_output_files_node)
        
        # Add edges
        workflow.add_edge(START, "generate_templates")
        workflow.add_edge("generate_templates", "generate_specs")
        workflow.add_edge("generate_specs", "human_review")
        workflow.add_edge("human_review", "apply_feedback")
        
        # Conditional edge for retry logic
        workflow.add_conditional_edges(
            "apply_feedback",
            self.should_retry_compilation,
            {
                "retry": "generate_templates",  # Loop back to start
                "continue": "write_output"      # Proceed to write files
            }
        )
        
        workflow.add_edge("write_output", END)
        
        return workflow
    
    def compile_workflow(self, checkpointer, durability: Literal["exit", "async", "sync"] = "async") -> StateGraph:
        """Compile workflow with checkpointer for durable execution."""
        
        workflow = self.build_workflow()
        
        # Compile with durable execution
        compiled = workflow.compile(
            checkpointer=checkpointer,
            durability=durability
        )
        
        self.log_info("Compilation workflow compiled with durable execution")
        return compiled


def create_compilation_workflow(checkpointer) -> StateGraph:
    """Factory function to create a compiled compilation workflow."""
    workflow = CompilationWorkflow()
    return workflow.compile_workflow(checkpointer)
