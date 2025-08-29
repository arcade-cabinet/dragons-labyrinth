extends Node
class_name CompanionPsychology

## COMPANION TRAUMA SYSTEM - Core horror mechanic
##
## Based on Architecture.md: "Companion Trauma: Psychological impact on party members"
## Companions break down psychologically as dread increases, requiring therapy

signal companion_stress_changed(companion_name: String, new_stress: float)
signal companion_breakdown(companion_name: String, breakdown_type: String)
signal therapy_session_started(companion_name: String)
signal therapy_session_completed(companion_name: String, success: bool)

# Companion psychological state tracking
var companion_states: Dictionary = {}

# Stress sources and their multipliers
const STRESS_SOURCES = {
	"combat_death": 15.0,      # Seeing allies die
	"horror_encounter": 10.0,   # Supernatural enemies
	"moral_conflict": 8.0,      # Actions against companion's values
	"environmental_decay": 5.0,  # Witnessing world corruption
	"dragon_proximity": 20.0,   # Being near the dragon
	"labyrinth_exposure": 25.0, # Time in Dragon's Labyrinth
	"void_exposure": 30.0      # Post-dragon void encounters
}

# Breakdown types and thresholds
const BREAKDOWN_THRESHOLDS = {
	"anxiety": 40.0,           # Refuses dangerous actions
	"depression": 60.0,        # Loses combat effectiveness
	"trauma": 80.0,            # Cannot enter dungeons
	"breakdown": 100.0         # Leaves party permanently
}

# Therapy requirements
const THERAPY_REQUIREMENTS = {
	"anxiety": {"time": 2, "skill_check": "easy"},
	"depression": {"time": 5, "skill_check": "medium"},
	"trauma": {"time": 10, "skill_check": "hard"},
	"breakdown": {"time": 20, "skill_check": "impossible"}
}

func _ready() -> void:
	# Connect to dread progression system
	if has_node("/root/DreadProgression"):
		var dread_system = get_node("/root/DreadProgression")
		dread_system.dread_level_changed.connect(_on_dread_level_changed)
		dread_system.system_corruption_updated.connect(_on_system_corruption_updated)
	
	# Connect to game events
	if GameState:
		GameState.companion_added.connect(_on_companion_added)
		GameState.combat_ended.connect(_on_combat_ended)
		GameState.story_choice_made.connect(_on_story_choice_made)

func _on_companion_added(companion_name: String) -> void:
	# Initialize psychological state for new companion
	companion_states[companion_name] = {
		"stress_level": 0.0,
		"loyalty": 50.0,
		"current_breakdown": "",
		"therapy_sessions": 0,
		"trauma_history": [],
		"comfort_sources": [],
		"stress_resistances": {},
		"breakdown_history": []
	}
	
	print("Companion joined: ", companion_name, " - Psychological profile initialized")

func add_companion_stress(companion_name: String, source: String, intensity: float = 1.0) -> void:
	if not companion_name in companion_states:
		return
	
	var state = companion_states[companion_name]
	var stress_amount = STRESS_SOURCES.get(source, 5.0) * intensity
	
	# Apply dread multiplier - stress increases faster at higher dread
	var dread_multiplier = 1.0
	if has_node("/root/DreadProgression"):
		dread_multiplier = get_node("/root/DreadProgression").get_companion_stress_multiplier()
	
	stress_amount *= dread_multiplier
	
	# Add stress, considering resistances
	var resistance = state.stress_resistances.get(source, 0.0)
	var final_stress = stress_amount * (1.0 - resistance)
	
	state.stress_level = min(100.0, state.stress_level + final_stress)
	state.trauma_history.append({"source": source, "intensity": final_stress, "time": Time.get_unix_time_from_system()})
	
	print("Companion stress: ", companion_name, " +", final_stress, " (", source, ") = ", state.stress_level)
	companion_stress_changed.emit(companion_name, state.stress_level)
	
	# Check for breakdown
	check_for_breakdown(companion_name)

