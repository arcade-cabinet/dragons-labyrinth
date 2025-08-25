"""
Main HBF Analysis Agent with LangGraph workflow orchestration.

This agent coordinates slice-by-slice HBF analysis and transformation using
sophisticated workflow patterns adapted from professor-pixels architecture.
"""

from __future__ import annotations

import os
import uuid
import json
from datetime import datetime
from pathlib import Path
from typing import Any, Literal

from langgraph.graph import StateGraph, START, END
from langgraph.checkpoint.sqlite import SqliteSaver
from langgraph.types import interrupt, Command
from langchain.cache import SQLAlchemyCache, set_llm_cache
from langchain_openai import ChatOpenAI
from sqlalchemy import create_engine
from pydantic import BaseModel, Field
import networkx as nx

from dragons_labyrinth.models import (
    HBFConfig, OrchestratorState, HBFSliceAnalysisState, 
    PatternSuggestion, ComponentSpecification, IntegrationMapping,
    SliceAnalysisRequest, SliceAnalysisResult, ValidationResult,
    WorkflowCheckpoint, MemoryBankEntry
)
from dragons_labyrinth.types import (
    SliceType, AnalysisDepth, IntegrationLevel, WorkflowStage, ApprovalStatus,
    ComponentType, GenerationType, EntityType, AnalysisStatus, WorkflowEvent,
    IntegrationPoint, PatternCategory, OutputFormat, ValidationLevel,
    DreadLevel, PhilosophyPath, CompanionStress, CorruptionLevel,
    EntityID, WorkflowID, SliceID, PatternID, ComponentID, ClusterID,
    EntityDict, PatternDict, ComponentDict, ReviewData, StateDict,
    HyperlinkRef, HTMLContent, JSONContent, 
    EntityCollection, PatternCollection, ComponentCollection,
    IntegrationPointList, HyperlinkCollection, ProbabilityTableCollection
)




