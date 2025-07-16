import { Link } from "@tanstack/react-router";
import boomBotLogo from "../assets/boom-bot.png";
import { Button } from "@/components/ui/button";

export default function Home() {
    return (
        <div className="flex flex-col items-center justify-center min-h-screen p-8 text-center">
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
                <Button asChild size="lg">
                    <Link to="/contribution">
                        Go to Contribution Page
                    </Link>
                </Button>
            </div>
        </div>
    );
}