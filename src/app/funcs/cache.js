/**
 * Will be a general caching system for all data
 */

import localforage from "localforage";
import { NASDAQ_NEWS, NASDAQ_TECHNICALS, OLLAMA_GENERATION } from "./stock_api";
import { REDDIT_CACHE } from "./reddit";

export const DEFAULT_EXPIRATION = 10; // minutes

/**
 * @param {String} key 
 * @param {Number} expiration - time in minutes
 */
export function create_cache_profile(key, expiration) {
    return {
        expiration: expiration,
        last_updated: 0
    };
}

export const STOCK_CACHE = localforage.createInstance({
    name: "historical_stock_data"
});

/**
 * @param {String} key - key of the cached item
 * @returns {Promise<Boolean>} checks if the current cache of a key is valid: exists and in the proper time frame
 */
export async function stock_cache_is_valid(key) {
    const item = await complex_retrieve(key, STOCK_CACHE);
    if (!item) return false;
    const now = Date.now();
    const { last_updated, expiration } = item;
    const cache_validity = now - last_updated < expiration * 60 * 1000 // minutes to miliseconds
    return cache_validity;
}

/**
 * 
 * @param {String} key 
 * @param {Object} item 
 * @returns {Promise<Boolean>} checks if the current cache of a key is valid: exists and in the proper time frame
 */
export async function cache_is_valid(key, item = "nothing") {
    // it is possible to have two identical keys that point to different caches
    // thus it is important that when the item is passed as a parameter it should know if it is a cache Object or nothing
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
 * @param {String} key 
 * @param {Object} value 
 * @param {Number} expiration time till expiration in minutes 
 * @param {localforage} custom_forage - custom localforage instance
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
/**
 * @param {String} key 
 * @param {localforage} custom_forage - custom localforage instance (optional)
 * @returns 
 */
export async function get_cache(key, custom_forage = null) {
    const item = await complex_retrieve(key, custom_forage);
    const valid_cache = await cache_is_valid(key, item);
    if (!item || !valid_cache) return null;
    return item;
}
/**
 * Clears most cache from the application storage
 */
export function clear_cache() {
    localStorage.clear();
    localforage.clear();
    STOCK_CACHE.clear();
    NASDAQ_NEWS.clear();
    NASDAQ_TECHNICALS.clear();
    OLLAMA_GENERATION.clear();
    STOCK_CACHE.clear();
    NASDAQ_NEWS.clear();
    NASDAQ_TECHNICALS.clear();
    REDDIT_CACHE.clear();
}

/**
 * basic wrapper around the localStorage API
 * @param {String} key - key of the item to store
 * @param {String} value - value of the item to store
 */
export function store(key, value) {
    localStorage.setItem(key, value)
}
/**
 * basic wrapper around the localStorage API
 * @param {String} key - key of the item to retrieve
 */
export function retrieve(key) {
    return localStorage.getItem(key)
}
/**
 * Used to store Objects, arrays, and other complex data types to a database
 * @param {String} key - key of the item to store
 * @param {Object} value - value of the item to store
 * @param {localforage} custom_forage - custom localforage instance (optional)
 */
export function complex_store(key, value, custom_forage = null) {
    if (custom_forage) {
        return custom_forage.setItem(key, value)
    }
    localforage.setItem(key, value)
}
/**
 * 
 * @param {String} key - key of the item to retrieve
 * @param {localforage} custom_forage - custom localforage instance (optional)
 * @returns {Object}
 */
export async function complex_retrieve(key, custom_forage = null) {
    if (custom_forage) {
        return await custom_forage.getItem(key)
    }
    return await localforage.getItem(key)
}
