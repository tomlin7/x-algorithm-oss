import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Sidebar } from "@/components/sidebar";
import { RightSidebar } from "@/components/right-sidebar";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Y - X Clone",
  description: "A production-ready X clone powered by x-algorithm",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="container mx-auto max-w-7xl h-full flex justify-center">
          <Sidebar />
          <main className="w-full max-w-[600px] border-r border-l min-h-screen">
            {children}
          </main>
          <RightSidebar />
        </div>
      </body>
    </html>
  );
}
