import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { MessageCircle, Repeat2, Heart, BarChart2, Share } from "lucide-react";

export interface Tweet {
    id: string;
    author: {
        name: string;
        handle: string;
        avatar?: string;
    };
    content: string;
    timestamp: string;
    stats: {
        replies: number;
        reposts: number;
        likes: number;
        views: number;
    }
}

export function TweetCard({ tweet }: { tweet: Tweet }) {
    return (
        <div className="p-4 border-b hover:bg-slate-50 transition cursor-pointer">
            <div className="flex gap-3">
                <Avatar>
                    <AvatarImage src={tweet.author.avatar} />
                    <AvatarFallback>{tweet.author.name[0]}</AvatarFallback>
                </Avatar>
                <div className="flex-1">
                    <div className="flex items-center gap-2">
                        <span className="font-bold">{tweet.author.name}</span>
                        <span className="text-muted-foreground">{tweet.author.handle}</span>
                        <span className="text-muted-foreground">·</span>
                        <span className="text-muted-foreground">{tweet.timestamp}</span>
                    </div>
                    <div className="mt-1 text-base leading-normal whitespace-pre-wrap">
                        {tweet.content}
                    </div>
                    <div className="flex justify-between mt-3 text-muted-foreground w-full max-w-md">
                        <ActionButton icon={<MessageCircle className="h-5 w-5" />} count={tweet.stats.replies} />
                        <ActionButton icon={<Repeat2 className="h-5 w-5" />} count={tweet.stats.reposts} />
                        <ActionButton icon={<Heart className="h-5 w-5" />} count={tweet.stats.likes} />
                        <ActionButton icon={<BarChart2 className="h-5 w-5" />} count={tweet.stats.views} />
                        <Button variant="ghost" size="icon" className="h-8 w-8 hover:text-blue-500 hover:bg-blue-50">
                            <Share className="h-5 w-5" />
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    );
}

function ActionButton({ icon, count }: { icon: React.ReactNode; count: number }) {
    return (
        <button className="flex items-center gap-2 group hover:text-blue-500 transition">
            <div className="p-2 rounded-full group-hover:bg-blue-50 transition">
                {icon}
            </div>
            <span className="text-sm">{formatCount(count)}</span>
        </button>
    )
}

function formatCount(count: number): string {
    if (count >= 1000000) return (count / 1000000).toFixed(1) + "M";
    if (count >= 1000) return (count / 1000).toFixed(1) + "K";
    return count.toString();
}
