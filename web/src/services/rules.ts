import { BaseApiService } from './base';
import type { DomainRule, SubmissionRequest } from '../types/api';
import { API_CONFIG } from '../config/api';
import { z } from 'zod';
import { domainRuleSchema, submissionRequestSchema } from '../schemas/api/rules';
import { ruleIdSchema } from '../schemas/api/common';

export class RulesService extends BaseApiService {
    // Get all pending rules
    static async getPendingRules(): Promise<DomainRule[]> {
        const response = await this.get<DomainRule[]>(API_CONFIG.ENDPOINTS.RULES.PENDING);
        const validated = z.array(domainRuleSchema).parse(response);
        return validated;
    }

    // Get all approved rules
    static async getApprovedRules(): Promise<DomainRule[]> {
        const response = await this.get<DomainRule[]>(API_CONFIG.ENDPOINTS.RULES.APPROVED);
        const validated = z.array(domainRuleSchema).parse(response);
        return validated;
    }

    // Submit a new rule
    static async submitRule(request: SubmissionRequest): Promise<void> {
        const validated = submissionRequestSchema.parse(request);
        return this.post<SubmissionRequest, void>(API_CONFIG.ENDPOINTS.RULES.SUBMIT, validated);
    }

    // Approve a rule
    static async approveRule(id: number): Promise<void> {
        const validatedId = ruleIdSchema.parse(id);
        return this.put<void>(API_CONFIG.ENDPOINTS.RULES.APPROVE(validatedId));
    }

    // Reject a rule
    static async rejectRule(id: number): Promise<void> {
        const validatedId = ruleIdSchema.parse(id);
        return this.put<void>(API_CONFIG.ENDPOINTS.RULES.REJECT(validatedId));
    }
} 