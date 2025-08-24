"""
LangGraph workflow for API analysis and pattern generation.

This module implements a durable, resumable workflow for analyzing game development
APIs and generating educational pattern schemas with human-in-the-loop review.
"""

from typing import Literal, Any
from datetime import datetime

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt, Command
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.models import (
    APIUsageAnalysis, SchemaAnalysisRequest, SchemaAnalysisResponse,
    PatternSuggestion, LessonProgressionSuggestion
)
from professor_pixel.schemas.ai.analysis import SchemaAIAnalyzer


class AnalysisWorkflowState(BaseModel):
    """State schema for the API analysis workflow."""
    
    # Input data
    library_name: str = Field(description="Game library being analyzed")
    api_functions: list[dict[str, Any]] = Field(description="API function data")
    usage_analysis: APIUsageAnalysis = Field(description="LibCST usage analysis")
    
    # Analysis configuration
    target_complexity_levels: list[int] = Field(description="Target complexity levels")
    max_patterns: int = Field(description="Maximum patterns to generate")
    lesson_count_target: int = Field(description="Target lesson count")
    autonomous_mode: bool = Field(description="Skip human review")
    
    # Intermediate results
    analysis_request: SchemaAnalysisRequest | None = Field(default=None, description="Analysis request")
    ai_response: SchemaAnalysisResponse | None = Field(default=None, description="AI analysis response")
    
    # Human review
    human_approval: str | None = Field(default=None, description="Human approval status")
    human_feedback: dict[str, str] | None = Field(default=None, description="Human feedback")
    
    # Final output
    final_patterns: list[PatternSuggestion] = Field(default_factory=list, description="Final approved patterns")
    final_lessons: list[LessonProgressionSuggestion] = Field(default_factory=list, description="Final lesson progression")
    
    # Metadata
    workflow_id: str = Field(description="Workflow identifier")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Number of completed steps")


