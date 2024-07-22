
import { invoke } from '@tauri-apps/api';
import { load } from 'cheerio';
import { complex_retrieve, complex_store, retrieve, store } from './cache';

const local_storage_key = "s&p500_ticker_name_percent";

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
    let data = await complex_retrieve(local_storage_key);
    const ten_minutes = 1000 * 60 * 10;
    const date_requested = data ? JSON.parse(data)["time_requested"] : false;
    if (!data || Date.now() - date_requested > ten_minutes) {
        console.log("Requesting top 500 company data from rust backend")
        data = await request_top_companies();
        complex_store(local_storage_key, JSON.stringify(data));
    } else {
        data = JSON.parse(data);
    }
    return data;
}

export async function get_index_stocks() {
    const index_data = await get_sp_500_data();
    return Object.keys(index_data);
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
    console.log(ticker_symbol)
    return data[ticker_symbol].portfolio_percent;
}