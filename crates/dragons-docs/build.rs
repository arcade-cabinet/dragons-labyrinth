//! Build script for Dragon's Labyrinth documentation
//! Generates mdBook theme from style-guide at build time

use std::path::Path;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=../style-guide/src");
    println!("cargo:rerun-if-changed=book.toml");
    println!("cargo:rerun-if-changed=book");
    
    // Generate theme in the book's theme directory
    let theme_dir = Path::new("theme");
    fs::create_dir_all(theme_dir).expect("Failed to create theme directory");
    
    // Generate theme using style-guide - this MUST work
    style_guide::mdbook_theme::generate_theme(theme_dir)
        .expect("CRITICAL: Failed to generate mdBook theme from style-guide");
    
    println!("✅ Generated mdBook theme from style-guide");
}
