"""
AI workflow for generating template rules from library scanning results.

This workflow takes the existing library rules and libcst usage analysis to generate
appropriate template generation patterns using AI, with human review.
"""

from typing import Literal, Any
from datetime import datetime

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.models import (
    APIUsageAnalysis, PatternSuggestion,
    TemplateRulesGenerationRequest, TemplateRulesGenerationResponse,
    CategoryTemplateRules
)
from professor_pixel.schemas.ai.analysis import SchemaAIAnalyzer
from professor_pixel.schemas.library_rules import LibraryScanRules
from professor_pixel.types import GameLibrary
from professor_pixel.database.models import TemplateRules as TemplateRulesDB


class TemplateRulesWorkflowState(BaseModel):
    """State for template rules generation workflow."""
    
    # Input
    library: GameLibrary = Field(description="Target library")
    library_rules: Any = Field(description="Library scan rules")  # LibraryScanRules type from schemas package
    usage_analysis: APIUsageAnalysis = Field(description="Usage analysis results")
    api_patterns: list[PatternSuggestion] = Field(description="API patterns")
    template_styles: list[str] = Field(description="Target styles")
    autonomous_mode: bool = Field(description="Skip human review")
    
    # Generated rules
    ai_generated_rules: TemplateRulesGenerationResponse | None = Field(default=None, description="AI-generated template rules")
    
    # Human review
    human_approval: str | None = Field(default=None, description="Human approval status")
    human_feedback: dict[str, str] | None = Field(default=None, description="Human feedback")
    
    # Final output
    final_template_rules: TemplateRulesGenerationResponse | None = Field(default=None, description="Final approved rules")
    
    # Metadata
    workflow_id: str = Field(description="Workflow identifier")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Steps completed")


