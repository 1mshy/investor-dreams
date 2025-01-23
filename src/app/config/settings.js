/**
 * @fileoverview User settings configuration module.
 * Defines global application settings and their default values.
 */

/**
 * Creates and returns the default user settings object.
 * 
 * @function
 * @returns {Object} User settings configuration object
 * @property {Object} show_relative_prices_on_graph - Setting for relative price display
 * @property {string} show_relative_prices_on_graph.display_name - Human-readable setting name
 * @property {boolean} show_relative_prices_on_graph.value - Whether to show relative prices
 */
const defined_user_settings = () => {
    return {
        show_relative_prices_on_graph: {
            display_name: "Show relative pricing on graphs",
            value: true,
        }
    }
}

export const user_settings = defined_user_settings();
