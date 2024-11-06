import { complex_retrieve, complex_store } from "./cache";

/**
 * gets all the custom sectors the user has made and the generated ones
 * @returns {Promise<{custom_sector: {tickers:[], default:Boolean, }}>}
 */
export async function get_custom_sectors() {
    const custom_sectors = await complex_retrieve("custom_sectors");
    if(!custom_sectors) return {};
    return custom_sectors;
}
/**
 * 
 * @param {String} name 
 * @param {{}} info 
 * 6/11/24
 * Note a custom sector is an object with the following properties
 * {
 * tickers: [String], - list of tickers in the sector
 * default: Boolean - weather or not the playground should default on this sector on first load
 * should_sort: Boolean - should tickers be sorted by sort method on first load
 * function: function - async function that returns a list of tickers
 * }
 */
export async function set_custom_sector(name, info) {
    const custom_sectors = await get_custom_sectors();
    custom_sectors[name] = info;
    return await complex_store("custom_sectors", custom_sectors)
}

/**
 * 
 * @param {String} name - name of the sector to be saved 
 * @param {Object} searching_options - searching options to be saved (see filter_tickers_async)
 * @returns 
 */
export async function save_dynamic_sector(name, searching_options) {
    const dynamic_sector = {
        function: `(async () => {
            const should_sort = ${!!searching_options.should_sort};
            const searching_options = ${JSON.stringify(searching_options)};
            const final_list = (await filter_tickers_async(searching_options)).map(item => item.symbol);
            return {tickers: final_list, should_sort };
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