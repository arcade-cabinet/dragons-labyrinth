extends RefCounted
class_name DataRepo

var base_path := ""
var pools := {} # seed -> {lang:[lemmas]}
var norse_themes := {} # theme -> [lemmas]
var manifest := {} # File manifest with SHA256 and provenance
var pools_by_file := {} # Debug index: filename -> pools loaded from that file
var validation_errors := []

func load_all():
	"""Load all linguistic data with validation and manifest support."""
	validation_errors.clear()
	pools.clear()
	norse_themes.clear()
	pools_by_file.clear()
	
	# First load manifest if available
	_load_manifest()
	
	var dir = DirAccess.open(base_path)
	if dir == null:
		validation_errors.append("Cannot access data directory: " + base_path)
		return
	
	var loaded_files = []
	dir.list_dir_begin()
	while true:
		var f = dir.get_next()
		if f == "": break
		if dir.current_is_dir(): continue
		if f.ends_with(".json") and f != "manifest.json":
			var full_path = base_path + "/" + f
			var data = _read_json(full_path)
			
			if data is Dictionary:
				if data.has("magic_key") and data["magic_key"] == "norse_themes":
					# Handle Norse themes format
					if data.has("themes"):
						norse_themes = data["themes"]
						pools_by_file[f] = {"norse_themes": len(norse_themes)}
						loaded_files.append(f)
					else:
						validation_errors.append("Norse themes file missing 'themes' key: " + f)
				else:
					# Handle OMW pools format
					var pools_count = 0
					for seed in data.keys():
						if data[seed] is Dictionary:
							pools[seed] = data[seed]
							pools_count += 1
						else:
							validation_errors.append("Invalid pool format for seed '" + seed + "' in file: " + f)
					
					pools_by_file[f] = {"omw_pools": pools_count}
					loaded_files.append(f)
			else:
				validation_errors.append("Invalid JSON structure in file: " + f)
	
	dir.list_dir_end()
	
	print("DataRepo loaded: ", len(pools), " seeds, ", len(norse_themes), " Norse themes from ", len(loaded_files), " files")
	if len(validation_errors) > 0:
		print("DataRepo validation errors: ", validation_errors)

func _load_manifest():
	"""Load manifest.json if available."""
	var manifest_path = base_path + "/manifest.json"
	if FileAccess.file_exists(manifest_path):
		manifest = _read_json(manifest_path)
		if manifest.has("files"):
			print("DataRepo manifest loaded: ", len(manifest["files"]), " files tracked")
		else:
			validation_errors.append("Manifest file missing 'files' key")

func _read_json(path: String):
	"""Read and parse JSON file with error handling."""
	var f = FileAccess.open(path, FileAccess.READ)
	if f == null: 
		validation_errors.append("Cannot read file: " + path)
		return {}
	
	var txt = f.get_as_text()
	if txt.is_empty():
		validation_errors.append("Empty file: " + path)
		return {}
	
	var json = JSON.new()
	var parse_result = json.parse(txt)
	if parse_result != OK:
		validation_errors.append("JSON parse error in " + path + ": " + json.get_error_message())
		return {}
	
	return json.data

func has_seed(seed: String) -> bool:
	"""Check if a seed exists in the pools."""
	return pools.has(seed)

func langs_for(seed: String) -> Array:
	"""Get available languages for a seed."""
	if not pools.has(seed):
		return []
	return pools[seed].keys()

func get_pool(seed: String) -> Dictionary:
	"""Get the complete language pool for a seed."""
	return pools.get(seed, {})

func get_norse_theme(theme: String) -> Array:
	"""Get lemmas for a Norse theme."""
	return norse_themes.get(theme, [])

func get_norse_themes() -> Array:
	"""Get all available Norse theme names."""
	return norse_themes.keys()

func validate_integrity() -> Dictionary:
	"""Validate data integrity and return report."""
	var report = {
		"valid": true,
		"errors": validation_errors.duplicate(),
		"warnings": [],
		"statistics": {
			"total_seeds": len(pools),
			"total_norse_themes": len(norse_themes),
			"files_loaded": len(pools_by_file)
		}
	}
	
	# Check for empty pools
	for seed in pools:
		var pool = pools[seed]
		if pool.is_empty():
			report["warnings"].append("Empty pool for seed: " + seed)
		else:
			var total_lemmas = 0
			for lang in pool:
				var lemmas = pool[lang]
				if lemmas is Array:
					total_lemmas += len(lemmas)
				else:
					report["errors"].append("Invalid lemmas array for " + seed + "." + lang)
					report["valid"] = false
			
			if total_lemmas == 0:
				report["warnings"].append("No lemmas found for seed: " + seed)
	
	# Check Norse themes
	for theme in norse_themes:
		var lemmas = norse_themes[theme]
		if not lemmas is Array or len(lemmas) == 0:
			report["warnings"].append("Empty Norse theme: " + theme)
	
	# Manifest validation
	if manifest.has("files"):
		for filename in manifest["files"]:
			var file_info = manifest["files"][filename]
			if not file_info.has("sha256"):
				report["warnings"].append("Manifest entry missing SHA256: " + filename)
	
	if len(report["errors"]) > 0:
		report["valid"] = false
	
	return report

func get_debug_info() -> Dictionary:
	"""Get debugging information about loaded data."""
	return {
		"base_path": base_path,
		"pools_by_file": pools_by_file,
		"manifest": manifest,
		"validation_errors": validation_errors,
		"sample_seeds": pools.keys().slice(0, 5),
		"sample_norse_themes": norse_themes.keys().slice(0, 3)
	}
