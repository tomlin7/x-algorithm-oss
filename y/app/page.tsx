import { TweetCard, Tweet } from "@/components/tweet-card";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

const DUMMY_TWEETS: Tweet[] = [
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
    }
];

export default function Home() {
    return (
        <div className="flex flex-col">
            <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-md border-b">
                <div className="flex">
                    <div className="flex-1 hover:bg-slate-100 transition cursor-pointer p-4 text-center font-bold border-b-4 border-blue-500">
                        For You
                    </div>
                    <div className="flex-1 hover:bg-slate-100 transition cursor-pointer p-4 text-center text-muted-foreground font-medium">
                        Following
                    </div>
                </div>
            </header>

            <div className="p-4 border-b flex gap-4">
                <Avatar>
                    <AvatarFallback>ME</AvatarFallback>
                </Avatar>
                <div className="flex-1 space-y-4">
                    <input
                        className="w-full text-xl outline-none placeholder:text-muted-foreground"
                        placeholder="What is happening?!"
                    />
                    <div className="flex justify-between items-center">
                        <div className="flex gap-2 text-blue-500">
                            {/* Icons for media would go here */}
                        </div>
                        <Button className="rounded-full font-bold px-6">Post</Button>
                    </div>
                </div>
            </div>

            <div>
                {DUMMY_TWEETS.map(tweet => (
                    <TweetCard key={tweet.id} tweet={tweet} />
                ))}
            </div>
        </div>
    );
}