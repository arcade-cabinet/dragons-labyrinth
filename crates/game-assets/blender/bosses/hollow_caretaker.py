# Boss: Hollow Caretaker – builds a spectral caretaker with a lantern on a small base.
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
    bsdf.inputs['Metallic'].default_value = 0.1
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


def build_caretaker():
    master = bpy.data.objects.new("HollowCaretaker", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Base", (0.15, 0.15, 0.2, 1.0))
    mat_cloak = new_mat("Cloak", (0.2, 0.2, 0.25, 1.0))
    mat_skin = new_mat("Skin", (0.8, 0.75, 0.7, 1.0))
    mat_lantern = new_mat("Lantern", (0.5, 0.5, 0.2, 1.0))
    mat_light = new_mat("Light", (0.9, 0.8, 0.3, 1.0))
    # small base
    base = add('cylinder', vertices=12, location=(0, 0, 0))
    base.scale = (0.4, 0.4, 0.05)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # body (cloak)
    body = add('cone', vertices=16, location=(0, 0, 0.05))
    body.scale = (0.2, 0.2, 0.6)
    body.data.materials.append(mat_cloak)
    shade_flat(body)
    body.parent = master
    # head
    bpy.ops.mesh.primitive_uv_sphere_add(segments=12, ring_count=6, location=(0, 0, 0.6))
    head = bpy.context.active_object
    head.scale = (0.1, 0.1, 0.1)
    head.data.materials.append(mat_skin)
    shade_flat(head)
    head.parent = master
    # arms (thin cylinders)
    for offset in [0.15, -0.15]:
        arm = add('cylinder', vertices=8, location=(offset, 0, 0.35))
        arm.scale = (0.03, 0.03, 0.35)
        arm.rotation_euler = (0, math.radians(90), 0)
        arm.data.materials.append(mat_cloak)
        shade_flat(arm)
        arm.parent = master
    # lantern – handle, frame and light
    handle = add('cylinder', vertices=8, location=(0.3, 0, 0.35))
    handle.scale = (0.02, 0.02, 0.2)
    handle.rotation_euler = (0, math.radians(90), 0)
    handle.data.materials.append(mat_lantern)
    shade_flat(handle)
    handle.parent = master
    frame = add('cube', size=0.1, location=(0.4, 0, 0.35))
    frame.scale = (0.05, 0.05, 0.1)
    frame.data.materials.append(mat_lantern)
    shade_flat(frame)
    frame.parent = master
    light = add('uv_sphere', location=(0.4, 0, 0.35))
    light.scale = (0.03, 0.03, 0.03)
    light.data.materials.append(mat_light)
    shade_flat(light)
    light.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "hollow_caretaker.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
caretaker = build_caretaker()
export_glb(caretaker)