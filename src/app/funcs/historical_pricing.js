/**
 * @param {Array} historical_prices 
 * All functions below are to be used with managing historical prices of an equity
 */

import { percentage_change } from "./stock_api";

export function get_month_prices(historical_prices) {
    return historical_prices ? historical_prices.slice(-30) : [];
}

export function get_ytd_prices(historical_prices) {
    const present_date = new Date();
    const one_day = 1000 * 60 * 60 * 24;
    const start_of_year = new Date(present_date.getFullYear(), 0, 0);
    const diff = present_date - start_of_year;
    const days_past = Math.floor(diff / one_day);
    return historical_prices ? historical_prices.slice(-days_past) : [];
}

export function get_year_prices(historical_prices) {
    return historical_prices ? historical_prices.slice(-365) : [];
}

export function get_five_year_prices(historical_prices) {
    return historical_prices ? historical_prices.slice(-365 * 5) : [];
}

export function get_ten_year_prices(historical_prices) {
    return historical_prices ? historical_prices.slice(-365 * 10) : [];
}

export function get_month_change(historical_prices) {
    const month_prices = get_month_prices(historical_prices);
    return month_prices.length > 0 ? month_prices[month_prices.length - 1] - month_prices[0] : 0;
}

export function get_ytd_change(historical_prices) {
    const ytd_prices = get_ytd_prices(historical_prices);
    return ytd_prices.length > 0 ? ytd_prices[ytd_prices.length - 1] - ytd_prices[0] : 0;
}

export function get_year_change(historical_prices) {
    const year_prices = get_year_prices(historical_prices);
    return year_prices.length > 0 ? year_prices[year_prices.length - 1] - year_prices[0] : 0;
}

export function get_five_year_change(historical_prices) {
    const five_year_prices = get_five_year_prices(historical_prices);
    return five_year_prices.length > 0 ? five_year_prices[five_year_prices.length - 1] - five_year_prices[0] : 0;
}

export function get_ten_year_change(historical_prices) {
    const ten_year_prices = get_ten_year_prices(historical_prices);
    return ten_year_prices.length > 0 ? ten_year_prices[ten_year_prices.length - 1] - ten_year_prices[0] : 0;
}

export function get_percent_change_month(historical_prices) {
    const month_prices = get_month_prices(historical_prices);
    return percentage_change(month_prices[month_prices.length - 1], month_prices[0]);
}

export function get_percent_change_ytd(historical_prices) {
    const month_prices = get_ytd_prices(historical_prices);
    return percentage_change(month_prices[month_prices.length - 1], month_prices[0]);
}

export function get_percent_change_year(historical_prices) {
    const year_prices = get_year_prices(historical_prices);
    return percentage_change(year_prices[year_prices.length - 1], year_prices[0]);
}

export function get_percent_change_five_year(historical_prices) {
    const five_year_prices = get_five_year_prices(historical_prices);
    return percentage_change(five_year_prices[five_year_prices.length - 1], five_year_prices[0]);
}

export function get_percent_change_ten_year(historical_prices) {
    const ten_year_prices = get_ten_year_prices(historical_prices);
    return percentage_change(ten_year_prices[ten_year_prices.length - 1], ten_year_prices[0]);
}

