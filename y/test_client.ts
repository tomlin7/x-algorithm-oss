
import { createPromiseClient } from "@connectrpc/connect";
import { createGrpcTransport } from "@connectrpc/connect-node";
import { ScoredPostsService } from "./gen/home_mixer_connect";

async function main() {
    const transport = createGrpcTransport({
        baseUrl: "http://localhost:50051",
        httpVersion: "2",
    });
    const client = createPromiseClient(ScoredPostsService, transport);

    try {
        console.log("Sending request...");
        const response = await client.getScoredPosts({
            viewerId: BigInt(123),
            clientAppId: BigInt(1),
            countryCode: "US",
            languageCode: "en",
        });
        console.log("Response received!");
        console.log(`Scored Posts: ${response.scoredPosts.length}`);
        response.scoredPosts.forEach(p => {
             console.log(`- Tweet ${p.tweetId} Score: ${p.score}`);
        });
    } catch (e) {
        console.error("Error:", e);
    }
}

main();
