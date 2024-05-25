export const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

const favourite_local_storage_key = "favourites";

export function get_favourites() {
    let favourites = localStorage.getItem(favourite_local_storage_key);
    if (!favourites) return {};
    return JSON.parse(favourites);
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

/**
 * 
 * @param {string} percent 
 * @returns {Number}
 */
export function percent_to_decimal(percent) {
    return Number(percent.replace("%", "")) / 100;
}

/**
 * 
 * @param {Number} decimal 
 * @returns {string}
 */
export function decimal_to_percent(decimal) {
    return (decimal * 100).toFixed(2) + "%";
}