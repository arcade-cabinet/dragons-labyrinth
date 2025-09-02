use std::path::PathBuf;
use builder::worldgen::WorldGenerator;

fn main() {
    // Idempotently generate all ECS code into apps/game/src/world
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let target = repo_root.join("apps/game/src/world");
    let gen = WorldGenerator::new(&repo_root);
    let _ = std::fs::create_dir_all(&target);
    let _ = gen.write_into(&target);
}


