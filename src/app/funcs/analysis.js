import { toast } from "react-toastify";
import { get_all_nasdaq_info } from "../networking/scraper";
import { get_state } from "./states";
import { get_all_symbols, get_all_technical_data, percentage_change, request_yahoo_big, fetch_widget_data, get_all_technical_data_keys, nasdaq_sorted_by } from "../networking/stock_api";
import { delay } from "./tools";
import { unformat_number } from "./formatting";
import { clean_ticker } from "./formatting";

// Global state for background data fetching
const BACKGROUND_FETCH_STATE = {
    is_running: false,
    current_symbol: '',
    progress: 0,
    total: 0,
    status: 'idle',
    session_id: null,
    listeners: new Set(),
};

/**
 * Subscribe to background fetch updates
 * @param {Function} callback - Function called with (state) on updates
 * @returns {Function} Unsubscribe function
 */
export function subscribeToBackgroundFetch(callback) {
    BACKGROUND_FETCH_STATE.listeners.add(callback);
    // Immediately call with current state
    callback({ ...BACKGROUND_FETCH_STATE });
    return () => BACKGROUND_FETCH_STATE.listeners.delete(callback);
}

/**
 * Notify all listeners of state changes
 */
function notifyListeners() {
    const state = {
        is_running: BACKGROUND_FETCH_STATE.is_running,
        current_symbol: BACKGROUND_FETCH_STATE.current_symbol,
        progress: BACKGROUND_FETCH_STATE.progress,
        total: BACKGROUND_FETCH_STATE.total,
        status: BACKGROUND_FETCH_STATE.status,
    };
    BACKGROUND_FETCH_STATE.listeners.forEach(listener => listener(state));
}

/**
 * Get current background fetch state
 */
export function getBackgroundFetchState() {
    return {
        is_running: BACKGROUND_FETCH_STATE.is_running,
        current_symbol: BACKGROUND_FETCH_STATE.current_symbol,
        progress: BACKGROUND_FETCH_STATE.progress,
        total: BACKGROUND_FETCH_STATE.total,
        status: BACKGROUND_FETCH_STATE.status,
    };
}

/**
 * Stop the background fetch process
 */
export function stopBackgroundFetch() {
    if (BACKGROUND_FETCH_STATE.session_id) {
        BACKGROUND_FETCH_STATE.session_id = null;
        BACKGROUND_FETCH_STATE.is_running = false;
        BACKGROUND_FETCH_STATE.status = 'stopped';
        notifyListeners();
    }
}

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
 * Get statistics about cached data
 * @returns {Promise<{total_symbols: number, cached_symbols: number, missing_symbols: number, cache_percentage: number}>}
 */
export async function getCacheStatistics() {
    const all_symbols = await get_all_symbols();
    const existing_keys = new Set(await get_all_technical_data_keys());
    
    return {
        total_symbols: all_symbols.length,
        cached_symbols: existing_keys.size,
        missing_symbols: all_symbols.length - existing_keys.size,
        cache_percentage: Math.round((existing_keys.size / all_symbols.length) * 100)
    };
}

/**
 * Starts background fetching of all stock data ordered by market cap (largest first)
 * This runs in the background and can be monitored via subscribeToBackgroundFetch
 * @param {boolean} force_restart - If true, restarts even if already running
 */
export async function startBackgroundStockFetch(force_restart = false) {
    // Don't start if already running (unless forced)
    if (BACKGROUND_FETCH_STATE.is_running && !force_restart) {
        console.log("Background fetch already running");
        return;
    }

    // Generate session ID to allow cancellation
    const session_id = `${Math.random()}_${Date.now()}`;
    BACKGROUND_FETCH_STATE.session_id = session_id;
    BACKGROUND_FETCH_STATE.is_running = true;
    BACKGROUND_FETCH_STATE.status = 'initializing';
    BACKGROUND_FETCH_STATE.progress = 0;
    notifyListeners();

    try {
        // Get all symbols sorted by market cap
        BACKGROUND_FETCH_STATE.status = 'loading stock list';
        notifyListeners();
        
        const all_symbols = await get_all_symbols();
        const sorted_symbols = await nasdaq_sorted_by("marketCap", all_symbols);
        
        if (BACKGROUND_FETCH_STATE.session_id !== session_id) {
            BACKGROUND_FETCH_STATE.status = 'cancelled';
            BACKGROUND_FETCH_STATE.is_running = false;
            notifyListeners();
            return;
        }

        BACKGROUND_FETCH_STATE.total = sorted_symbols.length;
        BACKGROUND_FETCH_STATE.status = 'fetching stock data';
        notifyListeners();

        // Get already cached symbols to skip them
        const existing_keys = new Set(await get_all_technical_data_keys());
        const symbols_to_fetch = sorted_symbols.filter(symbol => !existing_keys.has(symbol));

        console.log(`Background fetch: ${symbols_to_fetch.length} stocks to fetch out of ${sorted_symbols.length} total`);

        const CHUNK_SIZE = 3;
        const TIME_DELAY = 2000; // 2 seconds between chunks

        for (let i = 0; i < symbols_to_fetch.length; i += CHUNK_SIZE) {
            // Check if we should stop
            if (BACKGROUND_FETCH_STATE.session_id !== session_id) {
                BACKGROUND_FETCH_STATE.status = 'cancelled';
                BACKGROUND_FETCH_STATE.is_running = false;
                notifyListeners();
                return;
            }

            const chunk = symbols_to_fetch.slice(i, i + CHUNK_SIZE);
            const chunk_promises = chunk.map(async (symbol) => {
                try {
                    BACKGROUND_FETCH_STATE.current_symbol = symbol;
                    BACKGROUND_FETCH_STATE.progress = Math.round((i / symbols_to_fetch.length) * 100);
                    notifyListeners();
                    
                    await fetch_widget_data(symbol);
                    return symbol;
                } catch (error) {
                    console.error(`Background fetch failed for ${symbol}:`, error);
                    return null;
                }
            });

            // Wait for chunk and add delay
            chunk_promises.push(delay(TIME_DELAY));
            await Promise.all(chunk_promises);
        }

        // Completed successfully
        if (BACKGROUND_FETCH_STATE.session_id === session_id) {
            BACKGROUND_FETCH_STATE.is_running = false;
            BACKGROUND_FETCH_STATE.status = 'completed';
            BACKGROUND_FETCH_STATE.progress = 100;
            BACKGROUND_FETCH_STATE.current_symbol = '';
            notifyListeners();
            console.log("Background stock fetch completed!");
        }

    } catch (error) {
        console.error("Error in background stock fetch:", error);
        if (BACKGROUND_FETCH_STATE.session_id === session_id) {
            BACKGROUND_FETCH_STATE.is_running = false;
            BACKGROUND_FETCH_STATE.status = 'error: ' + error.message;
            notifyListeners();
        }
    }
}