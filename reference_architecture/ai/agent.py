"""
Main curriculum generation agent that coordinates analysis and compilation workflows.

This module implements a LangGraph agent that orchestrates API analysis and lesson
compilation as subgraphs with durable execution and human-in-the-loop capabilities.
"""

import uuid
from datetime import datetime
from typing import Literal
from pathlib import Path

from langgraph.graph import StateGraph, START, END
try:
    from langgraph.checkpoint.sqlite import SqliteSaver
except ImportError:
    # Handle case where checkpoint module isn't available during build
    SqliteSaver = None
from langgraph.types import interrupt, Command
try:
    from langchain.cache import SQLAlchemyCache, set_llm_cache
except ImportError:
    # Handle case where cache module isn't available during build
    SQLAlchemyCache = None
    set_llm_cache = None
from sqlalchemy import create_engine
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.models import (
    APIUsageAnalysis, CurriculumGenerationRequest, CurriculumGenerationResult,
    AnalysisWorkflowRequest, CompilationWorkflowRequest, PatternSuggestion,
    LessonProgressionSuggestion
)
from professor_pixel.settings import get_settings
from professor_pixel.schemas.ai.database_workflows.analysis_workflow import create_analysis_workflow
from professor_pixel.schemas.ai.database_workflows.compilation_workflow import create_compilation_workflow


class CurriculumAgentState(BaseModel):
    """State schema for the main curriculum generation agent."""
    
    # Input configuration
    library_name: str = Field(description="Game library being analyzed")
    api_functions: list[dict[str, object]] = Field(description="API function data")
    usage_analysis: APIUsageAnalysis = Field(description="LibCST usage analysis")
    
    # Generation settings
    target_complexity_levels: list[int] = Field(description="Target complexity levels")
    max_patterns: int = Field(description="Maximum patterns to generate")
    lesson_count_target: int = Field(description="Target lesson count")
    template_style: str = Field(description="Template complexity style")
    output_directory: str = Field(description="Output directory for files")
    autonomous_mode: bool = Field(description="Skip human review")
    
    # Workflow results
    analysis_results: dict[str, object] | None = Field(default=None, description="Analysis workflow results")
    compilation_results: dict[str, object] | None = Field(default=None, description="Compilation workflow results")
    
    # Final output
    generated_patterns: list[PatternSuggestion] = Field(default_factory=list, description="Generated patterns")
    generated_lessons: list[LessonProgressionSuggestion] = Field(default_factory=list, description="Generated lessons")
    generated_templates: dict[str, str] = Field(default_factory=dict, description="Generated templates")
    curriculum_structure: dict[str, object] = Field(default_factory=dict, description="Curriculum structure")
    output_paths: list[str] = Field(default_factory=list, description="Generated file paths")
    
    # Metadata
    agent_id: str = Field(description="Agent instance identifier")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    total_steps: int = Field(default=0, description="Number of completed steps")
    
    # Status tracking
    current_phase: str = Field(default="analysis", description="Current workflow phase")
    success: bool = Field(default=False, description="Whether generation was successful")


