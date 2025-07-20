import { useNavigate } from "@tanstack/react-router";
import { useAuth } from "../contexts/AuthContext";
import { useEffect } from "react";

interface ProtectedRouteProps {
    children: React.ReactNode;
}

export default function ProtectedRoute({ children }: ProtectedRouteProps) {
    const { isAuthenticated, loading, wasLoggedOut, resetLogoutFlag } = useAuth();
    const navigate = useNavigate();

    useEffect(() => {
        if (!loading && !isAuthenticated) {
            if (wasLoggedOut) {
                navigate({ to: '/contribution' });
                setTimeout(() => {
                    resetLogoutFlag();
                }, 100);
            } else {
                navigate({ to: '/admin/connection' });
            }
        }
    }, [isAuthenticated, loading, wasLoggedOut, navigate, resetLogoutFlag]);

    if (loading) {
        return (
          <div className="min-h-screen flex items-center justify-center">
            <div className="text-lg">Loading...</div>
          </div>
        );
    }

    if (!isAuthenticated) {
        return null;
    }

    return <>{children}</>;
}