func check_for_breakdown(companion_name: String) -> void:
	var state = companion_states[companion_name]
	var stress = state.stress_level
	
	# Check breakdown thresholds
	for breakdown_type in BREAKDOWN_THRESHOLDS:
		var threshold = BREAKDOWN_THRESHOLDS[breakdown_type]
		
		if stress >= threshold and state.current_breakdown == "":
			# Companion has breakdown
			state.current_breakdown = breakdown_type
			state.breakdown_history.append({
				"type": breakdown_type,
				"stress_at_breakdown": stress,
				"time": Time.get_unix_time_from_system()
			})
			
			companion_breakdown.emit(companion_name, breakdown_type)
			apply_breakdown_effects(companion_name, breakdown_type)
			break

func apply_breakdown_effects(companion_name: String, breakdown_type: String) -> void:
	print("=== COMPANION BREAKDOWN ===")
	print(companion_name, " is experiencing: ", breakdown_type)
	
	# Notify companion system to apply mechanical effects
	if has_node("/root/CompanionSystem"):
		var companion_system = get_node("/root/CompanionSystem")
		companion_system.apply_breakdown_effects(companion_name, breakdown_type)
	
	# Show breakdown dialogue
	match breakdown_type:
		"anxiety":
			show_breakdown_dialogue(companion_name, [
				"I... I can't do this anymore.",
				"Everything feels wrong. The shadows move.",
				"Maybe we should turn back?"
			])
		"depression": 
			show_breakdown_dialogue(companion_name, [
				"What's the point? We're all going to die anyway.",
				"The world is ending. Nothing matters.",
				"Leave me here. I'm done."
			])
		"trauma":
			show_breakdown_dialogue(companion_name, [
				"I can't... I can't go in there.",
				"The screams... I still hear the screams.",
				"Please don't make me go back in the dark."
			])
		"breakdown":
			show_breakdown_dialogue(companion_name, [
				"I'm sorry. I can't follow you anymore.",
				"You've become something I don't recognize.",
				"Goodbye. I hope you find what you're looking for."
			])

func show_breakdown_dialogue(companion_name: String, dialogue_lines: Array) -> void:
	# Show breakdown dialogue (would integrate with dialogue system)
	print("\n", companion_name, " says:")
	for line in dialogue_lines:
		print("  '", line, "'")
	print()

func start_therapy_session(companion_name: String) -> bool:
	if not companion_name in companion_states:
		return false
	
	var state = companion_states[companion_name]
	if state.current_breakdown == "":
		return false  # No breakdown to treat
	
	therapy_session_started.emit(companion_name)
	
	var breakdown_type = state.current_breakdown
	var requirements = THERAPY_REQUIREMENTS[breakdown_type]
	
	print("=== THERAPY SESSION ===")
	print("Helping ", companion_name, " with their ", breakdown_type)
	print("This will take ", requirements.time, " game hours")
	
	# Therapy mini-game would go here
	# For now, simulate based on player skill/luck
	var therapy_success = simulate_therapy_attempt(companion_name, breakdown_type)
	
	complete_therapy_session(companion_name, therapy_success)
	return therapy_success

func simulate_therapy_attempt(companion_name: String, breakdown_type: String) -> bool:
	var state = companion_states[companion_name]
	var requirements = THERAPY_REQUIREMENTS[breakdown_type]
	
	# Base success chance based on difficulty
	var base_chance = 0.7
	match requirements.skill_check:
		"easy": base_chance = 0.8
		"medium": base_chance = 0.6
		"hard": base_chance = 0.4
		"impossible": base_chance = 0.1
	
	# Loyalty affects success chance
	var loyalty_bonus = (state.loyalty - 50.0) * 0.01  # -0.5 to +0.5
	
	# Player's philosophy affects therapy effectiveness
	var philosophy_bonus = 0.0
	if GameState.philosophy_path == "light":
		philosophy_bonus = 0.2
	elif GameState.philosophy_path == "harmony":
		philosophy_bonus = 0.1
	
	var final_chance = base_chance + loyalty_bonus + philosophy_bonus
	return randf() < final_chance

