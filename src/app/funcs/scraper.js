
import { invoke } from '@tauri-apps/api/core';
import { load } from 'cheerio';
import { complex_retrieve, complex_store, retrieve, store } from './cache';

/**
 * @description Get the S&P 500 list of companies with their ticker symbol, company name and portfolio percentage
 * @returns {Promise<Object<string, {number: number, company: string, portfolio_percent: string, current_price:number, change:number, percent_change:number}>}
 */
async function request_top_companies() {
    const text = await invoke('get_index_info');
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
            data[ticker_symbol] = { number, ticker_symbol, company, portfolio_percent, current_price, change, percent_change };
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
        try{
            console.log("TRYING")
        const nasdaq_info = await get_all_nasdaq_info();
        console.log(nasdaq_info);
        }catch(e){
        }
        complex_store(local_storage_key, data);
    }
    return data;
}
/**
 * gets info on all known stocks using the nasdaq api
 * @returns {["symbol": "Symbol", "name": "Name","lastsale": "Last Sale","netchange": "Net Change","pctchange": "% Change","marketCap": "Market Cap","country": "Country","ipoyear": "IPO Year","volume": "Volume","sector": "Sector","industry": "Industry","url": "Url"]}
 */
export async function get_all_nasdaq_info() {
    const local_storage_key = "NASDAQ_ALL_INFO_TODAY";
    let data = await complex_retrieve(local_storage_key);
    const ten_minutes = 1000 * 60 * 10;
    const date_requested = data ? data["time_requested"] : false;
    // if (!data || Date.now() - date_requested > ten_minutes) {
    if (!data) {
        console.log("Requesting top 500 company data from rust backend")
        const json_data = await invoke('req_nasdaq_info')
        console.log(json_data);
        const all_data = json_data.data.rows; // array of objects
        let new_data = {};
        all_data.forEach((stock) => {
            new_data[stock.symbol] = stock;
        });
        data = new_data;
        complex_store(local_storage_key, data);
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
        }
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
    const data = await get_sp_500_data();
    return data[ticker_symbol].company;
}

export async function get_portfolio_weight(ticker_symbol) {
    const data = await get_sp_500_data();
    return data[ticker_symbol].portfolio_percent;
}