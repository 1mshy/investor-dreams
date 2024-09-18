import { invoke } from "@tauri-apps/api/core";
import localforage from "localforage";
import { cache_is_valid, complex_retrieve, get_cache, set_cache, STOCK_CACHE, stock_cache_is_valid } from "./cache";
import { get_all_nasdaq_info, ticker_to_name } from "./scraper";
import { delay, invoke_with_timeout, unformat_number } from "./tools";
let api_keys = []
/**
 * data on stock tickers, not related to price
 * ex: location, sector, industry, etc
 */
let all_data = undefined;
/**
 * @desc Get the api key from the backend
 */
invoke("get_api_keys")
    .then((keys) => {
        api_keys = keys.split(",");
        if (Math.random() > 0.5) {
            api_keys = api_keys.reverse();
        }
    })
    .catch((error) => console.error(`No api key found!!!: ${error}`));

const api_url = "https://api.twelvedata.com/time_series?interval=1day&format=JSON&outputsize=5000"
let stop_requesting = false;
const WAIT_TIME = 61_000; // milliseconds

/**
 * Cleans unwanted characters from a ticker symbol 
 * @param {String} ticker 
 * @returns {String}
 */
export function clean_ticker(ticker) {
    if (!ticker) return "";
    return `${ticker}`.replace("/", ".");
}

/**
 * @param {String} ticker_symbol 
 * @returns {Promise<{meta:{},values:[]}>}
 * @desc Request stock data from the API
 * @desc This function is rate limited to 8 requests per minute
 * @desc If the limit is reached, the function will wait for a minute and then resume
 * @example
 * const data = await request_ticker_data("AAPL")
 * console.log(data)
 */
export async function request_ticker_data(ticker_symbol) {
    if (api_keys.length === 0) {
        console.log("Cannot request ticker data as there is not api keys available")
        return;
    }
    ticker_symbol = clean_ticker(ticker_symbol);
    // console.log(ticker_symbol, stock_cache_is_valid(ticker_symbol))
    const valid_cache = await stock_cache_is_valid(ticker_symbol);
    if (valid_cache) {
        const cache = await get_cache(ticker_symbol, STOCK_CACHE);
        return cache.stock_data;
    }
    while (stop_requesting) {
        console.log("Too many requests, waiting for a minute to request " + ticker_symbol)
        await delay(WAIT_TIME); // Wait for the cooldown to end
        stop_requesting = false;
        console.log("Minute over, resuming requests")
    }
    console.log("requesting " + ticker_symbol)
    const url = `${api_url}&apikey=${get_next_api_key()}&symbol=${ticker_symbol}`;
    const response = await invoke("get_request_api", { url: url });
    const data = JSON.parse(response);
    if (is_error(data)) {
        stop_requesting = true;
        return await request_ticker_data(ticker_symbol);
    }
    set_cache(ticker_symbol, { stock_data: data }, 30, STOCK_CACHE);
    return data;
}

/**
 * 
 * @param {String} ticker_symbol 
 * @desc get information about the ticker symbol to create a stock widget
 */
export async function fetch_widget_data(ticker_symbol) {
    ticker_symbol = clean_ticker(ticker_symbol);
    const company_name = await ticker_to_name(ticker_symbol) // gets the name of the company
    const ticker_data = await request_ticker_data(ticker_symbol); // gets the stock data for the company, mostly historical prices
    const nasdaq_info = await get_all_nasdaq_info(); // gets the info on the company
    const nasdaq_news = await get_ticker_news(ticker_symbol);
    const nasdaq_technicals = await get_ticker_technicals(ticker_symbol);

    const news = nasdaq_news.data && nasdaq_news.data.rows ? nasdaq_news.data.rows : [];
    const technicals = nasdaq_technicals.data && nasdaq_technicals.data.summaryData ? nasdaq_technicals.data.summaryData : {};
    let nasdaq_ticker_info = nasdaq_info[ticker_symbol] ? nasdaq_info[ticker_symbol] : {};
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
    return {
        symbol: ticker_symbol,
        name: company_name,
        price: price.toFixed(2),
        percent_change: change.toFixed(2),
        percent_change_month: change_month.toFixed(2),
        date: date,
        historical_prices: historical_prices,
        news: news,
        technicals: technicals,
        ...nasdaq_ticker_info,
    };
}
/**
 * @returns {Promise<{}>}
 * @example {A: {symbol: 'A', name: 'Agilent Technologies, Inc.', summary: 'Agilent Technologies, Inc. provides application fo… and is headquartered in Santa Clara, California.', currency: 'USD', sector: 'Health Care', …}}
 * @desc possible keys: city, composite_figi, country, currency, description, cusip, exchange, figi, industry, industry_group, isin, market, market_cap, name, sector, shareclass_figi, state, summary, symbol, website, zipcode
*/
export async function get_all_static_ticker_info() {
    if (!all_data)
        all_data = await fetch("/json/global_stock_info.json").then(response => response.json());
    return all_data;
}

