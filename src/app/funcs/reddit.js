import { invoke } from "@tauri-apps/api/core";
import { cache_is_valid, get_cache, set_cache } from "./cache";

export async function getAccessToken() {
    try {
        const acess_token_data = await get_cache('reddit_access_token');
        if (acess_token_data && cache_is_valid('reddit_access_token', acess_token_data)) {
            console.log("valid cache for reddit access token")
            return acess_token_data.access_token;
        }
        const new_acess_token_string= await invoke('fetch_reddit_access_token');
        const new_acess_token_data = JSON.parse(new_acess_token_string);
        console.log('Access token:', new_acess_token_data);
        set_cache('reddit_access_token', new_acess_token_data, new_acess_token_data.expires_in / 60);
        return new_acess_token_data.access_token;
    } catch (error) {
        console.error('Error fetching access token:', error);
        return "";
    }
}

export async function fetchSubredditPosts(subreddit) {
    try {
        const accessToken = await getAccessToken();
        if (!accessToken) {
            throw new Error('Unable to fetch access token.');
        }

        const posts = await invoke('fetch_reddit_subreddit_posts', {
            accessToken,
            subreddit,
        });

        console.log(`Posts from r/${subreddit}:`, posts);
        return posts;
    } catch (error) {
        console.error('Error fetching subreddit posts:', error);
        return null;
    }
}


