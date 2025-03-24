/***
 * POLYGON API FUNCTIONS
 * MUST BE USED WITHIN POLYGON API
 * 
 */

import { get_request } from "./stock_api";

export function poly_market_overview() {
    return get_request("https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/tickers?apiKey=" + get_poly_key());
}

export function has_poly_key() {
    return localforage.getItem("polygon_key") !== null;
}

export function get_poly_key() {
    return localforage.getItem("polygon_key");
}

export function set_poly_key(key) {
    return localforage.setItem("polygon_key", key);
}


