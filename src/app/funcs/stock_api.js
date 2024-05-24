import { invoke } from "@tauri-apps/api";
import { delay } from "./tools";
require('dotenv').config()
// 62f59cf3c1fe46498ed297915d46dfac first one
let api_key = ""
/**
 * @desc Get the api key from the backend
 */
invoke("get_api_key")
        .then((key) => { api_key = key })
        .catch((error) => console.error(`No api key found!!!: ${error}`));

const api_url = "https://api.twelvedata.com/time_series?interval=1day&format=JSON"
let current_requests_per_minute = 0;
const max_requests_per_minute = 8;
const ONE_MINUTE = 60_000; // 60,000 milliseconds

import { listen } from '@tauri-apps/api/event';

// Function to listen for the 'api-key' event
const listenForApiKey = () => {
    listen('api-key', (event) => {
        console.log('API Key:', event.payload); // Log the API key received from the backend
    }).catch((error) => {
        console.error('Failed to listen to api-key event:', error);
    });
};

// Check if window is defined to ensure code runs in browser context
if (typeof window !== 'undefined') {
    listenForApiKey();
} else {
    console.warn('window is not defined. The code is running in a non-browser environment.');
}
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
    while (current_requests_per_minute >= max_requests_per_minute) {
        console.log("Too many requests, waiting for a minute to request" + ticker_symbol)
        await delay(ONE_MINUTE + 1000); // Wait for a minute
        current_requests_per_minute = 0;
        console.log("Minute over, resuming requests")
    }
    const cached_data = get_cache(ticker_symbol);
    console.log(cached_data);
    if (cached_data) {
        const { last_updated, stock_data } = cached_data;
        const current_hour = Number(Date.now()) / 1000 / 60 / 60;
        const last_updated_hour = Number(last_updated) / 1000 / 60 / 60;
        if (current_hour - last_updated_hour < 1) {
            return stock_data;
        }
    }
    console.log("requesting")
    current_requests_per_minute++;
    const url = `${api_url}&apikey=${api_key}&symbol=${ticker_symbol}`;
    const response = await fetch(url);
    const data = await response.json();
    if (is_error(data)) {
        current_requests_per_minute = max_requests_per_minute;
        return request_ticker_data(ticker_symbol);
    }
    await cache(data);
    return data;
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
 *
 * @param ticker_symbol {string}
 * @returns {Promise<any>}
 */
function get_cache(ticker_symbol) {
    return JSON.parse(localStorage.getItem(`${ticker_symbol.toUpperCase()}`))
}

export function price_from_data(stock_data) {
    return Number(stock_data["values"][0]["close"])
}

export function yesterday_close_from_data(stock_data) {
    return Number(stock_data["values"][1]["close"])
}

export function last_date_from_data(stock_data) {
    return stock_data["values"][0]["datetime"]
}

export function get_list_prices(stock_data) {
    return stock_data["values"].map(value => Number(value.close)).reverse();
}

export function change_from_data(stock_data) {
    const current_stock_price = price_from_data(stock_data);
    const yesterday_stock_price = yesterday_close_from_data(stock_data);
    return percent_change(current_stock_price, yesterday_stock_price)
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