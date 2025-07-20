import type { ApiResponse } from '../types/api';
import { API_CONFIG } from '../config/api';

export class BaseApiService {
    protected static async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
        if (response.ok) {
            return await response.json();
        } else {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
    }

    protected static async get<T>(endpoint: string): Promise<T> {
        const response = await fetch(`${API_CONFIG.BASE_URL}${endpoint}`);
        const result = await this.handleResponse<T>(response);

        if (result.success && result.data) {
            return result.data;
        } else {
            throw new Error(result.error || `Failed to GET ${endpoint}`);
        }
    }

    protected static async post<TRequest, TResponse>(
        endpoint: string, 
        data: TRequest
    ): Promise<TResponse> {
        const response = await fetch(`${API_CONFIG.BASE_URL}${endpoint}`, {
            method: 'POST',
            headers: API_CONFIG.HEADERS.JSON,
            body: JSON.stringify(data),
        });

        const result = await this.handleResponse<TResponse>(response);

        if (result.success) {
            return result.data as TResponse;
        } else {
            throw new Error(result.error || `Failed to POST ${endpoint}`);
        }
    }

    protected static async put<T = void>(endpoint: string): Promise<T> {
        const response = await fetch(`${API_CONFIG.BASE_URL}${endpoint}`, {
            method: 'PUT'
        });

        const result = await this.handleResponse<T>(response);

        if (result.success) {
            return result.data as T;
        } else {
            throw new Error(result.error || `Failed to PUT ${endpoint}`);
        }
    }
} 