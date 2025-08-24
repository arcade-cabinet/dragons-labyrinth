# Gear: Shield – creates a round shield with a simple design.
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
    bsdf.inputs['Metallic'].default_value = 0.3
    bsdf.inputs['Roughness'].default_value = 0.6
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_shield():
    master = bpy.data.objects.new("Shield", None)
    bpy.context.collection.objects.link(master)
    mat_body = new_mat("ShieldBody", (0.3, 0.25, 0.2, 1.0))
    mat_rim = new_mat("ShieldRim", (0.5, 0.5, 0.55, 1.0))
    # main disc
    disc = add('cylinder', vertices=32, location=(0, 0, 0))
    disc.scale = (0.3, 0.3, 0.04)
    disc.data.materials.append(mat_body)
    shade_flat(disc)
    disc.parent = master
    # rim
    rim = add('torus', location=(0, 0, 0.02))
    rim.scale = (0.35, 0.35, 0.02)
    rim.data.materials.append(mat_rim)
    shade_flat(rim)
    rim.parent = master
    # handle on back
    handle = add('cylinder', vertices=12, location=(0, -0.2, 0))
    handle.scale = (0.04, 0.1, 0.02)
    handle.rotation_euler = (math.radians(90), 0, 0)
    handle.data.materials.append(mat_rim)
    shade_flat(handle)
    handle.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "shield.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
shield = build_shield()
export_glb(shield)