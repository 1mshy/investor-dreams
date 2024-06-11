import { get_sp_500_data } from "./scraper";

const favourite_local_storage_key = "favourites";

export async function top_favourite_changes() {
    const all_data = await get_sp_500_data();
    const favourites = get_favourite_array();
    let top_3_changes = [];
    for (const ticker_symbol of favourites) {
        const stock_data = all_data[ticker_symbol];
        const change = Math.abs(stock_data.percent_change);
        top_3_changes.push({ ticker_symbol, change });
    }
    return top_3_changes.sort((a, b) => b.change - a.change).slice(0, 3).map(item => item.ticker_symbol);
}

/**
 * 
 * @returns {}
 * @example {AAPL: true, JS: true}
 */
export function get_favourites() {
    let favourites = localStorage.getItem(favourite_local_storage_key);
    if (!favourites) return {};
    return JSON.parse(favourites);
}
/**
 * 
 * @returns []
 */
export function get_favourite_array() {
    return Object.keys(get_favourites());
}

export function has_favourites() {
    return get_favourite_array().length > 0;
}

export function is_ticker_favourite(ticker_symbol) {
    let favourites = get_favourites();
    if (!favourites) return false;
    return favourites[ticker_symbol] === true;
}

export function add_favourite(ticker_symbol) {
    let favourites = get_favourites();
    favourites[ticker_symbol] = true;
    localStorage.setItem(favourite_local_storage_key, JSON.stringify(favourites));
}

export function remove_favourite(ticker_symbol) {
    let favourites = get_favourites();
    if (!favourites) return;
    delete favourites[ticker_symbol];
    localStorage.setItem(favourite_local_storage_key, JSON.stringify(favourites));
}

export function toggle_favourite(ticker_symbol) {
    if (is_ticker_favourite(ticker_symbol)) {
        remove_favourite(ticker_symbol);
    } else {
        add_favourite(ticker_symbol);
    }
}