//! Build the Dragon's Labyrinth player documentation

use anyhow::Result;
use std::process::Command;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("Building Dragon's Labyrinth Documentation...");
    
    // Ensure theme is generated
    let theme_dir = Path::new("crates/dragons-docs/theme");
    if !theme_dir.exists() {
        fs::create_dir_all(theme_dir)?;
        println!("Generating theme from style-guide...");
        style_guide::mdbook_theme::generate_theme(theme_dir)?;
    }
    
    // Build the book with mdBook
    let output = Command::new("mdbook")
        .arg("build")
        .arg("crates/dragons-docs")
        .output()?;
    
    if !output.status.success() {
        eprintln!("mdBook build failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    println!("Documentation built successfully!");
    println!("View at: docs/index.html");
    
    // Generate changelog
    println!("Generating changelog...");
    let changelog = dragons_docs::changelog::generate_changelog()?;
    fs::write("docs/changelog.html", format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Dragon's Labyrinth - Changelog</title>
    <style>
        body {{ font-family: 'Crimson Text', serif; max-width: 800px; margin: 0 auto; padding: 2em; }}
        h1 {{ color: #8B0000; }}
        h2 {{ color: #2F4F2F; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        markdown::to_html(&changelog)
    ))?;
    
    Ok(())
}