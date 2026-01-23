import { Sidebar } from "@/components/sidebar";
import { RightSidebar } from "@/components/right-sidebar";

export default function Profile() {
    return (
        <div className="flex flex-col">
            <div className="h-32 bg-slate-200"></div>
            <div className="px-4 pb-4 border-b">
                <div className="relative h-16 w-16 -mt-8 mb-4">
                    <div className="h-32 w-32 rounded-full border-4 border-white bg-slate-400 absolute bottom-0"></div>
                </div>
                <div className="font-extrabold text-2xl">User</div>
                <div className="text-muted-foreground">@user</div>
                <div className="mt-4">
                    I am a user of the Y platform.
                </div>
                <div className="mt-4 flex gap-4 text-muted-foreground">
                    <span><span className="font-bold text-black">100</span> Following</span>
                    <span><span className="font-bold text-black">500</span> Followers</span>
                </div>
            </div>
            <div className="h-[200px] flex items-center justify-center text-muted-foreground">
                No posts yet.
            </div>
        </div>
    );
}
