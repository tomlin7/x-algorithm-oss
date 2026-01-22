import Link from "next/link";
import { Home, User, Bell, Mail, Search, PenSquare, X } from "lucide-react";
import { Button } from "@/components/ui/button";

export function Sidebar() {
    return (
        <div className="flex flex-col h-screen fixed w-[275px] px-2 border-r">
            <div className="p-4">
                <Link href="/">
                    <X className="h-8 w-8" />
                </Link>
            </div>
            <nav className="flex-1 flex flex-col space-y-2 px-2">
                <SidebarItem icon={<Home />} label="Home" href="/" />
                <SidebarItem icon={<Search />} label="Explore" href="/explore" />
                <SidebarItem icon={<Bell />} label="Notifications" href="/notifications" />
                <SidebarItem icon={<Mail />} label="Messages" href="/messages" />
                <SidebarItem icon={<User />} label="Profile" href="/profile" />

                <Button className="w-full mt-4 rounded-full h-12 text-lg font-bold" size="lg">
                    Post
                </Button>
            </nav>

            <div className="p-4">
                <Button variant="ghost" className="w-full justify-start rounded-full h-16 px-4">
                    <div className="flex items-center gap-3">
                        <div className="h-10 w-10 rounded-full bg-slate-200" />
                        <div className="text-left">
                            <div className="font-bold">User</div>
                            <div className="text-muted-foreground">@user</div>
                        </div>
                    </div>
                </Button>
            </div>
        </div>
    );
}

function SidebarItem({ icon, label, href }: { icon: React.ReactNode; label: string; href: string }) {
    return (
        <Link href={href}>
            <Button variant="ghost" className="w-full justify-start gap-4 text-xl rounded-full h-14 px-4">
                {icon}
                <span>{label}</span>
            </Button>
        </Link>
    );
}
