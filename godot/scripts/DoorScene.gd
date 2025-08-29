extends Node3D
class_name DoorScene

## THE MOST IMPORTANT SCENE - Last moment of normalcy before horror begins
##
## Based on Architecture.md: "First-person door scene - the last peaceful moment"
## Critical transition from safety to horror, companion choice echoes through entire journey

signal door_opened()
signal companion_choice_made(took_companion: bool)
signal entered_hex_world(with_companion: bool)

@export var door_model: MeshInstance3D
@export var door_handle: Area3D
@export var player_camera: Camera3D
@export var companion_character: CharacterBody3D
@export var dialogue_ui: Control

# Scene state
var _can_interact_with_door: bool = true
var _door_opening: bool = false
var _choice_made: bool = false
var _companion_waiting: bool = false

# Audio players for the last normal sounds
var morning_audio: AudioStreamPlayer3D
var door_audio: AudioStreamPlayer3D
var footsteps_audio: AudioStreamPlayer3D

# Video player for existing assets
var video_player: VideoStreamPlayer

func _ready() -> void:
	print("=== DRAGON'S LABYRINTH ===")
	print("You stand in your home, looking at the front door.")
	print("Outside, birds sing. Inside, you are safe.")
	print("For now.")
	
	setup_first_person_view()
	setup_home_interior()
	setup_audio_systems()
	setup_door_interaction()
	setup_video_player()
	
	# This is home. This is safety. This is the last time.
	play_morning_ambience()

func setup_first_person_view() -> void:
	if player_camera:
		player_camera.fov = 75.0  # Human-like field of view
		player_camera.position = Vector3(0, 1.6, -3)  # Standing height, facing door
		player_camera.rotation_degrees = Vector3(0, 0, 0)
		
		# Subtle head bob for realism (last human comfort)
		add_gentle_head_bob()

func add_gentle_head_bob() -> void:
	var tween = create_tween()
	tween.set_loops()
	tween.tween_property(player_camera, "position:y", 1.62, 2.0)
	tween.tween_property(player_camera, "position:y", 1.58, 2.0)

func setup_home_interior() -> void:
	# Create the last safe space - simple, warm, everything they'll lose
	
	# Warm morning light - they'll miss this
	var sun = DirectionalLight3D.new()
	sun.light_energy = 0.8
	sun.light_color = Color(1.0, 0.95, 0.8)  # Warm but slightly wrong
	sun.rotation_degrees = Vector3(-30, -45, 0)
	add_child(sun)
	
	# Family photo on side table (barely visible, will never see again)
	var photo_frame = MeshInstance3D.new()
	photo_frame.position = Vector3(-2, 0.85, -2)
	photo_frame.scale = Vector3(0.2, 0.25, 0.02)
	add_child(photo_frame)
	
	# Clock showing wrong time (time itself is breaking)
	var clock = MeshInstance3D.new()
	clock.position = Vector3(2, 2, -3.9)
	clock.rotation_degrees = Vector3(90, 0, 0)
	add_child(clock)
	
	# Add subtle wrongness - shadow under door too dark
	add_foreshadowing_elements()

func add_foreshadowing_elements() -> void:
	# Subtle horror hints that will become obvious later
	if door_model:
		# Door grain pattern looks almost like... scales?
		var door_material = door_model.get_surface_override_material(0)
		if door_material and door_material is StandardMaterial3D:
			door_material.normal_scale = 1.2  # Enhance the grain pattern
		
		# Shadow under door unnaturally dark
		var shadow = CSGBox3D.new()
		shadow.size = Vector3(0.8, 0.01, 0.1)
		shadow.position = Vector3(0, -1.0, 0.5)
		door_model.add_child(shadow)

func setup_audio_systems() -> void:
	# Morning ambience - the last normal sounds they'll hear
	morning_audio = AudioStreamPlayer3D.new()
	add_child(morning_audio)
	
	# Door sounds - will haunt them later
	door_audio = AudioStreamPlayer3D.new()
	add_child(door_audio)
	
	# Footsteps - careful, not yet fearful
	footsteps_audio = AudioStreamPlayer3D.new()
	add_child(footsteps_audio)

