/**
 * Theme Switcher for Daisy UI 5
 * Provides functionality to change themes dynamically
 */

// Available Daisy UI themes
const AVAILABLE_THEMES = [
    'light', 'dark', 'cupcake', 'bumblebee', 'emerald', 'corporate',
    'synthwave', 'retro', 'cyberpunk', 'valentine', 'halloween', 'garden',
    'forest', 'aqua', 'lofi', 'pastel', 'fantasy', 'wireframe', 'black',
    'luxury', 'dracula', 'cmyk', 'autumn', 'business', 'acid', 'lemonade',
    'night', 'coffee', 'winter', 'dim', 'nord', 'sunset', 'caramellatte',
    'abyss', 'silk'
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
    return document.documentElement.getAttribute('data-theme') || 'light';
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
    const defaultTheme = 'light';

    if (savedTheme && AVAILABLE_THEMES.includes(savedTheme)) {
        changeTheme(savedTheme, false); // Don't persist since it's already saved
    } else {
        changeTheme(defaultTheme, true);
    }
}

/**
 * Toggles between light and dark themes
 */
function toggleDarkMode() {
    const currentTheme = getCurrentTheme();
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
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
    document.addEventListener('DOMContentLoaded', initializeTheme);
} else {
    initializeTheme();
}

// Export functions for module usage (if using modules)
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        changeTheme,
        getCurrentTheme,
        getSavedTheme,
        initializeTheme,
        toggleDarkMode,
        applyRandomTheme,
        getRandomTheme,
        getAvailableThemes,
        AVAILABLE_THEMES
    };
}

// Also make available globally
window.changeTheme = changeTheme;
window.getCurrentTheme = getCurrentTheme;
window.getSavedTheme = getSavedTheme;
window.initializeTheme = initializeTheme;
window.toggleDarkMode = toggleDarkMode;
window.applyRandomTheme = applyRandomTheme;
window.getRandomTheme = getRandomTheme;
window.getAvailableThemes = getAvailableThemes;
