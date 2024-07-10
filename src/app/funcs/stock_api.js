import { invoke } from "@tauri-apps/api";
import { delay } from "./tools";
import { ticker_to_name } from "./scraper";
require('dotenv').config()
let api_keys = []
let all_data = undefined;
/**
 * @desc Get the api key from the backend
 */
invoke("get_api_keys")
    .then((keys) => { api_keys = keys.split(",") })
    .catch((error) => console.error(`No api key found!!!: ${error}`));

const api_url = "https://api.twelvedata.com/time_series?interval=1day&format=JSON"
let stop_requesting = false;
const WAIT_TIME = 61_000; // 61,000 milliseconds

/**
 * @param {string} ticker_symbol 
 * @returns {Promise<{meta:{},values:[]}>}
 * @desc Request stock data from the API
 * @desc This function is rate limited to 8 requests per minute
 * @desc If the limit is reached, the function will wait for a minute and then resume
 * @example
 * const data = await request_ticker_data("AAPL")
 * console.log(data)
 */
export async function request_ticker_data(ticker_symbol) {
    const cached_data = get_cache(ticker_symbol);
    if (cached_data) {
        const { last_updated, stock_data } = cached_data;
        const current_hour = Date.now() / 1000 / 60 / 60;
        const last_updated_hour = Number(last_updated) / 1000 / 60 / 60;
        if (current_hour - last_updated_hour < 1) {
            return stock_data;
        }
    }
    while (stop_requesting) {
        console.log("Too many requests, waiting for a minute to request " + ticker_symbol)
        await delay(WAIT_TIME); // Wait for the cooldown to end
        stop_requesting = false;
        console.log("Minute over, resuming requests")
    }
    console.log("requesting " + ticker_symbol)
    const url = `${api_url}&apikey=${get_next_api_key()}&symbol=${ticker_symbol}`;
    const response = await fetch(url);
    const data = await response.json();
    if (is_error(data)) {
        stop_requesting = true;
        return request_ticker_data(ticker_symbol);
    }
    await cache(data);
    return data;
}
/**
 * 
 * @param {string} ticker_symbol 
 * @desc get information about the ticker symbol to create a stock widget
 */
export async function fetch_widget_data(ticker_symbol) {
    try {
        const company_name = await ticker_to_name(ticker_symbol) // gets the name of the company
        const ticker_data = await request_ticker_data(ticker_symbol); // gets the stock data for the company, mostly historical prices
        // this should never happen, but if it does we should log it
        if (ticker_data === undefined) {
            console.log("Error fetching data for " + ticker_symbol);
            return;
        }

        const price = current_price_from_data(ticker_data);
        const change = change_from_data(ticker_data);
        const change_month = monthly_change_from_data(ticker_data);
        const date = last_date_from_data(ticker_data);
        const historical_prices = get_list_prices(ticker_data);
        console.log(ticker_data)
        let data = {
            symbol: ticker_symbol,
            name: company_name,
            price: price.toFixed(2),
            percent_change: change.toFixed(2),
            percent_change_month: change_month.toFixed(2),
            date: date,
            historical_prices: historical_prices
        };

        return data;
    } catch (error) {
        console.log("Error fetching data for " + ticker_symbol + ": " + error.message);
    }
    return {};
}
/**
 * @returns {Promise<{}>}
 * @example {A: {symbol: 'A', name: 'Agilent Technologies, Inc.', summary: 'Agilent Technologies, Inc. provides application fo… and is headquartered in Santa Clara, California.', currency: 'USD', sector: 'Health Care', …}}
 * @desc possible keys: city, composite_figi, country, currency, description, cusip, exchange, figi, industry, industry_group, isin, market, market_cap, name, sector, shareclass_figi, state, summary, symbol, website, zipcode
*/
export async function get_all_data() {
    if (!all_data)
        all_data = await fetch("/json/index_data.json").then(response => response.json());
    return all_data;
}
/**
 * returns all the possible sectors
 * @returns {Promise<[]>}
 */
export async function get_all_sectors() {
    let sectors = [];
    const data = await get_all_data();
    Object.keys(data).forEach(key => {
        if (!sectors.includes(data[key]["sector"])) {
            sectors.push(data[key]["sector"]);
        }
    });
    sectors = sectors.filter(item => item !== undefined && item !== null && item !== "")
    return sectors.sort();
}

async function cache(stock_data) {
    const ticker_symbol = stock_data["meta"]["symbol"];
    let cache_data = {
        stock_data: stock_data,
        last_updated: Date.now()
    }
    localStorage.setItem(`${ticker_symbol}`, JSON.stringify(cache_data))
}
/**
 * finds the price of a company using the large dataset that is requested every few minutes. 
 * Note: This data is not completely up to date
 */
export async function lazy_price_of_ticker(ticker_symbol) {
    const all_data = await get_sp_500_data();
    return all_data[ticker_symbol].current_price;
}

let current_api_index = 0;
function get_next_api_key() {
    let api_key = api_keys[current_api_index];
    current_api_index = (current_api_index + 1) % api_keys.length;
    return api_key;
}

/**
 *
 * @param ticker_symbol {string}
 * @returns {Promise<any>}
 */
function get_cache(ticker_symbol) {
    return JSON.parse(localStorage.getItem(`${ticker_symbol.toUpperCase()}`))
}

export function current_price_from_data(stock_data) {
    return Number(stock_data["values"][0]["close"])
}

export function yesterday_close_from_data(stock_data) {
    return Number(stock_data["values"][1]["close"])
}
export function price_days_out_from_data(stock_data, days_out) {
    return Number(stock_data["values"][days_out]["close"])
}

export function last_date_from_data(stock_data) {
    return stock_data["values"][0]["datetime"]
}

export function get_list_prices(stock_data) {
    return stock_data["values"].map(value => Number(value.close)).reverse();
}

export function change_from_data(stock_data) {
    const current_stock_price = current_price_from_data(stock_data);
    const yesterday_stock_price = yesterday_close_from_data(stock_data);
    return percent_change(current_stock_price, yesterday_stock_price)
}

export function monthly_change_from_data(stock_data) {
    const THIRTY_DAYS_FROM_TODAY = 29;
    const current_stock_price = current_price_from_data(stock_data);
    const thiry_days_past = price_days_out_from_data(stock_data, THIRTY_DAYS_FROM_TODAY);
    return percent_change(current_stock_price, thiry_days_past);
}

function is_error(stock_data) {
    return stock_data.status === "error";
}

/**
 *
 * @param first {number}
 * @param second {number}
 * @returns {number}
 */
export function percent_change(first, second) {
    return (first - second) / second * 100;

}

export function set_api_key() {
    // api_key = 
}