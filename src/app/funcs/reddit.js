import { invoke } from "@tauri-apps/api/core";
import { cache_is_valid, get_cache, set_cache } from "./cache";
import localforage from "localforage";

export async function get_access_token() {
    try {
        const acess_token_data = await get_cache('reddit_access_token', REDDIT_CACHE);
        if (acess_token_data && cache_is_valid('reddit_access_token', acess_token_data)) {
            return acess_token_data.access_token;
        }
        const new_acess_token_string = await invoke('fetch_reddit_access_token');
        const new_acess_token_data = JSON.parse(new_acess_token_string);
        set_cache('reddit_access_token', new_acess_token_data, new_acess_token_data.expires_in / 60, REDDIT_CACHE);
        return new_acess_token_data.access_token;
    } catch (error) {
        console.error('Error fetching access token:', error);
        return "";
    }
}
/**
 * 
 * @param {String} subreddit 
 * @returns {Promise<SubredditData>}
 */
export async function fetch_subreddit_posts(subreddit) {
    const subreddit_cache = await get_cache(subreddit, REDDIT_CACHE);
    if (subreddit_cache && cache_is_valid(subreddit, subreddit_cache)) {
        return subreddit_cache.structured;
    }
    try {
        const accessToken = await get_access_token();
        if (!accessToken) {
            throw new Error('Unable to fetch access token.');
        }

        const posts = await invoke('fetch_reddit_subreddit_posts', {
            accessToken,
            subreddit,
        });

        if (posts.error >= 400) {
            console.log(`Subreddit r/${subreddit} is either private, banned or does not exist`);
            return [];
        }

        console.log(`Posts from r/${subreddit}:`, posts);


        const structured = posts.data.children.map(post => post.data);
        
        set_cache(subreddit, {structured}, 60, REDDIT_CACHE);

        return structured;
    } catch (error) {
        console.error('Error fetching subreddit posts:', error);
    }
    return [];
}

export const REDDIT_CACHE = localforage.createInstance({
    name: "reddit_cache"
});

