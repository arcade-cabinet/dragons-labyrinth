extends Node

## TransitionLoader autoload - Manages scene transitions for horror sequences
##
## Handles transitions between:
## - Overworld hex exploration (2.5D)
## - First-person horror sequences (3D)
## - Door scenes and labyrinth encounters

signal transition_started(from_scene: String, to_scene: String)
signal transition_completed(scene_name: String)
signal horror_sequence_triggered(sequence_type: String)

var current_scene_type: String = "overworld"
var transition_in_progress: bool = false

# Scene paths
const SCENES = {
	"overworld": "res://scenes/overworld/World.tscn",
	"door_sequence": "res://scenes/ui/DoorScene.tscn", 
	"labyrinth": "res://scenes/labyrinth/Labyrinth.tscn",
	"first_person": "res://scenes/labyrinth/FirstPersonHorror.tscn"
}

func _ready():
	# Connect to horror progression
	DreadProgression.dread_level_changed.connect(_on_dread_level_changed)
	DreadProgression.horror_event_triggered.connect(_on_horror_event)
	
	print("TransitionLoader initialized - scene transition system active")

## Transition to first-person horror sequence
func transition_to_horror_sequence(sequence_type: String = "labyrinth"):
	if transition_in_progress:
		return
	
	transition_in_progress = true
	horror_sequence_triggered.emit(sequence_type)
	
	var target_scene = SCENES.get("first_person", SCENES["labyrinth"])
	
	# Create transition effect
	create_horror_transition_effect()
	
	# Wait for effect, then change scene
	await get_tree().create_timer(1.0).timeout
	
	transition_started.emit(current_scene_type, sequence_type)
	get_tree().change_scene_to_file(target_scene)
	current_scene_type = sequence_type
	transition_in_progress = false
	transition_completed.emit(sequence_type)

## Transition back to overworld from horror sequence
func return_to_overworld():
	if transition_in_progress:
		return
	
	transition_in_progress = true
	transition_started.emit(current_scene_type, "overworld")
	
	get_tree().change_scene_to_file(SCENES["overworld"])
	current_scene_type = "overworld"
	transition_in_progress = false
	transition_completed.emit("overworld")

## Trigger door opening sequence (iconic horror moment)
func trigger_door_sequence():
	if transition_in_progress:
		return
	
	transition_in_progress = true
	transition_started.emit(current_scene_type, "door_sequence")
	
	# Load door scene
	get_tree().change_scene_to_file(SCENES["door_sequence"])
	current_scene_type = "door_sequence"
	transition_in_progress = false
	transition_completed.emit("door_sequence")

## Create horror transition visual effect
func create_horror_transition_effect():
	# Create screen darkening effect
	var overlay = ColorRect.new()
	overlay.color = Color.BLACK
	overlay.modulate.a = 0.0
	overlay.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	get_tree().current_scene.add_child(overlay)
	
	# Fade to black
	var tween = create_tween()
	tween.tween_property(overlay, "modulate:a", 1.0, 0.5)
	
	# Add horror sound effect
	var audio = AudioStreamPlayer.new()
	# TODO: Load horror transition sound from AssetCatalog
	get_tree().current_scene.add_child(audio)

## Check if should trigger horror sequence based on dread level
func should_trigger_horror_sequence() -> bool:
	return DreadProgression.get_dread_level() >= 4 and DreadProgression.is_dragon_hunting()

## Handle dread level changes
func _on_dread_level_changed(new_level: int):
	match new_level:
		4:  # Horror level - trigger labyrinth sequence
			if current_scene_type == "overworld" and should_trigger_horror_sequence():
				transition_to_horror_sequence("labyrinth")
		3:  # Terror level - reality becomes unreliable
			if randf() < 0.3:  # 30% chance of false horror transition
				create_horror_transition_effect()

## Handle horror events
func _on_horror_event(event_type: String):
	match event_type:
		"companion_breakdown":
			if randf() < 0.5:  # 50% chance of triggering door memory
				trigger_door_sequence()
		"dragon_proximity":
			transition_to_horror_sequence("first_person")

## Get current scene type
func get_current_scene_type() -> String:
	return current_scene_type

## Check if in horror sequence
func is_in_horror_sequence() -> bool:
	return current_scene_type in ["door_sequence", "labyrinth", "first_person"]
