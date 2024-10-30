import { get_all_nasdaq_info } from "./scraper";
import { get_all_symbols, get_all_technical_data, percentage_change } from "./stock_api";
import { unformat_number } from "./tools";

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
        // console.log(pe_ratio)
        const market_cap = unformat_number(summaryData["MarketCap"]["value"])
        if (isNaN(market_cap)) continue;
        if (market_cap < searching_options.min_market_cap || market_cap > searching_options.max_market_cap) continue;
        const final_data = {
            symbol: key, market_cap, current_price, price_target, percent_change, net_change,
            target_percent_difference, pe_ratio, forward_pe_ratio, divided_yield
        }
        final_list.push(final_data);
        // this.setState({ filtered_tickers: final_list })
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