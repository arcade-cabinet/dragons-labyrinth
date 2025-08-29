extends "res://addons/gd-plug/plug.gd"

func _plugging():
	# Dragon's Labyrinth Horror RPG Plugin Dependencies
	# All plugins required for our infinite hex world horror RPG
	
	# CORE REQUIRED PLUGINS - Essential for our horror RPG mechanics
	
	# Database integration - CRITICAL for our 50+ table system (GDExtension)
	plug("2shady4u/godot-sqlite", {"include": ["demo/addons/godot-sqlite/"], "on_updated": "_install_godot_sqlite"})
	
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
	
	# QUEST SYSTEMS - Multiple options for our narrative progression
	
	# Quest Manager - Lightweight quest system
	plug("Rubonnek/quest-manager", {"dev": true})
	
	# Quest System - More comprehensive quest framework
	plug("shomykohai/quest-system", {"dev": true})
	
	# Questify - Another quest system option
	plug("TheWalruzz/godot-questify", {"dev": true})

func _install_godot_sqlite(plugin):
	print("Installing godot-sqlite GDExtension...")
	
	# Copy the .gdextension file to root level
	var source_dir = "res://addons/godot-sqlite/"
	var dest_dir = "res://"
	
	# Copy necessary GDExtension files to root level
	var dir_access = DirAccess.open(source_dir)
	if dir_access:
		# Copy .gdextension file
		dir_access.copy(source_dir + "gdsqlite.gdextension", dest_dir + "gdsqlite.gdextension")
		
		# Copy bin directory
		var bin_source = source_dir + "bin/"
		var bin_dest = dest_dir + "bin/"
		DirAccess.open("res://").make_dir("bin")
		_copy_dir_recursive(bin_source, bin_dest)
		
		print("godot-sqlite GDExtension installed successfully!")
	else:
		print("Error: Could not access godot-sqlite source directory")

func _copy_dir_recursive(source: String, dest: String):
	var dir_access = DirAccess.open(source)
	if dir_access:
		DirAccess.open("res://").make_dir(dest)
		dir_access.list_dir_begin()
		var file_name = dir_access.get_next()
		while file_name != "":
			var source_path = source + file_name
			var dest_path = dest + file_name
			if dir_access.current_is_dir():
				_copy_dir_recursive(source_path + "/", dest_path + "/")
			else:
				dir_access.copy(source_path, dest_path)
			file_name = dir_access.get_next()
		dir_access.list_dir_end()
