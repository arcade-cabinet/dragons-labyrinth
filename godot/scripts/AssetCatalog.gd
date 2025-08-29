@tool
extends Node
class_name AssetCatalog

# ---- Configure these to match your project layout --------------------------

# Where to store metadata DB inside the project / user dir.
# Use res:// if you bake a seed DB into your PCK; use user:// for a writable catalog.
const DB_PATH: String = "user://library.db"  # Asset library database

# Root assets folder you want to scan (editor-only usage).
const ASSETS_ROOT: String = "res://assets"

# Auto-detect FTS5 support at runtime
var ENABLE_FTS5 := false

# ---- SQLite handle ----------------------------------------------------------
var db := null

# ---- Simple enums mirroring your Python types -------------------------------
enum AssetFileType {
	MODEL_3D = 1,
	AUDIO = 2,
	FONT = 3,
	IMAGE_2D = 4,
	SCENE = 5,
	DATA = 6,
	UNKNOWN = 99
}

enum AssetOrigin {
	UNKNOWN = 0,
	KENNEY = 1,
	QUATERNIUS = 2,
	GENERATED = 3,
	CC0 = 4
}

# ---- Lifecycle --------------------------------------------------------------

func _ready() -> void:
	# Lazy open so you can construct in editor scripts without side effects.
	pass

func open() -> void:
	if db != null:
		return
	db = SQLite.new()
	if not db.open(DB_PATH):
		push_error("Failed to open DB at %s" % DB_PATH)
		return
	_configure_sqlite()
	_ensure_schema()

func close() -> void:
	if db:
		db.close()
		db = null

# ---- SQLite setup -----------------------------------------------------------

func _configure_sqlite() -> void:
	# Bidirectional-safe pragmas for generator/editor use.
	db.execute("PRAGMA journal_mode=WAL;")
	db.execute("PRAGMA synchronous=NORMAL;")
	db.execute("PRAGMA foreign_keys=ON;")
	# Store TEXT as provided (affinity remains SQLite's), times in julianday() floats.
	
	# Test for FTS5 support at runtime
	_test_fts5_support()

func _test_fts5_support() -> void:
	"""Test if SQLite build includes FTS5 support."""
	var test_result = db.execute("CREATE VIRTUAL TABLE IF NOT EXISTS _fts_test USING fts5(content);")
	if test_result:
		ENABLE_FTS5 = true
		db.execute("DROP TABLE IF EXISTS _fts_test;")
		print("✅ FTS5 support detected - enabling full-text search")
	else:
		ENABLE_FTS5 = false  
		print("ℹ️  FTS5 not available - using LIKE-based search")

func _ensure_schema() -> void:
	# Core assets table: same as Python, minus vectors
	db.execute("""
	CREATE TABLE IF NOT EXISTS assets (
		id INTEGER PRIMARY KEY,
		asset_id TEXT UNIQUE NOT NULL,
		path TEXT NOT NULL,
		filename TEXT NOT NULL,
		file_type INTEGER NOT NULL,
		category TEXT NOT NULL,
		subcategory TEXT,
		origin INTEGER NOT NULL,
		attribution TEXT,
		keywords TEXT NOT NULL,
		description TEXT NOT NULL,
		metadata_json TEXT,
		file_size INTEGER,
		created_at REAL DEFAULT (julianday('now')),
		updated_at REAL DEFAULT (julianday('now'))
	);
	""")

	# Helpful indexes (same spirit as your Python init)
	db.execute("CREATE INDEX IF NOT EXISTS idx_assets_category ON assets(category);")
	db.execute("CREATE INDEX IF NOT EXISTS idx_assets_file_type ON assets(file_type);")
	db.execute("CREATE INDEX IF NOT EXISTS idx_assets_origin ON assets(origin);")
	db.execute("CREATE INDEX IF NOT EXISTS idx_assets_path ON assets(path);")
	db.execute("CREATE INDEX IF NOT EXISTS idx_assets_filename ON assets(filename);")

	# Update timestamps automatically on UPDATEs from either side
	db.execute("""
	CREATE TRIGGER IF NOT EXISTS trg_assets_updated
	AFTER UPDATE ON assets
	BEGIN
		UPDATE assets SET updated_at = julianday('now') WHERE id = NEW.id;
	END;
	""")

	# Optional FTS mirror (kept off by default for max portability)
	if ENABLE_FTS5:
		db.execute("""
		CREATE VIRTUAL TABLE IF NOT EXISTS assets_fts USING fts5(
			path, filename, category, subcategory, origin, keywords, description,
			content='assets', content_rowid='id'
		);
		""")
		# Keep FTS in sync
		db.execute("""
		CREATE TRIGGER IF NOT EXISTS trg_assets_ai AFTER INSERT ON assets BEGIN
			INSERT INTO assets_fts(rowid, path, filename, category, subcategory, origin, keywords, description)
			VALUES (new.id, new.path, new.filename, new.category, new.subcategory, CAST(new.origin AS TEXT), new.keywords, new.description);
		END;
		""")
		db.execute("""
		CREATE TRIGGER IF NOT EXISTS trg_assets_ad AFTER DELETE ON assets BEGIN
			INSERT INTO assets_fts(assets_fts, rowid, path) VALUES ('delete', old.id, old.path);
		END;
		""")
		db.execute("""
		CREATE TRIGGER IF NOT EXISTS trg_assets_au AFTER UPDATE ON assets BEGIN
			INSERT INTO assets_fts(assets_fts, rowid, path) VALUES ('delete', old.id, old.path);
			INSERT INTO assets_fts(rowid, path, filename, category, subcategory, origin, keywords, description)
			VALUES (new.id, new.path, new.filename, new.category, new.subcategory, CAST(new.origin AS TEXT), new.keywords, new.description);
		END;
		""")

