extends Node
class_name Worldbuilder

## LINGUISTIC WORLDBUILDING SYSTEM
##
## Integrates with DreadProgression to shift language blending as horror increases
## Names become more otherworldly and corrupted as dread progresses

const NameForge = preload("res://addons/worldbuilder/services/NameForge.gd")
const DataRepo  = preload("res://addons/worldbuilder/services/DataRepo.gd")

var repo: DataRepo
var forge: NameForge

# Language blend data loaded from external configuration
var blend_presets := {}
var regional_modifiers := {}
var language_info := {}
var blend_config_loaded := false

# References to other systems
var dread_progression: DreadProgression

func _ready():
	print("Worldbuilder initializing...")
	
	# Load blend presets configuration
	_load_blend_config()
	
	# Initialize linguistic data
	repo = DataRepo.new()
	repo.base_path = "res://addons/worldbuilder/data/lingua"
	repo.load_all()
	forge = NameForge.new(repo)
	
	# Connect to dread progression system
	dread_progression = get_node("/root/DreadProgression") if has_node("/root/DreadProgression") else null
	if dread_progression:
		dread_progression.emotional_stage_changed.connect(_on_emotional_stage_changed)
	
	print("Worldbuilder ready - linguistic blending active")

func _load_blend_config():
	"""Load blend presets from external JSON configuration."""
	var config_path = "res://addons/worldbuilder/data/blend_presets.json"
	
	if not FileAccess.file_exists(config_path):
		print("Worldbuilder: No blend_presets.json found, using fallback defaults")
		_setup_fallback_blends()
		return
	
	var file = FileAccess.open(config_path, FileAccess.READ)
	if file == null:
		print("Worldbuilder: Cannot read blend_presets.json, using fallback defaults")
		_setup_fallback_blends()
		return
	
	var json_text = file.get_as_text()
	file.close()
	
	var json = JSON.new()
	var parse_result = json.parse(json_text)
	if parse_result != OK:
		print("Worldbuilder: JSON parse error in blend_presets.json: ", json.get_error_message())
		_setup_fallback_blends()
		return
	
	var config = json.data
	if config.has("presets"):
		# Extract weight dictionaries from presets
		for preset_name in config["presets"]:
			var preset = config["presets"][preset_name]
			if preset.has("weights"):
				blend_presets[preset_name] = preset["weights"]
		
		print("Worldbuilder: Loaded ", len(blend_presets), " blend presets from configuration")
	
	if config.has("regional_modifiers"):
		regional_modifiers = config["regional_modifiers"]
		print("Worldbuilder: Loaded ", len(regional_modifiers), " regional modifiers")
	
	if config.has("language_info"):
		language_info = config["language_info"]
	
	blend_config_loaded = true

func _setup_fallback_blends():
	"""Setup hardcoded fallback blends if config loading fails."""
	blend_presets = {
		"peace": {"ang": 0.5, "cy": 0.4, "non": 0.1},
		"unease": {"ang": 0.4, "cy": 0.3, "non": 0.3},
		"dread": {"non": 0.5, "ang": 0.3, "cy": 0.2},
		"terror": {"non": 0.4, "ang": 0.2, "cy": 0.1, "ar": 0.15, "he": 0.15},
		"void": {"ar": 0.3, "he": 0.3, "non": 0.2, "ang": 0.1, "cy": 0.1}
	}
	
	regional_modifiers = {
		"meadows": {"modifiers": {"cy": 1.2}},
		"mountains": {"modifiers": {"non": 1.3}},
		"forests": {"modifiers": {"ang": 1.1}},
		"swamps": {"modifiers": {"ar": 1.2, "he": 1.2}},
		"ruins": {"modifiers": {"ar": 1.5, "he": 1.5}}
	}
	
	blend_config_loaded = true
	print("Worldbuilder: Using fallback blend configuration")

