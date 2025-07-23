export const API_CONFIG = {
    BASE_URL: import.meta.env.VITE_API_URL || "http://localhost:8000/api",
    ENDPOINTS: {
        RULES: {
            PENDING: "/rules/pending",
            APPROVED: "/rules/approved",
            SUBMIT: "/submit",
            APPROVE: (id: number) => `/rules/${id}/approve`,
            REJECT: (id: number) => `/rules/${id}/reject`,
        },
        ADMIN: {
            CREATE: "/admin/create",
            LOGIN: "/admin/login", // For future use
        },
        HEALTH: "/health",
    },
    HEADERS: {
        JSON: {
            'Content-Type': 'application/json',
        },
    },
} as const; 