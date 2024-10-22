import { complex_retrieve } from "./cache";

/**
 * gets all the custom sectors the user has made and the generated ones
 * @returns {Promise<{custom_sector: {tickers:[], default:Boolean, }}>}
 */
export async function get_custom_sectors() {
    return await complex_retrieve("custom_sectors")
}

/**
 * Add a sector to the playground without being on the playground page
 * 
 */
export async function add_sector_externally() {
}