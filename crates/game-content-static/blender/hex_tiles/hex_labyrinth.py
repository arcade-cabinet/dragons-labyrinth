# Hex labyrinth tile: generates a dark tile with subtle rune decorations for the labyrinth.
import bpy
import os
import math


def reset_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    for m in bpy.data.meshes:
        if m.users == 0:
            bpy.data.meshes.remove(m)
    for mat in bpy.data.materials:
        if mat.users == 0:
            bpy.data.materials.remove(mat)


def new_mat(name, rgba):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    bsdf.inputs['Base Color'].default_value = rgba
    bsdf.inputs['Metallic'].default_value = 0.0
    bsdf.inputs['Roughness'].default_value = 0.8
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_tile():
    master = bpy.data.objects.new("HexLabyrinthTile", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Stone", (0.05, 0.05, 0.07, 1.0))
    mat_rune = new_mat("Rune", (0.3, 0.1, 0.4, 1.0))
    # base dark slab
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.5, 0.5, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # top layer
    top = add('cylinder', vertices=6, location=(0, 0, 0.1))
    top.scale = (0.5, 0.5, 0.05)
    top.data.materials.append(mat_base)
    shade_flat(top)
    top.parent = master
    # rune decorations: triangular prisms at corners
    for ang in [0, 60, 120, 180, 240, 300]:
        rune = add('cone', vertices=3, location=(0.42 * math.cos(math.radians(ang)), 0.42 * math.sin(math.radians(ang)), 0.16))
        rune.scale = (0.03, 0.03, 0.05)
        rune.rotation_euler = (math.radians(90), 0, math.radians(ang))
        rune.data.materials.append(mat_rune)
        shade_flat(rune)
        rune.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "hex_labyrinth.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
tile = build_tile()
export_glb(tile)