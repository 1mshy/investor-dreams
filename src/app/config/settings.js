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
        Global: {
            display_name: "Global Settings",
            settings: {
                show_relative_prices_on_graph: {
                    display_name: "Show relative pricing on graphs",
                    value: true,
                },
            },
        },
        Medium_Stock_Widget: {
            display_name: "Medium Stock Widget",
            settings: {
                show_rsi: {
                    display_name: "Show RSI",
                    value: true,
                },
                show_ytd: {
                    display_name: "Show YTD price change",
                    value: true,
                },
                show_month: {
                    display_name: "Show Month price change",
                    value: true,
                },
                show_day: {
                    display_name: "Show Day price change",
                    value: true,
                },
                show_market_cap: {
                    display_name: "Show Market Cap",
                    value: true,
                }
            }
        },
    }
}

export const user_settings = defined_user_settings();
