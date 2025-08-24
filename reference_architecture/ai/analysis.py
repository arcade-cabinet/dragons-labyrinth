"""
Schema-focused AI analysis for API pattern generation.

This module provides AI-powered analysis of game development APIs specifically
for generating educational pattern schemas, separate from the cartridges compiler.
"""

import os
import time

from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.runnables import RunnableConfig

from professor_pixel.base import BaseComponent
from professor_pixel.models import (
    APIUsageAnalysis, PatternSuggestion, LessonProgressionSuggestion,
    SchemaAnalysisRequest, SchemaAnalysisResponse
)



class SchemaAIAnalyzer(BaseComponent):
    """AI analyzer specifically focused on schema generation from API analysis."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        
        # Get API key
        self.api_key = os.getenv("OPENAI_API_KEY")
        if not self.api_key:
            raise ValueError("OPENAI_API_KEY environment variable is required")
        
        # Initialize LLM
        self.llm = ChatOpenAI(
            model="gpt-4o-mini", 
            temperature=0.3,
            api_key=self.api_key
        )
        
        self.log_info("Schema AI analyzer initialized with OpenAI")
    
    def analyze_api_for_schemas(self, request: SchemaAnalysisRequest) -> SchemaAnalysisResponse:
        """
        Analyze API functions and usage patterns to generate educational schemas.
        
        This method combines LibCST usage analysis with AI insights to generate
        appropriate pattern suggestions and lesson progressions.
        LangChain SQLAlchemy cache handles automatic caching.
        """
        
        self.log_info("Running AI analysis for schema generation")
        start_time = time.time()
        
        response = self._ai_assisted_analysis(request)
        
        processing_time = time.time() - start_time
        self.log_success(f"AI analysis complete in {processing_time:.2f}s: {len(response.pattern_suggestions)} patterns generated")
        
        return response
    
    def _ai_assisted_analysis(self, request: SchemaAnalysisRequest) -> SchemaAnalysisResponse:
        """Use AI to generate sophisticated pattern and lesson suggestions."""
        
        # Create analysis prompt
        prompt_template = ChatPromptTemplate.from_template("""
You are an expert game development educator analyzing the {library_name} API to create educational programming patterns for students aged {min_age}-{max_age}.

## API Analysis Data
- Total functions analyzed: {function_count}
- Most frequent function calls: {top_functions}
- Common function sequences: {common_sequences}
- Safe educational functions: {safe_functions}
- Functions to avoid: {unsafe_functions}

## Your Task
Generate educational patterns that:
1. Focus on complexity levels {complexity_levels}
2. Create {max_patterns} or fewer patterns
3. Support {lesson_count} progressive lessons
4. Use real API usage patterns from the analysis

## Requirements
- Each pattern must have a clear educational purpose
- Suggest realistic student choice points based on common parameter usage
- Ensure logical prerequisite relationships
- Focus on practical game development concepts

## Output Format
Return a JSON object with this exact structure:

{{
    "library_analyzed": "{library_name}",
    "total_functions_analyzed": {function_count},
    "patterns_generated": <number>,
    "pattern_suggestions": [
        {{
            "opcode": "PATTERN_OPCODE",
            "title": "Human readable title",
            "description": "Clear educational description", 
            "complexity": <1-5>,
            "category": "sprites|visual|audio|input|collision|motion",
            "suggested_choices": [
                {{
                    "choice_id": "parameter_name",
                    "prompt": "Student-facing question",
                    "options": ["option1", "option2", "option3"],
                    "default": "option1"
                }}
            ],
            "source_function": "arcade.function_name",
            "common_parameters": ["param1", "param2"],
            "typical_values": {{"param1": ["value1", "value2"]}},
            "teaches_concepts": ["concept1", "concept2"],
            "prerequisites": ["OTHER_PATTERN"],
            "template_file": "category/pattern_name.jinja2"
        }}
    ],
    "lesson_progression": [
        {{
            "lesson_id": "y1_l1_intro",
            "title": "Lesson Title",
            "description": "Lesson description",
            "patterns_used": ["PATTERN1", "PATTERN2"],
            "new_patterns": ["PATTERN1"],
            "reinforced_patterns": [],
            "estimated_duration_minutes": <10-90>,
            "student_choice_points": <number>,
            "complexity_level": <1-5>,
            "prerequisites": []
        }}
    ],
    "complexity_distribution": {{
        "level_1": <count>,
        "level_2": <count>,
        "level_3": <count>
    }},
    "concept_coverage": {{
        "sprites": <count>,
        "collision": <count>,
        "input": <count>
    }},
    "prerequisite_chains": [["BASIC_SPRITE", "MOVE_SPRITE", "COLLISION_SPRITE"]],
    "pattern_coverage_percent": <0-100>,
    "educational_safety_score": <0.0-1.0>
}}

Focus on practical, buildable patterns that students can immediately use to create games.
""")
        
        # Prepare prompt variables
        usage = request.usage_analysis
        top_functions = list(sorted(usage.function_calls.items(), key=lambda x: x[1], reverse=True))[:10]
        
        # Invoke AI with structured output
        structured_llm = self.llm.with_structured_output(
            SchemaAnalysisResponse,
            method="json_mode"
        )
        
        response = structured_llm.invoke(
            prompt_template.format(
                library_name=request.library_name,
                function_count=len(request.api_functions),
                top_functions=[f[0] for f in top_functions],
                common_sequences=usage.common_sequences[:5],
                safe_functions=usage.safe_functions[:10],
                unsafe_functions=usage.unsafe_functions[:5],
                complexity_levels=request.target_complexity_levels,
                max_patterns=request.max_patterns,
                lesson_count=request.lesson_count_target,
                min_age=request.student_age_range[0],
                max_age=request.student_age_range[1]
            )
        )
        
        self.log_success(f"AI analysis complete: {len(response.pattern_suggestions)} patterns generated")
        return response
