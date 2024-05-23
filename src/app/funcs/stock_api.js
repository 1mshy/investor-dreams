const api_keys = ["62f59cf3c1fe46498ed297915d46dfac"]
const api_url = "https://api.twelvedata.com/time_series?interval=1day&format=JSON&apikey=62f59cf3c1fe46498ed297915d46dfac"

export async function ticker_price(ticker_symbol) {
    const data = await request_ticker_data(ticker_symbol)
    await cache(data);
    return price_from_data(data);
}

export async function request_ticker_data(ticker_symbol) {
    const cached_data = get_cache(ticker_symbol);
    console.log(cached_data)
    if (cached_data) {
        return cached_data;
    }
    const url = `${api_url}&symbol=${ticker_symbol}`;
    const response = await fetch(url)
    const data = await response.json()
    await cache(data)
    return data;
}

async function cache(stock_data) {
    const ticker_symbol = stock_data["meta"]["symbol"];
    localStorage.setItem(`${ticker_symbol}`, JSON.stringify(stock_data))
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
    return stock_data["values"].map(value => Number(value.close))
}

export function change_from_data(stock_data) {
    const current_stock_price = price_from_data(stock_data);
    const yesterday_stock_price = yesterday_close_from_data(stock_data);
    return percent_change(current_stock_price, yesterday_stock_price)
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

function random_key() {
    return api_keys[Math.floor(Math.random() * api_keys.length)]
}