func make_name(seed_en: String, region_type: String = "", override_key: String = "") -> String:
	# Get appropriate blend based on current emotional stage and region
	var blend_key = override_key
	if blend_key == "":
		blend_key = get_current_blend_key(region_type)
	
	var blend = get_blend_for_key(blend_key, region_type)
	var generated_name = forge.forge(seed_en, blend)
	
	# Debug output to show language evolution
	if dread_progression:
		print("Generated name '", generated_name, "' from '", seed_en, "' - Stage: ", 
		      dread_progression.emotional_stage, " Region: ", region_type)
	
	return generated_name

func get_current_blend_key(region_type: String = "") -> String:
	if not dread_progression:
		return "peace"  # Default fallback
	
	return dread_progression.emotional_stage

func get_blend_for_key(stage_key: String, region_type: String = "") -> Dictionary:
	var base_blend = blend_presets.get(stage_key, {"ang": 0.5, "cy": 0.5})
	
	# Apply regional modifiers if specified
	if region_type != "" and region_type in regional_modifiers:
		var region_info = regional_modifiers[region_type]
		var modifiers = region_info.get("modifiers", {})
		var modified_blend = base_blend.duplicate()
		
		# Apply modifiers to strengthen certain languages for this region
		for lang in modifiers:
			if lang in modified_blend:
				modified_blend[lang] *= modifiers[lang]
		
		# Normalize to ensure weights still sum to 1.0
		var total = 0.0
		for weight in modified_blend.values():
			total += weight
		
		if total > 0:
			for lang in modified_blend:
				modified_blend[lang] /= total
		
		return modified_blend
	
	return base_blend

func _on_emotional_stage_changed(new_stage: String) -> void:
	print("=== LINGUISTIC SHIFT ===")
	print("Language blending changed to match emotional stage: ", new_stage)
	
	# Show example of how names change
	var example_names = ["village", "tavern", "forest", "companion", "weapon"]
	for seed in example_names:
		var old_blend = blend_presets.get("peace", {"ang": 0.5, "cy": 0.5})
		var new_blend = blend_presets.get(new_stage, old_blend)
		
		var old_name = forge.forge(seed, old_blend, "example") 
		var new_name = forge.forge(seed, new_blend, "example")
		
		if old_name != new_name:
			print("  ", seed, ": '", old_name, "' → '", new_name, "'")

# Generate names for quest system integration
func generate_npc_name(role: String, region_type: String = "") -> String:
	var seed = "person"
	match role:
		"tavern_keeper": seed = "keeper"
		"merchant": seed = "trader"  
		"guard": seed = "warrior"
		"priest": seed = "holy"
		"cultist": seed = "dark"
		_: seed = "person"
	
	return make_name(seed, region_type)

func generate_location_name(location_type: String, region_type: String = "") -> String:
	var seed = location_type  # tavern, mill, shrine, ruin, etc.
	return make_name(seed, region_type)

func generate_item_name(base_name: String, corruption_level: float = 0.0) -> String:
	# Items become more otherworldly as corruption increases
	var region_type = ""
	if corruption_level > 0.7:
		region_type = "ruins"  # Forces otherworldly names
	elif corruption_level > 0.4:
		region_type = "swamps"  # Moderately corrupted
	
	return make_name(base_name, region_type)

# Integration with companion psychology system
func generate_companion_name(backstory_hint: String = "", trauma_level: float = 0.0) -> String:
	# Companions with higher trauma get more Norse/harsh names reflecting their state
	var seed = "friend"
	var region_override = ""
	
	if trauma_level > 0.5:
		region_override = "mountains"  # Harsh, Nordic feel
	
	return make_name(seed, region_override)

# Get language blend for current world state
func get_current_language_profile() -> Dictionary:
	var stage = "peace"
	if dread_progression:
		stage = dread_progression.emotional_stage
	
	return {
		"stage": stage,
		"blend": blend_presets.get(stage, {}),
		"corruption_influence": _calculate_corruption_influence()
	}

func _calculate_corruption_influence() -> float:
	if not dread_progression:
		return 0.0
	
	return float(dread_progression.current_dread_level) / 4.0  # 0.0 to 1.0
