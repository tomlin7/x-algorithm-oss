import Link from "next/link";
import { Home, User, Bell, Mail, Search, PenSquare, X } from "lucide-react";
import { Button } from "@/components/ui/button";

export function Sidebar() {
    return (
        <div className="flex flex-col h-screen sticky top-0 w-[275px] px-2 items-end">
            <div className="w-[250px] flex flex-col h-full">
                <div className="p-4">
                    <Link href="/">
                        <div className="hover:bg-slate-100 w-12 h-12 flex items-center justify-center rounded-full transition duration-200">
                            <X className="h-8 w-8" />
                        </div>
                    </Link>
                </div>
                <nav className="flex-1 flex flex-col space-y-2">
                    <SidebarItem icon={<Home className="h-7 w-7" />} label="Home" href="/" />
                    <SidebarItem icon={<Search className="h-7 w-7" />} label="Explore" href="/explore" />
                    <SidebarItem icon={<Bell className="h-7 w-7" />} label="Notifications" href="/notifications" />
                    <SidebarItem icon={<Mail className="h-7 w-7" />} label="Messages" href="/messages" />
                    <SidebarItem icon={<User className="h-7 w-7" />} label="Profile" href="/profile" />

                    <Button className="w-full mt-4 rounded-full h-14 text-xl font-bold bg-blue-500 hover:bg-blue-600 shadow-md" size="lg">
                        Post
                    </Button>
                </nav>

                <div className="my-4">
                    <Button variant="ghost" className="w-full h-16 rounded-full px-4 hover:bg-slate-100 flex items-center justify-between transition duration-200 group">
                        <div className="flex items-center gap-3">
                            <div className="h-10 w-10 rounded-full bg-slate-200" />
                            <div className="text-left leading-tight hidden xl:block">
                                <div className="font-bold">User</div>
                                <div className="text-muted-foreground text-sm">@user</div>
                            </div>
                        </div>
                    </Button>
                </div>
            </div>
        </div>
    );
}

function SidebarItem({ icon, label, href }: { icon: React.ReactNode; label: string; href: string }) {
    return (
        <Link href={href} className="group">
            <div className="flex items-center gap-4 text-xl rounded-full h-14 px-4 w-fit hover:bg-slate-200 transition duration-200 cursor-pointer">
                {icon}
                <span className="pr-4">{label}</span>
            </div>
        </Link>
    );
}
