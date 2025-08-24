"""
Generic BPY Batch Processor for Dragon's Labyrinth
CLEAN IMPLEMENTATION: Any AI workflow can generate BPY scripts and get back Bevy assets
"""

import json
import logging
import hashlib
import re
from pathlib import Path

import bpy
import bmesh
import mathutils
from typing import Any

# --- helper constants for OBJ family hashing / MTL parsing ---
MTL_MAP_KEYS = (
    "map_Kd","map_Ks","map_Ka","bump","map_bump","map_d",
    "map_Ns","norm","map_norm","map_Pr","map_Pm"
)

def _tokenize_path_for_hash(p: Path) -> bytes:
    # use name + mtime + size for fast idempotency; robust enough for content changes
    try:
        return (p.name + str(p.stat().st_mtime_ns) + str(p.stat().st_size)).encode()
    except Exception:
        return p.name.encode()


def _parse_mtl_for_textures(mtl_path: Path) -> list:
    tex = []
    try:
        for line in mtl_path.read_text(errors="ignore").splitlines():
            ls = line.strip()
            if any(ls.startswith(k) for k in MTL_MAP_KEYS):
                parts = ls.split()
                if len(parts) >= 2:
                    cand = (mtl_path.parent / parts[-1]).resolve()
                    if cand.exists():
                        tex.append(cand)
    except Exception:
        pass
    return tex


def _obj_family_hash(obj_path: Path) -> str:
    h = hashlib.sha256()
    if obj_path.exists():
        h.update(_tokenize_path_for_hash(obj_path))
        mtl = obj_path.with_suffix(".mtl")
        if mtl.exists():
            h.update(_tokenize_path_for_hash(mtl))
            for t in _parse_mtl_for_textures(mtl):
                try:
                    h.update(_tokenize_path_for_hash(t))
                except Exception:
                    pass
    return h.hexdigest()[:16]


