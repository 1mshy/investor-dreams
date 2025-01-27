/**
 * @fileoverview User settings configuration module.
 * Defines global application settings and their default values.
 */

/**
 * Creates and returns the default user settings object.
 * 
 * @function
 * @returns {Object} User settings configuration object
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
        Small_Stock_Widget: {
            display_name: "Small Stock Widget",
            settings: {
                show_month: {
                    display_name: "Show Month price change",
                    value: true,
                },
                show_day: {
                    display_name: "Show Day price change",
                    value: true,
                },
                show_name: {
                    display_name: "Show Company Name",
                    value: true,
                },
                show_graph: {
                    display_name: "Show Graph",
                    value: true,
                }
            }
        },
        Big_Stock_Widget: {
            display_name: "Big Stock Widget",
            settings: {
                show_technicals: {
                    display_name: "Show Technicals section",
                    value: true,
                },
                show_rsi: {
                    display_name: "Show RSI information",
                    value: true,
                },
                show_reddit_data: {
                    display_name: "Show Reddit headlines",
                    value: true,
                },
                show_news: {
                    display_name: "Show News headlines",
                    value: true,
                },
                show_company_info: {
                    display_name: "Show Company Information",
                    value: true,
                }
            }
        },
        TradingView_Widget: {
            display_name: "TradingView Widget",
            settings: {
                show_tradingview_on_home: {
                    display_name: "Show TradingView widget on Home page",
                    value: false,
                },
                dark_theme: {
                    display_name: "Use dark theme",
                    value: true,
                },
                default_symbol: {
                    display_name: "Default symbol",
                    value: "AAPL",
                },
                default_range: {
                    display_name: "Default time range",
                    value: "3M",
                }
            }
        }
    }
}

export const user_settings = defined_user_settings();
