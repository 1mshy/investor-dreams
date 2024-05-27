
import { invoke } from '@tauri-apps/api';
import cheerio, { load } from 'cheerio';

const local_storage_key = "s&p500_ticker_name_percent";

/**
 * @description Get the S&P 500 list of companies with their ticker symbol, company name and portfolio percentage
 * @returns {Promise<Object<string, {number: number, company: string, portfolio_percent: string}>}
 */
async function request_top_companies() {
    const text = await invoke('get_index_info');
    const $ = load(text);
    const data = {};
    $('table.table tbody tr').each((index, element) => {
        const cells = $(element).find('td');
        if (cells.length >= 4) {
            const number = index + 1;
            const company = $(cells[1]).text().trim();
            const ticker_symbol = $(cells[2]).text().trim();
            const portfolio_percent = Number($(cells[3]).text().trim().replace("%", ""));
            data[ticker_symbol] = { number, company, portfolio_percent };
        }
    });
    return data;
}
/**
 * @desc exposed caching function for the requesting of the S&P 500 list
 * @returns {Promise<Object<string, {number: number, company: string, portfolio_percent: string}>}
 */
export async function get_sp_500_data() {
    let data = localStorage.getItem(local_storage_key);
    if (!data) {
        data = await request_top_companies();
        localStorage.setItem(local_storage_key, JSON.stringify(data));
    } else {
        data = JSON.parse(data);
    }
    return data;
}


export async function ticker_to_name(ticker_symbol) {
    const data = await get_sp_500_data();
    return data[ticker_symbol].company;
}

export async function get_portfolio_weight(ticker_symbol) {
    const data = await get_sp_500_data();
    return data[ticker_symbol].portfolio_percent;
}