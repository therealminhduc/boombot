import { BaseApiService } from './base';
import type { CreateAdminRequest, AdminLoginRequest } from '../types/api';
import { API_CONFIG } from '../config/api';

export class AdminService extends BaseApiService {
    // Create admin user
    static async createAdmin(request: CreateAdminRequest): Promise<string> {
        return this.post<CreateAdminRequest, string>(API_CONFIG.ENDPOINTS.ADMIN.CREATE, request);
    }

    // Login admin user (for future implementation)
    static async loginAdmin(request: AdminLoginRequest): Promise<string> {
        return this.post<AdminLoginRequest, string>(API_CONFIG.ENDPOINTS.ADMIN.LOGIN, request);
    }
} 