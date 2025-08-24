# Dragon fragment: builds a stylised serpent head for the dragon.
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
    bsdf.inputs['Metallic'].default_value = 0.4
    bsdf.inputs['Roughness'].default_value = 0.5
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_head():
    master = bpy.data.objects.new("DragonHead", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_skin = new_mat("DragonSkin", (0.1, 0.05, 0.12, 1.0))
    mat_horn = new_mat("Horn", (0.25, 0.2, 0.18, 1.0))
    mat_eye = new_mat("Eye", (0.8, 0.1, 0.1, 1.0))
    # head base (elongated sphere)
    bpy.ops.mesh.primitive_uv_sphere_add(segments=24, ring_count=12, location=(0, 0, 0))
    head = bpy.context.active_object
    head.scale = (0.4, 0.3, 0.25)
    head.data.materials.append(mat_skin)
    shade_flat(head)
    head.parent = master
    # snout (cone)
    snout = add('cone', vertices=16, location=(0.5, 0, 0))
    snout.scale = (0.15, 0.1, 0.25)
    snout.rotation_euler = (0, math.radians(90), 0)
    snout.data.materials.append(mat_skin)
    shade_flat(snout)
    snout.parent = master
    # horns (two curved cones)
    for side in [1, -1]:
        horn = add('cone', vertices=12, location=(-0.1, 0.18 * side, 0.18))
        horn.scale = (0.05, 0.05, 0.3)
        horn.rotation_euler = (math.radians(45), 0, math.radians(90 * side))
        horn.data.materials.append(mat_horn)
        shade_flat(horn)
        horn.parent = master
    # eyes (spheres)
    for side in [0.12, -0.12]:
        eye = add('uv_sphere', location=(0.3, side, 0.08))
        eye.scale = (0.05, 0.05, 0.05)
        eye.data.materials.append(mat_eye)
        shade_flat(eye)
        eye.parent = master
    # spikes along snout top
    for i in range(4):
        spike = add('cone', vertices=8, location=(0.2 + i * 0.08, 0, 0.25 + i * 0.03))
        spike.scale = (0.02, 0.02, 0.1)
        spike.rotation_euler = (0, 0, 0)
        spike.data.materials.append(mat_horn)
        shade_flat(spike)
        spike.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "dragon_head.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
dragon_head = build_head()
export_glb(dragon_head)