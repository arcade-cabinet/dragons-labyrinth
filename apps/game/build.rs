use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get the OUT_DIR from dl_processors
    let processors_out_dir = dl_processors::generated_dir();
    let game_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Create symlinks or copy the generated files
    let regions_src = processors_out_dir.join("regions");
    let dungeons_src = processors_out_dir.join("dungeons");
    let settlements_src = processors_out_dir.join("settlements");
    let mod_src = processors_out_dir.join("mod.rs");
    
    let regions_dst = game_out_dir.join("regions");
    let dungeons_dst = game_out_dir.join("dungeons");
    let settlements_dst = game_out_dir.join("settlements");
    let mod_dst = game_out_dir.join("generated_world.rs");
    
    // Copy directories (symlinks would be better but more complex cross-platform)
    if regions_src.exists() {
        copy_dir_all(&regions_src, &regions_dst)?;
    }
    if dungeons_src.exists() {
        copy_dir_all(&dungeons_src, &dungeons_dst)?;
    }
    if settlements_src.exists() {
        copy_dir_all(&settlements_src, &settlements_dst)?;
    }
    if mod_src.exists() {
        fs::copy(&mod_src, &mod_dst)?;
    }
    
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
