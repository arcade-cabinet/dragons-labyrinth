extends Node

## HexTileData autoload - Manages hex grid data for Dragon's Labyrinth
##
## Integrates with:
## - godot-sqlite for database access
## - hexagon_tilemaplayer for cube coordinates
## - Python generator for world data population

var db: SQLite
var hex_data_cache = {}

func _ready():
	# Initialize database connection
	db = SQLite.new()
	db.path = "res://dragon_labyrinth.db"
	if not db.open_db():
		push_error("Failed to open hex tile database")
		return
	
	print("HexTileData initialized - ready for hex grid operations")

## Get hex tile data using cube coordinates
func get_hex_data(cube_pos: Vector3i) -> Dictionary:
	var coord_key = "%d,%d,%d" % [cube_pos.x, cube_pos.y, cube_pos.z]
	
	if hex_data_cache.has(coord_key):
		return hex_data_cache[coord_key]
	
	# Query database for hex data
	var query = "SELECT * FROM hex_tiles WHERE cube_x = ? AND cube_y = ? AND cube_z = ?"
	var result = db.query_with_bindings(query, [cube_pos.x, cube_pos.y, cube_pos.z])
	
	if result.size() > 0:
		var hex_data = result[0]
		hex_data_cache[coord_key] = hex_data
		return hex_data
	
	return {}

## Get all entities on a hex tile
func get_hex_entities(cube_pos: Vector3i) -> Array:
	var query = "SELECT * FROM entities WHERE hex_x = ? AND hex_y = ? AND hex_z = ?"
	var result = db.query_with_bindings(query, [cube_pos.x, cube_pos.y, cube_pos.z])
	return result

## Check if hex has settlement
func has_settlement(cube_pos: Vector3i) -> bool:
	var hex_data = get_hex_data(cube_pos)
	return hex_data.get("has_settlement", false)

## Check if hex has dungeon
func has_dungeon(cube_pos: Vector3i) -> bool:
	var hex_data = get_hex_data(cube_pos)
	return hex_data.get("has_dungeon", false)

## Get biome type for hex
func get_biome(cube_pos: Vector3i) -> String:
	var hex_data = get_hex_data(cube_pos)
	return hex_data.get("biome_type", "unknown")

## Calculate distance from base (0,0,0)
func distance_from_base(cube_pos: Vector3i) -> int:
	return (abs(cube_pos.x) + abs(cube_pos.y) + abs(cube_pos.z)) / 2

## Calculate dread level based on distance
func calculate_dread_level(cube_pos: Vector3i) -> int:
	var distance = distance_from_base(cube_pos)
	return min(4, distance / 20)  # 0-4 dread progression

## Update hex data in database
func update_hex_data(cube_pos: Vector3i, data: Dictionary):
	var coord_key = "%d,%d,%d" % [cube_pos.x, cube_pos.y, cube_pos.z]
	hex_data_cache[coord_key] = data
	
	# Update database
	var query = """
	UPDATE hex_tiles 
	SET biome_type = ?, corruption_level = ?, dread_level = ?
	WHERE cube_x = ? AND cube_y = ? AND cube_z = ?
	"""
	db.query_with_bindings(query, [
		data.get("biome_type", "unknown"),
		data.get("corruption_level", 0),
		data.get("dread_level", 0),
		cube_pos.x, cube_pos.y, cube_pos.z
	])
