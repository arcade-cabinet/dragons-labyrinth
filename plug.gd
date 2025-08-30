extends "res://addons/gd-plug/plug.gd"

func _plugging():
	# Dragon's Labyrinth Horror RPG Plugin Dependencies
	# All plugins required for our infinite hex world horror RPG
	
	# CORE REQUIRED PLUGINS - Essential for our horror RPG mechanics
	
	# Database integration - CRITICAL for our 50+ table system (GDExtension)
	# NOTE: godot-sqlite installed manually through Godot Asset Library
	# plug("2shady4u/godot-sqlite")
	
	# Hex tile system - REQUIRED for our infinite algorithmic world  
	plug("Zehir/godot-hexagon-tile-map-layer")
	
	# Combat & Health system - For our inverted power progression
	plug("cluttered-code/godot-health-hitbox-hurtbox")
	
	# AI Behavior Trees - For sophisticated companion psychology and dragon hunting behavior
	plug("limbonaut/limboai")
	
	# Dialogue system - For companion psychology dialogue
	plug("dialogic-godot/dialogic")
	
	# Behavior trees - Alternative/complementary to LimboAI for complex NPCs
	plug("bitbrain/beehave")
	
	# RPG Data Management - For sophisticated RPG content organization
	plug("bitbrain/pandora")
	
	# QUEST SYSTEMS - Multiple options for our narrative progression
	
	# Quest Manager - Lightweight quest system
	plug("Rubonnek/quest-manager", {"dev": true})
	
	# Quest System - More comprehensive quest framework
	plug("shomykohai/quest-system", {"dev": true})
	
	# Questify - Another quest system option
	plug("TheWalruzz/godot-questify", {"dev": true})
