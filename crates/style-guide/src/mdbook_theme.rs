//! Generate mdBook theme from style guide

use crate::{StyleGuide, DreadLevel};
use anyhow::Result;
use std::path::Path;
use std::fs;

/// Generate mdBook theme files based on style guide
pub fn generate_theme(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    
    // Generate CSS
    generate_css(output_dir)?;
    
    // Generate JavaScript  
    generate_js(output_dir)?;
    
    // Copy required mdBook theme files
    generate_index_hbs(output_dir)?;
    
    Ok(())
}

fn generate_css(theme_dir: &Path) -> Result<()> {
    let mut css = String::new();
    
    // Base theme from style guide
    css.push_str(r#"
/* Dragon's Labyrinth Theme - Generated from Style Guide */
@import url('https://fonts.googleapis.com/css2?family=Crimson+Text:ital,wght@0,400;0,700;1,400&display=swap');

:root {
    --sidebar-width: 300px;
    --page-padding: 15px;
    --content-max-width: 750px;
    --menu-bar-height: 50px;
"#);

    // Generate CSS variables for each dread level
    for level in 0..=4 {
        let dread = DreadLevel(level);
        let guide = StyleGuide::new(dread);
        let colors = guide.colors();
        
        css.push_str(&format!(
            r#"
    /* Dread Level {} */
    --dread-{}-primary: {};
    --dread-{}-bg: {};
    --dread-{}-text: {};
    --dread-{}-accent: {};
    --dread-{}-corruption: {};
"#,
            level,
            level, crate::color::ColorPalette::to_hex(colors.primary()),
            level, crate::color::ColorPalette::to_hex(colors.background()),
            level, crate::color::ColorPalette::to_hex(colors.ui_text()),
            level, crate::color::ColorPalette::to_hex(colors.secondary()),
            level, format!("{}%", guide.decay_intensity() * 100.0),
        ));
    }
    
    css.push_str(r#"
}

/* Base styling */
body {
    font-family: 'Crimson Text', serif;
    background: var(--dread-0-bg);
    color: var(--dread-0-text);
    transition: all 0.5s ease;
}

/* Dread level classes */
.dread-0 { 
    background: var(--dread-0-bg);
    color: var(--dread-0-text);
}
.dread-1 {
    background: var(--dread-1-bg);
    color: var(--dread-1-text);
}
.dread-2 {
    background: var(--dread-2-bg);
    color: var(--dread-2-text);
}
.dread-3 {
    background: var(--dread-3-bg);
    color: var(--dread-3-text);
    filter: contrast(1.1) saturate(0.9);
}
.dread-4 {
    background: var(--dread-4-bg);
    color: var(--dread-4-text);
    filter: contrast(1.2) saturate(0.7) hue-rotate(10deg);
    animation: corruption 2s infinite;
}

/* Horror effects */
@keyframes corruption {
    0% { filter: contrast(1.2) saturate(0.7) hue-rotate(10deg); }
    50% { filter: contrast(1.5) saturate(0.5) hue-rotate(-10deg); }
    100% { filter: contrast(1.2) saturate(0.7) hue-rotate(10deg); }
}

@keyframes glitch {
    0% { 
        text-shadow: 0.05em 0 0 rgba(255,0,0,0.75),
                    -0.025em -0.05em 0 rgba(0,255,0,0.75),
                    0.025em 0.05em 0 rgba(0,0,255,0.75);
    }
    15% {
        text-shadow: 0.05em 0 0 rgba(255,0,0,0.75),
                    -0.05em -0.025em 0 rgba(0,255,0,0.75),
                    0.025em 0.05em 0 rgba(0,0,255,0.75);
    }
    100% {
        text-shadow: 0.05em 0 0 rgba(255,0,0,0.75),
                    -0.025em -0.05em 0 rgba(0,255,0,0.75),
                    0.025em 0.05em 0 rgba(0,0,255,0.75);
    }
}

.dread-3 h1, .dread-3 h2,
.dread-4 h1, .dread-4 h2 {
    animation: glitch 0.5s infinite;
}

/* Code blocks with horror tint */
pre {
    background: #0A0A0A !important;
    border: 1px solid #8B0000;
    padding: 1em;
    border-radius: 4px;
}

code {
    background: rgba(139, 0, 0, 0.1);
    padding: 0.2em 0.4em;
    border-radius: 3px;
}

/* Companion quotes */
.companion-quote {
    font-style: italic;
    border-left: 3px solid #2F4F2F;
    padding-left: 1em;
    margin: 1em 0;
    opacity: 0.9;
}

/* Warning blocks */
.warning {
    border-left: 4px solid #8B0000;
    background: rgba(139, 0, 0, 0.1);
    padding: 1em;
    margin: 1em 0;
}

/* Sidebar theming */
.sidebar {
    background: rgba(0, 0, 0, 0.8);
    border-right: 1px solid #8B0000;
}

.chapter li.chapter-item a:hover {
    color: #8B0000;
}

/* Menu bar */
.menu-bar {
    background: rgba(0, 0, 0, 0.9);
    border-bottom: 1px solid #8B0000;
}

/* Tables */
table {
    border-collapse: collapse;
    width: 100%;
}

th {
    background: rgba(139, 0, 0, 0.2);
    padding: 0.5em;
    border: 1px solid #8B0000;
}

td {
    padding: 0.5em;
    border: 1px solid rgba(139, 0, 0, 0.3);
}

/* Search */
.searchbar {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid #8B0000;
}

/* Scrollbar */
::-webkit-scrollbar {
    width: 10px;
}

::-webkit-scrollbar-track {
    background: #0A0A0A;
}

::-webkit-scrollbar-thumb {
    background: #8B0000;
}

::-webkit-scrollbar-thumb:hover {
    background: #A00000;
}
"#);
    
    fs::write(theme_dir.join("variables.css"), css)?;
    Ok(())
}

fn generate_js(theme_dir: &Path) -> Result<()> {
    let js = r#"
// Dragon's Labyrinth Interactive Documentation
(function() {
    'use strict';
    
    let currentDread = 0;
    let readingTime = 0;
    let secretBuffer = '';
    
    // Track reading time and increase dread
    const readingTimer = setInterval(() => {
        readingTime++;
        
        // Every 3 minutes, increase dread
        if (readingTime % 180 === 0 && currentDread < 4) {
            increaseDread();
        }
        
        // Random corruption at high dread
        if (currentDread >= 3 && Math.random() < 0.01) {
            applyCorruption();
        }
    }, 1000);
    
    function increaseDread() {
        currentDread++;
        document.body.className = document.body.className.replace(/dread-\d/, '') + ' dread-' + currentDread;
        
        // Notify user
        showNotification(`Dread Level ${currentDread}: ${getDreadMessage(currentDread)}`);
        
        // Add environmental effects
        if (currentDread >= 2) addAmbientEffects();
        if (currentDread >= 3) addVisualDistortion();
        if (currentDread === 4) addFullCorruption();
    }
    
    function getDreadMessage(level) {
        const messages = [
            "You sense something watching...",
            "The text seems to shift when you're not looking.",
            "Words are beginning to decay.",
            "Reality is breaking down.",
            "THE VOID CONSUMES ALL."
        ];
        return messages[level] || "Unknown horror awaits.";
    }
    
    function showNotification(message) {
        const notification = document.createElement('div');
        notification.className = 'dread-notification';
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background: rgba(139, 0, 0, 0.9);
            color: #F5F5DC;
            padding: 1em 1.5em;
            border: 2px solid #8B0000;
            border-radius: 4px;
            z-index: 9999;
            animation: slideIn 0.5s ease;
            font-family: 'Crimson Text', serif;
        `;
        notification.textContent = message;
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.style.animation = 'slideOut 0.5s ease';
            setTimeout(() => notification.remove(), 500);
        }, 3000);
    }
    
    function applyCorruption() {
        const elements = document.querySelectorAll('p, li');
        if (elements.length === 0) return;
        
        const target = elements[Math.floor(Math.random() * elements.length)];
        const originalText = target.textContent;
        
        // Corrupt the text
        const corrupted = originalText.split('').map(char => {
            if (Math.random() < 0.1) {
                return String.fromCharCode(Math.floor(Math.random() * 94) + 33);
            }
            return char;
        }).join('');
        
        target.textContent = corrupted;
        target.style.transition = 'none';
        target.style.color = '#FF0000';
        
        // Restore after a moment
        setTimeout(() => {
            target.textContent = originalText;
            target.style.transition = 'color 0.5s';
            target.style.color = '';
        }, 500);
    }
    
    function addAmbientEffects() {
        // Add subtle pulsing to headers
        document.querySelectorAll('h1, h2, h3').forEach(el => {
            el.style.animation = 'pulse 3s infinite';
        });
    }
    
    function addVisualDistortion() {
        // Add glitch effect to images
        document.querySelectorAll('img').forEach(img => {
            img.style.filter = 'contrast(1.2) saturate(0.8)';
        });
    }
    
    function addFullCorruption() {
        // Maximum horror
        document.body.style.animation = 'corruption 2s infinite';
    }
    
    // Easter eggs
    document.addEventListener('keypress', (e) => {
        secretBuffer += e.key.toUpperCase();
        if (secretBuffer.length > 20) {
            secretBuffer = secretBuffer.slice(-20);
        }
        
        // Check for secret phrases
        if (secretBuffer.includes('THERAPIST')) {
            showCompanionMessage('The Therapist', 
                "Reading documentation won't save you from what's coming.");
            secretBuffer = '';
        }
        
        if (secretBuffer.includes('MOUNT')) {
            showCompanionMessage('Your Mount', 
                "Even in documentation, I remain by your side.");
            secretBuffer = '';
        }
        
        if (secretBuffer.includes('FORGE')) {
            showCompanionMessage('The Forge', 
                "Knowledge has a price. What will you sacrifice?");
            secretBuffer = '';
        }
    });
    
    function showCompanionMessage(companion, message) {
        const quote = document.createElement('div');
        quote.className = 'companion-quote';
        quote.style.cssText = `
            position: fixed;
            bottom: 20px;
            left: 50%;
            transform: translateX(-50%);
            background: rgba(47, 79, 47, 0.95);
            color: #F5F5DC;
            padding: 1.5em 2em;
            border: 2px solid #2F4F2F;
            border-radius: 8px;
            max-width: 500px;
            z-index: 9999;
            animation: fadeIn 0.5s ease;
            font-style: italic;
        `;
        quote.innerHTML = `<strong>${companion}:</strong> "${message}"`;
        document.body.appendChild(quote);
        
        setTimeout(() => {
            quote.style.animation = 'fadeOut 0.5s ease';
            setTimeout(() => quote.remove(), 500);
        }, 5000);
    }
    
    // Add CSS animations
    if (!document.querySelector('#dread-animations')) {
        const style = document.createElement('style');
        style.id = 'dread-animations';
        style.textContent = `
            @keyframes slideIn {
                from { transform: translateX(100%); opacity: 0; }
                to { transform: translateX(0); opacity: 1; }
            }
            @keyframes slideOut {
                from { transform: translateX(0); opacity: 1; }
                to { transform: translateX(100%); opacity: 0; }
            }
            @keyframes fadeIn {
                from { opacity: 0; transform: translateY(20px); }
                to { opacity: 1; transform: translateY(0); }
            }
            @keyframes fadeOut {
                from { opacity: 1; transform: translateY(0); }
                to { opacity: 0; transform: translateY(20px); }
            }
            @keyframes pulse {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.8; }
            }
        `;
        document.head.appendChild(style);
    }
    
    // Initialize
    console.log('%c🐉 Dragon\'s Labyrinth Documentation System Active', 
        'color: #8B0000; font-size: 14px; font-weight: bold;');
    console.log('%cReading increases dread. Type secret words for easter eggs.', 
        'color: #2F4F2F; font-style: italic;');
})();
"#;
    
    fs::write(theme_dir.join("book.js"), js)?;
    Ok(())
}

fn generate_index_hbs(theme_dir: &Path) -> Result<()> {
    // Minimal index.hbs that includes our custom files
    let index = r#"<!DOCTYPE HTML>
<html lang="{{ language }}" class="sidebar-visible no-js {{ default_theme }}">
<head>
    <!-- Book generated using mdBook -->
    <meta charset="UTF-8">
    <title>{{ title }}</title>
    {{#if is_print }}
    <meta name="robots" content="noindex" />
    {{/if}}
    {{#if base_url}}
    <base href="{{ base_url }}">
    {{/if}}

    <!-- Custom fonts -->
    <link rel="stylesheet" href="{{ path_to_root }}FontAwesome/css/font-awesome.css">
    {{#if copy_fonts}}
    <link rel="stylesheet" href="{{ path_to_root }}fonts/fonts.css">
    {{/if}}

    <!-- Custom CSS -->
    <link rel="stylesheet" href="{{ path_to_root }}css/variables.css">
    <link rel="stylesheet" href="{{ path_to_root }}css/general.css">
    <link rel="stylesheet" href="{{ path_to_root }}css/chrome.css">
    {{#if print_enable}}
    <link rel="stylesheet" href="{{ path_to_root }}css/print.css" media="print">
    {{/if}}

    <!-- Dragon's Labyrinth Theme -->
    <link rel="stylesheet" href="{{ path_to_root }}variables.css">

    <!-- Provide site root to javascript -->
    <script type="text/javascript">
        var path_to_root = "{{ path_to_root }}";
        var default_theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "{{ preferred_dark_theme }}" : "{{ default_theme }}";
    </script>

    <!-- Work around some values being stored in localStorage wrapped in quotes -->
    <script type="text/javascript">
        try {
            var theme = localStorage.getItem('mdbook-theme');
            var sidebar = localStorage.getItem('mdbook-sidebar');
            if (theme.startsWith('"') && theme.endsWith('"')) {
                localStorage.setItem('mdbook-theme', theme.slice(1, theme.length - 1));
            }
            if (sidebar.startsWith('"') && sidebar.endsWith('"')) {
                localStorage.setItem('mdbook-sidebar', sidebar.slice(1, sidebar.length - 1));
            }
        } catch (e) { }
    </script>

    <!-- Set the theme before any content is loaded -->
    <script type="text/javascript">
        var theme;
        try { theme = localStorage.getItem('mdbook-theme'); } catch(e) { }
        if (theme === null || theme === undefined) { theme = default_theme; }
        var html = document.querySelector('html');
        html.classList.remove('no-js')
        html.classList.remove('{{ default_theme }}')
        html.classList.add(theme);
        html.classList.add('js');
        // Start at dread level 0
        html.classList.add('dread-0');
    </script>
</head>
<body>
    <!-- Main mdBook content will be inserted here -->
    {{{ content }}}

    <!-- mdBook JS -->
    <script src="{{ path_to_root }}book.js" type="text/javascript" charset="utf-8"></script>
    
    <!-- Dragon's Labyrinth Interactive System -->
    <script src="{{ path_to_root }}book.js" type="text/javascript" charset="utf-8"></script>
</body>
</html>"#;
    
    fs::write(theme_dir.join("index.hbs"), index)?;
    Ok(())
}
