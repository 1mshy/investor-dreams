import {invoke} from "@tauri-apps/api/core";

/**
 * @param {string} url
 * @returns {Promise<string>} the body of the request
 */
export async function get_request(url) {
    return invoke("get_request_api", {url});
}

/**
 * @param {string} url
 * @returns {Promise<number>} the status code of the request
 */
export async function code_request(url) {
    return invoke("code_request_api", {url});
}