export async function get_ticker_info(ticker) {
    const data = await get_all_static_ticker_info();
    return data[ticker];
}

/**
 * returns all the possible sectors
 * @returns {Promise<[]>}
 */
export async function get_all_sectors() {
    let sectors = [];
    const data = await get_all_static_ticker_info();
    Object.keys(data).forEach(key => {
        if (!sectors.includes(data[key]["sector"])) {
            sectors.push(data[key]["sector"]);
        }
    });
    sectors = sectors.filter(item => item !== undefined && item !== null && item !== "")
    return sectors.sort();
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

export async function get_all_symbols() {
    const nasdaq_info = await get_all_nasdaq_info();
    // deleting custom keys
    delete nasdaq_info["last_updated"];
    delete nasdaq_info["expiration"];
    return Object.keys(nasdaq_info).map(ticker => ticker.replace("/", "."))
}
/**
 * 
 * @param {String} ticker_symbol 
 * @returns {Promise<Number>}
 */
export async function get_market_cap(ticker_symbol) {
    const all_tickers = await get_all_nasdaq_info();
    const ticker_info = all_tickers[ticker_symbol];
    if (!ticker_info) return 0;
    console.log(ticker_info)
    return unformat_number(ticker_info["marketCap"]);
}
/**
 * Get all nasdaq ticker sortd by one of the possible sorting methods
 * Sort method possible values: "symbol,name,lastsale,netchange,pctchange,marketCap,country,ipoyear,volume,sector,industry,url"
 * @returns {Promise<[String]>}
 */
export async function nasdaq_sorted_by(sort_method = "marketCap", ticker_list = []) {
    // sorting methods that are numbers
    const numbered_sorting = ["lastsale", "netchange", "pctchange", "marketCap", "ipoyear", "volume"];
    const all_tickers = await get_all_nasdaq_info();
    if (ticker_list.length === 0) {
        ticker_list = Object.keys(all_tickers);
    }
    const tickers_with_market_cap = ticker_list.map(ticker => {
        const raw_value = all_tickers[ticker][sort_method];
        if(!raw_value) return null;
        let formatted_value;
        if (numbered_sorting.includes(sort_method)) {
            formatted_value = unformat_number(raw_value);
            if (isNaN(formatted_value)) {
                return null;
            }
        } else {
            formatted_value = raw_value;
        }
        return {
            ticker: ticker,
            sorting_variable: formatted_value,
        }
    });
    return tickers_with_market_cap
        .filter(a => a !== null) // removes defective values
        .sort((a, b) => b.sorting_variable - a.sorting_variable)
        .map(item => item.ticker);
}

/**
 * 
 * @param {Object} stock_data 
 * @returns {Number}
 */
export function current_price_from_data(stock_data) {
    return Number(stock_data["values"][0]["close"])
}
/**
 * 
 * @param {Object} stock_data 
 * @returns {Number}
 */
export function yesterday_close_from_data(stock_data) {
    return Number(stock_data["values"][1]["close"])
}
/**
 * 
 * @param {Object} stock_data 
 * @param {Number} days_out 
 * @returns {Number}
 */
export function price_days_out_from_data(stock_data, days_out) {
    return Number(stock_data["values"][days_out]["close"])
}
/**
 * 
 * @param {Object} stock_data 
 * @returns {Number}
 */
export function last_date_from_data(stock_data) {
    return stock_data["values"][0]["datetime"]
}
/**
 * 
 * @param {Object} stock_data 
 * @returns {[number]}
 */
export function get_list_prices(stock_data) {
    return stock_data["values"].map(value => Number(value.close)).reverse();
}
/**
 * 
 * @param {Object} stock_data 
 * @returns {Number}
 */
export function change_from_data(stock_data) {
    const current_stock_price = current_price_from_data(stock_data);
    const yesterday_stock_price = yesterday_close_from_data(stock_data);
    return percentage_change(current_stock_price, yesterday_stock_price)
}

export function monthly_change_from_data(stock_data) {
    const THIRTY_DAYS_FROM_TODAY = 29;
    const current_stock_price = current_price_from_data(stock_data);
    const thiry_days_past = price_days_out_from_data(stock_data, THIRTY_DAYS_FROM_TODAY);
    return percentage_change(current_stock_price, thiry_days_past);
}

function is_error(stock_data) {
    return stock_data.status === "error";
}

/**
 * 
 * @param {Number} current 
 * @param {Number} old 
 * @returns {Number}
 */
export function percentage_change(current, old) {
    return (current - old) / old * 100;

}

export function set_api_key() {
    // api_key = 
}



/**
 * NASDAQ APIS HERE
 */

export const NASDAQ_NEWS = localforage.createInstance({
    name: "nasdaq_news_list"
})

/**
 * @param {String} ticker 
 * @returns {Promise<{"data":{"message":null,"rows":[{"ago":"1 hour ago","created":"Aug 21, 2024","id":22248436,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"aapl","primarytopic":"Markets|4006","publisher":"Validea","related_symbols":["aapl|stocks"],"title":"AAPL Factor-Based Stock Analysis","url":"/articles/aapl-factor-based-stock-analysis-22"},{"ago":"1 hour ago","created":"Aug 21, 2024","id":22249031,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"sdig","primarytopic":"Pre-Market|4291","publisher":"NASDAQ.com","related_symbols":["sdig|stocks","jd|stocks","nvda|stocks","sqqq|etf","aapl|stocks","tsll|etf","tgt|stocks","m|stocks","dell|stocks","nio|stocks","hd|stocks"],"title":"Pre-Market Most Active for Aug 21, 2024 :  SDIG, JD, NVDA, SQQQ, AAPL, TSLL, TGT, M, DELL, NIO, HD, F","url":"/articles/pre-market-most-active-aug-21-2024-sdig-jd-nvda-sqqq-aapl-tsll-tgt-m-dell-nio-hd-f"},{"ago":"3 hours ago","created":"Aug 21, 2024","id":22247771,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"","primarytopic":"Technology|4001","publisher":"Zacks","related_symbols":["jhml|etf","aapl|stocks","msft|stocks","nvda|stocks","spy|etf","ivv|etf"],"title":"Is John Hancock Multifactor Large Cap ETF (JHML) a Strong ETF Right Now?","url":"/articles/john-hancock-multifactor-large-cap-etf-jhml-strong-etf-right-now-0"},{"ago":"4 hours ago","created":"Aug 21, 2024","id":22247106,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"aapl","primarytopic":"Markets|4006","publisher":"The Motley Fool","related_symbols":["aapl|stocks","brk.a|stocks","brk.b|stocks","ko|stocks"],"title":"2 No-Brainer Warren Buffett Stocks to Buy Right Now","url":"/articles/2-no-brainer-warren-buffett-stocks-buy-right-now-3"},{"ago":"4 hours ago","created":"Aug 21, 2024","id":22247041,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"amd","primarytopic":"Markets|4006","publisher":"The Motley Fool","related_symbols":["amd|stocks","googl|stocks","msft|stocks","aapl|stocks","amzn|stocks","goog|stocks"],"title":"2 Artificial Intelligence (AI) Stocks to Buy After a Tech Market Sell-Off","url":"/articles/2-artificial-intelligence-ai-stocks-buy-after-tech-market-sell"},{"ago":"4 hours ago","created":"Aug 21, 2024","id":22246986,"image":"/2023/10/09/wall-street-brendan-mcdermid-reuters.jpeg","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"brk.b","primarytopic":"Markets|4006","publisher":"The Motley Fool","related_symbols":["brk.b|stocks","brk.a|stocks","bac|stocks","aapl|stocks","cvx|stocks","siri|stocks","ulta|stocks","oxy|stocks","cb|stocks","itocy|stocks","itocf|stocks","maruy|stocks","mitsy|stocks","lsxma|stocks","lsxmk|stocks","msbhf|stocks","mitsf|stocks"],"title":"Here Are All 45 Stocks Warren Buffett Holds for Berkshire Hathaway's $314 Billion Portfolio","url":"/articles/here-are-all-45-stocks-warren-buffett-holds-berkshire-hathaways-314-billion-portfolio"},{"ago":"5 hours ago","created":"Aug 21, 2024","id":22247201,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"","primarytopic":"Technology|4001","publisher":"Zacks","related_symbols":["aapl|stocks","ko|stocks","nvda|stocks","sgu|stocks","awre|stocks"],"title":"The Zacks Analyst Blog Highlights Apple, NVIDIA, Coca-Cola, Star Group and Aware","url":"/articles/zacks-analyst-blog-highlights-apple-nvidia-coca-cola-star-group-and-aware"},{"ago":"17 hours ago","created":"Aug 20, 2024","id":22245821,"image":"/barchart/Technology%2520abstract%2520by%2520TU%2520IS%2520via%2520iStock.jpg","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"","primarytopic":"Stocks|4301","publisher":"Barchart","related_symbols":["aapl|stocks","ttdky|stocks","tm|stocks","nsany|stocks","hmc|stocks","nvda|stocks","tsla|stocks"],"title":"1 Apple Supplier to Buy on Its Solid-State Battery Breakthrough","url":"/articles/1-apple-supplier-buy-its-solid-state-battery-breakthrough"},{"ago":"21 hours ago","created":"Aug 20, 2024","id":22244396,"image":"","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"","primarytopic":"Technology|4001","publisher":"Zacks","related_symbols":["ge|stocks","aapl|stocks","ko|stocks","pfe|stocks","nvda|stocks","tmo|stocks","sgu|stocks","awre|stocks"],"title":"Top Analyst Reports for Apple, NVIDIA \u0026 Coca-Cola","url":"/articles/top-analyst-reports-apple-nvidia-coca-cola"},{"ago":"22 hours ago","created":"Aug 20, 2024","id":22242766,"image":"/barchart/Double%2520explosure%2520with%2520businesss%2520charts%2520and%2520financial%2520district%2520of%2520megapolis%2520city%2520by%2520Golden%2520Dayz%2520via%2520Shutterstock.jpg","imagedomain":"https://www.nasdaq.com/sites/acquia.prod/files","primarysymbol":"","primarytopic":"Stocks|4301","publisher":"Barchart","related_symbols":["aapl|stocks","nke|stocks","arm|stocks","rig|stocks","ulta|stocks","brk.b|stocks","kmi|stocks"],"title":"5 'Buy'-Rated Stocks Billionaires Were Buying in Q2","url":"/articles/5-buy-rated-stocks-billionaires-were-buying-q2"}],"totalrecords":4997},"message":null,"status":{"rCode":200,"bCodeMessage":null,"developerMessage":null}}>}
 */
export async function get_ticker_news(ticker) {
    ticker = clean_ticker(ticker);
    const local_storage_key = `${ticker}`;
    let cached_news = await get_cache(local_storage_key, NASDAQ_NEWS);
    if (cached_news) {
        return cached_news;
    }
    const amount_of_articles = 15;
    const url = `https://www.nasdaq.com/api/news/topic/articlebysymbol?q=${ticker}|STOCKS&offset=0&limit=${amount_of_articles}&fallback=true`;
    const news_data = await invoke("get_request_api", { url: url });
    const parsed_news = JSON.parse(news_data);
    set_cache(local_storage_key, parsed_news, 60, NASDAQ_NEWS);
    return parsed_news;
}

export async function get_company_summary(ticker) {
    const local_storage_key = `SUMMARY_${ticker}`;
    let cached_summary = await get_cache(local_storage_key);
    if (cached_summary) {
        return cached_summary;
    }
    const url = `https://api.nasdaq.com/api/company/${ticker}/company-profile`;
    const summary_data = await invoke("get_request_api", { url: url });
    const parsed_summary = JSON.parse(summary_data);
    set_cache(local_storage_key, parsed_summary, 60 * 24 * 7);
    return parsed_summary;
}


export const NASDAQ_TECHNICALS = localforage.createInstance({
    name: "nasdaq_technicals"
})

export async function get_cached_ticker_technicals(ticker) {
    const local_storage_key = `${ticker}`;
    let cached_technicals = await complex_retrieve(local_storage_key, NASDAQ_TECHNICALS);
    return cached_technicals;
}
/**
 * 
 * @param {String} ticker 
 * @returns {Promise<{"data":{"symbol":"AAPL","summaryData":{"Exchange":{"label":"Exchange","value":"NASDAQ-GS"},"Sector":{"label":"Sector","value":"Technology"},"Industry":{"label":"Industry","value":"Computer Manufacturing"},"OneYrTarget":{"label":"1 Year Target","value":"$250.00"},"TodayHighLow":{"label":"Today's High/Low","value":"$227.0699/$225.91"},"ShareVolume":{"label":"Share Volume","value":"5,436,150"},"AverageVolume":{"label":"Average Volume","value":"67,622,607"},"PreviousClose":{"label":"Previous Close","value":"$226.51"},"FiftTwoWeekHighLow":{"label":"52 Week High/Low","value":"$237.23/$164.075"},"MarketCap":{"label":"Market Cap","value":"3,452,099,305,850"},"PERatio":{"label":"P/E Ratio","value":37.03},"ForwardPE1Yr":{"label":"Forward P/E 1 Yr.","value":"33.91"},"EarningsPerShare":{"label":"Earnings Per Share(EPS)","value":"$6.12"},"AnnualizedDividend":{"label":"Annualized Dividend","value":"$1.00"},"ExDividendDate":{"label":"Ex Dividend Date","value":"Aug 12, 2024"},"DividendPaymentDate":{"label":"Dividend Pay Date","value":"Aug 15, 2024"},"Yield":{"label":"Current Yield","value":"0.44%"}},"assetClass":"STOCKS","additionalData":null,"bidAsk":{"Bid * Size":{"label":"Bid * Size","value":"$227.05 * 206"},"Ask * Size":{"label":"Ask * Size","value":"$227.07 * 247"}}},"message":null,"status":{"rCode":200,"bCodeMessage":null,"developerMessage":null}}>}
 */
export async function get_ticker_technicals(ticker) {
    ticker = clean_ticker(ticker);
    const local_storage_key = `${ticker}`;
    const cached_technicals = await get_cached_ticker_technicals(ticker);
    const is_cache_valid = await cache_is_valid(local_storage_key, cached_technicals);
    if (is_cache_valid)
        return cached_technicals;
    const url = `https://api.nasdaq.com/api/quote/${ticker}/summary?assetclass=stocks`;
    const technical_data = await invoke_with_timeout("get_request_api", { url: url });
    console.log(technical_data)
    const parsed_technicals = JSON.parse(technical_data);
    set_cache(local_storage_key, parsed_technicals, 60, NASDAQ_TECHNICALS);
    return parsed_technicals;
}

export async function get_all_historical_keys() {
    return STOCK_CACHE.keys();
}

export async function get_all_technical_data_keys() {
    return NASDAQ_TECHNICALS.keys();
}
/**
 * Deletes all keys in the technical data cache
 * @returns {Promise<void>}
 */
export async function clear_all_technical_data() {
    return NASDAQ_TECHNICALS.clear();
}

export async function export_all_historical_data() {
    const keys = await get_all_historical_keys();
    const all_data = await Promise.all(keys.map(key => complex_retrieve(key, STOCK_CACHE)));
    return all_data;
}

export async function export_all_technical_data() {
    const keys = await get_all_technical_data_keys();
    const all_technicals = await Promise.all(keys.map(key => get_cached_ticker_technicals(key)));
    return all_technicals;
}

export const OLLAMA_GENERATION = localforage.createInstance({
    name: "ollama_prompt_generations"
})

/**
 * Ollama wrapper
 * @param {String} prompt 
 * @returns {Promise<String>}
 */
export async function generate_ollama_message(prompt) {
    OLLAMA_GENERATION.keys().then(keys => {
        console.log(keys)
    })
    const cached = await get_ollama_cached_generation(prompt);
    if (cached)
        return cached

    const generated = await invoke("ollama_generate", { prompt });
    await OLLAMA_GENERATION.setItem(prompt, generated);
    return generated;
}

export async function get_ollama_cached_generation(prompt) {
    return OLLAMA_GENERATION.getItem(prompt);
}