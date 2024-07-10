/**
 * Will be a general caching system for all data
 */

export const DEFAULT_EXPIRATION = 5; // minutes

/**
 * 
 * @param {string} key 
 * @param {Number} expiration - time in minutes
 */
export function create_cache_profile(key, expiration) {
    localStorage.setItem(key, JSON.stringify({
        expiration: expiration,
        last_updated: 0
    }));
}
/**
 * @param {string} key - key of the cached item
 * @returns {boolean} checks if the current cache of a key is valid: exists and in the proper time frame
 */
export function cache_is_valid(key) {
    const value = localStorage.getItem(key);
    if (!value) return false;
    const item = JSON.parse(value)
    const now = Date.now();
    const { last_updated, expiration } = item;
    return now - last_updated < expiration * 60 * 1000 // minutes to miliseconds
}
/**
 * 
 * @param {string} key 
 * @param {Object} value 
 */
export function set_cache(key, value) {
    if (!localStorage.getItem(key))
        create_cache_profile(key, DEFAULT_EXPIRATION)
    const item = JSON.parse(localStorage.getItem(key));
    console.log(item)
    const writable_value = JSON.stringify({
        ...value,
        last_updated: Date.now(),
        expiration: item.expiration
    });
    localStorage.setItem(`${key}`, writable_value);
}

export function get_cache(key) {
    return JSON.parse(localStorage.getItem(key))
}

export function clear_cache() {
    localStorage.clear();
}

/**
 * basic wrappers around the localStorage API so that it can be changed if ever needed
 */
export function store(key, value) {
    localStorage.setItem(key, value)
}
export function retrieve(key) {
    return localStorage.getItem(key)
}