# ---- Hashing (stable asset_id based on relative path) -----------------------

static func _sha256_hex_16(s: String) -> String:
	var ctx := HashingContext.new()
	ctx.start(HashingContext.HASH_SHA256)
	ctx.update(s.to_utf8_buffer())
	var full := ctx.finish()
	# first 16 hex chars for compatibility with your Python code
	return full.hex_encode().substr(0, 16)

# ---- Heuristics (ports of your Python helpers) ------------------------------

static func _detect_file_type_by_ext(ext: String) -> int:
	var e := ext.to_lower()
	if e in [".fbx", ".glb", ".gltf", ".obj"]:
		return AssetFileType.MODEL_3D
	elif e in [".mp3", ".wav", ".ogg", ".flac"]:
		return AssetFileType.AUDIO
	elif e in [".ttf", ".otf", ".woff", ".woff2"]:
		return AssetFileType.FONT
	elif e in [".png", ".jpg", ".jpeg", ".gif", ".bmp"]:
		return AssetFileType.IMAGE_2D
	elif e in [".tscn", ".scn"]:
		return AssetFileType.SCENE
	elif e in [".json", ".yaml", ".yml", ".toml"]:
		return AssetFileType.DATA
	return AssetFileType.UNKNOWN

static func _detect_origin(filename: String) -> Dictionary:
	var lower := filename.to_lower()
	if lower.begins_with("k_"):
		return {"origin": AssetOrigin.KENNEY, "attribution": "https://kenney.itch.io/kenney-game-assets"}
	elif lower.begins_with("q_"):
		return {"origin": AssetOrigin.QUATERNIUS, "attribution": "https://quaternius.com/"}
	elif lower.findn("generated") >= 0 or lower.findn("ai_") == 0:
		return {"origin": AssetOrigin.GENERATED, "attribution": "AI Generated"}
	elif lower.findn("cc0") >= 0:
		return {"origin": AssetOrigin.CC0, "attribution": "CC0 Licensed"}
	return {"origin": AssetOrigin.UNKNOWN, "attribution": ""}

static func _parse_descriptive_name(stem: String) -> Dictionary:
	var s := stem
	var lower := s.to_lower()
	if lower.begins_with("k_") or lower.begins_with("q_"):
		s = s.substr(2)
	var parts: PackedStringArray = PackedStringArray([])
	for p in s.split("_"):
		if p != "":
			parts.append(p)
	if parts.is_empty():
		return {"category": "unknown", "description": stem, "keywords": [stem]}
	var category := parts[0].to_lower()
	var description := " ".join(parts).capitalize() # Title-ish
	var keywords := []
	for p in parts:
		keywords.append(p.to_lower())

	var low_stem := s.to_lower()
	if low_stem.findn("house") >= 0 or low_stem.findn("building") >= 0 or low_stem.findn("structure") >= 0:
		keywords.append("architecture")
		keywords.append("building")
	if low_stem.findn("tree") >= 0 or low_stem.findn("plant") >= 0 or low_stem.findn("grass") >= 0 or low_stem.findn("flower") >= 0:
		keywords.append("nature")
		keywords.append("vegetation")
	if low_stem.findn("rock") >= 0 or low_stem.findn("stone") >= 0 or low_stem.findn("mountain") >= 0:
		keywords.append("terrain")
		keywords.append("geology")

	return {"category": category, "description": description, "keywords": keywords}

