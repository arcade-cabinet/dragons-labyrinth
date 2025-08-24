# Weapon: Sword – builds a simple low‑poly sword suitable for attachment to villager models.
import bpy
import os


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
    bsdf.inputs['Metallic'].default_value = 0.5
    bsdf.inputs['Roughness'].default_value = 0.4
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_sword():
    master = bpy.data.objects.new("Sword", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_blade = new_mat("Blade", (0.7, 0.7, 0.75, 1.0))
    mat_handle = new_mat("Handle", (0.3, 0.2, 0.1, 1.0))
    # blade (long cone)
    blade = add('cone', vertices=16, location=(0, 0, 0.2))
    blade.scale = (0.04, 0.04, 0.5)
    blade.rotation_euler = (0, 0, 0)
    blade.data.materials.append(mat_blade)
    shade_flat(blade)
    blade.parent = master
    # crossguard
    guard = add('cylinder', vertices=12, location=(0, 0, -0.05))
    guard.scale = (0.1, 0.02, 0.02)
    guard.data.materials.append(mat_handle)
    shade_flat(guard)
    guard.parent = master
    # handle grip
    handle = add('cylinder', vertices=12, location=(0, 0, -0.15))
    handle.scale = (0.03, 0.03, 0.15)
    handle.data.materials.append(mat_handle)
    shade_flat(handle)
    handle.parent = master
    # pommel
    pommel = add('uv_sphere', location=(0, 0, -0.3))
    pommel.scale = (0.04, 0.04, 0.04)
    pommel.data.materials.append(mat_handle)
    shade_flat(pommel)
    pommel.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "weapon_sword.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
sword = build_sword()
export_glb(sword)