class BPYProcessor:
    """
    Generic BPY processor that executes ANY BPY script and exports GLB for Bevy.
    This is the ONE processor that ALL AI workflows use.
    """
    
    def __init__(self, logger: logging.Logger = None):
        self.logger = logger or logging.getLogger(__name__)
        
    def execute_bpy_script(self, script_content: str, export_path: str) -> dict[str, Any]:
        """
        Execute BPY script directly in Blender context and export GLB.
        
        Args:
            script_content: Python script content using bpy
            export_path: Where to save the GLB file
            
        Returns:
            Result dictionary with success status and file info
        """
        export_path = Path(export_path)
        export_path.parent.mkdir(parents=True, exist_ok=True)
        
        try:
            # Clear scene first
            self._clear_scene()
            
            # Execute the BPY script directly
            exec(script_content, {"bpy": bpy, "bmesh": bmesh, "mathutils": mathutils})
            
            # Export to GLB for Bevy
            self._export_to_glb(str(export_path))
            
            self.logger.info(f"BPY script executed and exported to {export_path}")
            
            return {
                "success": True,
                "output_file": str(export_path),
                "file_size": export_path.stat().st_size if export_path.exists() else 0,
                "vertex_count": self._get_vertex_count(),
                "face_count": self._get_face_count()
            }
            
        except Exception as e:
            self.logger.error(f"BPY script execution failed: {e}")
            return {
                "success": False,
                "error": str(e),
                "output_file": str(export_path)
            }
    
    def process_batch(self, batch_scripts: list[dict[str, str]], output_dir: str) -> dict[str, Any]:
        """
        Process multiple BPY scripts in batch.
        
        Args:
            batch_scripts: List of {"script": "...", "filename": "..."}
            output_dir: Directory for output files
            
        Returns:
            Batch result with individual file results
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        results = []
        successful = 0
        total_files = 0
        
        for script_data in batch_scripts:
            script_content = script_data["script"]
            filename = script_data["filename"]
            
            if not filename.endswith(".glb"):
                filename = f"{filename}.glb"
                
            export_path = output_dir / filename
            
            result = self.execute_bpy_script(script_content, str(export_path))
            results.append({
                "filename": filename,
                "result": result
            })
            
            if result["success"]:
                successful += 1
                total_files += 1
        
        self.logger.info(f"Batch complete: {successful}/{len(batch_scripts)} successful")
        
        return {
            "success": True,
            "total_scripts": len(batch_scripts),
            "successful": successful,
            "failed": len(batch_scripts) - successful,
            "total_files": total_files,
            "results": results,
            "output_dir": str(output_dir)
        }
    
    # -----------------------------
    # Public: convert a single model file to GLB
    # -----------------------------
    def convert_file_to_glb(self, src_path: str, dst_path: str, scale: float = 1.0) -> dict:
        """
        Import OBJ/MTL, FBX, or glTF/GLB into the current Blender scene and export a GLB.
        Packs images, applies transforms, and returns basic scene stats.
        """
        src = Path(src_path)
        dst = Path(dst_path)
        dst.parent.mkdir(parents=True, exist_ok=True)
        try:
            self._clear_scene()
            ext = src.suffix.lower()
            if ext == ".obj":
                self._import_obj(str(src), scale)
            elif ext == ".fbx":
                self._import_fbx(str(src), scale)
            elif ext in (".gltf", ".glb"):
                self._import_gltf(str(src), scale)
            else:
                raise ValueError(f"Unsupported model extension: {ext}")
            self._apply_transforms()
            self._export_to_glb(str(dst))
            return {
                "success": True,
                "input": str(src),
                "output_file": str(dst),
                "file_size": dst.stat().st_size if dst.exists() else 0,
                "vertex_count": self._get_vertex_count(),
                "face_count": self._get_face_count(),
            }
        except Exception as e:
            self.logger.error(f"Conversion failed for {src}: {e}")
            return {"success": False, "error": str(e), "input": str(src), "output_file": str(dst)}

    # ------------------------------------------
    # Public: idempotent batch conversion
    # ------------------------------------------
    def convert_batch(self, jobs: list, manifest_path: str = "") -> dict:
        """
        Convert many files to GLB. Each job is a dict with keys: {'src','dst','scale'}.
        Idempotent via a JSON manifest that caches per-input hashes.
        """
        manifest = {}
        if manifest_path:
            mp = Path(manifest_path)
            if mp.exists():
                try:
                    manifest = json.loads(mp.read_text())
                except Exception:
                    manifest = {}

        results = []
        converted = 0
        for job in jobs:
            src = Path(job.get("src", "")).resolve()
            dst = Path(job.get("dst", "")).resolve()
            scale = float(job.get("scale", 1.0))
            if not src.exists():
                results.append({"success": False, "error": "missing source", "input": str(src)})
                continue

            ext = src.suffix.lower()
            if ext == ".obj":
                h = _obj_family_hash(src)
            else:
                h = hashlib.sha256(_tokenize_path_for_hash(src)).hexdigest()[:16]

            key = str(src)
            prev = manifest.get(key)
            if prev and prev.get("hash") == h and prev.get("dst") and Path(prev["dst"]).exists():
                results.append({"success": True, "input": str(src), "output_file": prev["dst"], "skipped": True})
                continue

            r = self.convert_file_to_glb(str(src), str(dst), scale=scale)
            results.append(r)
            if r.get("success"):
                converted += 1
                manifest[key] = {"hash": h, "dst": str(dst)}

        if manifest_path:
            try:
                Path(manifest_path).parent.mkdir(parents=True, exist_ok=True)
                Path(manifest_path).write_text(json.dumps(manifest, indent=2))
            except Exception as e:
                self.logger.warning(f"Could not write manifest: {e}")

        return {"success": True, "converted": converted, "total": len(jobs), "results": results}

    # -----------------------------
    # Importers / transforms
    # -----------------------------
    def _import_obj(self, path: str, scale: float):
        bpy.ops.import_scene.obj(
            filepath=path,
            axis_forward='-Z', axis_up='Y',
            use_split_objects=True,
            use_image_search=True,
            split_mode='ON',
        )
        self._uniform_scale_selected(scale)
        self._smooth_selected()

    def _import_fbx(self, path: str, scale: float):
        bpy.ops.import_scene.fbx(
            filepath=path,
            use_anim=False,
            automatic_bone_orientation=True,
        )
        self._uniform_scale_selected(scale)
        self._smooth_selected()

    def _import_gltf(self, path: str, scale: float):
        bpy.ops.import_scene.gltf(filepath=path)
        self._uniform_scale_selected(scale)
        self._smooth_selected()

    def _uniform_scale_selected(self, s: float):
        if s == 1.0:
            return
        for obj in bpy.context.selected_objects:
            try:
                obj.scale = (s, s, s)
            except Exception:
                pass

    def _smooth_selected(self):
        for obj in bpy.context.selected_objects:
            if obj.type == 'MESH':
                try:
                    bpy.context.view_layer.objects.active = obj
                    bpy.ops.object.shade_smooth()
                    obj.data.use_auto_smooth = True
                except Exception:
                    pass

    def _apply_transforms(self):
        bpy.ops.object.select_all(action='SELECT')
        try:
            bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)
        except Exception:
            pass

    def _clear_scene(self):
        """Clear all objects from the current Blender scene."""
        # Select all objects
        bpy.ops.object.select_all(action='SELECT')
        
        # Delete all objects
        bpy.ops.object.delete()
        
        # Clear orphan data blocks
        for block in bpy.data.meshes:
            if block.users == 0:
                bpy.data.meshes.remove(block)
        
        for block in bpy.data.materials:
            if block.users == 0:
                bpy.data.materials.remove(block)
                
        for block in bpy.data.textures:
            if block.users == 0:
                bpy.data.textures.remove(block)
                
        for block in bpy.data.images:
            if block.users == 0:
                bpy.data.images.remove(block)
    
    def _export_to_glb(self, export_path: str):
        """Export scene to GLB format for Bevy."""
        # Select all visible objects for export
        bpy.ops.object.select_all(action='DESELECT')
        for obj in bpy.context.scene.objects:
            if not obj.hide_get():
                obj.select_set(True)
        
        # Export using bpy.ops.export_scene.gltf as documented
        bpy.ops.export_scene.gltf(
            filepath=export_path,
            export_format='GLB',
            use_selection=True,
            export_apply=True,
            export_texcoords=True,
            export_normals=True,
            export_tangents=True,
            export_materials='EXPORT',
            export_cameras=False,
            export_lights=False,
            export_animations=False,
            export_yup=True,
            export_extras=False,
            export_images='AUTO',
            export_image_format='AUTO'
        )
    
    def _get_vertex_count(self) -> int:
        """Get total vertex count in scene."""
        total = 0
        for obj in bpy.context.scene.objects:
            if obj.type == 'MESH':
                total += len(obj.data.vertices)
        return total
    
    def _get_face_count(self) -> int:
        """Get total face count in scene."""
        total = 0
        for obj in bpy.context.scene.objects:
            if obj.type == 'MESH':
                total += len(obj.data.polygons)
        return total


class BPYBatchManager:
    """
    Manager for handling batch BPY script processing.
    Designed to work with ANY AI workflow that generates BPY scripts.
    """
    
    def __init__(self, logger: logging.Logger = None):
        self.logger = logger or logging.getLogger(__name__)
        self.processor = BPYProcessor(logger)
    
    def create_batch_manifest(self, scripts: list[dict[str, str]], output_dir: str) -> str:
        """
        Create a manifest file for batch processing.
        
        Args:
            scripts: List of {"script": "...", "filename": "..."}
            output_dir: Directory for manifest and outputs
            
        Returns:
            Path to created manifest file
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        manifest = {
            "batch_type": "bpy_scripts",
            "total_scripts": len(scripts),
            "output_directory": str(output_dir),
            "scripts": []
        }
        
        for i, script_data in enumerate(scripts):
            filename = script_data["filename"]
            if not filename.endswith(".glb"):
                filename = f"{filename}.glb"
                
            manifest["scripts"].append({
                "index": i,
                "script_content": script_data["script"],
                "output_filename": filename,
                "output_path": str(output_dir / filename)
            })
        
        manifest_path = output_dir / "batch_manifest.json"
        with open(manifest_path, 'w') as f:
            json.dump(manifest, f, indent=2)
        
        self.logger.info(f"Created batch manifest: {manifest_path}")
        return str(manifest_path)
    
    def execute_batch_from_manifest(self, manifest_path: str) -> dict[str, Any]:
        """
        Execute batch processing from manifest file.
        
        Args:
            manifest_path: Path to batch manifest JSON
            
        Returns:
            Batch execution results
        """
        manifest_path = Path(manifest_path)
        
        with open(manifest_path, 'r') as f:
            manifest = json.load(f)
        
        batch_scripts = []
        for script_entry in manifest["scripts"]:
            batch_scripts.append({
                "script": script_entry["script_content"],
                "filename": script_entry["output_filename"]
            })
        
        return self.processor.process_batch(batch_scripts, manifest["output_directory"])
    
    def process_scripts_direct(
        self, 
        scripts: list[dict[str, str]], 
        output_dir: str,
        create_manifest: bool = True
    ) -> dict[str, Any]:
        """
        Process BPY scripts directly without manifest file.
        
        Args:
            scripts: List of {"script": "...", "filename": "..."}
            output_dir: Directory for outputs
            create_manifest: Whether to create manifest file
            
        Returns:
            Processing results
        """
        if create_manifest:
            manifest_path = self.create_batch_manifest(scripts, output_dir)
            self.logger.info(f"Manifest created: {manifest_path}")
        
        return self.processor.process_batch(scripts, output_dir)