# ---- Public API (mirrors Python names where sensible) -----------------------

func add_asset(abs_path: String, base_path: String = "") -> String:
	# abs_path like "res://assets/foo/bar.glb"
	open()
	if not FileAccess.file_exists(abs_path):
		push_error("Asset not found: %s" % abs_path)
		return ""
	var filename := abs_path.get_file()
	var ext := "." + abs_path.get_extension().to_lower()

	var rel_path := abs_path
	if base_path != "":
		var prefix := base_path.rstrip("/")
		rel_path = abs_path.replace(prefix + "/", "")
	# asset_id = sha256(rel_path)[:16]
	var asset_id := _sha256_hex_16(rel_path)

	var t := _detect_file_type_by_ext(ext)
	var org := _detect_origin(filename)
	var parsed := _parse_descriptive_name(filename.get_basename())

	var metadata := {
		"dimensions": null,
		"duration": null,
		"format_specific": {}
	}

	# File size (safe open/close)
	var size := 0
	var fa := FileAccess.open(abs_path, FileAccess.READ)
	if fa:
		size = fa.get_length()
		fa.close()

	# INSERT OR REPLACE keeps `id` stable by UNIQUE on asset_id
	var ok := db.execute("""
	INSERT OR REPLACE INTO assets (
		asset_id, path, filename, file_type, category, subcategory, origin,
		attribution, keywords, description, metadata_json, file_size
	) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
	""", [
		asset_id, rel_path, filename, t,
		parsed.category, null, org.origin,
		org.attribution, " ".join(parsed.keywords), parsed.description,
		JSON.stringify(metadata), size
	])
	if not ok:
		push_error("DB insert failed for %s" % abs_path)
	return asset_id

func scan_assets_directory(root_dir: String = ASSETS_ROOT) -> Dictionary:
	open()
	var stats := {"total_files": 0, "indexed": 0, "skipped": 0, "errors": 0}
	var exts := {
		".fbx": true, ".glb": true, ".gltf": true, ".obj": true,
		".mp3": true, ".wav": true, ".ogg": true, ".flac": true,
		".ttf": true, ".otf": true, ".woff": true, ".woff2": true,
		".png": true, ".jpg": true, ".jpeg": true, ".gif": true, ".bmp": true,
		".tscn": true, ".scn": true,
		".json": true, ".yaml": true, ".yml": true, ".toml": true
	}

	# simple DFS stack
	var stack := [root_dir]
	while stack.size() > 0:
		var dir_path := stack.pop_back()
		var dir := DirAccess.open(dir_path)
		if dir == null:
			continue
		dir.list_dir_begin()
		while true:
			var name := dir.get_next()
			if name == "":
				break
			if name.begins_with("."):
				continue
			var full := dir_path.path_join(name)
			if dir.current_is_dir():
				stack.push_back(full)
				continue
			stats.total_files += 1
			var ext := "." + full.get_extension().to_lower()
			if not exts.has(ext):
				stats.skipped += 1
				continue
			var id := ""
			var ok := true
			# Wrap per-file to keep errors isolated
			id = add_asset(full, root_dir)
			if id == "":
				stats.errors += 1
				ok = false
			if ok:
				stats.indexed += 1
		dir.list_dir_end()
	return stats

# Keyword/field search (non-vector)

