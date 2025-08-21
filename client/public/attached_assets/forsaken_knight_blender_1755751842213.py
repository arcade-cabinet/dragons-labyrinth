# Blender script: Low‑poly Forsaken Knight on a hex base
# Usage: paste in Blender's Scripting editor and Run Script.

import bpy
import math
import os

# --- utils ---
def reset_scene():
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    # remove unused data blocks
    for b in bpy.data.meshes:
        if b.users == 0:
            bpy.data.meshes.remove(b)
    for m in bpy.data.materials:
        if m.users == 0:
            bpy.data.materials.remove(m)

def new_mat(name, rgba=(0.2, 0.2, 0.2, 1.0)):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    bsdf.inputs["Base Color"].default_value = (rgba[0], rgba[1], rgba[2], rgba[3])
    bsdf.inputs["Metallic"].default_value = 0.0
    bsdf.inputs["Roughness"].default_value = 0.8
    return mat

def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)

def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f"primitive_{objtype}_add")(**kwargs)
    return bpy.context.active_object

# --- build knight ---
def build_knight():
    # master empty
    master = bpy.data.objects.new("ForsakenKnightRig", None)
    bpy.context.collection.objects.link(master)

    # materials
    mat_base = new_mat("Base", (0.1, 0.1, 0.12, 1))
    mat_armor = new_mat("Armor", (0.25, 0.25, 0.28, 1))
    mat_cloth = new_mat("Cloth", (0.15, 0.05, 0.05, 1))
    mat_metal = new_mat("Metal", (0.6, 0.6, 0.65, 1))

    # hex base
    base = add("cylinder", vertices=6, enter_editmode=False, align='WORLD', location=(0, 0, 0))
    base.scale = (0.65, 0.65, 0.1)
    base.location.z = 0.1
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master

    # torso
    torso = add("cylinder", vertices=12, location=(0, 0, 0))
    torso.scale = (0.28, 0.28, 0.4)
    torso.location.z = 0.5
    torso.data.materials.append(mat_armor)
    shade_flat(torso)
    torso.parent = master

    # head (helm)
    bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=1, location=(0, 0, 0))
    head = bpy.context.active_object
    head.scale = (0.14, 0.14, 0.14)
    head.location.z = 0.95
    head.data.materials.append(mat_armor)
    shade_flat(head)
    head.parent = master

    # plume / helmet top
    plume = add("cone", vertices=16, location=(0, 0, 0))
    plume.scale = (0.05, 0.05, 0.2)
    plume.location.z = 1.1
    plume.data.materials.append(mat_cloth)
    shade_flat(plume)
    plume.parent = master

    # shoulders
    shoulder_left = add("uv_sphere", location=(0.22, 0, 0))
    shoulder_left.scale = (0.15, 0.08, 0.08)
    shoulder_left.location.z = 0.75
    shoulder_left.data.materials.append(mat_armor)
    shade_flat(shoulder_left)
    shoulder_left.parent = master

    shoulder_right = add("uv_sphere", location=(-0.22, 0, 0))
    shoulder_right.scale = (0.15, 0.08, 0.08)
    shoulder_right.location.z = 0.75
    shoulder_right.data.materials.append(mat_armor)
    shade_flat(shoulder_right)
    shoulder_right.parent = master

    # arms (simple cylinders)
    arm_left = add("cylinder", vertices=12, location=(0.32, 0, 0.55))
    arm_left.scale = (0.05, 0.05, 0.25)
    arm_left.rotation_euler = (0, math.radians(90), math.radians(20))
    arm_left.data.materials.append(mat_armor)
    shade_flat(arm_left)
    arm_left.parent = master

    arm_right = add("cylinder", vertices=12, location=(-0.32, 0, 0.55))
    arm_right.scale = (0.05, 0.05, 0.25)
    arm_right.rotation_euler = (0, math.radians(90), math.radians(-20))
    arm_right.data.materials.append(mat_armor)
    shade_flat(arm_right)
    arm_right.parent = master

    # sword
    blade = add("cone", vertices=12, location=(0.35, 0, 0.35))
    blade.scale = (0.05, 0.05, 0.35)
    blade.rotation_euler = (0, math.radians(90), 0)
    blade.data.materials.append(mat_metal)
    shade_flat(blade)
    blade.parent = master

    grip = add("cylinder", vertices=12, location=(0.27, 0, 0.35))
    grip.scale = (0.02, 0.02, 0.1)
    grip.rotation_euler = (0, math.radians(90), 0)
    grip.data.materials.append(mat_cloth)
    shade_flat(grip)
    grip.parent = master

    # shield
    shield = add("cylinder", vertices=12, location=(-0.35, 0, 0.35))
    shield.scale = (0.18, 0.18, 0.05)
    shield.rotation_euler = (math.radians(0), math.radians(90), 0)
    shield.data.materials.append(mat_armor)
    shade_flat(shield)
    shield.parent = master

    return master

# --- camera, light, render ---
def setup_scene():
    scn = bpy.context.scene
    scn.render.engine = 'CYCLES' if 'CYCLES' in bpy.context.preferences.addons else 'BLENDER_EEVEE_NEXT'
    if hasattr(scn, "cycles"):
        scn.cycles.samples = 32
    scn.render.resolution_x = 512
    scn.render.resolution_y = 512
    scn.render.film_transparent = True
    scn.render.image_settings.file_format = 'PNG'
    scn.render.image_settings.color_mode = 'RGBA'
    # world light
    scn.world.use_nodes = True
    bg = scn.world.node_tree.nodes.get("Background")
    if bg:
        bg.inputs[1].default_value = 0.9
    # camera
    cam_data = bpy.data.cameras.new("OrthoCam")
    cam_data.type = 'ORTHO'
    cam_data.ortho_scale = 1.8
    cam = bpy.data.objects.new("OrthoCam", cam_data)
    bpy.context.collection.objects.link(cam)
    cam.location = (0, 0, 5)
    cam.rotation_euler = (math.radians(90), 0, 0)
    scn.camera = cam
    # light
    light_data = bpy.data.lights.new(name="Sun", type='SUN')
    light_data.energy = 4.0
    sun = bpy.data.objects.new(name="Sun", object_data=light_data)
    bpy.context.collection.objects.link(sun)
    sun.location = (3.0, -3.0, 5.0)
    sun.rotation_euler = (math.radians(45), 0, math.radians(45))

def ensure_dir(path):
    if not os.path.exists(path):
        os.makedirs(path, exist_ok=True)

def render_turntable(obj, outdir="//renders/forsaken_knight/", angles_deg=(0, 60, 120, 180, 240, 300)):
    # Resolve '//' relative to .blend or temp dir
    if outdir.startswith("//"):
        try:
            absdir = bpy.path.abspath(outdir)
        except:
            absdir = outdir.replace("//", "./")
    else:
        absdir = outdir
    ensure_dir(bpy.path.abspath(outdir))
    scn = bpy.context.scene
    for ang in angles_deg:
        obj.rotation_euler[2] = math.radians(ang)
        scn.render.filepath = os.path.join(outdir, f"forsaken_knight_{ang:03d}.png")
        bpy.ops.render.render(write_still=True)

# --- run ---
reset_scene()
rig = build_knight()
setup_scene()
render_turntable(rig)
print("Done: renders written to //renders/forsaken_knight/")