func setup_door_interaction() -> void:
	if door_handle:
		door_handle.mouse_entered.connect(_on_door_hover)
		door_handle.mouse_exited.connect(_on_door_unhover)
		door_handle.input_event.connect(_on_door_clicked)

func setup_video_player() -> void:
	# Use existing video assets for transitions
	video_player = VideoStreamPlayer.new()
	video_player.visible = false
	add_child(video_player)

func play_morning_ambience() -> void:
	# Load existing audio if available
	var audio_path = "res://assets/audio/morning_peace.ogg"
	if ResourceLoader.exists(audio_path):
		morning_audio.stream = load(audio_path)
		morning_audio.play()
	else:
		print("Morning audio not found - will be generated later")

func _on_door_hover() -> void:
	if _can_interact_with_door and not _door_opening:
		Input.set_default_cursor_shape(Input.CURSOR_POINTING_HAND)

func _on_door_unhover() -> void:
	Input.set_default_cursor_shape(Input.CURSOR_ARROW)

func _on_door_clicked(camera: Node, event: InputEvent, position: Vector3, normal: Vector3, shape_idx: int) -> void:
	if event is InputEventMouseButton and event.pressed and _can_interact_with_door and not _door_opening:
		start_door_sequence()

func start_door_sequence() -> void:
	_door_opening = true
	_can_interact_with_door = false
	
	print("You reach for the door handle...")
	
	# Play door handle sound
	play_door_sound("handle_turn")
	
	# Turn handle animation
	if door_handle:
		var handle_tween = create_tween()
		handle_tween.tween_property(door_handle, "rotation_degrees:z", 45, 0.5)
		handle_tween.tween_callback(handle_turned)

func handle_turned() -> void:
	print("The handle turns. The door opens outward.")
	
	# Door opening sound
	play_door_sound("door_creak")
	
	# Door swings open
	if door_model:
		var door_tween = create_tween()
		door_tween.tween_property(door_model, "rotation_degrees:y", -90, 2.0)
		door_tween.tween_callback(door_opened_complete)
	
	# Fade morning sounds
	var audio_tween = create_tween()
	audio_tween.tween_property(morning_audio, "volume_db", -60, 3.0)

func door_opened_complete() -> void:
	door_opened.emit()
	print("You step through the doorway.")
	print("Behind you, the door closes. You cannot go back.")
	
	# Use existing video asset for village transition
	play_transition_video("videos/villager-intro_1755767336555.mp4")

func play_door_sound(sound_type: String) -> void:
	var sound_path = "res://assets/audio/" + sound_type + ".ogg"
	if ResourceLoader.exists(sound_path):
		door_audio.stream = load(sound_path)
		door_audio.play()

func play_transition_video(video_path: String) -> void:
	if ResourceLoader.exists("res://assets/" + video_path):
		video_player.stream = load("res://assets/" + video_path)
		video_player.visible = true
		video_player.play()
		video_player.finished.connect(show_village_choice)
	else:
		# Fallback to immediate village choice
		await get_tree().create_timer(1.0).timeout
		show_village_choice()

func show_village_choice() -> void:
	print("\n=== THE CRITICAL CHOICE ===")
	print("Your childhood friend waits by the village gate.")
	print("'I heard what you're planning,' they say.")
	print("'The birds have stopped singing. Something's wrong.'")
	print("'I'm coming with you.'")
	
	# Show companion character
	if companion_character:
		companion_character.visible = true
		companion_character.position = Vector3(2, 0, 3)
		companion_character.look_at(player_camera.global_position, Vector3.UP)
	
	# Present the choice that echoes through everything
	if dialogue_ui:
		show_companion_dialogue()
	else:
		# Fallback to console choice
		show_console_choice()

func show_companion_dialogue() -> void:
	var choices = [
		{"text": "Come with me. I could use a friend.", "choice": "take_companion"},
		{"text": "Stay here where it's safe. I'll handle this alone.", "choice": "go_alone"}
	]
	
	# This would connect to a proper dialogue UI system
	# For now, simulate the choice
	await get_tree().create_timer(2.0).timeout
	handle_companion_choice("take_companion")  # Default choice for testing

