export interface AuthContextType {
    isAuthenticated: boolean;
    token: string | null;
    login: (token: string) => void;
    logout: () => void;
    loading: boolean;
    wasLoggedOut: boolean;
    resetLogoutFlag: () => void;
}