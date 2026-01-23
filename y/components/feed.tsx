"use client";

import { useState } from "react";
import { Tweet, TweetCard } from "@/components/tweet-card";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Image, Smile, CalendarClock, MapPin } from "lucide-react";

export function Feed({ initialTweets }: { initialTweets: Tweet[] }) {
    const [tweets, setTweets] = useState<Tweet[]>(initialTweets);
    const [content, setContent] = useState("");

    const handlePost = () => {
        if (!content.trim()) return;

        const newTweet: Tweet = {
            id: Date.now().toString(),
            author: {
                name: "User",
                handle: "@user",
                avatar: "",
            },
            content: content,
            timestamp: "now",
            stats: {
                replies: 0,
                reposts: 0,
                likes: 0,
                views: 0
            }
        };

        setTweets([newTweet, ...tweets]);
        setContent("");
    };

    return (
        <div className="flex flex-col">
            <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-md border-b">
                <div className="flex">
                    <div className="flex-1 hover:bg-slate-100 transition duration-200 cursor-pointer p-4 text-center font-bold border-b-4 border-blue-500">
                        For You
                    </div>
                    <div className="flex-1 hover:bg-slate-100 transition duration-200 cursor-pointer p-4 text-center text-muted-foreground font-medium">
                        Following
                    </div>
                </div>
            </header>

            <div className="p-4 border-b flex gap-4">
                <Avatar>
                    <AvatarFallback>U</AvatarFallback>
                </Avatar>
                <div className="flex-1 space-y-4">
                    <textarea
                        className="w-full text-xl outline-none placeholder:text-muted-foreground resize-none bg-transparent"
                        placeholder="What is happening?!"
                        rows={2}
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                    />
                    <div className="flex justify-between items-center">
                        <div className="flex gap-2 text-blue-500">
                            <Button variant="ghost" size="icon" className="group">
                                <Image className="h-5 w-5 group-hover:text-blue-500" />
                            </Button>
                            <Button variant="ghost" size="icon" className="group">
                                <div className="border border-current rounded h-4 w-4 flex items-center justify-center text-[10px] font-bold">GIF</div>
                            </Button>
                            <Button variant="ghost" size="icon" className="group">
                                <Smile className="h-5 w-5 group-hover:text-blue-500" />
                            </Button>
                            <Button variant="ghost" size="icon" className="group">
                                <CalendarClock className="h-5 w-5 group-hover:text-blue-500" />
                            </Button>
                            <Button variant="ghost" size="icon" className="group">
                                <MapPin className="h-5 w-5 group-hover:text-blue-500" />
                            </Button>
                        </div>
                        <Button
                            className="rounded-full font-bold px-6 bg-blue-500 hover:bg-blue-600 disabled:opacity-50"
                            onClick={handlePost}
                            disabled={!content.trim()}
                        >
                            Post
                        </Button>
                    </div>
                </div>
            </div>

            <div>
                {tweets.map(tweet => (
                    <TweetCard key={tweet.id} tweet={tweet} />
                ))}
            </div>
        </div>
    );
}
