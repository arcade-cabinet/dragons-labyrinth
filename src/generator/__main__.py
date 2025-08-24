#!/usr/bin/env python3
"""
Dragon's Labyrinth - AI Asset Generator

Entry point for AI-powered asset generation using specialized agents.
Build-time Python AI generation → Runtime Rust game engine.

Usage:
    python -m src.generator [--agent AGENT] [--dread LEVEL]
    
Examples:
    python -m src.generator --agent tiles --dread 0
    python -m src.generator --agent all --dread 2
"""

import argparse
import sys
import os
import logging
from pathlib import Path

from .constants import (
    GAME_NAME,
    DREAD_LEVELS,
    LOGGER_NAME,
    LOG_FORMAT,
)


def setup_logging():
    """Setup logging for the generator."""
    logger = logging.getLogger(LOGGER_NAME)
    logger.setLevel(logging.INFO)
    
    # Console handler
    handler = logging.StreamHandler()
    handler.setFormatter(logging.Formatter(LOG_FORMAT))
    logger.addHandler(handler)
    
    return logger


def run_tiles_agent(dread_level: int, logger: logging.Logger):
    """Run the tiles agent for tile generation."""
    try:
        from .ai.tiles_agent import TilesAgent
        
        logger.info(f"🎨 Running TilesAgent for dread level {dread_level}")
        agent = TilesAgent()
        
        # Generate tiles for specified dread level
        tile_types = ["grassland", "forest", "mountain", "swamp"]
        descriptions = agent.generate_tile_descriptions(tile_types, dread_level)
        
        logger.info(f"Generated descriptions for {len(descriptions)} tile types")
        
        # Convert to BPY scripts
        results = agent.generate_bpy_scripts_from_descriptions(descriptions, dread_level)
        
        logger.info(f"Generated BPY batch: {results.get('total_scripts', 0)} scripts")
        logger.info(f"Output directory: {results.get('output_dir', 'Unknown')}")
        
        return results
        
    except Exception as e:
        logger.error(f"TilesAgent failed: {e}")
        return None


def run_maps_agent(dread_level: int, logger: logging.Logger):
    """Run the maps agent for world generation."""
    try:
        from .ai.maps_agent import MapsAgent, HexWorldConfig
        
        logger.info(f"🗺️ Running MapsAgent for dread level {dread_level}")
        agent = MapsAgent()
        
        config = HexWorldConfig(
            world_name=f"test_world_dread_{dread_level}",
            dread_level=dread_level,
            size=25,  # Medium size for testing
            biome="grassland" if dread_level < 3 else "corrupted"
        )
        
        result = agent.generate_world_layout(config)
        
        if result["success"]:
            logger.info(f"Generated world: {result['generation_id']}")
            
            # Generate Rust loader
            rust_path = agent.generate_rust_loader(result)
            logger.info(f"Rust loader: {rust_path}")
        else:
            logger.error(f"World generation failed: {result['error']}")
        
        return result
        
    except Exception as e:
        logger.error(f"MapsAgent failed: {e}")
        return None


def run_all_agents(dread_level: int, logger: logging.Logger):
    """Run all available agents for the specified dread level."""
    logger.info(f"🤖 Running ALL agents for dread level {dread_level}")
    
    results = {}
    
    # Run tiles agent
    tiles_result = run_tiles_agent(dread_level, logger)
    if tiles_result:
        results["tiles"] = tiles_result
    
    # Run maps agent  
    maps_result = run_maps_agent(dread_level, logger)
    if maps_result:
        results["maps"] = maps_result
    
    # Could add other agents here as they're fixed
    # ui_result = run_ui_agent(dread_level, logger)
    # dialogue_result = run_dialogue_agent(dread_level, logger)
    # audio_result = run_audio_agent(dread_level, logger)
    
    logger.info(f"Completed {len(results)} agent runs")
    return results


def validate_environment():
    """Validate that required environment is set up."""
    issues = []
    
    # Check OpenAI API key
    if not os.getenv("OPENAI_API_KEY"):
        issues.append("OPENAI_API_KEY environment variable not set")
    
    # Check assets directory exists
    assets_dir = Path("assets")
    if not assets_dir.exists():
        issues.append(f"Assets directory not found: {assets_dir}")
    
    return issues


def main():
    """Main entry point for Dragon's Labyrinth AI asset generator."""
    parser = argparse.ArgumentParser(
        description="Dragon's Labyrinth AI Asset Generator - Revolutionary build-time generation"
    )
    parser.add_argument(
        "--agent",
        choices=["tiles", "maps", "ui", "dialogue", "audio", "all"],
        default="tiles",
        help="Which AI agent to run (default: tiles)"
    )
    parser.add_argument(
        "--dread",
        type=int,
        choices=range(5),
        default=0,
        help="Dread level 0-4 (default: 0 - Peace)"
    )
    parser.add_argument(
        "--validate",
        action="store_true",
        help="Validate environment and exit"
    )
    parser.add_argument(
        "--info",
        action="store_true", 
        help="Show system information and exit"
    )
    
    args = parser.parse_args()
    
    # Setup logging
    logger = setup_logging()
    
    # Validate environment
    issues = validate_environment()
    if issues:
        logger.error("Environment validation failed:")
        for issue in issues:
            logger.error(f"  - {issue}")
        if args.validate:
            sys.exit(1)
        logger.warning("Continuing despite issues...")
    elif args.validate:
        logger.info("✅ Environment validation passed")
        sys.exit(0)
    
    # Show info if requested
    if args.info:
        logger.info(f"🐉 {GAME_NAME} AI Asset Generator")
        logger.info(f"📁 Working directory: {Path.cwd()}")
        logger.info(f"🎯 Available agents: tiles, maps, ui, dialogue, audio")
        logger.info(f"😱 Dread levels: {list(DREAD_LEVELS.keys())}")
        logger.info(f"🤖 Current agent: {args.agent}")
        logger.info(f"📊 Current dread: {args.dread} ({DREAD_LEVELS[args.dread]['name']})")
        return
    
    # Show current configuration
    logger.info(f"🐉 {GAME_NAME} - AI Asset Generator")
    logger.info(f"🤖 Agent: {args.agent}")
    logger.info(f"😱 Dread Level: {args.dread} ({DREAD_LEVELS[args.dread]['name']})")
    logger.info(f"📝 {DREAD_LEVELS[args.dread]['description']}")
    logger.info("")
    
    try:
        # Route to appropriate agent
        if args.agent == "tiles":
            results = run_tiles_agent(args.dread, logger)
        elif args.agent == "maps":
            results = run_maps_agent(args.dread, logger)
        elif args.agent == "all":
            results = run_all_agents(args.dread, logger)
        else:
            logger.error(f"Agent '{args.agent}' not yet implemented")
            logger.info("Available agents: tiles, maps, all")
            sys.exit(1)
        
        if results:
            logger.info("✅ Generation completed successfully!")
            logger.info("Next steps:")
            logger.info("  1. Review generated BPY scripts in assets/generated/")
            logger.info("  2. Execute in Blender: blender --python <script>")
            logger.info("  3. GLB assets will be ready for Bevy runtime")
        else:
            logger.error("❌ Generation failed")
            sys.exit(1)
            
    except KeyboardInterrupt:
        logger.info("Generation cancelled by user")
        sys.exit(0)
    except Exception as e:
        logger.error(f"Generation failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