class AnalysisWorkflow(BaseComponent):
    """LangGraph workflow for API analysis with durable execution."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.ai_analyzer = SchemaAIAnalyzer()
    
    def create_analysis_request_node(self, state: AnalysisWorkflowState) -> dict[str, object]:
        """Node: Create structured analysis request from input data."""
        
        self.log_info(f"Creating analysis request for {state.library_name}")
        
        # Create analysis request
        request = SchemaAnalysisRequest(
            library_name=state.library_name,
            api_functions=state.api_functions,
            usage_analysis=state.usage_analysis,
            target_complexity_levels=state.target_complexity_levels,
            max_patterns=state.max_patterns,
            lesson_count_target=state.lesson_count_target
        )
        
        return {
            "analysis_request": request,
            "step_count": state.step_count + 1
        }
    
    def run_ai_analysis_node(self, state: AnalysisWorkflowState) -> dict[str, object]:
        """Node: Execute AI analysis with caching."""
        
        self.log_info("Running AI analysis...")
        
        # Run analysis (with caching)
        response = self.ai_analyzer.analyze_api_for_schemas(state.analysis_request)
        
        self.log_success(f"AI analysis complete: {len(response.pattern_suggestions)} patterns generated")
        
        return {
            "ai_response": response,
            "step_count": state.step_count + 1
        }
    
    def human_review_node(self, state: AnalysisWorkflowState) -> dict[str, object]:
        """Node: Human review with interrupt for non-autonomous mode."""
        
        if state.autonomous_mode:
            self.log_info("Autonomous mode: Skipping human review")
            return {
                "human_approval": "approved",
                "step_count": state.step_count + 1
            }
        
        # Prepare structured review data using proper models
        response = state.ai_response
        
        # Create structured pattern summaries
        sample_patterns = []
        for p in response.pattern_suggestions[:10]:
            pattern_summary = {
                "opcode": p.opcode,
                "title": p.title,
                "complexity": p.complexity,
                "category": p.category,
                "choice_count": len(p.suggested_choices),
                "source_function": p.source_function,
                "teaches_concepts": p.teaches_concepts[:3]  # First 3 concepts
            }
            sample_patterns.append(pattern_summary)
        
        # Create structured lesson summaries
        lesson_overview = []
        for l in response.lesson_progression:
            lesson_summary = {
                "id": l.lesson_id,
                "title": l.title,
                "duration": l.estimated_duration_minutes,
                "patterns": len(l.patterns_used),
                "choices": l.student_choice_points,
                "complexity": l.complexity_level,
                "new_patterns_count": len(l.new_patterns)
            }
            lesson_overview.append(lesson_summary)
        
        review_data = {
            "workflow_id": state.workflow_id,
            "library": state.library_name,
            "patterns_generated": len(response.pattern_suggestions),
            "lessons_designed": len(response.lesson_progression),
            "complexity_distribution": response.complexity_distribution,
            "pattern_coverage": response.pattern_coverage_percent,
            "educational_safety": response.educational_safety_score,
            "sample_patterns": sample_patterns,
            "lesson_overview": lesson_overview,
            "prerequisite_chains": response.prerequisite_chains[:3]  # First 3 dependency chains
        }
        
        # Interrupt for human review
        human_response = interrupt({
            "type": "analysis_review",
            "message": f"Review AI analysis results for {state.library_name}",
            "data": review_data,
            "actions": [
                "approve - Accept all analysis results",
                "filter_complexity - Reduce complexity levels",
                "filter_count - Reduce pattern count", 
                "reject - Reject and restart analysis"
            ]
        })
        
        # Process human response with proper type handling
        if isinstance(human_response, dict):
            approval = str(human_response.get("action", "approved"))
            feedback = {k: str(v) for k, v in human_response.get("feedback", {}).items()}
        else:
            approval = str(human_response) if human_response else "approved"
            feedback = {}
        
        return {
            "human_approval": approval,
            "human_feedback": feedback,
            "step_count": state.step_count + 1
        }
    
    def apply_human_feedback_node(self, state: AnalysisWorkflowState) -> dict[str, object]:
        """Node: Apply human feedback to filter/modify results."""
        
        response = state.ai_response
        
        if state.human_approval == "reject":
            self.log_info("Human rejected analysis - workflow will restart")
            # Clear results for restart
            return {
                "ai_response": None,
                "final_patterns": [],
                "final_lessons": [],
                "step_count": state.step_count + 1
            }
        
        elif state.human_approval == "filter_complexity":
            # Filter patterns by complexity
            max_complexity = state.human_feedback.get("max_complexity", 3)
            filtered_patterns = [
                p for p in response.pattern_suggestions 
                if p.complexity <= max_complexity
            ]
            
            # Filter lessons that use filtered patterns
            pattern_opcodes = {p.opcode for p in filtered_patterns}
            filtered_lessons = [
                l for l in response.lesson_progression
                if any(p in pattern_opcodes for p in l.patterns_used)
            ]
            
            self.log_info(f"Filtered to {len(filtered_patterns)} patterns (complexity <= {max_complexity})")
            
        elif state.human_approval == "filter_count":
            # Limit pattern count
            max_count = state.human_feedback.get("max_patterns", 25)
            filtered_patterns = response.pattern_suggestions[:max_count]
            filtered_lessons = response.lesson_progression  # Keep all lessons
            
            self.log_info(f"Limited to {len(filtered_patterns)} patterns")
            
        else:
            # Approved - use all patterns
            filtered_patterns = response.pattern_suggestions
            filtered_lessons = response.lesson_progression
            
            self.log_info("All patterns approved")
        
        return {
            "final_patterns": filtered_patterns,
            "final_lessons": filtered_lessons,
            "step_count": state.step_count + 1
        }
    
    def finalize_analysis_node(self, state: AnalysisWorkflowState) -> dict[str, object]:
        """Node: Finalize analysis and mark completion."""
        
        self.log_success(f"Analysis workflow complete: {len(state.final_patterns)} patterns, {len(state.final_lessons)} lessons")
        
        return {
            "completed_at": datetime.now(),
            "step_count": state.step_count + 1
        }
    
    def should_retry_analysis(self, state: AnalysisWorkflowState) -> Literal["retry", "continue"]:
        """Conditional edge: Determine if analysis should be retried."""
        
        if state.human_approval == "reject":
            return "retry"
        else:
            return "continue"
    
    def build_workflow(self) -> StateGraph:
        """Build the complete analysis workflow graph."""
        
        workflow = StateGraph(AnalysisWorkflowState)
        
        # Add nodes
        workflow.add_node("create_request", self.create_analysis_request_node)
        workflow.add_node("run_analysis", self.run_ai_analysis_node)
        workflow.add_node("human_review", self.human_review_node)
        workflow.add_node("apply_feedback", self.apply_human_feedback_node)
        workflow.add_node("finalize", self.finalize_analysis_node)
        
        # Add edges
        workflow.add_edge(START, "create_request")
        workflow.add_edge("create_request", "run_analysis")
        workflow.add_edge("run_analysis", "human_review")
        workflow.add_edge("human_review", "apply_feedback")
        
        # Conditional edge for retry logic
        workflow.add_conditional_edges(
            "apply_feedback",
            self.should_retry_analysis,
            {
                "retry": "create_request",  # Loop back to start
                "continue": "finalize"      # Proceed to end
            }
        )
        
        workflow.add_edge("finalize", END)
        
        return workflow
    
    def compile_workflow(self, checkpointer, durability: Literal["exit", "async", "sync"] = "async") -> StateGraph:
        """Compile workflow with checkpointer for durable execution."""
        
        workflow = self.build_workflow()
        
        # Compile with durable execution
        compiled = workflow.compile(
            checkpointer=checkpointer,
            durability=durability
        )
        
        self.log_info("Analysis workflow compiled with durable execution")
        return compiled


def create_analysis_workflow(checkpointer) -> StateGraph:
    """Factory function to create a compiled analysis workflow."""
    workflow = AnalysisWorkflow()
    return workflow.compile_workflow(checkpointer)
