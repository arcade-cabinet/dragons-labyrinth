extends Node

## DreadProgression autoload - Manages horror progression system for Dragon's Labyrinth
##
## Handles the mathematical progression from Peace → Unease → Dread → Terror → Horror
## Based on distance from starting point and companion psychology states

signal dread_level_changed(new_level: int)
signal horror_event_triggered(event_type: String)
signal corruption_spread(cube_pos: Vector3i)

var current_dread_level: int = 0
var player_hex_position: Vector3i = Vector3i.ZERO
var total_distance_traveled: int = 0
var corruption_events: Array = []

# Horror progression thresholds
const DREAD_THRESHOLDS = {
	0: "Peace",      # 0-20 hex distance
	1: "Unease",     # 20-40 hex distance  
	2: "Dread",      # 40-60 hex distance
	3: "Terror",     # 60-120 hex distance
	4: "Horror"      # 120+ hex distance
}

func _ready():
	print("DreadProgression initialized - horror system active")

## Update player position and calculate dread
func update_player_position(new_cube_pos: Vector3i):
	var old_position = player_hex_position
	player_hex_position = new_cube_pos
	total_distance_traveled += HexTileData.distance_from_base(new_cube_pos)
	
	var new_dread = calculate_current_dread_level()
	if new_dread != current_dread_level:
		var old_dread = current_dread_level
		current_dread_level = new_dread
		dread_level_changed.emit(new_dread)
		print("Dread escalated from %s to %s" % [DREAD_THRESHOLDS[old_dread], DREAD_THRESHOLDS[new_dread]])

## Calculate dread level based on distance and corruption
func calculate_current_dread_level() -> int:
	var distance = HexTileData.distance_from_base(player_hex_position)
	var base_dread = min(4, distance / 20)
	
	# Modify based on corruption events
	var corruption_modifier = corruption_events.size() * 0.1
	
	return min(4, int(base_dread + corruption_modifier))

## Get current dread level
func get_dread_level() -> int:
	return current_dread_level

## Get current dread stage name
func get_dread_stage() -> String:
	return DREAD_THRESHOLDS[current_dread_level]

## Trigger horror event
func trigger_horror_event(event_type: String, cube_pos: Vector3i):
	horror_event_triggered.emit(event_type)
	corruption_events.append({
		"type": event_type,
		"position": cube_pos,
		"timestamp": Time.get_unix_time_from_system()
	})
	
	# Check if this triggers corruption spread
	if event_type in ["companion_breakdown", "moral_failure", "reality_distortion"]:
		corruption_spread.emit(cube_pos)

## Check if dragon is hunting (proximity mechanics)
func is_dragon_hunting() -> bool:
	return current_dread_level >= 4 and total_distance_traveled > 100

## Get corruption intensity for environmental effects
func get_corruption_intensity() -> float:
	return float(current_dread_level) / 4.0

## Save/load progression state
func save_progression_state() -> Dictionary:
	return {
		"dread_level": current_dread_level,
		"player_position": var_to_str(player_hex_position),
		"distance_traveled": total_distance_traveled,
		"corruption_events": corruption_events
	}

func load_progression_state(data: Dictionary):
	current_dread_level = data.get("dread_level", 0)
	player_hex_position = str_to_var(data.get("player_position", "Vector3i(0,0,0)"))
	total_distance_traveled = data.get("distance_traveled", 0)
	corruption_events = data.get("corruption_events", [])
