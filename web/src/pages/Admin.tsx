import { useState, useEffect } from "react";
import { RulesService } from "../services";
import type { DomainRule } from "../services";
import RulesTable from "../components/RulesTable";
import CreateAdminForm from "../components/CreateAdminForm";
import { Button } from "@/components/ui/button";
import { Link } from "@tanstack/react-router";
import { useAuth } from "../contexts/AuthContext";
import { ArrowLeft, Users, ChevronDown, ChevronUp } from "lucide-react";

export default function Admin() {
    const { logout } = useAuth();

    const [pendingRules, setPendingRules] = useState<DomainRule[]>([]);
    const [approvedRules, setApprovedRules] = useState<DomainRule[]>([]);
    const [showAdminCreation, setShowAdminCreation] = useState(false);

    const loadRules = async () => {
        try {
            const [pending, approved] = await Promise.all([
                RulesService.getPendingRules(),
                RulesService.getApprovedRules(),
            ]);

            setPendingRules(pending);
            setApprovedRules(approved);
        } catch (error) {
            console.error("Error loading rules:", error);
        }
    };

    useEffect(() => {
        loadRules();
    }, []);

    return (
        <div className="p-8 max-w-[1200px] mx-auto flex flex-col items-center min-h-screen relative">
            <div className="absolute top-5 right-5">
                <Button variant="outline" onClick={logout}>
                    Logout
                </Button>
            </div>

            <h1 className="text-center mb-8">Admin Panel</h1>
            
            <div className="w-full max-w-[1000px] space-y-8">
                {/* Admin Creation Section */}
                <div className="w-full">
                    <Button
                        variant="outline"
                        onClick={() => setShowAdminCreation(!showAdminCreation)}
                        className="w-full flex items-center justify-between"
                    >
                        <span className="flex items-center gap-2">
                            <Users className="w-4 h-4" />
                            Admin Management
                        </span>
                        {showAdminCreation ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
                    </Button>
                    
                    <div className={`mt-4 transition-all duration-200 ease-in-out ${showAdminCreation ? 'opacity-100 max-h-[500px]' : 'opacity-0 max-h-0'}`} style={{ overflow: 'hidden' }}>
                        {showAdminCreation && <CreateAdminForm />}
                    </div>
                </div>

                {/* Rules Section */}
                <div className="w-full space-y-8">
                    <RulesTable 
                        title="Pending Rules" 
                        rules={pendingRules} 
                        type="pending"
                        onAction={loadRules}
                        isAdmin={true}
                    />

                    <RulesTable 
                        title="Approved Rules" 
                        rules={approvedRules} 
                        type="approved"
                    />
                </div>
            </div>

            <Button asChild size="lg" variant="ghost">
                <Link to="/" className="flex items-center gap-2">
                    <ArrowLeft className="w-6 h-6" /> Back to Home
                </Link>
            </Button>
        </div>
    );
}