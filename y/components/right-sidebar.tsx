import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

export function RightSidebar() {
    return (
        <div className="flex flex-col h-screen sticky top-0 w-[350px] pl-8 py-2 hidden lg:flex space-y-4">
            <Input placeholder="Search" className="rounded-full bg-slate-100 border-none h-12 px-12" />

            <Card className="rounded-2xl bg-slate-50 border-none">
                <CardHeader>
                    <CardTitle className="text-xl font-extrabold">Subscribe to Premium</CardTitle>
                </CardHeader>
                <CardContent>
                    <p className="mb-4">Subscribe to unlock new features and if eligible, receive a share of ads revenue.</p>
                    <Button className="rounded-full font-bold">Subscribe</Button>
                </CardContent>
            </Card>

            <Card className="rounded-2xl bg-slate-50 border-none">
                <CardHeader>
                    <CardTitle className="text-xl font-extrabold">What's happening</CardTitle>
                </CardHeader>
                <CardContent>
                    {/* Trends would go here */}
                    <div className="space-y-4">
                        <Trend topic="Technology" title="AI Agents" posts="100K posts" />
                        <Trend topic="Sports" title="Super Bowl" posts="50K posts" />
                    </div>
                </CardContent>
            </Card>

            <Card className="rounded-2xl bg-slate-50 border-none">
                <CardHeader>
                    <CardTitle className="text-xl font-extrabold">Who to follow</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                    <FollowSuggestion name="Elon Musk" handle="@elonmusk" />
                    <FollowSuggestion name="X" handle="@X" />
                </CardContent>
            </Card>
        </div>
    );
}

function Trend({ topic, title, posts }: { topic: string; title: string, posts: string }) {
    return (
        <div className="cursor-pointer hover:bg-slate-200 p-2 -mx-2 rounded-lg transition">
            <div className="text-xs text-muted-foreground">{topic} · Trending</div>
            <div className="font-bold">{title}</div>
            <div className="text-xs text-muted-foreground">{posts}</div>
        </div>
    )
}

function FollowSuggestion({ name, handle }: { name: string; handle: string }) {
    return (
        <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
                <Avatar className="h-10 w-10">
                    <AvatarFallback>{name[0]}</AvatarFallback>
                </Avatar>
                <div className="leading-tight">
                    <div className="font-bold hover:underline cursor-pointer">{name}</div>
                    <div className="text-muted-foreground">{handle}</div>
                </div>
            </div>
            <Button variant="secondary" className="rounded-full font-bold">Follow</Button>
        </div>
    )
}
