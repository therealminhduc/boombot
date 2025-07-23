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

export interface CreateAdminRequest {
    username: string;
    password: string;
}

export interface AdminLoginRequest {
    username: string;
    password: string;
} 