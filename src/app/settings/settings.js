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
                    possible_values: [true, false]
                },
            },
        },
        Medium_Stock_Widget: {
            display_name: "Medium Stock Widget",
            settings: {
                show_rsi: {
                    display_name: "Show RSI",
                    value: true,
                    possible_values: [true, false]
                },
                show_ytd: {
                    display_name: "Show YTD price change",
                    value: true,
                    possible_values: [true, false]
                },
                show_month: {
                    display_name: "Show Month price change",
                    value: true,
                    possible_values: [true, false]
                },
                show_day: {
                    display_name: "Show Day price change",
                    value: true,
                    possible_values: [true, false]
                },
                show_market_cap: {
                    display_name: "Show Market Cap",
                    value: true,
                    possible_values: [true, false]
                }
            }
        },
        Small_Stock_Widget: {
            display_name: "Small Stock Widget",
            settings: {
                show_month: {
                    display_name: "Show Month price change",
                    value: true,
                    possible_values: [true, false]
                },
                show_day: {
                    display_name: "Show Day price change",
                    value: true,
                    possible_values: [true, false]
                },
                show_name: {
                    display_name: "Show Company Name",
                    value: true,
                    possible_values: [true, false]
                },
                show_graph: {
                    display_name: "Show Graph",
                    value: true,
                    possible_values: [true, false]
                }
            }
        },
        Home_Page: {
            display_name: "Home Page Settings",
            settings: {
                default_widget_size: {
                    display_name: "Widget Size",
                    value: "small",
                    possible_values: ["mini", "small", "medium"]
                },
            },
        },
        Playground_Page: {
            display_name: "Playground Page Settings",
            settings: {
                default_widget_size: {
                    display_name: "Widget Size",
                    value: "medium",
                    possible_values: ["mini", "small", "medium"]
                },
            },
        },
        Analysis_Page: {
            display_name: "Analysis Page Settings",
            settings: {
                default_widget_size: {
                    display_name: "Widget Size",
                    value: "small",
                    possible_values: ["mini", "small", "medium"]
                },
            },
        },
        Big_Stock_Widget: {
            display_name: "Big Stock Widget",
            settings: {
                show_technicals: {
                    display_name: "Show Technicals section",
                    value: true,
                    possible_values: [true, false]
                },
                show_company_info: {
                    display_name: "Show Company Information",
                    value: true,
                    possible_values: [true, false]
                },
                show_reddit_data: {
                    display_name: "Show Reddit headlines",
                    value: true,
                    possible_values: [true, false]
                },
                show_news: {
                    display_name: "Show News headlines",
                    value: true,
                    possible_values: [true, false]
                },
                show_financial_data: {
                    display_name: "Show Financial Data",
                    value: true,
                    possible_values: [true, false]
                }
            }
        },
        TradingView_Widget: {
            display_name: "TradingView Widget",
            settings: {
                show_tradingview_on_home: {
                    display_name: "Show TradingView widget on Home page",
                    value: true,
                    possible_values: [true, false]
                },
                show_rsi: {
                    display_name: "Show RSI",
                    value: true,
                    possible_values: [true, false]
                },
                show_macd: {
                    display_name: "Show MACD",
                    value: true,
                    possible_values: [true, false]
                },
                show_bollinger_bands: {
                    display_name: "Show Bollinger Bands",
                    value: false,
                    possible_values: [true, false]
                },
                show_volume: {
                    display_name: "Show Volume",
                    value: false,
                    possible_values: [true, false]
                },
                chart_style: {
                    display_name: "Chart Style",
                    value: "1",
                    possible_values: ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                }
            }
        }
    }
}

export const user_settings = defined_user_settings();
