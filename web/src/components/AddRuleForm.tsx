import { RulesService } from "../services";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { toast } from "sonner";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { type SubmissionFormData, submissionSchema } from "../schemas/forms/rules";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";

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
        <div className=" space-y-6">
            <h2 className="text-2xl font-semibold mb-4">Add a new rule</h2>

            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">Rule Information</CardTitle>
                    <CardDescription>Provide the details of the rule you want to submit</CardDescription>
                </CardHeader>

                <CardContent>
                    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                        <div className="spacy-y-2">
                            <Label htmlFor="contributor" className="text-sm font-medium">
                                Your name or email *
                            </Label>

                            <Input 
                                id="contributor"
                                type="text"
                                placeholder="e.g. john.doe@example.com or John Doe"
                                className={errors.contributor ? "border-red-500" : ""}
                                {...register("contributor")}
                            />
                            {errors.contributor && (
                                <p className="text-red-500 text-sm">{errors.contributor.message}</p>
                            )}
                        </div>

                        <Separator />

                        <div className="space-y-2">
                            <Label htmlFor="domain" className="text-sm font-medium">Domain *</Label>

                            <Input
                                id="domain"
                                type="text"
                                placeholder="e.g. twitter.com"
                                className={errors.domain ? "border-red-500" : ""}
                                {...register("domain")}
                            />
                            {errors.domain && (
                                <p className="text-red-500 text-sm">{errors.domain.message}</p>
                            )}
                        </div>

                        <Separator />

                        <div className="space-y-4">
                            <div className="space-y-2">
                                <Label htmlFor="keys" className="text-sm font-medium">
                                Parameters to Remove *
                                </Label>
                                <Input
                                id="keys"
                                type="text"
                                placeholder="e.g. utm_source, fbclid, ref"
                                className={errors.keys ? "border-red-500" : ""}
                                {...register("keys")}
                                />
                                {errors.keys && (
                                    <p className="text-red-500 text-sm">{errors.keys.message}</p>
                                )}
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="startsWith" className="text-sm font-medium">
                                Parameters Starting With (Optional)
                                </Label>
                                <Input
                                id="startsWith"
                                type="text"
                                placeholder="e.g. utm_, ref_, fb_"
                                {...register("startsWith")}
                                />
                            </div>
                        </div>

                        <Button 
                            type="submit" 
                            disabled={isSubmitting}
                            className="w-full"
                        >
                            {isSubmitting ? 'Submitting...' : 'Submit Rule'}
                        </Button>
                    </form>
                </CardContent>
            </Card>
        </div>
    );
}