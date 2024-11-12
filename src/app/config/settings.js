const defined_user_settings = () => {
    return {
        show_relative_prices_on_graph: {
            display_name: "Show relative pricing on graphs",
            value: true,
        }
    }
}

export const user_settings = defined_user_settings();
