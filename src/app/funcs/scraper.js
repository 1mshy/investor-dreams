
import { invoke } from '@tauri-apps/api/core';
import { load } from 'cheerio';
import localforage from 'localforage';
import { complex_retrieve, complex_store, get_cache, set_cache } from './cache';
import { clean_ticker, get_company_summary } from './stock_api';

/**
 * @description Get the S&P 500 list of companies with their ticker symbol, company name and portfolio percentage
 * @returns {Promise<Object<string, {number: number, company: string, portfolio_percent: string, current_price:number, change:number, percent_change:number}>}
 */
async function request_top_companies() {
    const text = await invoke('get_all_static_ticker_info');
    const nasdaq_info = await get_all_nasdaq_info();
    const $ = load(text);
    const data = {};
    $('table.table tbody tr').each((index, element) => {
        const cells = $(element).find('td');
        // TODO create a regex function to remove all stuff that can be around a number to cause it to be parsed null
        if (cells.length >= 4) {
            const number = index + 1;
            const company = $(cells[1]).text().trim();
            const ticker_symbol = $(cells[2]).text().trim();
            const portfolio_percent = Number($(cells[3]).text().trim().replace("%", ""));
            const current_price = Number($(cells[4]).text().trim().replace(",", ""));
            const change = Number($(cells[5]).text().trim());
            const percent_change = Number($(cells[6]).text().trim().replace(/%(|)|\(|\)/g, "")); //replaces '%', '(', ')'
            if (!isNaN(ticker_symbol)) return;
            data[ticker_symbol] = { number, ticker_symbol, company, portfolio_percent, current_price, change, percent_change, ...nasdaq_info[ticker_symbol] };
        }
    });
    data["time_requested"] = Date.now();
    return data;
}
/**
 * @desc exposed caching function for the requesting of the S&P 500 list
 *@returns {Promise<Object<string, {number: number, company: string, portfolio_percent: string, current_price:number, change:number, percent_change:number}>}
 */
export async function get_sp_500_data() {
    const local_storage_key = "s&p500_ticker_name_percent";
    let data = await complex_retrieve(local_storage_key);
    const ten_minutes = 1000 * 60 * 10;
    const date_requested = data ? data["time_requested"] : false;
    if (!data || Date.now() - date_requested > ten_minutes) {
        console.log("Requesting top 500 company data from rust backend")
        data = await request_top_companies();
        complex_store(local_storage_key, data);
    }
    return data;
}
/**
 * gets info on all known stocks using the nasdaq api
 * @returns {Promise<["symbol": "Symbol", "name": "Name","lastsale": "Last Sale","netchange": "Net Change","pctchange": "% Change","marketCap": "Market Cap","country": "Country","ipoyear": "IPO Year","volume": "Volume","sector": "Sector","industry": "Industry","url": "Url"]>}
 */
export async function get_all_nasdaq_info() {
    const local_storage_key = "NASDAQ_ALL_INFO_TODAY";
    let data = await get_cache(local_storage_key);
    if (!data) {
        const raw_json_data = await invoke('req_nasdaq_info')
        const json_data = JSON.parse(raw_json_data);
        const all_data = json_data.data.rows; // array of objects
        let new_data = {};
        all_data.forEach((stock) => {
            new_data[clean_ticker(stock.symbol)] = stock;
        });
        data = new_data;
        set_cache(local_storage_key, data, 2);
    }
    return data;
}

export async function get_index_stocks() {
    const index_data = await get_sp_500_data();
    return Object.keys(index_data);
}

/**
 * fetch local json data
 * @param {String} url 
 * @returns {Object} fetch result
 */
export async function fetch_json(url) {
    return fetch(url, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        },
        mode: 'no-cors'
    }).then(res => res.json());
}
/**
 * Returns the change from the last list of all the stocks
 * It is not always spot on.
 */
export async function get_lazy_percent_change(ticker_symbol) {
    const data = await get_sp_500_data();
    return data[ticker_symbol].percent_change;
}

export async function ticker_to_name(ticker_symbol) {
    const data = await get_company_summary();
    if(!data["data"]) return "Unknown";
    return data["data"]["CompanyName"]["value"];
}

export async function get_portfolio_weight(ticker_symbol) {
    const data = await get_sp_500_data();
    if(!data[ticker_symbol]) return 0;
    return data[ticker_symbol].portfolio_percent;
}


/**
 * NASDAQ SCRAPING
 */

const NASDAQ_SCRAPED_STORAGE = localforage.createInstance({
    name: "nasdaq_scraped_storage"
})

export async function get_all_news_bodies(news_list, symbol) {
    let promises = [];
    for (let news of news_list) {
        promises.push(get_nasdaq_news_body(news, symbol));
    }
    const news_bodies = await Promise.all(promises)
    return news_bodies.join('');
}

export async function get_nasdaq_news_body(news, symbol) {
    const url = get_whole_nasdaq_news_url(news.url);
    const key = `${symbol}_${news.url}`;
    const cached_item = await NASDAQ_SCRAPED_STORAGE.getItem(key);
    if (cached_item) return cached_item;
    const base_html = await invoke("get_request_api", { url });
    // console.log(base_html);
    const $ = load(base_html);
    const divs = $(".body__content")
    let paragraph_content = `${news.title}:\n`
    divs.each((i, div) => {
        const possibilities = $(div).text().split("\n");
        for (let message of possibilities) {
            const illegal_words = [
                " var ", "function(", "function (", "==", ".push", "document.", "googletag", "[0]"
            ]
            // for (let illegal_word of illegal_words) {
            //     if (message.includes(illegal_word))
            //         continue;
            // }
            if (message.length > 1000 && message.includes(`${symbol}`)) {
                // console.log(message)
                paragraph_content += message.trim();
            }
            // console.log(message)
        }
        paragraph_content += "\n"
    })
    await NASDAQ_SCRAPED_STORAGE.setItem(key, paragraph_content);
    return paragraph_content;
}

export async function get_whole_nasdaq_news_url(url) {
    return `https://www.nasdaq.com${url}`;
}
