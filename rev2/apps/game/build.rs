use std::env;
use std::process::Command;

fn main() {
    // Re-run if worldbook or narrative changes
    println!("cargo:rerun-if-changed=../../build/world/worldbook.json");
    println!("cargo:rerun-if-changed=../../build/narrative/");

    // Optional: integrate bevy-agent at build time if explicitly requested
    if env::var("BEVY_AGENT_AUTOGEN").ok().as_deref() == Some("1") {
        // Try to call the locally built agent_bridge binary (feature gated at workspace level)
        let _ = Command::new("cargo")
            .args(["run", "-p", "agent_bridge", "--features", "agent", "--release"])
            .status();
    }
}
