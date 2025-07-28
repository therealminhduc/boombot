import { z } from "zod";

export const domainRuleSchema = z.object({
    id: z.number(),
    domain: z.string(),
    keys: z.array(z.string()),
    starts_with: z.array(z.string()),
    contributors: z.array(z.string()).optional(),
    status: z.string(),
});

export const submissionRequestSchema = z.object({
    domain: z.string(),
    keys: z.array(z.string()),
    starts_with: z.array(z.string()).optional(),
    contributor: z.string(),
});

export type DomainRule = z.infer<typeof domainRuleSchema>;
export type SubmissionRequest = z.infer<typeof submissionRequestSchema>; 