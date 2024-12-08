import { toast } from "react-toastify";
import { get_all_nasdaq_info } from "./scraper";
import { get_state } from "./states";
import { clean_ticker, get_all_symbols, get_all_technical_data, percentage_change, request_yahoo_big } from "./stock_api";
import { delay, unformat_number } from "./tools";

export function filter_tickers(searching_options, all_keys, all_nasdaq_info, all_technical_data) {
    const final_list = [];
    for (let key of all_keys) {
        const data = all_technical_data[key];
        if (!data || !data["data"] || !data["data"]["summaryData"] || !all_nasdaq_info[key]) continue;
        const summaryData = data["data"]["summaryData"];
        const current_price = unformat_number(all_nasdaq_info[key]["lastsale"])
        const price_target = unformat_number(summaryData["OneYrTarget"]["value"])
        const pe_ratio = unformat_number(summaryData["PERatio"]["value"])
        const forward_pe_ratio = unformat_number(summaryData["ForwardPE1Yr"]["value"])
        const divided_yield = unformat_number(summaryData["Yield"]["value"])
        const net_change = unformat_number(all_nasdaq_info[key]["netchange"])
        const percent_change = unformat_number(all_nasdaq_info[key]["pctchange"])
        if (current_price === 0 || price_target === 0) continue;
        const target_percent_difference = percentage_change(price_target, current_price)
        const market_cap = unformat_number(summaryData["MarketCap"]["value"])
        if (isNaN(market_cap)) continue;
        if (market_cap < searching_options.min_market_cap || market_cap > searching_options.max_market_cap) continue;
        const final_data = {
            symbol: key, market_cap, current_price, price_target, percent_change, net_change,
            target_percent_difference, pe_ratio, forward_pe_ratio, divided_yield
        }
        final_list.push(final_data);
    }
    final_list.sort((a, b) => b[searching_options.sort_by] - a[searching_options.sort_by]);
    if (searching_options.reverse) final_list.reverse();
    return final_list.slice(0, searching_options.tickers_shown);
}

export async function filter_tickers_async(searching_options) {
    const all_nasdaq_info = await get_all_nasdaq_info();
    const all_tickers = await get_all_symbols();
    const all_technical_data = await get_all_technical_data();
    return filter_tickers(searching_options, all_tickers, all_nasdaq_info, all_technical_data);
}
/**
 * Requests the historical data of all the known ticker symbols
 */
export async function request_database() {
    const all_symbols = await get_all_symbols();
    const random_num_hash = `${Math.random()}_${Date.now()}`;
    const state_key = 'getting_all_historical_data'
    let state = get_state();
    state[state_key] = random_num_hash; // used to ensure two instances of this function are not running simultaneously.
    let searched_symbols = new Set();
    const time_delta = 2000 // in ms
    const MAX_CHUNK_SIZE = 10; // Number of symbols to fetch at once
    let symbol_chunks = []; // array of the chunks
    let i = 0; // index in all_symbols
    let chunk = []; // current chunk
    const eval_chunks = () => {
        if (chunk.length >= MAX_CHUNK_SIZE || (i >= all_symbols.length - 1 && chunk.length > 0)) {
            symbol_chunks.push(chunk);
            chunk = [];
        }
    }
    while (i < all_symbols.length) {
        eval_chunks();
        const symbol = clean_ticker(all_symbols[i]);
        if(symbol.includes("^") || symbol.includes("/")) {
            i++;
            continue;
        }
        searched_symbols.add(symbol);
        chunk.push(all_symbols[i++]);
        i++;
    }
    eval_chunks();

    for (let chunk of symbol_chunks) {
        const start = Date.now();
        // Skip symbols if skip_cached is true and already cached
        let promises = chunk.map(async symbol => {
            return request_yahoo_big(symbol).then(() => {
                searched_symbols.add(symbol);
            }).catch(() => {
                toast.error(`${symbol} failed to fetch`);
            });
        });
        promises.push(delay(time_delta)); // Delay between each chunk
        await Promise.all(promises); // Wait for all 3 requests to complete
        const end = Date.now();
        console.log(`Chunk took ${(end - start) / 1000}s`);
        if (state[state_key] !== random_num_hash) {
            console.log("Stopping current fetch for technicals");
            toast.warn(`Stopping current fetch for technicals`);
            return;
        }
    }
}