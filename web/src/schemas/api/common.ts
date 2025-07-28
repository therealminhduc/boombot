import { z } from "zod";

export const apiResponseSchema = z.object({
    success: z.boolean(),
    data: z.any().optional(),
    message: z.string().optional(),
    error: z.string().optional(),
});

export const ruleIdSchema = z.number().int().positive("Rule ID must be a positive integer");

export type RuleId = z.infer<typeof ruleIdSchema>;