func show_console_choice() -> void:
	print("\nChoices:")
	print("1. 'Come with me. I could use a friend.'")
	print("2. 'Stay here where it's safe. I'll handle this alone.'")
	print("(Auto-selecting option 1 for demo)")
	
	await get_tree().create_timer(2.0).timeout
	handle_companion_choice("take_companion")

func handle_companion_choice(choice: String) -> void:
	if _choice_made:
		return
	
	_choice_made = true
	companion_choice_made.emit(choice == "take_companion")
	
	match choice:
		"take_companion":
			companion_accepts()
		"go_alone":
			companion_stays()

func companion_accepts() -> void:
	print("\nYour friend smiles, scared but grateful.")
	print("'Thank you. I couldn't let you face this alone.'")
	print("'Remember when we dreamed about adventures?'")
	print("'I suppose we should be careful what we wish for.'")
	
	# Set game state for companion journey
	GameState.starting_companion = true
	GameState.companion_name = "Childhood Friend"  # Will be generated by AI
	GameState.companion_loyalty = 25  # Starting loyalty
	
	# Achievement unlock
	GameState.unlock_achievement("not_alone")
	
	await get_tree().create_timer(3.0).timeout
	transition_to_hex_world(true)

func companion_stays() -> void:
	print("\nYour friend nods, hurt but understanding.")
	print("'I... I understand. Maybe you're right.'")
	print("'Just promise me you'll be careful?'")
	print("'Good luck. Come back to us.'")
	
	# Set game state for solo journey
	GameState.starting_companion = false
	GameState.can_hire_mercenary = true
	GameState.difficulty_modifier = 1.2  # Harder alone
	
	# Achievement unlock
	GameState.unlock_achievement("lone_wolf")
	
	await get_tree().create_timer(3.0).timeout
	transition_to_hex_world(false)

func transition_to_hex_world(with_companion: bool) -> void:
	print("\n=== ENTERING THE HEX WORLD ===")
	print("The world shifts around you...")
	print("Your perspective changes...")
	print("The journey begins.")
	
	# Use existing portal transition video if available
	if ResourceLoader.exists("res://assets/videos/traveler-portal_1755767336514.mp4"):
		video_player.stream = load("res://assets/videos/traveler-portal_1755767336514.mp4")
		video_player.play()
		video_player.finished.connect(complete_transition.bind(with_companion))
	else:
		# Fade transition fallback
		var fade = ColorRect.new()
		fade.color = Color.WHITE
		fade.modulate.a = 0.0
		fade.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
		get_viewport().add_child(fade)
		
		var tween = create_tween()
		tween.tween_property(fade, "modulate:a", 1.0, 2.0)
		tween.tween_callback(complete_transition.bind(with_companion))

func complete_transition(with_companion: bool) -> void:
	entered_hex_world.emit(with_companion)
	
	# Initialize dread progression system
	if DreadProgression:
		DreadProgression.player_progression = 1
		DreadProgression.update_emotional_stage()
	
	# Load hex world scene based on companion choice
	if with_companion:
		get_tree().change_scene_to_file("res://scenes/HexWorld.tscn")
	else:
		get_tree().change_scene_to_file("res://scenes/HexWorldSolo.tscn")

# Input handling for first-person interaction
func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_accept") and _can_interact_with_door:
		start_door_sequence()
	
	# Look around with mouse (subtle movement)
	if event is InputEventMouseMotion and not _door_opening:
		var sensitivity = 0.001
		var rotation_change = Vector3(-event.relative.y * sensitivity, -event.relative.x * sensitivity, 0)
		player_camera.rotation += rotation_change
		
		# Limit look around (can't look too far from door)
		player_camera.rotation.x = clamp(player_camera.rotation.x, -0.3, 0.3)
		player_camera.rotation.y = clamp(player_camera.rotation.y, -0.5, 0.5)

# They will dream of this door. They will dream of going back.
# They never can.
