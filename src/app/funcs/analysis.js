import { toast } from "react-toastify";
import { get_all_nasdaq_info } from "../networking/scraper";
import { get_state } from "./states";
import { get_all_symbols, get_all_technical_data, percentage_change, request_yahoo_big, fetch_widget_data, get_all_technical_data_keys } from "../networking/stock_api";
import { delay } from "./tools";
import { unformat_number } from "./formatting";
import { clean_ticker } from "./formatting";

export function filter_tickers(searching_options, all_keys, all_nasdaq_info, all_technical_data) {
    const final_list = [];
    
    if (!all_keys || !all_nasdaq_info || !all_technical_data) {
        console.warn("Missing data for filtering tickers");
        return final_list;
    }
    
    for (let key of all_keys) {
        try {
            const data = all_technical_data[key];
            if (!data || !data["data"] || !data["data"]["summaryData"] || !all_nasdaq_info[key]) continue;
            
            const summaryData = data["data"]["summaryData"];
            const nasdaq_data = all_nasdaq_info[key];
            
            const current_price = unformat_number(nasdaq_data["lastsale"]);
            const price_target = unformat_number(summaryData["OneYrTarget"]["value"]);
            const pe_ratio = unformat_number(summaryData["PERatio"]["value"]);
            const forward_pe_ratio = unformat_number(summaryData["ForwardPE1Yr"]["value"]);
            const divided_yield = unformat_number(summaryData["Yield"]["value"]);
            const net_change = unformat_number(nasdaq_data["netchange"]);
            const percent_change = unformat_number(nasdaq_data["pctchange"]);
            const market_cap = unformat_number(summaryData["MarketCap"]["value"]);
            
            // Skip if essential data is missing or invalid
            if (current_price === 0 || isNaN(market_cap) || market_cap <= 0) continue;
            
            // Apply market cap filter
            if (market_cap < searching_options.min_market_cap || market_cap > searching_options.max_market_cap) continue;
            
            // Calculate target percent difference (handle case where price_target might be 0 or missing)
            const target_percent_difference = (price_target > 0) ? percentage_change(price_target, current_price) : 0;
            
            const final_data = {
                symbol: key, 
                market_cap, 
                current_price, 
                price_target, 
                percent_change: isNaN(percent_change) ? 0 : percent_change, 
                net_change: isNaN(net_change) ? 0 : net_change,
                target_percent_difference, 
                pe_ratio: isNaN(pe_ratio) ? 0 : pe_ratio, 
                forward_pe_ratio: isNaN(forward_pe_ratio) ? 0 : forward_pe_ratio, 
                divided_yield: isNaN(divided_yield) ? 0 : divided_yield
            };
            
            final_list.push(final_data);
        } catch (error) {
            console.warn(`Error processing ticker ${key}:`, error);
            continue;
        }
    }
    
    // Sort the results
    final_list.sort((a, b) => {
        const valueA = a[searching_options.sort_by] || 0;
        const valueB = b[searching_options.sort_by] || 0;
        return valueB - valueA;
    });
    
    if (searching_options.reverse) final_list.reverse();
    
    return final_list.slice(0, searching_options.tickers_shown || 50);
}

