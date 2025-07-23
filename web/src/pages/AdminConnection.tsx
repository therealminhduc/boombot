import { useEffect, useState } from 'react';
import { Button } from '../../components/ui/button';
import { Input } from '../../components/ui/input';
import { AdminService } from '../services';
import { toast } from 'sonner';
import { Label } from '@/components/ui/label';
import { useAuth } from '../contexts/AuthContext';
import { useNavigate } from '@tanstack/react-router';
import { ArrowLeft } from 'lucide-react';

export default function AdminConnection() {
    const { login, resetLogoutFlag } = useAuth();
    const navigate = useNavigate();

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    
    useEffect(() => {
        resetLogoutFlag();
    }, [resetLogoutFlag]);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        
        if (!username.trim() || !password.trim()) {
            toast.error('Username and password are required');
            return;
        }

        setIsLoading(true);

        try {
            const request = { username, password };
            const token = await AdminService.loginAdmin(request);
            
            login(token);
            toast.success('Login successful');
            navigate({ to: '/admin' });
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'An error occurred';
            toast.error(errorMessage);
        } finally {
            setIsLoading(false);
        }
    };

    const handleBack = () => {
        // Always navigate to home page for consistency
        navigate({ to: '/contribution' });
    };

    return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div className="max-w-md w-full space-y-8">
                {/* Back Button */}
                <div className="flex justify-start">
                    <Button onClick={handleBack}>
                        <ArrowLeft />
                    </Button>
                </div>

                <div>
                    <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        Admin Sign In
                    </h2>
                </div>

                <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
                    <div className="space-y-4">
                        <div className="grid w-full items-center gap-3">
                            <Label htmlFor="username">Username</Label>
                            <Input
                                id="username"
                                name="username"
                                type="text"
                                required
                                value={username}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUsername(e.target.value)}
                                placeholder="Enter username"
                                disabled={isLoading}
                            />
                        </div>

                        <div className="grid w-full items-center gap-3">
                            <Label htmlFor="password">Password</Label>
                            <Input
                                id="password"
                                name="password"
                                type="password"
                                required
                                value={password}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
                                placeholder="Enter password"
                                disabled={isLoading}
                            />
                        </div>
                    </div>

                    <div className="w-full">
                        <Button
                            type="submit"
                            className="w-full"
                            disabled={isLoading}
                        >
                            {isLoading ? 'Signing in...' : 'Sign in'}
                        </Button>
                    </div>
                </form>
            </div>
        </div>
    );
}