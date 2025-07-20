import { Link } from "@tanstack/react-router";
import boomBotLogo from "../assets/boom-bot.png";
import { Button } from "@/components/ui/button";
import { cn } from "../lib/utils";
import { ArrowRight, GitBranch } from "lucide-react";

export default function Home() {
    return (
        <div className="relative flex h-[50rem] w-full items-center justify-center bg-white dark:bg-black">
            <div
                className={cn(
                "absolute inset-0",
                "[background-size:20px_20px]",
                "[background-image:radial-gradient(#d4d4d4_1px,transparent_1px)]",
                "dark:[background-image:radial-gradient(#404040_1px,transparent_1px)]",
                )}
            />
            <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-white [mask-image:radial-gradient(ellipse_at_center,transparent_20%,black)] dark:bg-black"></div>
            <div className="relative z-20 flex flex-col items-center justify-center p-8 text-center">
                <div className="flex flex-col items-center gap-8 max-w-2xl">
                    <img 
                        src={boomBotLogo} 
                        alt="Project Logo" 
                        className="w-80 h-auto drop-shadow-lg transition-all duration-300 ease-in-out hover:scale-105 hover:drop-shadow-xl"
                    />
                    <h1 className="text-5xl font-light m-0 tracking-tight">
                        Boombot
                    </h1>
                    <p className="text-lg m-0 mb-8 max-w-lg leading-relaxed">
                        Drop your messy links, it will clean them with a boom !
                    </p>

                    <div className="flex gap-4 items-center">
                        <Button asChild size="lg" variant="outline">
                            <a
                                href="https://github.com/therealminhduc/boombot"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="flex items-center gap-2"
                            >
                                <GitBranch className="w-6 h-6" />
                                View on GitHub
                            </a>
                        </Button>

                        <Button asChild size="lg">
                            <Link to="/contribution" className="flex items-center gap-2">
                                Contribute <ArrowRight className="w-6 h-6" />
                            </Link>
                        </Button>
                    </div>
                </div>
            </div>
            <footer className="absolute bottom-4 w-full flex justify-center z-30">
                <p className="text-xs text-gray-500 dark:text-gray-400 text-center max-w-xl px-2">
                    Name inspired by Valorant. Not affiliated with Riot Games.                
                </p>
            </footer>
        </div>
    );
}