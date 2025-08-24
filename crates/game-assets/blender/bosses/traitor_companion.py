# Boss: Traitor Companion – builds a corrupted villager with dark colours and horns.
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
    bsdf.inputs['Roughness'].default_value = 0.7
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_traitor():
    master = bpy.data.objects.new("TraitorCompanion", None)
    bpy.context.collection.objects.link(master)
    # materials (dark palette)
    mat_base = new_mat("Base", (0.15, 0.15, 0.2, 1.0))
    mat_skin = new_mat("Skin", (0.6, 0.5, 0.45, 1.0))
    mat_clothes = new_mat("Clothes", (0.15, 0.1, 0.25, 1.0))
    mat_horn = new_mat("Horn", (0.4, 0.2, 0.1, 1.0))
    # base
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.6, 0.6, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # body
    body = add('cylinder', vertices=12, location=(0, 0, 0.2))
    body.scale = (0.22, 0.22, 0.35)
    body.data.materials.append(mat_clothes)
    shade_flat(body)
    body.parent = master
    # head
    bpy.ops.mesh.primitive_uv_sphere_add(segments=12, ring_count=6, location=(0, 0, 0.55))
    head = bpy.context.active_object
    head.scale = (0.12, 0.12, 0.12)
    head.data.materials.append(mat_skin)
    shade_flat(head)
    head.parent = master
    # horns
    for direction in [1, -1]:
        horn = add('cone', vertices=8, location=(0.08 * direction, 0, 0.65))
        horn.scale = (0.03, 0.03, 0.15)
        horn.rotation_euler = (0, math.radians(90), math.radians(30 * direction))
        horn.data.materials.append(mat_horn)
        shade_flat(horn)
        horn.parent = master
    # arms (slightly raised)
    for side in [0.32, -0.32]:
        arm = add('cylinder', vertices=8, location=(side, 0, 0.3))
        arm.scale = (0.05, 0.05, 0.25)
        arm.rotation_euler = (0, math.radians(90), math.radians(20 if side>0 else -20))
        arm.data.materials.append(mat_clothes)
        shade_flat(arm)
        arm.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "traitor_companion.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
traitor = build_traitor()
export_glb(traitor)