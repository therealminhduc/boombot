import { z } from "zod";

const domainRegex = /^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;
const urlRegex = /^https?:\/\/.+/;

export const submissionSchema = z.object({
    contributor: z.string().min(1, "Contributor is required"),
    domain: z.string()
        .min(1, "Domain is required")
        .refine((value) => {
            return domainRegex.test(value) || urlRegex.test(value);
        }, "Must be a valid domain name or URL"),
    keys: z.string().min(1, "At least one key is required"),
    startsWith: z.string().optional(),
});

export type SubmissionFormData = z.infer<typeof submissionSchema>; 