func complete_therapy_session(companion_name: String, success: bool) -> void:
	var state = companion_states[companion_name]
	state.therapy_sessions += 1
	
	if success:
		print("Therapy successful! ", companion_name, " feels better.")
		
		# Reduce stress and clear breakdown
		state.stress_level = max(0.0, state.stress_level - 25.0)
		state.current_breakdown = ""
		
		# Increase loyalty from successful therapy
		state.loyalty = min(100.0, state.loyalty + 10.0)
		
		# Add stress resistance to prevent immediate re-breakdown
		var breakdown_source = state.trauma_history[-1].source if state.trauma_history.size() > 0 else "general"
		state.stress_resistances[breakdown_source] = min(0.5, state.stress_resistances.get(breakdown_source, 0.0) + 0.1)
		
	else:
		print("Therapy failed. ", companion_name, " remains broken.")
		
		# Failed therapy increases stress slightly
		state.stress_level = min(100.0, state.stress_level + 5.0)
		
		# Decrease loyalty from failed help
		state.loyalty = max(0.0, state.loyalty - 5.0)
	
	therapy_session_completed.emit(companion_name, success)

func _on_dread_level_changed(new_level: int) -> void:
	# Increase ambient stress as dread increases
	for companion_name in companion_states:
		add_companion_stress(companion_name, "environmental_decay", new_level * 0.5)

func _on_system_corruption_updated(system_name: String, corruption: float) -> void:
	if system_name == "companions":
		# Apply corruption effects to all companions
		for companion_name in companion_states:
			var state = companion_states[companion_name]
			state.stress_level = min(100.0, state.stress_level + (corruption * 10.0))

func _on_combat_ended(victory: bool, casualties: Array) -> void:
	# Combat affects companion psychology
	if victory:
		# Victory reduces stress slightly
		for companion_name in companion_states:
			add_companion_stress(companion_name, "combat_victory", -2.0)
	
	# Casualties cause major trauma
	for casualty in casualties:
		for companion_name in companion_states:
			if casualty != companion_name:  # Don't stress dead companions
				add_companion_stress(companion_name, "combat_death", 1.5)

func _on_story_choice_made(choice_id: String, choice_result: String) -> void:
	# Story choices affect companions based on their values
	# This would be data-driven from AI-generated companion personalities
	pass

func get_companion_combat_effectiveness(companion_name: String) -> float:
	if not companion_name in companion_states:
		return 1.0
	
	var state = companion_states[companion_name]
	var stress = state.stress_level
	
	# High stress reduces combat effectiveness
	var effectiveness = 1.0 - (stress * 0.008)  # 100 stress = 20% effectiveness
	
	# Breakdown types have specific effects
	match state.current_breakdown:
		"anxiety": effectiveness *= 0.7   # -30% effectiveness
		"depression": effectiveness *= 0.5  # -50% effectiveness
		"trauma": effectiveness *= 0.3     # -70% effectiveness  
		"breakdown": effectiveness = 0.0   # Cannot fight
	
	return max(0.0, effectiveness)

func get_companion_dialogue_modifier(companion_name: String) -> Dictionary:
	if not companion_name in companion_states:
		return {}
	
	var state = companion_states[companion_name]
	
	return {
		"stress_level": state.stress_level,
		"breakdown_type": state.current_breakdown,
		"loyalty": state.loyalty,
		"trauma_count": state.trauma_history.size(),
		"therapy_sessions": state.therapy_sessions
	}

func can_companion_enter_dungeon(companion_name: String) -> bool:
	if not companion_name in companion_states:
		return true
	
	var state = companion_states[companion_name]
	
	# Trauma breakdown prevents dungeon entry
	return state.current_breakdown != "trauma" and state.current_breakdown != "breakdown"

func load_companion_from_data(companion_data: Dictionary) -> void:
	# Load companion from AI-generated data
	var name = companion_data.get("name", "Unknown")
	
	# Initialize with AI-generated personality
	companion_states[name] = {
		"stress_level": 0.0,
		"loyalty": companion_data.get("starting_loyalty", 50.0),
		"current_breakdown": "",
		"therapy_sessions": 0,
		"trauma_history": [],
		"comfort_sources": companion_data.get("comfort_sources", []),
		"stress_resistances": companion_data.get("stress_resistances", {}),
		"breakdown_history": [],
		"personality_traits": companion_data.get("traits", []),
		"backstory": companion_data.get("backstory", "")
	}
	
	print("Loaded companion psychology: ", name)

# Save/Load psychological state
func save_psychology_state() -> Dictionary:
	return {
		"companion_states": companion_states
	}

func load_psychology_state(state: Dictionary) -> void:
	companion_states = state.get("companion_states", {})
