import { retrieve, store } from "../networking/cache";
import { get_all_nasdaq_info } from "../networking/scraper";
import { unformat_number } from "./formatting";

const favourite_local_storage_key = "favourites";

/**
 * Returns the favourite by order of changes in percentage
 * @returns {Promise<String[]>}
 */
export async function top_favourite_changes() {
    const all_data = await get_all_nasdaq_info();
    const all_possible_tickers = Object.keys(all_data);
    const favourites = get_favourite_array();
    let top_3_changes = [];
    for (const ticker_symbol of favourites) {
        if (!all_possible_tickers.includes(ticker_symbol)) continue;
        const stock_data = all_data[ticker_symbol];
        const change = Math.abs(unformat_number(stock_data.pctchange));
        top_3_changes.push({ ticker_symbol, change });
    }
    return top_3_changes.sort((a, b) => b.change - a.change)
    .map(item => item.ticker_symbol);
}

/**
 * 
 * @returns {{}}
 * @example {AAPL: true, JS: true}
 */
export function get_favourites() {
    let favourites = retrieve(favourite_local_storage_key);
    if (!favourites) return {};
    return JSON.parse(favourites);
}
/**
 * gets the favourite symbols in an array
 * @returns {[String]}
 */
export function get_favourite_array() {
    return Object.keys(get_favourites());
}
/**
 * 
 * @returns {Boolean} if there are any favourites
 */
export function has_favourites() {
    return get_favourite_array().length > 0;
}
/**
 * 
 * @param {String} ticker_symbol 
 * @returns {Boolean} if the ticker is a favourite
 */
export function is_ticker_favourite(ticker_symbol) {
    let favourites = get_favourites();
    if (!favourites) return false;
    return favourites[ticker_symbol] === true;
}
/**
 * Add ticker to the favourties object in storage
 * @param {String} ticker_symbol 
 * @returns {void}
 */
export function add_favourite(ticker_symbol) {
    let favourites = get_favourites();
    favourites[ticker_symbol] = true;
    store(favourite_local_storage_key, JSON.stringify(favourites));
}

/**
 * Removed ticker from the favourites object in storage
 * @param {String} ticker_symbol 
 * @returns {void} 
 */
export function remove_favourite(ticker_symbol) {
    let favourites = get_favourites();
    if (!favourites) return;
    delete favourites[ticker_symbol];
    store(favourite_local_storage_key, JSON.stringify(favourites));
}
/**
 * Removes the ticker from favourites if currently a favourite, adds it if not
 * @param {String} ticker_symbol 
 */
export function toggle_favourite(ticker_symbol) {
    if (is_ticker_favourite(ticker_symbol)) {
        remove_favourite(ticker_symbol);
    } else {
        add_favourite(ticker_symbol);
    }
}