class TemplateRulesWorkflow(BaseComponent):
    """Workflow for generating template rules from library analysis."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.ai_analyzer = SchemaAIAnalyzer()
    
    def analyze_library_patterns_node(self, state: TemplateRulesWorkflowState) -> dict[str, object]:
        """Node: Use AI to analyze library rules and usage patterns for template generation."""
        
        self.log_info(f"Analyzing {state.library.name} patterns for template rule generation")
        
        # Create comprehensive request model
        analysis_request = TemplateRulesGenerationRequest(
            library=state.library,
            library_rules=state.library_rules,
            usage_analysis=state.usage_analysis,
            api_patterns=state.api_patterns,
            template_styles=state.template_styles,
            autonomous_mode=state.autonomous_mode,
            workflow_id=state.workflow_id
        )
        
        # Create analysis prompt using structured data
        analysis_prompt = f"""
        You are an expert in educational template generation for game development libraries.
        
        Generate comprehensive template rules for {state.library.name} based on:
        
        ## Library Analysis Data
        - Core modules: {state.library_rules.core_modules}
        - Educational categories: {list(state.library_rules.category_mappings.keys())[:10]}
        - Complexity indicators: {dict(list(state.library_rules.complexity_indicators.items())[:5])}
        
        ## Real Usage Patterns
        - Top functions: {dict(list(state.usage_analysis.function_calls.items())[:10])}
        - Function sequences: {state.usage_analysis.common_sequences[:5]}
        - Educational-safe functions: {state.usage_analysis.safe_functions[:10]}
        - Parameter usage patterns: {dict(list(state.usage_analysis.parameter_patterns.items())[:5])}
        
        ## Pattern Coverage
        - Total patterns: {len(state.api_patterns)}
        - Categories: {list(set(p.category for p in state.api_patterns))}
        - Complexity distribution: {list(set(p.complexity for p in state.api_patterns))}
        
        Generate scalable template generation rules for all styles: {state.template_styles}
        """
        
        # Use structured output to generate template rules
        ai_response = self.ai_analyzer.llm.with_structured_output(
            TemplateRulesGenerationResponse,
            method="json_mode"
        ).invoke(analysis_prompt)
        
        self.log_success(f"AI generated template rules for {len(ai_response.categories_supported)} categories")
        
        return {
            "ai_generated_rules": ai_response,
            "step_count": state.step_count + 1
        }
    
    def human_review_template_rules_node(self, state: TemplateRulesWorkflowState) -> dict[str, object]:
        """Node: Human review of AI-generated template rules."""
        
        if state.autonomous_mode:
            self.log_info("Autonomous mode: Skipping template rules review")
            return {
                "human_approval": "approved",
                "step_count": state.step_count + 1
            }
        
        rules = state.ai_generated_rules
        
        # Prepare structured review data using proper models
        rules_summary = {
            "categories_supported": rules.categories_supported,
            "patterns_covered": rules.patterns_covered,
            "styles_available": [style.style_name for style in rules.style_configurations],
            "total_category_templates": len(rules.category_templates)
        }
        
        # Get sample templates from category rules
        sample_templates = {}
        for category_rule in rules.category_templates[:3]:
            sample_templates[category_rule.category] = category_rule.intermediate_templates[:5]
        
        review_data = {
            "library": state.library,
            "workflow_id": state.workflow_id,
            "rules_summary": rules_summary,
            "sample_templates": sample_templates,
            "complexity_progression": rules.complexity_progression,
            "customization_points": dict(list(rules.student_customization_points.items())[:3])
        }
        
        # Interrupt for human review
        human_response = interrupt({
            "type": "template_rules_review",
            "message": f"Review AI-generated template rules for {state.library.name}",
            "data": review_data,
            "actions": [
                "approve - Accept all template rules",
                "adjust_complexity - Modify complexity progression",
                "add_categories - Add support for more categories",
                "simplify_templates - Make templates more beginner-friendly",
                "reject - Reject and regenerate rules"
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
            "human_approval": approval,
            "human_feedback": feedback,
            "step_count": state.step_count + 1
        }
    
    def apply_template_rules_feedback_node(self, state: TemplateRulesWorkflowState) -> dict[str, object]:
        """Node: Apply human feedback to template rules."""
        
        rules = state.ai_generated_rules
        
        if state.human_approval == "reject":
            self.log_info("Template rules rejected - will regenerate")
            return {
                "ai_generated_rules": None,
                "final_template_rules": None,
                "step_count": state.step_count + 1
            }
        
        elif state.human_approval == "adjust_complexity":
            # Modify complexity progression based on feedback
            adjusted_progression = state.human_feedback.get("complexity_progression", rules.complexity_progression)
            rules.complexity_progression = adjusted_progression
            self.log_info("Adjusted complexity progression based on feedback")
        
        elif state.human_approval == "add_categories":
            # Add support for additional categories using proper models
            new_categories = state.human_feedback.get("additional_categories", [])
            for category in new_categories:
                if category not in rules.categories_supported:
                    rules.categories_supported.append(category)
                    # Create new category template rule
                    new_category_rule = CategoryTemplateRules(
                        category=category,
                        beginner_templates=[f"# Basic {category} template"],
                        intermediate_templates=[f"# Intermediate {category} template"],
                        advanced_templates=[f"# Advanced {category} template"]
                    )
                    rules.category_templates.append(new_category_rule)
            self.log_info(f"Added support for categories: {new_categories}")
        
        elif state.human_approval == "simplify_templates":
            # Enhance beginner templates based on feedback using proper models
            for category_rule in rules.category_templates:
                # Add more comments and explanations to beginner templates
                enhanced_beginner = []
                for line in category_rule.beginner_templates:
                    enhanced_beginner.append(line)
                    if line.startswith("#"):
                        enhanced_beginner.append("# (This line creates...)")
                category_rule.beginner_templates = enhanced_beginner
            self.log_info("Enhanced beginner templates with more guidance")
        
        # Always use the (possibly modified) rules
        final_rules = rules
        
        return {
            "final_template_rules": final_rules,
            "step_count": state.step_count + 1
        }
    
    def finalize_template_rules_node(self, state: TemplateRulesWorkflowState) -> dict[str, object]:
        """Node: Finalize and save template rules."""
        
        rules = state.final_template_rules
        
        self.log_success(f"Template rules generation complete for {state.library.name}")
        self.log_info(f"Generated rules for {len(rules.categories_supported)} categories")
        self.log_info(f"Covers {rules.patterns_covered} patterns")
        
        return {
            "completed_at": datetime.now(),
            "step_count": state.step_count + 1
        }
    
    def should_retry_generation(self, state: TemplateRulesWorkflowState) -> Literal["retry", "continue"]:
        """Conditional edge: Determine if generation should be retried."""
        
        if state.human_approval == "reject":
            return "retry"
        else:
            return "continue"
    
    def build_workflow(self) -> StateGraph:
        """Build the template rules generation workflow."""
        
        workflow = StateGraph(TemplateRulesWorkflowState)
        
        # Add nodes
        workflow.add_node("analyze_patterns", self.analyze_library_patterns_node)
        workflow.add_node("human_review", self.human_review_template_rules_node)
        workflow.add_node("apply_feedback", self.apply_template_rules_feedback_node)
        workflow.add_node("finalize", self.finalize_template_rules_node)
        
        # Add edges
        workflow.add_edge(START, "analyze_patterns")
        workflow.add_edge("analyze_patterns", "human_review")
        workflow.add_edge("human_review", "apply_feedback")
        
        # Conditional edge for retry logic
        workflow.add_conditional_edges(
            "apply_feedback",
            self.should_retry_generation,
            {
                "retry": "analyze_patterns",  # Loop back to regenerate
                "continue": "finalize"        # Proceed to finalize
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
        
        self.log_info("Template rules workflow compiled with durable execution")
        return compiled


def create_template_rules_workflow(checkpointer) -> StateGraph:
    """Factory function to create template rules generation workflow."""
    workflow = TemplateRulesWorkflow()
    return workflow.compile_workflow(checkpointer)


class LibraryTemplateRulesGenerator(BaseComponent):
    """Generator that creates template rules from library scanning results."""
    
    def __init__(self, library: GameLibrary, **kwargs):
        super().__init__(**kwargs)
        self.library = library
        self.ai_analyzer = SchemaAIAnalyzer()
    
    def generate_template_rules_from_scan(
        self,
        library_rules: LibraryScanRules,
        usage_analysis: APIUsageAnalysis,
        api_patterns: list[PatternSuggestion],
        autonomous_mode: bool = False
    ) -> TemplateRulesGenerationResponse:
        """
        Generate template rules from library scanning results.
        
        This method analyzes:
        1. Library rules (categories, complexity indicators, etc.)
        2. Real usage patterns from libcst analysis
        3. Generated API patterns with student choices
        
        And produces comprehensive template generation rules.
        """
        
        self.log_info(f"Generating template rules for {self.library.name}")
        
        # Create comprehensive analysis prompt
        # Create structured request for comprehensive analysis
        request = TemplateRulesGenerationRequest(
            library=self.library.name,
            library_rules=library_rules,
            usage_analysis=usage_analysis,
            api_patterns=api_patterns,
            autonomous_mode=autonomous_mode,
            workflow_id=f"template_rules_{self.library.name}"
        )
        
        prompt = f"""
        Create comprehensive template generation rules for {self.library.name} based on:
        
        ## Library Scan Rules Analysis
        - Core modules: {library_rules.core_modules}
        - Educational categories: {list(library_rules.category_mappings.keys())[:10]}
        - Complexity indicators: {dict(list(library_rules.complexity_indicators.items())[:5])}
        
        ## Real Usage Patterns (from LibCST)
        - Most frequent functions: {dict(list(usage_analysis.function_calls.items())[:10])}
        - Common sequences: {usage_analysis.common_sequences[:5]}
        - Safe functions: {usage_analysis.safe_functions[:10]}
        - Parameter usage: {dict(list(usage_analysis.parameter_patterns.items())[:5])}
        
        ## Generated API Patterns
        - Total patterns: {len(api_patterns)}
        - Sample patterns: {[p.opcode for p in api_patterns[:10]]}
        - Categories: {list(set(p.category for p in api_patterns))}
        - Common parameters: {set().union(*[p.common_parameters for p in api_patterns[:10] if p.common_parameters])}
        
        Generate structured template rules with proper Pydantic models:
        1. CategoryTemplateRules for each category with beginner/intermediate/advanced templates
        2. StyleConfiguration for each complexity level
        3. FunctionMapping for pattern-to-function relationships
        4. Educational progression and customization points
        
        Return structured TemplateRulesGenerationResponse.
        """
        
        # Generate template rules using structured output
        response = self.ai_analyzer.llm.with_structured_output(
            TemplateRulesGenerationResponse,
            method="json_mode"
        ).invoke(prompt)
        
        self.log_success(f"Generated template rules covering {response.patterns_covered} patterns")
        
        return response
    
    def create_scalable_template_generator(
        self, 
        template_rules: TemplateRulesGenerationResponse
    ) -> "ScalableTemplateGenerator":
        """Create a scalable template generator from AI-generated rules."""
        
        # This would create a dynamic template generator based on the AI rules
        # For now, return a placeholder that indicates the rules are ready
        
        self.log_info(f"Template rules ready for {self.library.name} - {len(template_rules.categories_supported)} categories supported")
        self.log_info(f"Generated {len(template_rules.category_templates)} category template rules")
        self.log_info(f"Created {len(template_rules.style_configurations)} style configurations")
        
        # Return the template rules for integration with the compilation workflow
        return template_rules
