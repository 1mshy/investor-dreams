export const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

/**
 * 
 * @param {string} percent 
 * @returns {Number}
 */
export function percent_to_decimal(percent) {
    return Number(percent.replace("%", "")) / 100;
}

/**
 * 
 * @param {Number} decimal 
 * @returns {string}
 */
export function decimal_to_percent(decimal) {
    return (decimal * 100).toFixed(2) + "%";
}

/**
 * gets the lowest n values from an array
 * @param {[number]} array 
 * @param {number} n 
 */
export function get_first(array, n) {
    return array
        .slice(changes.length - 1 - n, changes.length - 1)
        .map(ticker_symbol => response[ticker_symbol])
        .reverse();
}
/**
 * gets last n values from an array
 * @param {[number]} array 
 * @param {number} n 
 * @returns 
 */
export function get_last(array, n) {
    return array
        .slice(0, n)
        .map(ticker_symbol => response[ticker_symbol]);
}