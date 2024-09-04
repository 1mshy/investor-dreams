/**
 * Will be a general caching system for all data
 */

import localforage from "localforage";

export const DEFAULT_EXPIRATION = 10; // minutes

/**
 * @param {string} key 
 * @param {Number} expiration - time in minutes
 */
export function create_cache_profile(key, expiration) {
    return {
        expiration: expiration,
        last_updated: 0
    };
}

export const STOCK_CACHE = localforage.createInstance({
    name: "stock_cache"
});

/**
 * @param {string} key - key of the cached item
 * @returns {boolean} checks if the current cache of a key is valid: exists and in the proper time frame
 */
export async function stock_cache_is_valid(key) {
    const item = await complex_retrieve(key, STOCK_CACHE);
    if (!item) return false;
    const now = Date.now();
    const { last_updated, expiration } = item;
    const cache_validity = now - last_updated < expiration * 60 * 1000 // minutes to miliseconds
    return cache_validity;
}

export async function cache_is_valid(key, item = "nothing") {
    // it is possible to have two identical keys that point to different caches
    // thus it is important that when the item is passed as a parameter it should know if it is a cache object or nothing
    if (item === "nothing")
        item = await complex_retrieve(key);
    if (!item)
        return false;
    const now = Date.now();
    const { last_updated, expiration } = item;
    return now - last_updated < expiration * 60 * 1000 // minutes to miliseconds
}

/**
 * 
 * @param {string} key 
 * @param {Object} value 
 * @param {Number} expiration time till expiration in minutes 
 */
export function set_cache(key, value, expiration = DEFAULT_EXPIRATION, custom_forage = null) {
    complex_retrieve(key).then(item => {
        if (!item)
            item = create_cache_profile(key, expiration)
        const writable_value = {
            ...value,
            last_updated: Date.now(),
            expiration: item.expiration
        };
        complex_store(`${key}`, writable_value, custom_forage = custom_forage);
    })

}

export async function get_cache(key, custom_forage = null) {
    const item = await complex_retrieve(key, custom_forage);
    const valid_cache = await cache_is_valid(key, item);
    if (!item || !valid_cache) return null;
    return item;
}

export function clear_cache() {
    localStorage.clear();
    localforage.clear();
}

/**
 * basic wrapper around the localStorage API
 */
export function store(key, value) {
    localStorage.setItem(key, value)
}
/**
 * basic wrapper around the localStorage API
 */
export function retrieve(key) {
    return localStorage.getItem(key)
}
/**
 * Used to store objects, arrays, and other complex data types to a database
 * @param {string} key 
 * @param {object} value 
 */
export function complex_store(key, value, custom_forage = null) {
    if (custom_forage) {
        return custom_forage.setItem(key, value)
    }
    localforage.setItem(key, value)
}
/**
 * 
 * @param {string} key 
 * @returns {object}
 */
export async function complex_retrieve(key, custom_forage = null) {
    if (custom_forage) {
        return await custom_forage.getItem(key)
    }
    return await localforage.getItem(key)
}
