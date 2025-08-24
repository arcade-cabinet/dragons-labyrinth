//! Serve the Dragon's Labyrinth documentation locally

use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    println!("Starting Dragon's Labyrinth Documentation Server...");
    println!("The documentation will evolve with your dread level as you read...");
    println!();
    println!("Access at: http://localhost:3000");
    println!("Press Ctrl+C to stop");
    println!();
    
    // Serve the book with mdBook
    let status = Command::new("mdbook")
        .arg("serve")
        .arg("crates/dragons-docs")
        .arg("--port")
        .arg("3000")
        .status()?;
    
    if !status.success() {
        eprintln!("Failed to start documentation server");
        std::process::exit(1);
    }
    
    Ok(())
}