export async function filter_tickers_async(searching_options) {
    try {
        const all_nasdaq_info = await get_all_nasdaq_info();
        const all_tickers = await get_all_symbols();
        const all_technical_data = await get_all_technical_data();
        return filter_tickers(searching_options, all_tickers, all_nasdaq_info, all_technical_data);
    } catch (error) {
        console.error("Error in filter_tickers_async:", error);
        toast.error("Failed to filter tickers: " + error.message);
        return [];
    }
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
        if (symbol.includes("^") || symbol.includes("/")) {
            i++;
            continue;
        }
        searched_symbols.add(symbol);
        chunk.push(all_symbols[i]);
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
/**
 * Efficiently fetches missing stock data for analysis with progress tracking
 * @param {Object} searching_options - The search criteria for filtering stocks
 * @param {Function} onProgress - Callback function to track progress (symbol, progress_percent)
 * @param {Function} onStatusChange - Callback function for status updates
 * @returns {Promise<Array>} Array of filtered ticker data
 */
export async function fetch_and_filter_stocks(searching_options, onProgress = null, onStatusChange = null) {
    const random_num_hash = `${Math.random()}_${Date.now()}`;
    const state_key = 'fetching_analysis_data';
    let state = get_state();
    state[state_key] = random_num_hash;

    try {
        if (onStatusChange) onStatusChange("Loading basic stock data...");
        
        // Get all available symbols first
        const all_symbols = await get_all_symbols();
        const all_nasdaq_info = await get_all_nasdaq_info();
        
        if (state[state_key] !== random_num_hash) {
            if (onStatusChange) onStatusChange("Cancelled");
            return [];
        }

        // Filter symbols by market cap first to reduce the workload
        const pre_filtered_symbols = all_symbols.filter(symbol => {
            const nasdaq_data = all_nasdaq_info[symbol];
            if (!nasdaq_data || !nasdaq_data.marketCap) return false;
            
            const market_cap = unformat_number(nasdaq_data.marketCap);
            if (isNaN(market_cap)) return false;
            
            return market_cap >= searching_options.min_market_cap && 
                   market_cap <= searching_options.max_market_cap;
        });

        if (onStatusChange) onStatusChange(`Found ${pre_filtered_symbols.length} stocks matching market cap criteria`);

        // Get existing technical data
        const existing_technical_keys = new Set(await get_all_technical_data_keys());
        const symbols_needing_data = pre_filtered_symbols.filter(symbol => 
            !existing_technical_keys.has(symbol)
        );

        if (symbols_needing_data.length > 0) {
            if (onStatusChange) onStatusChange(`Fetching data for ${symbols_needing_data.length} stocks...`);
            
            const CHUNK_SIZE = 5;
            const chunks = [];
            for (let i = 0; i < symbols_needing_data.length; i += CHUNK_SIZE) {
                chunks.push(symbols_needing_data.slice(i, i + CHUNK_SIZE));
            }

            for (let i = 0; i < chunks.length; i++) {
                if (state[state_key] !== random_num_hash) {
                    if (onStatusChange) onStatusChange("Cancelled");
                    return [];
                }

                const chunk = chunks[i];
                const progress = Math.round((i / chunks.length) * 100);
                
                if (onStatusChange) onStatusChange(`Processing chunk ${i + 1}/${chunks.length} (${progress}%)`);

                const promises = chunk.map(async (symbol) => {
                    try {
                        await fetch_widget_data(symbol);
                        if (onProgress) onProgress(symbol, Math.round(((i * CHUNK_SIZE + chunk.indexOf(symbol) + 1) / symbols_needing_data.length) * 100));
                        return symbol;
                    } catch (error) {
                        console.error(`Failed to fetch data for ${symbol}:`, error);
                        return null;
                    }
                });

                // Add delay to prevent rate limiting
                promises.push(delay(2000));
                await Promise.all(promises);
            }
        }

        if (state[state_key] !== random_num_hash) {
            if (onStatusChange) onStatusChange("Cancelled");
            return [];
        }

        if (onStatusChange) onStatusChange("Filtering and sorting results...");
        
        // Now get all technical data and filter
        const all_technical_data = await get_all_technical_data();
        const filtered_results = filter_tickers(searching_options, pre_filtered_symbols, all_nasdaq_info, all_technical_data);
        
        if (onStatusChange) onStatusChange(`Analysis complete! Found ${filtered_results.length} matching stocks.`);
        
        return filtered_results;

    } catch (error) {
        console.error("Error in fetch_and_filter_stocks:", error);
        toast.error("Failed to fetch and filter stocks: " + error.message);
        if (onStatusChange) onStatusChange("Error occurred");
        return [];
    } finally {
        // Clean up state
        if (state[state_key] === random_num_hash) {
            delete state[state_key];
        }
    }
}