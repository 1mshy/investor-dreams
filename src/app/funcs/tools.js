import { invoke } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

export const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

/**
 * 
 * @param {String} percent 
 * @returns {Number}
 */
export function percent_to_decimal(percent) {
    return Number(percent.replace("%", "")) / 100;
}

/**
 * 
 * @param {Number} decimal 
 * @returns {String}
 */
export function decimal_to_percent(decimal) {
    return (decimal * 100).toFixed(2) + "%";
}

/**
 * 
 * @param {Number} number 
 * @returns {String}
 */
export function format_number(number) {
    if(isNaN(number)) return 0;
    const formatting_suffixed = ["", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "De", "UnDe", "DuDe", "TrDe", "QaDe", "QiDe", "SxDe", "SpDe", "OcDe", "NoDe", "Vi"]; // making sure it will be inflation proof :)
    let usable_num = Number(number);
    while (usable_num >= 1000) {
        usable_num /= 1000;
        formatting_suffixed.shift();
    }
    return `${usable_num.toFixed(2)}${formatting_suffixed[0]}`;
}
/**
 * @param {String} number 
 * @returns {Number}
 */
export function unformat_number(number) {
    if (!number) return 0;
    return Number(`${number}`.replace("$", "").replace(",", "").replace("'", ""));
}
/**
 * 
 * @param {Number} number 
 * @returns {String}
 */
export function format_currency(number) {
    return `$${format_number(number)}`;
}
/**
 * 
 * @param {Number} number 
 * @returns {String}
 */
export function format_percentage(number) {
    return `${unformat_number(number).toFixed(2)}%`;
}

export function is_market_open() {
    const current_hour = new Date().getHours();
    const current_day = new Date().getDay();
    const outside_trading_hours = current_hour < 9 || current_hour >= 16
        || (current_day <= 0 && current_day >= 6);
    return !outside_trading_hours;
}
/**
 * 
 * @param {String} prompt 
 * @returns {String} hash
 */
export function sha256(prompt) {
    return crypto.createHash("sha256").update(prompt).digest("hex")
}


/**
 * Function to invoke a Tauri command with a timeout
 * @param {String} command 
 * @param {Object} args 
 * @param {Number} timeout 
 * @returns 
 */
export function invoke_with_timeout(command, args = {}, timeout = 7000) {
    return Promise.race([
        invoke(command, args),
        new Promise((_, reject) =>
            setTimeout(() => reject(new Error('Operation timed out')), timeout)
        )
    ]);
}


export function upload_json(json_data, filename) {
    const jsonContent = JSON.stringify(json_data)
    // console.log(filename)
    // console.log(jsonContent)
    toast.warn("Writing file to downloads folder, please wait...")
    invoke("save_json_file", { filename, jsonContent }).then((result) => {
        toast.success(result)
    })


    // const jsonString = `data:text/json;chatset=utf-8,${encodeURIComponent(
    //     JSON.stringify(json_data)
    // )}`;
    // const link = document.createElement("a");
    // link.href = jsonString;
    // link.download = filename;

    // link.click();
}