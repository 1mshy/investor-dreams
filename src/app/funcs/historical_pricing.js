/**
 * @param {Array} historical_prices 
 * All functions below are to be used with managing historical prices of an equity
 * The historical prices are expected to be an array of numbers where the first element 
 * is the oldest price and the last element is the most recent price
 */

import { percentage_change } from "./stock_api";
import { unformat_number } from "./tools";

export const ONE_DAY = 1000* 60 * 60 * 24; // in ms
export const THIRTY_DAYS = ONE_DAY * 30; // in ms
export const ONE_YEAR = ONE_DAY * 365; // in ms
export const FIVE_YEARS = ONE_YEAR * 5; // in ms
export const TEN_YEARS = ONE_YEAR * 10; // in ms

export const ONE_DAY_AGO = new Date(Date.now() - ONE_DAY);
export const THIRTY_DAYS_AGO = new Date(Date.now() - THIRTY_DAYS);
export const YTD_AGO = new Date(new Date().getFullYear(), 0, 1);
export const ONE_YEAR_AGO = new Date(Date.now() - ONE_YEAR);
export const FIVE_YEARS_AGO = new Date(Date.now() - FIVE_YEARS);
export const TEN_YEARS_AGO = new Date(Date.now() - TEN_YEARS);

/**
 * 
 * @param {HistoricalData} historical_data 
 * @param {Date} date_to_end 
 * @returns 
 */
export function get_price_range(historical_data, date_to_end) {
    if(!historical_data || historical_data.length === 0) return [];
    let prices = [];
    let selected_day = new Date();
    for (let i = 0; selected_day >= date_to_end; i++) {
        if(i >= historical_data.length) break;
        selected_day = historical_data[i].datetime;
        prices.push(unformat_number(historical_data[i].close));
    }
    return prices.reverse()
}

export function get_month_prices(historical_data) {
    return get_price_range(historical_data, THIRTY_DAYS_AGO);
}

export function get_ytd_prices(historical_data) {
    return get_price_range(historical_data, YTD_AGO);
}

export function get_year_prices(historical_data) {
    return get_price_range(historical_data, ONE_YEAR_AGO);
}

export function get_five_year_prices(historical_data) {
    return get_price_range(historical_data, FIVE_YEARS_AGO);
}

export function get_ten_year_prices(historical_data) {
    return get_price_range(historical_data, TEN_YEARS_AGO);
}

export function get_all_prices(historical_data) {
    return get_price_range(historical_data, new Date(0));
}

export function get_month_change(historical_data) {
    const month_prices = get_month_prices(historical_data);
    return month_prices.length > 0 ? month_prices[month_prices.length - 1] - month_prices[0] : 0;
}

export function get_ytd_change(historical_data) {
    const ytd_prices = get_ytd_prices(historical_data);
    return ytd_prices.length > 0 ? ytd_prices[ytd_prices.length - 1] - ytd_prices[0] : 0;
}

export function get_year_change(historical_data) {
    const year_prices = get_year_prices(historical_data);
    return year_prices.length > 0 ? year_prices[year_prices.length - 1] - year_prices[0] : 0;
}

export function get_five_year_change(historical_data) {
    const five_year_prices = get_five_year_prices(historical_data);
    return five_year_prices.length > 0 ? five_year_prices[five_year_prices.length - 1] - five_year_prices[0] : 0;
}

export function get_ten_year_change(historical_data) {
    const ten_year_prices = get_ten_year_prices(historical_data);
    return ten_year_prices.length > 0 ? ten_year_prices[ten_year_prices.length - 1] - ten_year_prices[0] : 0;
}

export function get_percent_change_month(historical_data) {
    const month_prices = get_month_prices(historical_data);
    return percentage_change(month_prices[month_prices.length - 1], month_prices[0]);
}

export function get_percent_change_ytd(historical_data) {
    const ytd_prices = get_ytd_prices(historical_data);
    return percentage_change(ytd_prices[ytd_prices.length - 1], ytd_prices[0]);
}

export function get_percent_change_year(historical_data) {
    const year_prices = get_year_prices(historical_data);
    return percentage_change(year_prices[year_prices.length - 1], year_prices[0]);
}

export function get_percent_change_five_year(historical_data) {
    const five_year_prices = get_five_year_prices(historical_data);
    return percentage_change(five_year_prices[five_year_prices.length - 1], five_year_prices[0]);
}

export function get_percent_change_ten_year(historical_data) {
    const ten_year_prices = get_ten_year_prices(historical_data);
    return percentage_change(ten_year_prices[ten_year_prices.length - 1], ten_year_prices[0]);
}

export function get_percent_change_all(historical_data) {
    const all_prices = get_all_prices(historical_data);
    return percentage_change(all_prices[all_prices.length - 1], all_prices[0]);
}

