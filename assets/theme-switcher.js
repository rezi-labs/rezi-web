/**
 * Theme Switcher for Daisy UI 5
 * Provides functionality to change themes dynamically
 */

// Available themes - Pure Swiss design only
const AVAILABLE_THEMES = [
   'swiss', 'swiss-dark'
];

/**
 * Changes the current theme
 * @param {string} theme - The theme name to apply
 * @param {boolean} persist - Whether to save the theme to localStorage (default: true)
 * @returns {boolean} - True if theme was applied successfully, false otherwise
 */
function changeTheme(theme, persist = true) {
    // Validate theme
    if (!theme || typeof theme !== 'string') {
        console.warn('Theme switcher: Invalid theme provided');
        return false;
    }

    const normalizedTheme = theme.toLowerCase().trim();

    if (!AVAILABLE_THEMES.includes(normalizedTheme)) {
        console.warn(`Theme switcher: Theme "${theme}" is not available. Available themes:`, AVAILABLE_THEMES);
        return false;
    }

    try {
        // Method 1: Set data-theme attribute on html element (primary method for Daisy UI 5)
        const htmlElement = document.documentElement;
        htmlElement.setAttribute('data-theme', normalizedTheme);

        // Method 2: Also set on body for compatibility
        document.body.setAttribute('data-theme', normalizedTheme);

        // Method 3: Update any theme controller inputs
        const themeControllers = document.querySelectorAll('input.theme-controller');
        themeControllers.forEach(controller => {
            if (controller.value === normalizedTheme) {
                controller.checked = true;
            } else {
                controller.checked = false;
            }
        });

        // Persist theme to localStorage if requested
        if (persist) {
            try {
                localStorage.setItem('daisy-theme', normalizedTheme);
            } catch (storageError) {
                console.warn('Theme switcher: Could not save theme to localStorage:', storageError);
            }
        }

        // Dispatch custom event for other components to listen to
        const themeChangeEvent = new CustomEvent('themeChanged', {
            detail: { theme: normalizedTheme, previousTheme: getCurrentTheme() }
        });
        document.dispatchEvent(themeChangeEvent);

        console.log(`Theme switched to: ${normalizedTheme}`);
        return true;

    } catch (error) {
        console.error('Theme switcher: Error changing theme:', error);
        return false;
    }
}

/**
 * Gets the current active theme
 * @returns {string} - The current theme name
 */
function getCurrentTheme() {
    return document.documentElement.getAttribute('data-theme') || 'swiss';
}

/**
 * Loads the saved theme from localStorage
 * @returns {string|null} - The saved theme or null if not found
 */
function getSavedTheme() {
    try {
        return localStorage.getItem('daisy-theme');
    } catch (error) {
        console.warn('Theme switcher: Could not access localStorage:', error);
        return null;
    }
}

/**
 * Initializes the theme system
 * Loads the saved theme or applies a default theme
 */
function initializeTheme() {
    const savedTheme = getSavedTheme();
    const defaultTheme = 'swiss'; // Swiss design as default

    if (savedTheme && AVAILABLE_THEMES.includes(savedTheme)) {
        changeTheme(savedTheme, false); // Don't persist since it's already saved
    } else {
        changeTheme(defaultTheme, true);
    }
    
    // Initialize toggle state after theme is set
    setTimeout(initializeThemeToggle, 100);
}

/**
 * Toggles between Swiss light and dark themes only
 */
function toggleTheme() {
    const currentTheme = getCurrentTheme();
    const newTheme = currentTheme === 'swiss-dark' ? 'swiss' : 'swiss-dark';
    
    changeTheme(newTheme);
    
    // Update the checkbox state to match the theme
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.checked = (newTheme === 'swiss');
    }
}

/**
 * Initializes the theme toggle checkbox state
 */
function initializeThemeToggle() {
    const currentTheme = getCurrentTheme();
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.checked = (currentTheme === 'swiss');
    }
}

/**
 * Toggles between Swiss light and dark themes only
 */
function toggleDarkMode() {
    const currentTheme = getCurrentTheme();
    const newTheme = currentTheme === 'swiss-dark' ? 'swiss' : 'swiss-dark';
    
    changeTheme(newTheme);
}

/**
 * Gets a random theme from available themes
 * @returns {string} - A random theme name
 */
function getRandomTheme() {
    const randomIndex = Math.floor(Math.random() * AVAILABLE_THEMES.length);
    return AVAILABLE_THEMES[randomIndex];
}

/**
 * Applies a random theme
 */
function applyRandomTheme() {
    const randomTheme = getRandomTheme();
    changeTheme(randomTheme);
}

/**
 * Gets all available themes
 * @returns {string[]} - Array of available theme names
 */
function getAvailableThemes() {
    return [...AVAILABLE_THEMES];
}

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
        initializeTheme();
        // Ensure functions are available globally
        window.toggleTheme = toggleTheme;
        window.initializeThemeToggle = initializeThemeToggle;
    });
} else {
    initializeTheme();
    // Ensure functions are available globally
    window.toggleTheme = toggleTheme;
    window.initializeThemeToggle = initializeThemeToggle;
}

// Export functions for module usage (if using modules)
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        changeTheme,
        getCurrentTheme,
        getSavedTheme,
        initializeTheme,
        toggleDarkMode,
        toggleTheme,
        initializeThemeToggle,
        applyRandomTheme,
        getRandomTheme,
        getAvailableThemes,
        AVAILABLE_THEMES
    };
}

// Also make available globally (declared early to ensure availability)
window.changeTheme = changeTheme;
window.getCurrentTheme = getCurrentTheme;
window.getSavedTheme = getSavedTheme;
window.initializeTheme = initializeTheme;
window.toggleDarkMode = toggleDarkMode;
window.toggleTheme = toggleTheme;
window.initializeThemeToggle = initializeThemeToggle;
window.applyRandomTheme = applyRandomTheme;
window.getRandomTheme = getRandomTheme;
window.getAvailableThemes = getAvailableThemes;

console.log('Theme switcher loaded, toggleTheme available:', typeof window.toggleTheme);
