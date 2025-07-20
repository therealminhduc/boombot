import { BaseApiService } from './base';
import type { DomainRule, SubmissionRequest } from '../types/api';
import { API_CONFIG } from '../config/api';

export class RulesService extends BaseApiService {
    // Get all pending rules
    static async getPendingRules(): Promise<DomainRule[]> {
        return this.get<DomainRule[]>(API_CONFIG.ENDPOINTS.RULES.PENDING);
    }

    // Get all approved rules
    static async getApprovedRules(): Promise<DomainRule[]> {
        return this.get<DomainRule[]>(API_CONFIG.ENDPOINTS.RULES.APPROVED);
    }

    // Submit a new rule
    static async submitRule(request: SubmissionRequest): Promise<void> {
        return this.post<SubmissionRequest, void>(API_CONFIG.ENDPOINTS.RULES.SUBMIT, request);
    }

    // Approve a rule
    static async approveRule(id: number): Promise<void> {
        return this.put<void>(API_CONFIG.ENDPOINTS.RULES.APPROVE(id));
    }

    // Reject a rule
    static async rejectRule(id: number): Promise<void> {
        return this.put<void>(API_CONFIG.ENDPOINTS.RULES.REJECT(id));
    }
} 