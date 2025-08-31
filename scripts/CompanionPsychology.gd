extends Node

## CompanionPsychology autoload - Manages companion trauma/therapy system
##
## Integrates with:
## - godot-sqlite for companion data
## - dialogic for dialogue interactions
## - pandora for companion entity management

signal companion_trauma_gained(companion_id: String, trauma_type: String)
signal companion_trauma_healed(companion_id: String, trauma_type: String)
signal companion_left_party(companion_id: String, reason: String)
signal therapy_session_completed(companion_id: String, effectiveness: float)

var db: SQLite
var active_companions: Dictionary = {}
var trauma_states: Dictionary = {}

# Trauma types and their effects
const TRAUMA_TYPES = {
	"witnessing_death": {"severity": 0.8, "therapy_difficulty": 0.7},
	"moral_conflict": {"severity": 0.6, "therapy_difficulty": 0.5},
	"reality_distortion": {"severity": 1.0, "therapy_difficulty": 0.9},
	"abandonment": {"severity": 0.7, "therapy_difficulty": 0.6},
	"corruption_exposure": {"severity": 0.9, "therapy_difficulty": 0.8}
}

func _ready():
	# Initialize database connection
	db = SQLite.new()
	db.path = "metadata/game.db"
	if not db.open_db():
		push_error("Failed to open companion psychology database")
		return
	
	# Connect to dread progression
	DreadProgression.dread_level_changed.connect(_on_dread_level_changed)
	DreadProgression.horror_event_triggered.connect(_on_horror_event)
	
	print("CompanionPsychology initialized - companion system active with simple schema")

## Load companion from database
func load_companion(companion_id: String) -> Dictionary:
	var query = "SELECT * FROM game_companions WHERE companion_id = ?"
	var result = db.query_with_bindings(query, [companion_id])
	
	if result.size() > 0:
		var companion_data = result[0]
		active_companions[companion_id] = companion_data
		return companion_data
	
	return {}

## Add companion to party
func add_companion_to_party(companion_id: String):
	var companion_data = load_companion(companion_id)
	if companion_data.is_empty():
		push_error("Companion not found: " + companion_id)
		return
	
	active_companions[companion_id] = companion_data
	trauma_states[companion_id] = companion_data.get("current_traumas", [])
	print("Companion joined party: " + companion_data.get("name", "Unknown"))

## Apply trauma to companion
func apply_trauma(companion_id: String, trauma_type: String, severity: float = 1.0):
	if not active_companions.has(companion_id):
		return
	
	if not TRAUMA_TYPES.has(trauma_type):
		push_warning("Unknown trauma type: " + trauma_type)
		return
	
	# Add trauma to companion's state
	if not trauma_states.has(companion_id):
		trauma_states[companion_id] = []
	
	trauma_states[companion_id].append({
		"type": trauma_type,
		"severity": severity,
		"timestamp": Time.get_unix_time_from_system()
	})
	
	companion_trauma_gained.emit(companion_id, trauma_type)
	
	# Check if trauma exceeds companion's tolerance
	var total_trauma = calculate_total_trauma(companion_id)
	var companion = active_companions[companion_id]
	var tolerance = companion.get("trauma_tolerance", 0.7)
	
	if total_trauma > tolerance:
		companion_breakdown(companion_id)

## Calculate total trauma for companion
func calculate_total_trauma(companion_id: String) -> float:
	if not trauma_states.has(companion_id):
		return 0.0
	
	var total = 0.0
	for trauma in trauma_states[companion_id]:
		var trauma_type = trauma.get("type", "")
		var severity = trauma.get("severity", 1.0)
		if TRAUMA_TYPES.has(trauma_type):
			total += TRAUMA_TYPES[trauma_type]["severity"] * severity
	
	return total

## Companion breakdown - they leave the party
func companion_breakdown(companion_id: String):
	if not active_companions.has(companion_id):
		return
	
	var companion = active_companions[companion_id]
	var name = companion.get("name", "Unknown")
	
	# Remove from party
	active_companions.erase(companion_id)
	
	# Trigger horror event
	DreadProgression.trigger_horror_event("companion_breakdown", DreadProgression.player_hex_position)
	
	companion_left_party.emit(companion_id, "trauma_breakdown")
	print("Companion breakdown: " + name + " has left the party due to trauma")

## Attempt therapy session
func attempt_therapy(therapist_id: String, patient_id: String) -> float:
	if not active_companions.has(therapist_id) or not active_companions.has(patient_id):
		return 0.0
	
	var therapist = active_companions[therapist_id]
	var patient = active_companions[patient_id]
	
	# Calculate therapy effectiveness
	var therapy_skill = therapist.get("therapy_skill", 0.5)
	var patient_receptiveness = patient.get("therapy_receptiveness", 0.7)
	var dread_penalty = DreadProgression.get_corruption_intensity() * 0.3
	
	var effectiveness = (therapy_skill + patient_receptiveness) / 2.0 - dread_penalty
	effectiveness = max(0.0, min(1.0, effectiveness))
	
	# Apply healing if effective
	if effectiveness > 0.3 and trauma_states.has(patient_id):
		if trauma_states[patient_id].size() > 0:
			var healed_trauma = trauma_states[patient_id].pop_back()
			companion_trauma_healed.emit(patient_id, healed_trauma.get("type", "unknown"))
	
	therapy_session_completed.emit(patient_id, effectiveness)
	return effectiveness

## Get companion trust level
func get_companion_trust(companion_id: String) -> float:
	if companion_id not in active_companions:
		return 0.0
	
	var companion = active_companions[companion_id]
	var base_trust = companion.get("loyalty_level", 0.5)
	
	# Reduce trust based on trauma
	var trauma_penalty = calculate_total_trauma(companion_id) * 0.2
	
	return max(0.0, base_trust - trauma_penalty)

## Check if companion will abandon player
func will_companion_abandon(companion_id: String) -> bool:
	var trust = get_companion_trust(companion_id)
	var dread = DreadProgression.get_corruption_intensity()
	
	return trust < 0.3 or (trust < 0.5 and dread > 0.7)

## Handle dread level changes
func _on_dread_level_changed(new_level: int):
	# Apply environmental trauma to all companions
	for companion_id in active_companions.keys():
		if new_level >= 3:  # Terror level and above
			apply_trauma(companion_id, "reality_distortion", 0.3)
		elif new_level >= 2:  # Dread level
			apply_trauma(companion_id, "corruption_exposure", 0.2)

## Handle horror events
func _on_horror_event(event_type: String):
	# Apply trauma based on event type
	for companion_id in active_companions.keys():
		match event_type:
			"companion_breakdown":
				apply_trauma(companion_id, "abandonment", 0.5)
			"moral_failure":
				apply_trauma(companion_id, "moral_conflict", 0.4)
			"reality_distortion":
				apply_trauma(companion_id, "reality_distortion", 0.6)