func search_keywords(query_terms: PackedStringArray, limit: int = 20, category_filter: String = "") -> Array:
	open()
	if ENABLE_FTS5:
		var q := " ".join(query_terms)
		var sql := """
			SELECT a.asset_id, a.path, a.filename, a.category, a.description, a.keywords, a.attribution
			FROM assets_fts f
			JOIN assets a ON a.id = f.rowid
			WHERE assets_fts MATCH ?
		"""
		var params := [q]
		if category_filter != "":
			sql += " AND a.category = ?"
			params.append(category_filter)
		sql += " ORDER BY a.updated_at DESC LIMIT ?"
		params.append(limit)
		return db.select(sql, params)
	else:
		# LIKE AND chain – good enough for modest corpora
		var clauses := []
		var params := []
		for t in query_terms:
			clauses.append("keywords LIKE '%' || ? || '%'")
			params.append(t)
		var sql := """
			SELECT asset_id, path, filename, category, description, keywords, attribution
			FROM assets
		"""
		if clauses.size() > 0 or category_filter != "":
			sql += " WHERE "
			var conds := []
			if category_filter != "":
				conds.append("category = ?")
				params.insert(0, category_filter)
			if clauses.size() > 0:
				conds.append("(" + " AND ".join(clauses) + ")")
			sql += " AND ".join(conds)
		sql += " ORDER BY updated_at DESC LIMIT ?"
		params.append(limit)
		return db.select(sql, params)

func get_categories() -> PackedStringArray:
	open()
	var rows := db.select("SELECT DISTINCT category FROM assets ORDER BY category;", [])
	var out := PackedStringArray()
	for r in rows:
		out.append(str(r["category"]))
	return out

func get_stats() -> Dictionary:
	open()
	var stats := {}
	var row := db.select("SELECT COUNT(*) AS c FROM assets;", [])
	stats["total_assets"] = (row.size() > 0) ? int(row[0]["c"]) : 0

	var by_type := db.select("SELECT file_type, COUNT(*) AS c FROM assets GROUP BY file_type;", [])
	var by_type_map := {}
	for r in by_type:
		var k := int(r["file_type"])
		# Map enum -> name for readability
		var name := ""
		match k:
			AssetFileType.MODEL_3D: name = "MODEL_3D"
			AssetFileType.AUDIO: name = "AUDIO"
			AssetFileType.FONT: name = "FONT"
			AssetFileType.IMAGE_2D: name = "IMAGE_2D"
			AssetFileType.SCENE: name = "SCENE"
			AssetFileType.DATA: name = "DATA"
			_: name = "UNKNOWN"
		by_type_map[name] = int(r["c"])
	stats["by_file_type"] = by_type_map

	var by_origin := db.select("SELECT origin, COUNT(*) AS c FROM assets GROUP BY origin;", [])
	var by_origin_map := {}
	for r in by_origin:
		var o := int(r["origin"])
		var oname := ""
		match o:
			AssetOrigin.KENNEY: oname = "KENNEY"
			AssetOrigin.QUATERNIUS: oname = "QUATERNIUS"
			AssetOrigin.GENERATED: oname = "GENERATED"
			AssetOrigin.CC0: oname = "CC0"
			_: oname = "UNKNOWN"
		by_origin_map[oname] = int(r["c"])
	stats["by_origin"] = by_origin_map

	var by_cat := db.select("""
		SELECT category, COUNT(*) AS c
		FROM assets
		GROUP BY category
		ORDER BY COUNT(*) DESC;
	""", [])
	var cat_map := {}
	for r in by_cat:
		cat_map[str(r["category"])] = int(r["c"])
	stats["by_category"] = cat_map

	return stats

# Convenience: rebuild in one go (editor tool)
func rebuild_asset_database() -> Dictionary:
	var t0 := Time.get_unix_time_from_system()
	var stats := scan_assets_directory(ASSETS_ROOT)
	var scan_time := Time.get_unix_time_from_system() - t0
	stats["scan_time"] = float(scan_time)
	stats["database_path"] = DB_PATH
	return stats

# Helper: Python-compat output shape for your current tools (no distance now)
func search_assets_keywords(query: String, limit: int = 8, category: String = "") -> Array:
	var terms := PackedStringArray(query.strip_edges().split(" ", false))
	var rows := search_keywords(terms, limit, category)
	var out := []
	for r in rows:
		var fn := str(r["filename"])
		var stem := fn.get_basename()
		var ext := "." + fn.get_extension()
		out.append({
			"path": r["path"],
			"category": r["category"],
			"subcategory": null,
			"filename": fn,
			"stem": stem,
			"ext": ext,
			"full_path": "%s/%s" % [r["category"], r["path"]],
			"content": str(r["keywords"]).split(" ")
		})
	return out
