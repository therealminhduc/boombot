import { Link } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import { RulesService } from "../services";
import type { DomainRule } from "../services";
import AddRuleForm from "../components/AddRuleForm";
import RulesTable from "../components/RulesTable";
import { Button } from "@/components/ui/button";
import { ArrowLeft } from "lucide-react";

export default function Contribution() {
    const [pendingRules, setPendingRules] = useState<DomainRule[]>([]);
    const [approvedRules, setApprovedRules] = useState<DomainRule[]>([]);

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
    }

    useEffect(() => {
        loadRules();
    }, []);

    return (
        <div className="p-8 max-w-[1200px] mx-auto flex flex-col items-center min-h-screen relative">
            <div className="absolute top-8 right-8 z-10">
                <Button asChild size="sm" variant="outline">
                    <Link to="/admin" className="flex items-center gap-2">
                        Admin
                    </Link>
                </Button>
            </div>

            <h1 className="text-center mb-8">Boombot Contribution</h1>

            <div className="w-full max-w-[1000px]">
                <AddRuleForm onSuccess={loadRules} />

                <RulesTable
                    title="Pending Rules"
                    rules={pendingRules}
                    type="pending"
                    onAction={loadRules}
                    isAdmin={false}
                />

                <RulesTable
                    title="Approved Rules"
                    rules={approvedRules}
                    type="approved"
                />
            </div>

            <Button asChild size="lg" variant="ghost">
                <Link to="/" className="flex items-center gap-2">
                    <ArrowLeft className="w-6 h-6" /> Back to Home
                </Link>
            </Button>
      </div>
    );

}