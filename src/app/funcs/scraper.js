
import { invoke } from '@tauri-apps/api/core';
import { load } from 'cheerio';
import localforage from 'localforage';
import { get_cache, set_cache } from './cache';
import { clean_ticker, get_company_summary } from './stock_api';

/**
 * gets info on all known stocks using the nasdaq api and returns it as an object in the format:
 * {"symbol" : {"symbol": "Symbol", "name": "Name","lastsale": "Last Sale","netchange": "Net Change","pctchange": "% Change","marketCap": "Market Cap","country": "Country","ipoyear": "IPO Year","volume": "Volume","sector": "Sector","industry": "Industry","url": "Url"}}
 * @returns {Promise<{{}}>} - object with ticker as key and object as value
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

/**
 * Returns the name of the company
 * @param {String} ticker_symbol 
 * @returns {Promise<{string}} - name of the company
 */
export async function ticker_to_name(ticker_symbol) {
    return (await get_all_nasdaq_info())[ticker_symbol].name;
}

/**
 * fetch local json data
 * @param {String} url 
 * @returns {Object} fetch result
 */
// export async function fetch_json(url) {
//     return fetch(url, {
//         headers: {
//             'Content-Type': 'application/json',
//             'Accept': 'application/json'
//         },
//         mode: 'no-cors'
//     }).then(res => res.json());
// }


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

export function get_whole_nasdaq_news_url(url) {
    return `https://www.nasdaq.com${url}`;
}
