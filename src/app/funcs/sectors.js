import { complex_retrieve, complex_store } from "./cache";

/**
 * gets all the custom sectors the user has made and the generated ones
 * @returns {Promise<{custom_sector: {tickers:[], default:Boolean, }}>}
 */
export async function get_custom_sectors() {
    return await complex_retrieve("custom_sectors")
}
/**
 * 
 * @param {String} name 
 * @param {{}} info 
 */
export async function set_custom_sector(name, info) {
    const custom_sectors = await get_custom_sectors();
    custom_sectors[name] = info;
    return await complex_store("custom_sectors", custom_sectors)
}

export async function save_dynamic_sector(name, searching_options) {
    const dynamic_sector = {
        function: `(async () => {
            const searching_options = ${JSON.stringify(searching_options)};
            const final_list = (await filter_tickers_async(searching_options)).map(item => item.symbol);
            return {tickers: final_list};
        })()`,
        tickers: [],
        default: false,
    }
    return await set_custom_sector(name, dynamic_sector)
}

/**
 * Add a sector to the playground without being on the playground page
 * 
 */
export async function add_sector_externally() {
}