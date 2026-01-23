import { Tweet } from "@/components/tweet-card";
import { homeMixerClient } from "@/lib/client";
import { Feed } from "@/components/feed";

export default async function Home() {
    let tweets: Tweet[] = [];

    const DUMMY_TWEETS_FALLBACK: Tweet[] = [
        {
            id: "1",
            author: {
                name: "Elon Musk",
                handle: "@elonmusk",
                avatar: "https://github.com/shadcn.png"
            },
            content: "We are making X the everything app.",
            timestamp: "2h",
            stats: {
                replies: 1200,
                reposts: 5000,
                likes: 25000,
                views: 1000000
            }
        },
        {
            id: "2",
            author: {
                name: "Y Project",
                handle: "@y_project",
                avatar: ""
            },
            content: "Just launched the new version of Y! Powered by Rust 🦀 and Next.js.",
            timestamp: "5m",
            stats: {
                replies: 10,
                reposts: 5,
                likes: 42,
                views: 156
            }
        },
        {
            id: "3",
            author: {
                name: "Algorithm Bot",
                handle: "@algo_bot",
                avatar: ""
            },
            content: "Beep boop. I am serving you content from the x-home-mixer service. If you see this, the connection is working!",
            timestamp: "1m",
            stats: {
                replies: 1,
                reposts: 0,
                likes: 10,
                views: 50
            }
        }
    ];

    try {
        // Dummy query parameters for now
        const response = await homeMixerClient.getScoredPosts({
            viewerId: BigInt(123),
            clientAppId: BigInt(1),
            countryCode: "US",
            languageCode: "en",
        });

        tweets = response.scoredPosts.map(post => ({
            id: post.tweetId.toString(),
            author: {
                name: "User " + post.authorId.toString(),
                handle: "@user" + post.authorId.toString(),
                avatar: "",
            },
            content: `This is a generated tweet from the algorithm. \nRequest ID: ${post.predictionRequestId.toString()}\nScore: ${post.score.toFixed(4)}`,
            timestamp: "1h",
            stats: {
                replies: Math.floor(Math.random() * 100),
                reposts: Math.floor(Math.random() * 50),
                likes: Math.floor(Math.random() * 500),
                views: Math.floor(Math.random() * 10000),
            }
        }));

        if (tweets.length === 0) {
            console.log("Backend returned 0 tweets. Using fallback data.");
            tweets = DUMMY_TWEETS_FALLBACK;
        }

    } catch (e) {
        console.error("Failed to fetch tweets:", e);
        // Fallback for demo if backend is offline or errors out
        tweets = [
            ...DUMMY_TWEETS_FALLBACK,
            {
                id: "error",
                author: { name: "System", handle: "@system" },
                content: "Could not connect to X-Algorithm backend. Showing cached/dummy data.",
                timestamp: "now",
                stats: { replies: 0, reposts: 0, likes: 0, views: 0 }
            }
        ];
    }

    return <Feed initialTweets={tweets} />;
}