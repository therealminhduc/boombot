const API_URL = "http://localhost:8000/api";

export interface DomainRule {
    id: number;
    domain: string;
    keys: string[];
    starts_with: string[];
    contributors?: string[];
    status: string;
}

export interface ApiResponse<T> {
    success: boolean;
    data?: T;
    message?: string;
    error?: string;
}

export interface SubmissionRequest {
    domain: string;
    keys: string[];
    starts_with?: string[];
    contributor: string;
}

export class ApiService {
    private static async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
        if (response.ok) {
            return await response.json();
        } else {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
    }

    // Get all pending rules
    static async getPendingRules(): Promise<DomainRule[]> {
        const response = await fetch(`${API_URL}/rules/pending`);
        const result = await this.handleResponse<DomainRule[]>(response);

        if (result.success && result.data) {
            return result.data;
        } else {
            throw new Error(result.error || 'Failed to fetch pending rules');
        }
    }

    // Get all approved rules
    static async getApprovedRules(): Promise<DomainRule[]> {
        const response = await fetch(`${API_URL}/rules/approved`);
        const result = await this.handleResponse<DomainRule[]>(response);
        
        if (result.success && result.data) {
            return result.data;
        } else {
            throw new Error(result.error || 'Failed to fetch approved rules');
        }
    }

    // Submit a new rule
    static async submitRule(request: SubmissionRequest): Promise<void> {
        const response = await fetch(`${API_URL}/submit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        const result = await this.handleResponse<void>(response);
        
        if (!result.success) {
            throw new Error(result.error || 'Failed to submit rule');
        }
    }

    // Approve a rule
    static async approveRule(id: number): Promise<void> {
        const response = await fetch(`${API_URL}/rules/${id}/approve`, {
            method: 'PUT'
        });

        const result = await this.handleResponse<void>(response);
        
        if (!result.success) {
            throw new Error(result.error || 'Failed to approve rule');
        }
    }

    // Reject a rule
    static async rejectRule(id: number): Promise<void> {
        const response = await fetch(`${API_URL}/rules/${id}/reject`, {
            method: 'PUT'
        });

        const result = await this.handleResponse<void>(response);
        
        if (!result.success) {
            throw new Error(result.error || 'Failed to reject rule');
        }
    }
}