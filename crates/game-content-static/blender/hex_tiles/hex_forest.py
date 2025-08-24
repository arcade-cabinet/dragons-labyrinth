# Hex forest tile: generates a hex tile with a central low‑poly tree.
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
    bsdf.inputs['Roughness'].default_value = 0.9
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
    master = bpy.data.objects.new("HexForestTile", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Soil", (0.1, 0.07, 0.05, 1.0))
    mat_grass = new_mat("Grass", (0.08, 0.3, 0.08, 1.0))
    mat_trunk = new_mat("Trunk", (0.15, 0.08, 0.05, 1.0))
    mat_leaves = new_mat("Leaves", (0.05, 0.25, 0.05, 1.0))
    # base soil layer
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.5, 0.5, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # grass layer
    top = add('cylinder', vertices=6, location=(0, 0, 0.1))
    top.scale = (0.5, 0.5, 0.05)
    top.data.materials.append(mat_grass)
    shade_flat(top)
    top.parent = master
    # tree trunk
    trunk = add('cylinder', vertices=12, location=(0, 0, 0.15))
    trunk.scale = (0.05, 0.05, 0.2)
    trunk.data.materials.append(mat_trunk)
    shade_flat(trunk)
    trunk.parent = master
    # canopy
    canopy = add('cone', vertices=16, location=(0, 0, 0.32))
    canopy.scale = (0.2, 0.2, 0.25)
    canopy.data.materials.append(mat_leaves)
    shade_flat(canopy)
    canopy.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "hex_forest.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
tile = build_tile()
export_glb(tile)