def process_bpy_script_to_glb(script_content: str, output_path: str) -> dict[str, Any]:
    """
    Convenience function for single script processing.
    This is the main entry point for AI workflows.
    
    Args:
        script_content: BPY Python script content
        output_path: Where to save the GLB file
        
    Returns:
        Processing result
    """
    processor = BPYProcessor()
    return processor.execute_bpy_script(script_content, output_path)


def process_bpy_batch_to_glb(
    scripts: list[dict[str, str]], 
    output_dir: str
) -> dict[str, Any]:
    """
    Convenience function for batch processing.
    
    Args:
        scripts: List of {"script": "...", "filename": "..."}
        output_dir: Directory for GLB outputs
        
    Returns:
        Batch processing results
    """
    manager = BPYBatchManager()
    return manager.process_scripts_direct(scripts, output_dir)


def convert_model_files_to_glb(files: list, output_dir: str, scale: float = 1.0, manifest_path: str = "") -> dict:
    """
    Convert a list of model files to GLB.
    Each item in `files` is a dict with keys:
      - 'src': path to source model (OBJ/FBX/GLTF/GLB)
      - 'dst_filename' (optional): name for the output .glb; defaults to src.stem + '.glb'
    """
    out = Path(output_dir)
    out.mkdir(parents=True, exist_ok=True)
    jobs = []
    for f in files:
        src = Path(f.get('src', '')).resolve()
        if not src.exists():
            continue
        name = f.get('dst_filename')
        if not name:
            name = src.stem + ".glb"
        dst = out / name
        jobs.append({"src": str(src), "dst": str(dst), "scale": float(f.get('scale', scale))})
    proc = BPYProcessor()
    return proc.convert_batch(jobs, manifest_path=manifest_path)
