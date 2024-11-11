import { invoke } from "@tauri-apps/api/core";

// possible reddit function
export async function extractInfo() {
    return;
    const json_data = await invoke("reddit_request_api", { url: 'https://oauth.reddit.com/r/apple/new/' });
    const formatted_json = JSON.parse(json_data);
    console.log(formatted_json)
    console.log(formatted_json["kind"])
    console.log(formatted_json["data"])
    const { kind, data } = formatted_json;
    console.log(kind);
    console.log(data)
    if (kind === 'Listing') {
        console.log(`Kind: ${kind}`);
        console.log(`Data: ${JSON.stringify(data)}`);

        const children = data.children;
        children.forEach((child) => {
            const { kind, data: childData } = child;
            console.log(`Child Kind: ${kind}`);
            console.log(`Child Data: ${JSON.stringify(childData)}`);

            // Extract specific information from child data
            console.log(`Title: ${childData.title}`);
            console.log(`Author: ${childData.author}`);
            console.log(`Upvotes: ${childData.ups}`);
            console.log(`Comments: ${childData.num_comments}`);
        });
    }
}

