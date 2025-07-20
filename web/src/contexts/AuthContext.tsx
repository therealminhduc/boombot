import { createContext, useContext, useEffect, useState, type ReactNode } from "react";
import type { AuthContextType } from "../types/auth";

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [token, setToken] = useState<string | null>(null);
    const [loading, setLoading] = useState(true);
    const [wasLoggedOut, setWasLoggedOut] = useState(false);

    useEffect(() => {
        // Check for existing token on app load
        const savedToken = localStorage.getItem("admin_token");
        if (savedToken) {
            setToken(savedToken);
        }
        setLoading(false);
    }, []);

    const login = (newToken: string) => {
        localStorage.setItem("admin_token", newToken);
        setToken(newToken);
        setWasLoggedOut(false);
    };

    const logout = () => {
        localStorage.removeItem("admin_token");
        setToken(null);
        setWasLoggedOut(true);
    };

    const value = {
        isAuthenticated: !!token,
        token,
        login,
        logout,
        loading,
        wasLoggedOut,
        resetLogoutFlag: () => setWasLoggedOut(false),
    };

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
    const context = useContext(AuthContext);

    if (context === undefined) {
        throw new Error("useAuth must be used within an AuthProvider");
    }

    return context;
}