class CurriculumAgent(BaseComponent):
    """
    Main agent for curriculum generation using LangGraph subgraphs.
    
    This agent coordinates:
    1. Analysis workflow (API → patterns → lessons)
    2. Compilation workflow (patterns → templates → curriculum)
    
    With durable execution and human-in-the-loop at key decision points.
    """
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        
        # Setup LangChain cache and LangGraph checkpointer
        self._setup_persistence()
        
        # Create checkpointer for subgraph workflows
        self.checkpointer = self.get_checkpointer("curriculum_workflows")
        
        # Create subgraph workflows (handle None checkpointer during build)
        try:
            self.analysis_workflow = create_analysis_workflow(self.checkpointer)
            self.compilation_workflow = create_compilation_workflow(self.checkpointer)
        except Exception as e:
            self.log_warning(f"Workflows not available during build: {e}")
            self.analysis_workflow = None
            self.compilation_workflow = None
        
        self.log_info("Curriculum agent initialized with analysis and compilation subgraphs")
    
    def _setup_persistence(self) -> None:
        """Setup LangChain cache and LangGraph checkpointer."""
        
        self.settings = get_settings()
        self.settings.paths.ensure_directories()
        
        # Setup LangChain SQLAlchemy cache
        cache_db_path = self.settings.paths.state_dir / "ai_cache.db"
        cache_engine = create_engine(f"sqlite:///{cache_db_path}")
        
        if SQLAlchemyCache is not None and set_llm_cache is not None:
            set_llm_cache(SQLAlchemyCache(cache_engine))
        else:
            self.log_warning("LangChain cache not available during build - LLM responses will not be cached")
        
        # Setup LangGraph SQLite checkpointer
        self.checkpoint_db_path = self.settings.paths.state_dir / "ai_checkpoints.db"
        
        self.log_info(f"AI persistence configured: cache={cache_db_path}, checkpoints={self.checkpoint_db_path}")
    
    def get_checkpointer(self, workflow_type: str) -> "SqliteSaver | None":
        """Get LangGraph SQLite checkpointer."""
        
        if SqliteSaver is None:
            self.log_warning("SqliteSaver not available during build - workflows will run without checkpointing")
            return None
        
        # Create connection string with workflow-specific namespace
        connection_string = f"sqlite:///{self.checkpoint_db_path}"
        return SqliteSaver.from_conn_string(connection_string)
    
    def create_workflow_session(self, workflow_type: str, input_data: dict[str, object]) -> tuple[str, dict[str, object]]:
        """Create workflow session with unique thread ID."""
        
        thread_id = f"{workflow_type}_{uuid.uuid4().hex[:16]}"
        
        config = {
            "configurable": {
                "thread_id": thread_id,
                "workflow_type": workflow_type
            }
        }
        
        self.log_info(f"Created workflow session: {thread_id}")
        return thread_id, config
    
    def get_cache_stats(self) -> dict[str, object]:
        """Get LangChain cache statistics."""
        
        import sqlite3
        
        cache_db_path = self.settings.paths.state_dir / "ai_cache.db"
        
        # LangChain creates its own cache tables, query those
        with sqlite3.connect(cache_db_path) as conn:
            cursor = conn.cursor()
            
            # Check if LangChain cache tables exist
            cursor.execute("""
                SELECT name FROM sqlite_master 
                WHERE type='table' AND name LIKE '%cache%'
            """)
            tables = [row[0] for row in cursor.fetchall()]
            
            if not tables:
                return {
                    "cache_enabled": True,
                    "cache_entries": 0,
                    "cache_tables": [],
                    "cache_db_size_mb": 0
                }
            
            # Get total cache entries (LangChain table structure)
            total_entries = 0
            for table in tables:
                cursor.execute(f"SELECT COUNT(*) FROM {table}")
                count = cursor.fetchone()[0]
                total_entries += count
            
            return {
                "cache_enabled": True,
                "cache_entries": total_entries,
                "cache_tables": tables,
                "cache_db_size_mb": cache_db_path.stat().st_size / (1024 * 1024) if cache_db_path.exists() else 0
            }
    
    def cleanup_old_data(self, days_old: int = 30) -> dict[str, int]:
        """Clean up old checkpoint data."""
        
        # LangGraph manages its own checkpoint cleanup
        # This method exists for API compatibility
        self.log_info("LangGraph manages checkpoint cleanup automatically")
        return {"checkpoints": 0}
    
    def run_analysis_subgraph_node(self, state: CurriculumAgentState) -> dict[str, object]:
        """Node: Execute analysis workflow as subgraph."""
        
        self.log_info(f"Starting analysis subgraph for {state.library_name}")
        
        # Create analysis workflow request
        analysis_request = AnalysisWorkflowRequest(
            library_name=state.library_name,
            api_functions=state.api_functions,
            usage_analysis=state.usage_analysis,
            target_complexity_levels=state.target_complexity_levels,
            max_patterns=state.max_patterns,
            lesson_count_target=state.lesson_count_target,
            autonomous_mode=state.autonomous_mode
        )
        
        # Create session for analysis subgraph
        thread_id, config = self.create_workflow_session(
            workflow_type="analysis_subgraph",
            input_data=analysis_request.model_dump()
        )
        
        # Execute analysis subgraph - LangChain supports Pydantic models directly
        analysis_result = self.analysis_workflow.invoke(analysis_request.model_dump(), config)
        
        self.log_success(f"Analysis subgraph completed: {len(analysis_result['final_patterns'])} patterns")
        
        return {
            "analysis_results": analysis_result,
            "generated_patterns": analysis_result["final_patterns"],
            "generated_lessons": analysis_result["final_lessons"],
            "current_phase": "compilation",
            "total_steps": state.total_steps + analysis_result["step_count"]
        }
    
    def run_compilation_subgraph_node(self, state: CurriculumAgentState) -> dict[str, object]:
        """Node: Execute compilation workflow as subgraph."""
        
        self.log_info("Starting compilation subgraph")
        
        # Create compilation workflow request from analysis results
        compilation_request = CompilationWorkflowRequest(
            patterns=state.generated_patterns,
            lessons=state.generated_lessons,
            library_name=state.library_name,
            template_style=state.template_style,
            output_directory=state.output_directory,
            autonomous_mode=state.autonomous_mode
        )
        
        # Create session for compilation subgraph
        thread_id, config = self.create_workflow_session(
            workflow_type="compilation_subgraph", 
            input_data=compilation_request.model_dump()
        )
        
        # Execute compilation subgraph - LangChain supports Pydantic models directly
        compilation_result = self.compilation_workflow.invoke(compilation_request.model_dump(), config)
        
        self.log_success(f"Compilation subgraph completed: {len(compilation_result['output_paths'])} files generated")
        
        return {
            "compilation_results": compilation_result,
            "generated_templates": compilation_result["final_templates"],
            "curriculum_structure": compilation_result["final_curriculum"],
            "output_paths": compilation_result["output_paths"],
            "current_phase": "complete",
            "success": True,
            "completed_at": datetime.now(),
            "total_steps": state.total_steps + compilation_result["step_count"]
        }
    
    def final_summary_node(self, state: CurriculumAgentState) -> dict[str, object]:
        """Node: Provide final summary of curriculum generation."""
        
        if state.success:
            summary = {
                "agent_id": state.agent_id,
                "library": state.library_name,
                "generation_summary": {
                    "patterns_generated": len(state.generated_patterns),
                    "lessons_created": len(state.generated_lessons),
                    "templates_generated": len(state.generated_templates),
                    "files_written": len(state.output_paths),
                    "total_processing_time": (
                        state.completed_at - state.started_at
                    ).total_seconds() if state.completed_at else 0
                },
                "output_location": state.output_directory,
                "curriculum_info": {
                    "title": state.curriculum_structure.get("title", ""),
                    "estimated_hours": state.curriculum_structure.get("estimated_total_hours", 0),
                    "progression_type": state.curriculum_structure.get("progression_type", "linear")
                }
            }
            
            if not state.autonomous_mode:
                # Show success summary to human
                interrupt({
                    "type": "completion_summary",
                    "message": f"Curriculum generation complete for {state.library_name}!",
                    "data": summary,
                    "actions": ["continue"]
                })
            
            self.log_success(f"Curriculum generation complete: {summary['generation_summary']}")
            
        else:
            self.log_error("Curriculum generation failed")
        
        return {
            "total_steps": state.total_steps + 1
        }
    
    def should_continue_to_compilation(self, state: CurriculumAgentState) -> Literal["compilation", "end"]:
        """Conditional edge: Determine if we should proceed to compilation."""
        
        if state.current_phase == "compilation" and state.generated_patterns:
            return "compilation"
        else:
            return "end"
    
    def should_show_summary(self, state: CurriculumAgentState) -> Literal["summary", "end"]:
        """Conditional edge: Determine if we should show final summary."""
        
        if state.current_phase in ["complete", "failed"]:
            return "summary"
        else:
            return "end"
    
    def build_agent_workflow(self) -> StateGraph:
        """Build the main agent workflow with subgraphs."""
        
        workflow = StateGraph(CurriculumAgentState)
        
        # Add nodes
        workflow.add_node("analysis", self.run_analysis_subgraph_node)
        workflow.add_node("compilation", self.run_compilation_subgraph_node)
        workflow.add_node("summary", self.final_summary_node)
        
        # Add edges
        workflow.add_edge(START, "analysis")
        
        # Conditional edges
        workflow.add_conditional_edges(
            "analysis",
            self.should_continue_to_compilation,
            {
                "compilation": "compilation",
                "end": "summary"
            }
        )
        
        workflow.add_conditional_edges(
            "compilation",
            self.should_show_summary,
            {
                "summary": "summary",
                "end": END
            }
        )
        
        workflow.add_edge("summary", END)
        
        return workflow
    
    def compile_agent(self, durability: Literal["exit", "async", "sync"] = "async") -> StateGraph:
        """Compile the main agent with durable execution."""
        
        workflow = self.build_agent_workflow()
        
        # Get checkpointer for main agent
        checkpointer = self.get_checkpointer("curriculum_agent")
        
        # Compile with durable execution (handle None checkpointer during build)
        if checkpointer is not None:
            compiled = workflow.compile(
                checkpointer=checkpointer,
                durability=durability
            )
        else:
            # Build time - no checkpointing available
            compiled = workflow.compile()
        
        self.log_info("Main curriculum agent compiled with durable execution")
        return compiled
    
    def create_generation_request(
        self,
        library_name: str,
        api_functions: list[dict[str, object]],
        usage_analysis: APIUsageAnalysis,
        output_directory: str | Path,
        target_complexity_levels: list[int] = None,
        max_patterns: int = 50,
        lesson_count_target: int = 8,
        template_style: str = "intermediate",
        autonomous_mode: bool = False
    ) -> CurriculumGenerationRequest:
        """Create initial request for curriculum generation."""
        
        return CurriculumGenerationRequest(
            library_name=library_name,
            api_functions=api_functions,
            usage_analysis=usage_analysis,
            output_directory=str(output_directory),
            target_complexity_levels=target_complexity_levels or [1, 2, 3],
            max_patterns=max_patterns,
            lesson_count_target=lesson_count_target,
            template_style=template_style,
            autonomous_mode=autonomous_mode
        )
    
    def generate_curriculum(
        self,
        library_name: str,
        api_functions: list[dict[str, object]],
        usage_analysis: APIUsageAnalysis,
        output_directory: str | Path,
        **kwargs
    ) -> CurriculumGenerationResult:
        """
        Main entry point for curriculum generation.
        
        This method orchestrates the entire pipeline:
        1. API analysis → patterns + lessons
        2. Template compilation → Jinja2 templates + curriculum structure  
        3. File output → ready-to-use educational content
        
        Returns structured result with all details.
        """
        
        start_time = datetime.now()
        agent_id = f"curriculum_{uuid.uuid4().hex[:12]}"
        
        # Create agent workflow
        agent = self.compile_agent()
        
        # Create initial request
        request = self.create_generation_request(
            library_name=library_name,
            api_functions=api_functions,
            usage_analysis=usage_analysis,
            output_directory=output_directory,
            **kwargs
        )
        
        # Create session for main agent
        thread_id, config = self.create_workflow_session(
            workflow_type="curriculum_agent",
            input_data=request.model_dump()
        )
        
        self.log_info(f"Starting curriculum generation: {agent_id}")
        
        # Execute agent workflow - convert request to dict for LangGraph
        final_state = agent.invoke(self._request_to_state(request, agent_id), config)
        
        # Convert to structured result
        processing_time = (datetime.now() - start_time).total_seconds()
        
        result = CurriculumGenerationResult(
            agent_id=agent_id,
            library_analyzed=library_name,
            patterns_generated=final_state["generated_patterns"],
            lessons_created=final_state["generated_lessons"],
            templates_generated=final_state["generated_templates"],
            curriculum_structure=final_state["curriculum_structure"],
            output_directory=str(output_directory),
            output_files=final_state["output_paths"],
            total_processing_time_seconds=processing_time,
            patterns_count=len(final_state["generated_patterns"]),
            lessons_count=len(final_state["generated_lessons"]),
            templates_count=len(final_state["generated_templates"]),
            success=final_state["success"]
        )
        
        if result.success:
            self.log_success(f"Curriculum generation successful: {result.patterns_count} patterns, {result.templates_count} templates")
        else:
            self.log_error("Curriculum generation failed")
        
        return result
    
    def _request_to_state(self, request: CurriculumGenerationRequest, agent_id: str) -> dict[str, object]:
        """Convert Pydantic request to workflow state dict."""
        
        return {
            # Input configuration
            "library_name": request.library_name,
            "api_functions": request.api_functions,
            "usage_analysis": request.usage_analysis,
            
            # Generation settings
            "target_complexity_levels": request.target_complexity_levels,
            "max_patterns": request.max_patterns,
            "lesson_count_target": request.lesson_count_target,
            "template_style": request.template_style,
            "output_directory": request.output_directory,
            "autonomous_mode": request.autonomous_mode,
            
            # Initialize empty results
            "analysis_results": None,
            "compilation_results": None,
            "generated_patterns": [],
            "generated_lessons": [],
            "generated_templates": {},
            "curriculum_structure": {},
            "output_paths": [],
            
            # Metadata
            "agent_id": agent_id,
            "started_at": datetime.now(),
            "completed_at": None,
            "total_steps": 0,
            
            # Status
            "current_phase": "analysis",
            "success": False
        }


# Factory function for easy import
def create_curriculum_agent() -> CurriculumAgent:
    """Factory function to create a curriculum generation agent."""
    return CurriculumAgent()
