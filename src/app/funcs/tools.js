import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { toast } from "react-toastify";

export const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

/**
 * Does ticker have any special characters
 * @param {string} ticker 
 * @returns {bool}
 */
export function is_complex_ticker(ticker) {
    if (!ticker) return true;
    return ticker.includes("/") || ticker.includes("^") || ticker.includes("*") || ticker.includes(".") || ticker.includes(",")
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

/**
 * 
 * @param {BigStockWidgetRange} range 
 * @returns {TradingViewRange}
 */
export function to_tradingview_range(range) {
    switch (range) {
        case "D":
            return "1D";
        case "M":
            return "1M";
        case "YTD":
            return "YTD";
        case "Y":
            return "1Y";
        case "5Y":
            return "5Y";
        case "10Y":
        case "ALL":
            return "ALL";
    }
    return "1Y";
}