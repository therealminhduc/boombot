import { useState } from "react";
import { RulesService, type SubmissionRequest } from "../services";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { toast } from "sonner";

interface AddRuleFormProps {
    onSuccess: () => void;
}

export default function AddRuleForm({ onSuccess }: AddRuleFormProps) {
    const [formData, setFormData] = useState({
        contributor: "",
        domain: "",
        keys: "",
        startsWith: "",
    });

    const [isSubmitting, setIsSubmitting] = useState(false);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsSubmitting(true);

        try {
            const keys = formData.keys.split(',').map(key => key.trim()).filter(key => key.length > 0);
            const startsWith = formData.startsWith ? formData.startsWith.split(',').map(s => s.trim()).filter(s => s.length > 0) : undefined;
            
            const request: SubmissionRequest = {
                domain: formData.domain,
                keys: keys,
                starts_with: startsWith,
                contributor: formData.contributor,
            };

            await RulesService.submitRule(request);

            // Reset form
            setFormData({
                contributor: "",
                domain: "",
                keys: "",
                startsWith: "",
            });

            onSuccess();
        } catch (error) {
            console.error("Submission error:", error);
            const errorMessage = error instanceof Error ? error.message : 'Unknown error';
            
            toast.error(`Failed to submit rule: ${errorMessage}`);
        } finally {
            setIsSubmitting(false);
        }
    }

    return (
        <section style={{ marginBottom: '2rem' }}>
            <h2 className="text-2xl font-semibold mb-4">Add a new rule</h2>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '1rem' }}>
                <Input 
                    type="text" 
                    placeholder="Your name or email" 
                    value={formData.contributor}
                    onChange={(e) => setFormData(prev => ({ ...prev, contributor: e.target.value }))}
                    required
                />

                <Input
                    type="text"
                    placeholder="e.g. twitter.com"
                    value={formData.domain}
                    onChange={(e) => setFormData(prev => ({ ...prev, domain: e.target.value }))}
                    required
                />

                <Input
                    type="text"
                    placeholder="e.g. utm_source, fbclid"
                    value={formData.keys}
                    onChange={(e) => setFormData(prev => ({ ...prev, keys: e.target.value }))}
                />

                <Input
                    type="text"
                    placeholder="e.g. utm_, ref_"
                    value={formData.startsWith}
                    onChange={(e) => setFormData(prev => ({ ...prev, startsWith: e.target.value }))}
                />
            </div>

            <Button 
                type="submit" 
                disabled={isSubmitting}
                className="px-4 py-2 bg-blue-500 text-white border-none rounded cursor-pointer disabled:cursor-not-allowed hover:bg-blue-600"
            >
                {isSubmitting ? 'Submitting...' : 'Submit Rule'}
            </Button>
            </form>
        </section>
    )
}