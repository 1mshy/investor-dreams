import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { toast } from "react-toastify";

export const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

/**
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
    return `${Number(usable_num.toFixed(2))}${formatting_suffixed[0]}`;
}
/**
 * 
 * @param {Number} number 
 * @returns {String} - formatted with commas
 */
export function format_number_with_commas(number) {
    if(isNaN(number)) return ""; 
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
    return `$${format_number(number)}`;
}

/**
 * @param {Number} number 
 * @returns {String}
 */
export function format_percentage(number) {
    return `${unformat_number(number).toFixed(2)}%`;
}

/**
 * Checks if the market is open
 * @returns {Boolean}
 */
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

export async function open_external_link(url) {
    return await open(url);
}

/**
 * 
 * @param {String} keys
 * @returns {Array<String>} 
 */
export function format_api_keys(keys) {
    if(!keys || keys=== "") return null;
    let api_keys = keys.trim().split(",").map(key => key.trim()).filter(a => a !== "");
    if (Math.random() > 0.5) {
        api_keys = api_keys.reverse();
    }
    return api_keys;
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

export function trim_title(title) {
    return title.length > 100 ? title.substring(0, 100) + "..." : title;
}

/**
 * Writes multiple json files to the select folder of the user
 * The data will be seperated into file names for each key, with the data in the file being the stringified value 
 * @param {Object} json_data - json data to be written
 * @param {String} folder - folder name to save the files to
 */
export async function write_chunks(json_data, folder) {
    let promises = [];
    toast.warn("Writing file to downloads folder, please wait...");
    for (let key of Object.keys(json_data)) {
        const data = json_data[key];
        const jsonContent = JSON.stringify(data);
        const fileNameWithKey = `${key}.json`;

        // Ensure that both filename and jsonContent are correctly passed
        if (jsonContent && fileNameWithKey) {
            promises.push(invoke("save_json_to_folder", { filename: fileNameWithKey, folder, jsonContent })
                .catch((error) => {
                    toast.error(`Error: ${error}`);
                }));
        } else {
            toast.error("Invalid json content or filename.");
        }
    }
    Promise.all(promises).then((results) => {
        toast.success("Saved all data to downloads folder.");
    });
}
