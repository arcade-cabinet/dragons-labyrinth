extends Node

func _ready():
	var seeds = ["forest","night","wolf","winter","dragon"]
	var keys = ["meadows_act1","warfront_act2","void_act3"]
	for k in keys:
		print("=== Blend:", k, "===")
		for s in seeds:
			var name = Worldbuilder.make_name(s, k)
			print(s, " -> ", name)
