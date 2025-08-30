extends Node
class_name DreadProgression

## MASTER ORCHESTRATOR - Controls the emotional journey from peace → terror
##
## Based on Architecture.md vision: "Every system reinforces growing dread"
## Dread levels 0-4 affect ALL other systems dynamically

signal dread_level_changed(new_level: int)
signal emotional_stage_changed(new_stage: String)
signal system_corruption_updated(system_name: String, corruption: float)

@export var current_dread_level: int = 0
@export var emotional_stage: String = "peace"
@export var player_progression: int = 1  # 1-180 journey progression

# Core dread thresholds based on Themes.md
const DREAD_THRESHOLDS = {
	"peace": { "min": 1, "max": 20, "dread": 0 },
	"unease": { "min": 21, "max": 40, "dread": 1 }, 
	"dread": { "min": 41, "max": 60, "dread": 2 },
	"terror": { "min": 61, "max": 120, "dread": 3 },
	"void": { "min": 121, "max": 180, "dread": 4 }
}

# System corruption multipliers by dread level
var system_corruption_levels = {
	"companions": 0.0,
	"npcs": 0.0,
	"environment": 0.0,
	"audio": 0.0,
	"combat": 0.0
}

# References to systems that respond to dread
@onready var companion_system = get_node("/root/CompanionSystem") if has_node("/root/CompanionSystem") else null
@onready var audio_system = get_node("/root/AudioSystem") if has_node("/root/AudioSystem") else null
@onready var hex_world = get_node("/root/HexWorld") if has_node("/root/HexWorld") else null

func _ready() -> void:
	# Initialize dread progression system
	update_emotional_stage()
	
	# Connect to progression events
	if GameState:
		GameState.progression_changed.connect(_on_progression_changed)
		GameState.dragon_defeated.connect(_on_dragon_defeated)
		GameState.companion_trauma_event.connect(_on_companion_trauma)

func _on_progression_changed(new_progression: int) -> void:
	player_progression = new_progression
	update_emotional_stage()
	update_all_systems()

func update_emotional_stage() -> void:
	var old_stage = emotional_stage
	var old_dread = current_dread_level
	
	# Determine new emotional stage based on progression
	for stage_name in DREAD_THRESHOLDS:
		var threshold = DREAD_THRESHOLDS[stage_name]
		if player_progression >= threshold.min and player_progression <= threshold.max:
			emotional_stage = stage_name
			current_dread_level = threshold.dread
			break
	
	# Notify systems if changed
	if old_stage != emotional_stage:
		emotional_stage_changed.emit(emotional_stage)
		print("Emotional stage changed: ", old_stage, " → ", emotional_stage)
	
	if old_dread != current_dread_level:
		dread_level_changed.emit(current_dread_level)
		print("Dread level increased: ", old_dread, " → ", current_dread_level)

func _on_dragon_defeated() -> void:
	# Dragon defeat triggers Act 2: political/social horror
	print("Dragon defeated - world corruption accelerates")
	
	# Override normal progression - corruption spreads faster
	for system in system_corruption_levels:
		system_corruption_levels[system] = min(1.0, system_corruption_levels[system] + 0.3)
		system_corruption_updated.emit(system, system_corruption_levels[system])

func _on_companion_trauma(companion_name: String, trauma_type: String, intensity: float) -> void:
	# Companion trauma accelerates dread progression
	var trauma_multiplier = 1.0 + (intensity * 0.1)
	system_corruption_levels["companions"] = min(1.0, system_corruption_levels["companions"] + (0.1 * trauma_multiplier))
	
	print("Companion trauma detected: ", companion_name, " - ", trauma_type)
	update_all_systems()

func update_all_systems() -> void:
	# Update companion psychology based on dread
	if companion_system:
		companion_system.update_dread_effects(current_dread_level, system_corruption_levels["companions"])
	
	# Update audio atmosphere
	if audio_system:
		audio_system.set_dread_atmosphere(current_dread_level, emotional_stage)
	
	# Update world rendering
	if hex_world:
		hex_world.apply_corruption_effects(current_dread_level, system_corruption_levels["environment"])

func force_dread_increase(amount: int, reason: String = "") -> void:
	# Manual dread increase from story events
	current_dread_level = min(4, current_dread_level + amount)
	
	# Increase corruption across systems
	for system in system_corruption_levels:
		system_corruption_levels[system] = min(1.0, system_corruption_levels[system] + (amount * 0.2))
		system_corruption_updated.emit(system, system_corruption_levels[system])
	
	print("Dread forced increase by ", amount, " - Reason: ", reason)
	dread_level_changed.emit(current_dread_level)

func get_dread_multiplier(system_name: String) -> float:
	# Get dread multiplier for specific system
	var base_multiplier = 1.0 + (current_dread_level * 0.25)  # 1.0 → 2.0 across dread levels
	var system_corruption = system_corruption_levels.get(system_name, 0.0)
	return base_multiplier * (1.0 + system_corruption)

func get_corruption_color() -> Color:
	# Get color tint for corruption effects
	match current_dread_level:
		0: return Color.WHITE  # Pure, untainted
		1: return Color(0.95, 0.9, 0.85)  # Slight warmth loss
		2: return Color(0.8, 0.7, 0.6)  # Noticeable decay
		3: return Color(0.6, 0.5, 0.4)  # Heavy corruption
		4: return Color(0.3, 0.2, 0.4)  # Void-touched purple
	
	return Color.WHITE

func get_audio_pitch_shift() -> float:
	# Audio becomes more distorted with dread
	return 1.0 - (current_dread_level * 0.1)  # 1.0 → 0.6

func get_npc_flee_threshold() -> float:
	# NPCs flee more easily as dread increases
	return 0.8 - (current_dread_level * 0.15)  # 0.8 → 0.2

func get_companion_stress_multiplier() -> float:
	# Companion stress increases faster with dread
	return 1.0 + (current_dread_level * 0.5)  # 1.0 → 3.0

func is_labyrinth_ready() -> bool:
	# Dragon's Labyrinth unlocks at dread level 2+
	return current_dread_level >= 2

# Save/Load dread progression state
func save_progression_state() -> Dictionary:
	return {
		"dread_level": current_dread_level,
		"emotional_stage": emotional_stage,
		"progression": player_progression,
		"system_corruption": system_corruption_levels
	}

func load_progression_state(state: Dictionary) -> void:
	current_dread_level = state.get("dread_level", 0)
	emotional_stage = state.get("emotional_stage", "peace")
	player_progression = state.get("progression", 1)
	system_corruption_levels = state.get("system_corruption", {})
	
	# Apply loaded state to all systems
	update_all_systems()
