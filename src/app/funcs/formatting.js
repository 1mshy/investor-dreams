/**
 * Generates a financials income statement link for a given stock symbol and exchange.
 *
 * @param {string} symbol - The stock ticker symbol.
 * @param {string} exchange - The stock market exchange name.
 * @returns {string} - A URL pointing to the financials income statement page for the specified symbol and exchange.
 */
export function financials_link(symbol, exchange) {
    return `https://ca.advfn.com/stock-market/${exchange}/${symbol}/financials/income-statement`
}
/**
 *
 * @param {Number} number
 * @returns {String}
 */

export function format_number(number) {
    if (isNaN(number)) return 0;
    const formatting_suffixed = ["", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "De", "UnDe", "DuDe", "TrDe", "QaDe", "QiDe", "SxDe", "SpDe", "OcDe", "NoDe", "Vi"]; // making sure it will be inflation proof :)
    let usable_num = Number(number);
    while (usable_num >= 1000) {
        usable_num /= 1000;
        formatting_suffixed.shift();
    }
    return `${Number(usable_num.toFixed(2))}${formatting_suffixed[0]}`;
}/**
 * @param {String} percent - percentage value in string format (ex: 10%)
 * @returns {Number} - decimal value of that percentage
 */

export function percent_to_decimal(percent) {
    return unformat_number(percent) / 100;
}
/**
 *
 * @param {Number} decimal
 * @returns {String}
 */

export function decimal_to_percent(decimal) {
    return (decimal * 100).toFixed(2) + "%";
}

export function remove_decimal_zeros(number) {
    return Number(number).toString();
}
/**
 * Removes unwanted characters from a ticker symbol string
 * @param {String} ticker
 * @returns {String}
 */

export function clean_ticker(ticker) {
    if (!ticker) return "";
    return `${ticker}`.replace("/", "."); //.replace("^", ".");
}
/**
 *  cleans the ticker for the specifications of the yahoo api
 * @param {string} ticker
 * @returns {string}
 */
export function clean_ticker_for_yahoo(ticker) {
    if (!ticker) return "";
    return `${ticker}`.replace("/", "-").replace("^", "-").replace("*", "-").replace(".", "-").replace(",", "-");
}
/**
 *
 * @param {Number} number
 * @returns {String} - formatted with commas
 */

export function format_number_with_commas(number) {
    if (isNaN(number)) return "";
    return `${Number(number).toFixed(2)}`.replace(/\B(?=(\d{3})+(?!\d))/g, ",").replace(".00", "");
}
/**
 * @param {String} number - String formatted number
 * @returns {Number} - unformatted
 * @example unformat_number("$1,000") => 1000
 */

export function unformat_number(number) {
    if (!number && number !== 0) return NaN;
    return Number(`${number}`.replaceAll("$", "").replaceAll(",", "").replaceAll("'", "").replaceAll("%", ""));
}
/**
 *
 * @param {Number} number
 * @returns {String} - formatted with commas
 */
export function format_currency(number) {
    return `$${format_number_with_commas(number)}`;
}
/**
 *
 * @param {Number} number
 * @returns {String} - formatted with currency symbol
 */

export function format_currency_with_symbols(number) {
    if (isNaN(number) || number === null || number === undefined) return "Unknown";
    return `${format_number(number)}`;
}
/**
 * @param {Number} number
 * @returns {String}
 */

export function format_percentage(number) {
    return `${unformat_number(number).toFixed(2)}%`;
}
/**
 * tries to map exhange name to the nasdaq or nyse (most common ones)
 * @param exchange {string}
 * @returns {string} the correct exchange
 */
export function map_to_exchange(exchange) {
    // 2) Map codes → main exchange
    const map = {
      NMS:  "NASDAQ",   // Nasdaq Global Market          :contentReference[oaicite:0]{index=0}
      NDAQ: "NASDAQ",   // (sometimes seen)
      NAS:  "NASDAQ",   // (rare)
      NYQ:  "NYSE",     // New York Stock Exchange        :contentReference[oaicite:1]{index=1}
      NYS:  "NYSE",     // (alias)
      P:    "NYSE"      // (for some preferred shares)
      // …add more if you hit other listings (ARCA, AMEX, etc.)
    };

    // 3) Fallback: look at the exchangeName prefix
    let mainExchange = map[exchange];
    if (!mainExchange) {
      if (name.toUpperCase().startsWith("NASDAQ")) mainExchange = "NASDAQ";
      else if (name.toUpperCase().startsWith("NYSE")) mainExchange = "NYSE";
      else mainExchange = name;  // whatever else it is
    }
    return mainExchange;
}

/**
 *
 * @param {String} keys
 * @returns {Array<String>}
 */

export function format_api_keys(keys) {
    if (!keys || keys === "") return null;
    let api_keys = keys.trim().split(",").map(key => key.trim()).filter(a => a !== "");
    if (Math.random() > 0.5) {
        api_keys = api_keys.reverse();
    }
    return api_keys;
}

