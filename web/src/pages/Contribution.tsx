import { Link } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import { ApiService } from "../services/api";
import type { DomainRule } from "../services/api";
import AddRuleForm from "../components/AddRuleForm";
import RulesTable from "../components/RulesTable";
import { Button } from "@/components/ui/button";


export default function Contribution() {
    const [pendingRules, setPendingRules] = useState<DomainRule[]>([]);
    const [approvedRules, setApprovedRules] = useState<DomainRule[]>([]);

    const loadRules = async () => {
        try {
            const [pending, approved] = await Promise.all([
                ApiService.getPendingRules(),
                ApiService.getApprovedRules(),
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
        <div style={{ 
            padding: '2rem', 
            maxWidth: '1200px', 
            margin: '0 auto',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            minHeight: '100vh'
        }}>
        <h1 style={{ textAlign: 'center', marginBottom: '2rem' }}>Boombot Contribution</h1>
        
        <div style={{ width: '100%', maxWidth: '1000px' }}>
            <AddRuleForm onSuccess={loadRules} />
            
            <RulesTable 
              title="Pending Rules" 
              rules={pendingRules} 
              type="pending"
              onAction={loadRules}
            />
            
            <RulesTable 
              title="Approved Rules" 
              rules={approvedRules} 
              type="approved"
            />
        </div>

        
        <Button asChild size="lg" variant="ghost">
            <Link to="/" className="mt-4">
                ← Back to Home
            </Link>
        </Button>
      </div>
    );

}