class HBFAnalysisAgent:
    """
    Main agent for HBF slice-by-slice analysis using LangGraph workflows.
    
    Adapted from professor-pixels architecture for HBF → Rust generation
    with human review checkpoints and durable execution.
    """
    
    def __init__(self, config: HBFConfig):
        self.config = config
        self.setup_persistence()
        self.setup_llm()
        self.setup_memory()
        
    def setup_persistence(self) -> None:
        """Setup LangChain cache and LangGraph checkpointer."""
        
        # Ensure state directory exists
        state_dir = self.config.output_dir / "state"
        state_dir.mkdir(parents=True, exist_ok=True)
        
        # Setup LangChain SQLAlchemy cache
        cache_db_path = state_dir / "llm_cache.db"
        cache_engine = create_engine(f"sqlite:///{cache_db_path}")
        
        if SQLAlchemyCache is not None and set_llm_cache is not None:
            set_llm_cache(SQLAlchemyCache(cache_engine))
        
        # Setup LangGraph SQLite checkpointer
        self.checkpoint_db_path = state_dir / "workflow_checkpoints.db"
        
    def get_checkpointer(self) -> SqliteSaver | None:
        """Get LangGraph SQLite checkpointer for durable execution."""
        
        if SqliteSaver is None:
            return None
        
        connection_string = f"sqlite:///{self.checkpoint_db_path}"
        return SqliteSaver.from_conn_string(connection_string)
    
    def setup_llm(self) -> None:
        """Setup OpenAI LLM for analysis."""
        
        import os
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable is required")
        
        self.llm = ChatOpenAI(
            model="gpt-4o-mini",
            temperature=0.3,
            api_key=api_key
        )
        
    def setup_memory(self) -> None:
        """Setup memory systems for workflow state."""
        
        # NetworkX graph for entity relationships
        self.entity_graph = nx.DiGraph()
        
        # Workflow memory
        self.workflow_memory = {
            "processed_slices": [],
            "entity_relationships": {},
            "horror_integration_patterns": {},
            "bevy_component_registry": {}
        }
    
    def extract_slice_entities_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Extract entities for the specific slice type."""
        
        print(f"🔍 Extracting {state.slice_type} entities from HBF")
        
        # Import here to avoid circular imports
        from dragons_labyrinth.hbf.orchestrator import HBFOrchestrator
        
        # Create orchestrator with existing config
        orchestrator_state = OrchestratorState(config=self.config)
        orchestrator = HBFOrchestrator()
        orchestrator.initialize(orchestrator_state)
        
        # Load and filter entities for this slice
        entities_df = orchestrator.load_entities()
        
        # Filter entities by slice type using existing entity classifier
        slice_entities = self._filter_entities_by_slice(entities_df, state.slice_type)
        
        print(f"✅ Found {len(slice_entities)} {state.slice_type} entities")
        
        return {
            "raw_entities": slice_entities.to_dict('records'),
            "step_count": state.step_count + 1
        }
    
    def analyze_html_content_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Deep analysis of HTML content for slice entities."""
        
        print(f"🔬 Analyzing HTML content for {len(state.raw_entities)} entities")
        
        html_content = {}
        probability_tables = {}
        hyperlink_refs = {}
        
        # Import content processor
        from dragons_labyrinth.hbf.content_processor import ContentProcessor
        processor = ContentProcessor()
        
        for entity in state.raw_entities:
            entity_id = entity['uuid']
            content = entity.get('value', '')
            
            if content:
                # Parse HTML content
                html_content[entity_id] = content
                
                # Extract probability tables (look for dice notation, percentages)
                tables = processor.extract_probability_tables(content)
                if tables:
                    probability_tables[entity_id] = tables
                
                # Extract hyperlink references
                refs = processor.extract_hyperlink_references(content)
                if refs:
                    hyperlink_refs[entity_id] = refs
        
        print(f"✅ Extracted {len(probability_tables)} probability tables")
        print(f"✅ Found {len(hyperlink_refs)} entities with hyperlinks")
        
        return {
            "html_content": html_content,
            "probability_tables": probability_tables,
            "hyperlink_refs": hyperlink_refs,
            "step_count": state.step_count + 1
        }
    
    def discover_patterns_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Discover patterns specific to this slice type."""
        
        print(f"🧩 Discovering patterns in {state.slice_type} slice")
        
        patterns = {}
        integration_points = []
        
        # Analyze patterns based on slice type
        if state.slice_type == "region":
            patterns = self._discover_region_patterns(state)
            integration_points = [
                "Regional dread amplification",
                "Weather corruption effects", 
                "Biome-specific companion stress",
                "Philosophy path resonance"
            ]
        elif state.slice_type == "dungeon":
            patterns = self._discover_dungeon_patterns(state)
            integration_points = [
                "Depth-based dread escalation",
                "Room corruption mechanics",
                "Monster behavior changes",
                "Companion claustrophobia triggers"
            ]
        elif state.slice_type == "settlement":
            patterns = self._discover_settlement_patterns(state)
            integration_points = [
                "Population decay systems",
                "Social breakdown mechanics",
                "Companion civilization comfort",
                "Philosophy alignment with governance"
            ]
        elif state.slice_type == "faction":
            patterns = self._discover_faction_patterns(state)
            integration_points = [
                "Conspiracy revelation mechanics",
                "Trust degradation systems",
                "Companion loyalty conflicts",
                "Philosophy faction alignment"
            ]
        
        print(f"✅ Discovered {len(patterns)} patterns")
        print(f"✅ Identified {len(integration_points)} horror integration points")
        
        return {
            "slice_patterns": patterns,
            "integration_points": integration_points,
            "step_count": state.step_count + 1
        }
    
    def human_review_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Human review of slice analysis with structured interrupt."""
        
        if state.autonomous_mode:
            print("🤖 Autonomous mode: Skipping human review")
            return {
                "human_approval": "approved",
                "step_count": state.step_count + 1
            }
        
        # Prepare structured review data
        review_data = {
            "workflow_id": state.workflow_id,
            "slice_type": state.slice_type,
            "entities_analyzed": len(state.raw_entities),
            "probability_tables_found": len(state.probability_tables),
            "hyperlinks_mapped": len(state.hyperlink_refs),
            "patterns_discovered": len(state.slice_patterns),
            "horror_integration_points": state.integration_points,
            "sample_entities": state.raw_entities[:3],  # First 3 for review
            "sample_patterns": dict(list(state.slice_patterns.items())[:3])
        }
        
        # Interrupt for human review
        human_response = interrupt({
            "type": "slice_analysis_review",
            "message": f"Review {state.slice_type} slice analysis results",
            "data": review_data,
            "actions": [
                "approve - Accept analysis and proceed to transformation",
                "request_deeper_analysis - Analyze more patterns",
                "adjust_integration - Modify horror integration points",
                "reject - Restart analysis with different parameters"
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
    
    def generate_bevy_components_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Generate Bevy components with horror integration."""
        
        print(f"🦀 Generating Bevy components for {state.slice_type}")
        
        # Generate components based on slice patterns
        bevy_components = []
        rust_code_parts = []
        
        # Import Rust code generator
        from dragons_labyrinth.hbf.rust_generator import RustComponentGenerator
        generator = RustComponentGenerator()
        
        for pattern_name, pattern_data in state.slice_patterns.items():
            # Generate component struct
            component = generator.generate_component(
                slice_type=state.slice_type,
                pattern_name=pattern_name,
                pattern_data=pattern_data,
                integration_points=state.integration_points
            )
            
            bevy_components.append(component)
            
            # Generate Rust code
            rust_code = generator.generate_rust_code(component)
            rust_code_parts.append(rust_code)
        
        # Combine all Rust code
        combined_rust_code = generator.combine_slice_code(
            state.slice_type, 
            rust_code_parts
        )
        
        print(f"✅ Generated {len(bevy_components)} Bevy components")
        
        return {
            "bevy_components": bevy_components,
            "rust_code": combined_rust_code,
            "step_count": state.step_count + 1
        }
    
    def finalize_slice_node(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Node: Finalize slice processing and update HBF database."""
        
        print(f"🎯 Finalizing {state.slice_type} slice processing")
        
        # Write output files
        output_files = []
        slice_output_dir = state.output_dir / f"{state.slice_type}_slice"
        slice_output_dir.mkdir(parents=True, exist_ok=True)
        
        # Write Rust component code
        if state.rust_code:
            rust_file = slice_output_dir / f"{state.slice_type}_components.rs"
            rust_file.write_text(state.rust_code, encoding="utf-8")
            output_files.append(str(rust_file))
        
        # Write analysis results
        import json
        analysis_file = slice_output_dir / f"{state.slice_type}_analysis.json"
        analysis_data = {
            "patterns": state.slice_patterns,
            "integration_points": state.integration_points,
            "probability_tables": state.probability_tables,
            "hyperlink_refs": state.hyperlink_refs,
            "entities_processed": len(state.raw_entities)
        }
        analysis_file.write_text(json.dumps(analysis_data, indent=2), encoding="utf-8")
        output_files.append(str(analysis_file))
        
        # Mark entities as processed
        processed_entity_ids = [entity['uuid'] for entity in state.raw_entities]
        
        # Update HBF database (remove processed entities)
        self._update_hbf_database(processed_entity_ids)
        
        print(f"✅ Generated {len(output_files)} output files")
        print(f"✅ Processed {len(processed_entity_ids)} entities")
        
        return {
            "processed_entity_ids": processed_entity_ids,
            "output_files": output_files,
            "completed_at": datetime.now(),
            "step_count": state.step_count + 1
        }
    
    def should_continue_to_generation(self, state: HBFSliceAnalysisState) -> Literal["generate", "end"]:
        """Conditional edge: Determine if we should proceed to generation."""
        
        if state.human_approval == "approve" and state.slice_patterns:
            return "generate"
        else:
            return "end"
    
    def build_slice_workflow(self) -> StateGraph:
        """Build the slice analysis workflow graph."""
        
        workflow = StateGraph(HBFSliceAnalysisState)
        
        # Add nodes
        workflow.add_node("extract_entities", self.extract_slice_entities_node)
        workflow.add_node("analyze_html", self.analyze_html_content_node)
        workflow.add_node("discover_patterns", self.discover_patterns_node)
        workflow.add_node("human_review", self.human_review_node)
        workflow.add_node("generate_components", self.generate_bevy_components_node)
        workflow.add_node("finalize_slice", self.finalize_slice_node)
        
        # Add edges
        workflow.add_edge(START, "extract_entities")
        workflow.add_edge("extract_entities", "analyze_html")
        workflow.add_edge("analyze_html", "discover_patterns")
        workflow.add_edge("discover_patterns", "human_review")
        
        # Conditional edges
        workflow.add_conditional_edges(
            "human_review",
            self.should_continue_to_generation,
            {
                "generate": "generate_components",
                "end": "finalize_slice"
            }
        )
        
        workflow.add_edge("generate_components", "finalize_slice")
        workflow.add_edge("finalize_slice", END)
        
        return workflow
    
    def compile_workflow(self) -> StateGraph:
        """Compile workflow with checkpointer for durable execution."""
        
        workflow = self.build_slice_workflow()
        checkpointer = self.get_checkpointer()
        
        if checkpointer is not None:
            compiled = workflow.compile(
                checkpointer=checkpointer,
                durability="async"
            )
        else:
            compiled = workflow.compile()
        
        return compiled
    
    def analyze_slice(
        self, 
        slice_type: str,
        autonomous_mode: bool = False
    ) -> dict[str, Any]:
        """
        Main entry point for slice analysis.
        
        Args:
            slice_type: Type of slice to analyze (region, dungeon, settlement, faction, etc.)
            autonomous_mode: Skip human review checkpoints
            
        Returns:
            Analysis results with generated components and files
        """
        
        # Create workflow
        agent = self.compile_workflow()
        
        # Create initial state
        workflow_id = f"{slice_type}_{uuid.uuid4().hex[:8]}"
        initial_state = {
            "slice_type": slice_type,
            "hbf_path": self.config.hbf_path,
            "output_dir": self.config.output_dir,
            "autonomous_mode": autonomous_mode,
            "workflow_id": workflow_id,
            "started_at": datetime.now(),
            "step_count": 0
        }
        
        # Create session config
        config = {
            "configurable": {
                "thread_id": workflow_id,
                "slice_type": slice_type
            }
        }
        
        print(f"🚀 Starting {slice_type} slice analysis: {workflow_id}")
        
        # Execute workflow
        final_state = agent.invoke(initial_state, config)
        
        return final_state
    
    # Helper methods for pattern discovery
    
    def _filter_entities_by_slice(self, entities_df, slice_type: str):
        """Filter entities DataFrame by slice type."""
        # This will be implemented based on entity classification logic
        # For now, return a placeholder
        return entities_df.head(10)  # Placeholder
    
    def _discover_region_patterns(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Discover patterns specific to region entities."""
        return {"weather_tables": [], "biome_distributions": [], "hex_coordinates": []}
    
    def _discover_dungeon_patterns(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Discover patterns specific to dungeon entities."""
        return {"room_connections": [], "monster_tables": [], "treasure_placement": []}
    
    def _discover_settlement_patterns(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Discover patterns specific to settlement entities.""" 
        return {"population_data": [], "building_types": [], "trade_routes": []}
    
    def _discover_faction_patterns(self, state: HBFSliceAnalysisState) -> dict[str, Any]:
        """Discover patterns specific to faction entities."""
        return {"member_hierarchies": [], "conspiracy_goals": [], "territory_control": []}
    
    def _update_hbf_database(self, processed_entity_ids: list[str]) -> None:
        """Update HBF database by removing processed entities."""
        # This will create a new HBF file without the processed entities
        print(f"📝 Would remove {len(processed_entity_ids)} entities from HBF database")
        # Implementation will be added when we have the database update logic


def create_hbf_analysis_agent(config: HBFConfig) -> HBFAnalysisAgent:
    """Factory function to create an HBF analysis agent."""
    return HBFAnalysisAgent(config)
