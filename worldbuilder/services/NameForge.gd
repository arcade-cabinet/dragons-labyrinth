extends RefCounted
class_name NameForge

var repo
var rng: RandomNumberGenerator
var deterministic_mode: bool = true
var latinize_output: bool = false

func _init(r): 
	repo = r
	rng = RandomNumberGenerator.new()

static func _latinize(s: String) -> String:
	return s.to_lower()

static func _polish_phonotactics(s: String) -> String:
	"""Apply phonotactic rules for more natural sounding names."""
	if s.is_empty():
		return s
	
	# Collapse duplicate boundary consonants (e.g., "kallkeld" -> "kalkeld")
	var result = s
	var consonants = "bcdfghjklmnpqrstvwxyz"
	for i in range(result.length() - 1):
		var current = result[i].to_lower()
		var next = result[i + 1].to_lower()
		if current == next and consonants.find(current) != -1:
			result = result.substr(0, i) + result.substr(i + 1)
			i -= 1  # Check this position again
	
	# Compress double vowels in the middle (e.g., "kaarl" -> "karl")
	var vowels = "aeiou"
	for i in range(1, result.length() - 2):  # Skip first and last positions
		var current = result[i].to_lower()
		var next = result[i + 1].to_lower()
		if current == next and vowels.find(current) != -1:
			result = result.substr(0, i) + result.substr(i + 1)
			i -= 1
	
	return result

static func _compound(a: String, b: String, use_hyphen: bool = false) -> String:
	var la = _latinize(a)
	var lb = _latinize(b)
	
	if la.is_empty() or lb.is_empty():
		return (la + lb).capitalize()
	
	# Optional hyphen joiner for very long combinations
	if use_hyphen and (la.length() + lb.length()) > 10:
		return (la + "-" + lb).capitalize()
	
	# Standard compound logic
	if la[la.length()-1] == lb[0]:
		var compound = la + lb.substr(1, lb.length())
	else:
		var compound = la + lb
	
	# Apply phonotactic polish
	compound = _polish_phonotactics(compound)
	return compound.capitalize()

func _pick_deterministic(arr: Array, extra_seed: String = "") -> String:
	"""Deterministic array selection using seeded RNG."""
	if arr.is_empty(): 
		return ""
	
	# Use extra seed for more entropy while maintaining determinism
	if not extra_seed.is_empty():
		var temp_seed = rng.seed + extra_seed.hash()
		rng.seed = temp_seed
	
	var result = arr[rng.randi() % arr.size()]
	return result

func forge(seed_en: String, blend: Dictionary, region_key: String = "", seed_override: int = -1) -> String:
	"""
	Generate a name using deterministic RNG.
	
	Args:
		seed_en: English seed word
		blend: Language blend weights  
		region_key: Regional context for deterministic seeding
		seed_override: Override RNG seed for testing
	"""
	# Set deterministic seed from inputs
	if seed_override != -1:
		rng.seed = seed_override
	elif deterministic_mode:
		var deterministic_seed = (seed_en + region_key).hash()
		rng.seed = deterministic_seed
	
	var pool = repo.pools.get(seed_en, {})
	if pool.is_empty(): 
		return seed_en.capitalize()

	# Build weighted language selection array
	var langs = []
	for k in blend.keys():
		var weight = blend[k]
		var count = max(1, int(weight * 10.0))
		for i in range(count):
			langs.append(k)
	
	if langs.is_empty(): 
		langs = pool.keys()

	# Deterministic language selection
	var langA = langs[rng.randi() % langs.size()]
	var langB = langs[rng.randi() % langs.size()]

	# Deterministic lemma selection
	var la = _pick_deterministic(pool.get(langA, []), "A")
	var lb = _pick_deterministic(pool.get(langB, []), "B")

	# Generate result
	var result = ""
	if la.is_empty() and lb.is_empty(): 
		result = seed_en.capitalize()
	elif not la.is_empty() and not lb.is_empty(): 
		result = _compound(la, lb, false)
	else:
		result = _latinize(la if not la.is_empty() else lb).capitalize()
	
	# Apply latinization if configured
	if latinize_output:
		result = _convert_to_ascii(result)
	
	return result

static func _convert_to_ascii(text: String) -> String:
	"""Convert non-ASCII characters to ASCII equivalents."""
	var ascii_map = {
		# Arabic transliterations
		"ا": "a", "أ": "a", "إ": "i", "آ": "aa", "ب": "b", "ت": "t", "ث": "th", 
		"ج": "j", "ح": "h", "خ": "kh", "د": "d", "ذ": "dh", "ر": "r", "ز": "z", 
		"س": "s", "ش": "sh", "ص": "s", "ض": "d", "ط": "t", "ظ": "z", "ع": "a", 
		"غ": "gh", "ف": "f", "ق": "q", "ك": "k", "ل": "l", "م": "m", "ن": "n", 
		"ه": "h", "و": "w", "ي": "y", "ء": "'", "ى": "a", "ة": "a",
		
		# Hebrew transliterations
		"א": "a", "ב": "b", "ג": "g", "ד": "d", "ה": "h", "ו": "v", "ז": "z", 
		"ח": "kh", "ט": "t", "י": "y", "כ": "k", "ך": "k", "ל": "l", "מ": "m", 
		"ם": "m", "נ": "n", "ן": "n", "ס": "s", "ע": "a", "פ": "p", "ף": "p", 
		"צ": "ts", "ץ": "ts", "ק": "k", "ר": "r", "ש": "sh", "ת": "t", "־": "-",
		
		# Common European diacritics
		"à": "a", "á": "a", "â": "a", "ã": "a", "ä": "ae", "å": "a", "æ": "ae",
		"ç": "c", "è": "e", "é": "e", "ê": "e", "ë": "e", "ì": "i", "í": "i", 
		"î": "i", "ï": "i", "ð": "d", "ñ": "n", "ò": "o", "ó": "o", "ô": "o", 
		"õ": "o", "ö": "oe", "ø": "o", "ù": "u", "ú": "u", "û": "u", "ü": "ue", 
		"ý": "y", "þ": "th", "ÿ": "y", "ß": "ss"
	}
	
	var result = ""
	for i in range(text.length()):
		var char = text[i]
		if ascii_map.has(char):
			result += ascii_map[char]
		else:
			result += char
	
	return result

func set_deterministic_mode(enabled: bool) -> void:
	"""Enable or disable deterministic name generation."""
	deterministic_mode = enabled

func set_latinize_output(enabled: bool) -> void:
	"""Enable or disable ASCII-only output."""
	latinize_output = enabled
