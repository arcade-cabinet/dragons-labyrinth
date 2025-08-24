
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
