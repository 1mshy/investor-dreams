// possible reddit function
function extractInfo(jsonData) {
    const { kind, data } = jsonData;

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
