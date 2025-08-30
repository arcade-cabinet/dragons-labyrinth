## Horror RPG Player State Management
## Adapted from OpenRPG player.gd - preserves excellent state management while adding horror mechanics
##
## Manages player curse progression, companion relationships, and horror state tracking
## Integrates with DreadProgression and CompanionPsychology systems
extends Node
class_name HorrorPlayerState

## Player state changes
signal gamepiece_changed
signal curse_level_changed(new_level: int)
signal hex_position_changed(new_coords: Vector3i)
signal companion_count_changed(count: int)
signal horror_event_triggered(event_type: String)

## Player movement and exploration
signal player_path_set(gamepiece: Gamepiece, destination_hex: Vector3i)
signal door_interaction(door_opened: bool)
signal region_discovered(region_name: String)

## The gamepiece the player controls - adapted for hex movement
var gamepiece: Gamepiece = null:
	set(value):
		if value != gamepiece:
			gamepiece = value
			gamepiece_changed.emit()

## Horror RPG specific state
var curse_level: int = 0  # Increases as we approach dragon (0-180)
var current_hex: Vector3i = Vector3i.ZERO  # Cube coordinates for distance calculation
var home_hex: Vector3i = Vector3i.ZERO  # Starting position - the door
var companions: Array[Dictionary] = []  # Active companions with psychology state
var sentimental_items: Array[Dictionary] = []  # Items for forge system
var moral_choices_made: Array[Dictionary] = []  # Choices that affect ending
var regions_discovered: Array[String] = []  # For progression tracking
var horror_events_experienced: Array[String] = []  # Tracks horror exposure

## Connection to horror systems
@onready var dread_system = DreadProgression
@onready var companion_system = CompanionPsychology

func _ready() -> void:
	# Connect to horror progression
	if dread_system:
		dread_system.dread_level_changed.connect(_on_dread_level_changed)
		dread_system.emotional_stage_changed.connect(_on_emotional_stage_changed)
	
	# Track companion changes
	if companion_system:
		companion_system.companion_stress_changed.connect(_on_companion_stress_changed)
		companion_system.companion_breakdown.connect(_on_companion_breakdown)

func move_to_hex(new_coords: Vector3i) -> void:
	"""Move player to new hex coordinates and update progression"""
	var old_hex = current_hex
	current_hex = new_coords
	
	# Calculate distance from home for curse progression
	var distance = hex_distance(home_hex, current_hex)
	var old_curse = curse_level
	curse_level = distance
	
	# Update dread progression if moving away from home
	if distance > hex_distance(home_hex, old_hex):
		dread_system.player_progression = distance
		dread_system.update_emotional_stage()
	
	hex_position_changed.emit(current_hex)
	
	if old_curse != curse_level:
		curse_level_changed.emit(curse_level)

func hex_distance(hex1: Vector3i, hex2: Vector3i) -> int:
	"""Calculate cube coordinate distance between two hex positions"""
	return (abs(hex1.x - hex2.x) + abs(hex1.y - hex2.y) + abs(hex1.z - hex2.z)) / 2

func add_companion(companion_data: Dictionary) -> void:
	"""Add companion with psychology integration"""
	companions.append(companion_data)
	companion_count_changed.emit(companions.size())
	
	# Initialize psychology for new companion
	if companion_system:
		companion_system.load_companion_from_data(companion_data)

func remove_companion(companion_name: String, reason: String = "left") -> void:
	"""Remove companion (breakdown, death, abandonment)"""
	for i in range(companions.size()):
		if companions[i].get("name") == companion_name:
			companions.remove_at(i)
			break
	
	companion_count_changed.emit(companions.size())
	horror_event_triggered.emit("companion_loss")
	
	print("Companion lost: ", companion_name, " - ", reason)

func add_sentimental_item(item_data: Dictionary) -> void:
	"""Add item that becomes forge reagent"""
	sentimental_items.append(item_data)
	print("Sentimental item gained: ", item_data.get("name", "Unknown"))

func make_moral_choice(choice_data: Dictionary) -> void:
	"""Record moral choice that affects ending"""
	moral_choices_made.append(choice_data)
	
	# Moral choices may affect companion psychology
	if companion_system:
		for companion_name in get_companion_names():
			# Check if choice conflicts with companion values
			var stress_amount = calculate_moral_stress(choice_data, companion_name)
			if stress_amount > 0:
				companion_system.add_companion_stress(companion_name, "moral_conflict", stress_amount)

func calculate_moral_stress(choice_data: Dictionary, companion_name: String) -> float:
	"""Calculate psychological stress from moral choice based on companion personality"""
	# This would integrate with AI-generated companion personality data
	# For now, basic implementation
	var choice_alignment = choice_data.get("alignment", "neutral")
	return 0.0  # Placeholder for complex psychology calculation

func trigger_horror_event(event_type: String, intensity: float = 1.0) -> void:
	"""Trigger horror event that affects multiple systems"""
	horror_events_experienced.append(event_type)
	horror_event_triggered.emit(event_type)
	
	# Add stress to all companions
	if companion_system:
		for companion_name in get_companion_names():
			companion_system.add_companion_stress(companion_name, event_type, intensity)

func get_companion_names() -> Array[String]:
	"""Get list of active companion names"""
	var names: Array[String] = []
	for companion in companions:
		names.append(companion.get("name", "Unknown"))
	return names

func discover_region(region_name: String) -> void:
	"""Discover new region and trigger related events"""
	if region_name not in regions_discovered:
		regions_discovered.append(region_name)
		region_discovered.emit(region_name)
		print("Region discovered: ", region_name)

func _on_dread_level_changed(new_level: int) -> void:
	"""React to dread progression changes"""
	print("Player experiencing dread level ", new_level, " effects")

func _on_emotional_stage_changed(new_stage: String) -> void:
	"""React to emotional stage transitions"""
	print("Player entered emotional stage: ", new_stage)
	horror_event_triggered.emit("stage_transition")

func _on_companion_stress_changed(companion_name: String, stress: float) -> void:
	"""React to companion psychology changes"""
	if stress > 80.0:  # High stress threshold
		print("Warning: ", companion_name, " is under severe psychological stress")

func _on_companion_breakdown(companion_name: String, breakdown_type: String) -> void:
	"""React to companion psychological breakdown"""
	horror_event_triggered.emit("companion_breakdown")
	
	# Some breakdowns remove companions
	if breakdown_type == "breakdown":
		remove_companion(companion_name, "psychological_breakdown")

# Save/Load horror state
func save_player_state() -> Dictionary:
	return {
		"curse_level": curse_level,
		"current_hex": [current_hex.x, current_hex.y, current_hex.z],
		"home_hex": [home_hex.x, home_hex.y, home_hex.z],
		"companions": companions,
		"sentimental_items": sentimental_items,
		"moral_choices": moral_choices_made,
		"regions_discovered": regions_discovered,
		"horror_events": horror_events_experienced
	}

func load_player_state(state: Dictionary) -> void:
	curse_level = state.get("curse_level", 0)
	
	var current_hex_data = state.get("current_hex", [0, 0, 0])
	current_hex = Vector3i(current_hex_data[0], current_hex_data[1], current_hex_data[2])
	
	var home_hex_data = state.get("home_hex", [0, 0, 0])
	home_hex = Vector3i(home_hex_data[0], home_hex_data[1], home_hex_data[2])
	
	companions = state.get("companions", [])
	sentimental_items = state.get("sentimental_items", [])
	moral_choices_made = state.get("moral_choices", [])
	regions_discovered = state.get("regions_discovered", [])
	horror_events_experienced = state.get("horror_events", [])
