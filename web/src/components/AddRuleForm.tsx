import { RulesService } from "../services";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { toast } from "sonner";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { type SubmissionFormData, submissionSchema } from "../schemas/forms/rules";

interface AddRuleFormProps {
    onSuccess: () => void;
}

export default function AddRuleForm({ onSuccess }: AddRuleFormProps) {

    const { 
        register,
        handleSubmit,
        reset,
        formState: { isSubmitting, errors },
    } = useForm<SubmissionFormData>({
        resolver: zodResolver(submissionSchema),
    });

    const onSubmit = async (data: SubmissionFormData) => {
        try {
            const keys = data.keys
                .split(',')
                .map(key => key.trim())
                .filter(Boolean);
            
            const startsWith = data.startsWith
                ? data.startsWith
                    .split(',')
                    .map(s => s.trim())
                    .filter(Boolean)
                : undefined;

            await RulesService.submitRule({
                domain: data.domain,
                keys,
                starts_with: startsWith,
                contributor: data.contributor,
            });

            reset();

            onSuccess();
            toast.success("Rule submitted successfully");

        } catch (error) {
            console.error("Submission error:", error);
            toast.error("Failed to submit rule");
        }
    }

    return (
        <section style={{ marginBottom: '2rem' }}>
            <h2 className="text-2xl font-semibold mb-4">Add a new rule</h2>
            <form onSubmit={handleSubmit(onSubmit)} style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '1rem' }}>
                <div>
                    <Input 
                        type="text" 
                        placeholder="Your name or email" 
                        required
                        {...register("contributor")}
                    />
                    {errors.contributor && (
                        <p className="text-red-500 text-sm">{errors.contributor.message}</p>
                    )}
                </div>

                <div>
                    <Input
                        type="text"
                        placeholder="e.g. twitter.com"
                        required
                        {...register("domain")}
                    />
                    {errors.domain && (
                        <p className="text-red-500 text-sm">{errors.domain.message}</p>
                    )}  
                </div>

                <div>
                    <Input
                        type="text"
                        placeholder="e.g. utm_source, fbclid"
                        {...register("keys")}
                    />
                    {errors.keys && (
                        <p className="text-red-500 text-sm">{errors.keys.message}</p>
                    )}
                </div>

                <div>
                    <Input
                        type="text"
                        placeholder="e.g. utm_, ref_"
                        {...register("startsWith")}
                    /> 
                </div>
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