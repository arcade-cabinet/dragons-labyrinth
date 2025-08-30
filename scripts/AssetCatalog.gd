extends Node

## AssetCatalog autoload - Manages game assets and sprite loading
##
## Integrates with:
## - godot-sqlite for asset metadata
## - pandora for entity-based asset management
## - OpenAI-generated assets from Python generator

var db: SQLite
var asset_cache: Dictionary = {}
var sprite_sheets: Dictionary = {}

func _ready():
	# Initialize database connection
	db = SQLite.new()
	db.path = "res://dragon_labyrinth.db"
	if not db.open_db():
		push_error("Failed to open asset catalog database")
		return
	
	print("AssetCatalog initialized - asset management active")

## Load sprite for entity
func get_entity_sprite(entity_id: String) -> Texture2D:
	if entity_id in asset_cache:
		return asset_cache[entity_id]
	
	# Query database for asset path
	var query = "SELECT asset_path FROM assets WHERE entity_id = ?"
	var result = db.query_with_bindings(query, [entity_id])
	
	if result.size() > 0:
		var asset_path = result[0].get("asset_path", "")
		if asset_path != "":
			var texture = load(asset_path) as Texture2D
			if texture:
				asset_cache[entity_id] = texture
				return texture
	
	# Return default sprite if not found
	return load("res://art/character-models-base.png")

## Get hex biome texture
func get_biome_texture(biome_type: String) -> Texture2D:
	var cache_key = "biome_" + biome_type
	
	if cache_key in asset_cache:
		return asset_cache[cache_key]
	
	# Check for biome-specific texture
	var texture_path = "res://art/biomes/%s.png" % biome_type.to_lower()
	if ResourceLoader.exists(texture_path):
		var texture = load(texture_path) as Texture2D
		asset_cache[cache_key] = texture
		return texture
	
	# Fallback to default biome texture
	return load("res://art/biomes.png")

## Get companion sprite with trauma overlay
func get_companion_sprite(companion_id: String, trauma_level: float = 0.0) -> Texture2D:
	var base_sprite = get_entity_sprite(companion_id)
	
	# Apply trauma visual effects if needed
	if trauma_level > 0.5:
		# Load trauma overlay texture
		var overlay_path = "res://art/trauma_overlay.png"
		if ResourceLoader.exists(overlay_path):
			# TODO: Composite base sprite with trauma overlay
			pass
	
	return base_sprite

## Load sprite sheet for animations
func get_sprite_sheet(sheet_id: String) -> Dictionary:
	if sheet_id in sprite_sheets:
		return sprite_sheets[sheet_id]
	
	var query = "SELECT * FROM sprite_sheets WHERE sheet_id = ?"
	var result = db.query_with_bindings(query, [sheet_id])
	
	if result.size() > 0:
		var sheet_data = result[0]
		sprite_sheets[sheet_id] = sheet_data
		return sheet_data
	
	return {}

## Get asset metadata
func get_asset_metadata(asset_id: String) -> Dictionary:
	var query = "SELECT * FROM asset_metadata WHERE asset_id = ?"
	var result = db.query_with_bindings(query, [asset_id])
	
	if result.size() > 0:
		return result[0]
	
	return {}

## Preload assets for a region
func preload_region_assets(region_name: String):
	var query = "SELECT DISTINCT asset_path FROM assets WHERE region_context = ?"
	var result = db.query_with_bindings(query, [region_name])
	
	for row in result:
		var asset_path = row.get("asset_path", "")
		if asset_path != "" and not asset_path in asset_cache:
			var texture = load(asset_path) as Texture2D
			if texture:
				asset_cache[asset_path] = texture

## Clear unused assets from cache
func cleanup_asset_cache():
	# Keep only recently used assets
	var current_time = Time.get_unix_time_from_system()
	# Simple cleanup - in a real implementation you'd track usage timestamps
	if asset_cache.size() > 100:
		asset_cache.clear()
		print("Asset cache cleared - memory optimization")
