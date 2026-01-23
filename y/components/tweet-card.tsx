import { useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { MessageCircle, Repeat2, Heart, BarChart2, Share, Bookmark } from "lucide-react";
import { cn } from "@/lib/utils";

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
    const [liked, setLiked] = useState(false);
    const [reposted, setReposted] = useState(false);
    const [bookmarked, setBookmarked] = useState(false);
    const [localStats, setLocalStats] = useState(tweet.stats);

    const handleLike = () => {
        setLiked(!liked);
        setLocalStats(prev => ({
            ...prev,
            likes: liked ? prev.likes - 1 : prev.likes + 1
        }));
    };

    const handleRepost = () => {
        setReposted(!reposted);
        setLocalStats(prev => ({
            ...prev,
            reposts: reposted ? prev.reposts - 1 : prev.reposts + 1
        }));
    };

    return (
        <div className="p-4 border-b hover:bg-slate-50 transition cursor-pointer">
            <div className="flex gap-3">
                <Avatar>
                    <AvatarImage src={tweet.author.avatar} />
                    <AvatarFallback>{tweet.author.name[0]}</AvatarFallback>
                </Avatar>
                <div className="flex-1">
                    <div className="flex items-center gap-2">
                        <span className="font-bold hover:underline">{tweet.author.name}</span>
                        <span className="text-muted-foreground">{tweet.author.handle}</span>
                        <span className="text-muted-foreground">·</span>
                        <span className="text-muted-foreground hover:underline">{tweet.timestamp}</span>
                    </div>
                    <div className="mt-1 text-base leading-normal whitespace-pre-wrap">
                        {tweet.content}
                    </div>
                    <div className="flex justify-between mt-3 text-muted-foreground w-full max-w-md">
                        <ActionButton icon={<MessageCircle className="h-5 w-5" />} count={localStats.replies} color="blue" />
                        <ActionButton
                            icon={<Repeat2 className="h-5 w-5" />}
                            count={localStats.reposts}
                            color="green"
                            active={reposted}
                            onClick={handleRepost}
                        />
                        <ActionButton
                            icon={<Heart className={cn("h-5 w-5", liked && "fill-current")} />}
                            count={localStats.likes}
                            color="pink"
                            active={liked}
                            onClick={handleLike}
                        />
                        <ActionButton icon={<BarChart2 className="h-5 w-5" />} count={localStats.views} color="blue" />
                        <div className="flex">
                            <Button variant="ghost" size="icon" className="h-8 w-8 hover:text-blue-500 hover:bg-blue-50">
                                <Bookmark className={cn("h-5 w-5", bookmarked && "fill-current")} onClick={() => setBookmarked(!bookmarked)} />
                            </Button>
                            <Button variant="ghost" size="icon" className="h-8 w-8 hover:text-blue-500 hover:bg-blue-50">
                                <Share className="h-5 w-5" />
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

interface ActionButtonProps {
    icon: React.ReactNode;
    count: number;
    color: "blue" | "green" | "pink";
    active?: boolean;
    onClick?: () => void;
}

function ActionButton({ icon, count, color, active, onClick }: ActionButtonProps) {
    const colorClass = {
        blue: "hover:text-blue-500 group-hover:bg-blue-50",
        green: "hover:text-green-500 group-hover:bg-green-50",
        pink: "hover:text-pink-500 group-hover:bg-pink-50",
    };

    const textClass = {
        blue: "group-hover:text-blue-500",
        green: "group-hover:text-green-500",
        pink: "group-hover:text-pink-500",
    };

    const activeClass = {
        blue: "text-blue-500",
        green: "text-green-500",
        pink: "text-pink-500",
    };

    return (
        <button
            className={cn("flex items-center gap-2 group transition", active && activeClass[color])}
            onClick={(e) => {
                e.stopPropagation();
                onClick?.();
            }}
        >
            <div className={cn("p-2 rounded-full transition", colorClass[color])}>
                {icon}
            </div>
            <span className={cn("text-sm", textClass[color])}>{formatCount(count)}</span>
        </button>
    )
}

function formatCount(count: number): string {
    if (count >= 1000000) return (count / 1000000).toFixed(1) + "M";
    if (count >= 1000) return (count / 1000).toFixed(1) + "K";
    return count.toString();
}
