import { useEffect } from 'react';
import { Button } from '../../components/ui/button';
import { Input } from '../../components/ui/input';
import { AdminService } from '../services';
import { toast } from 'sonner';
import { Label } from '@/components/ui/label';
import { useAuth } from '../contexts/AuthContext';
import { useNavigate } from '@tanstack/react-router';
import { ArrowLeft } from 'lucide-react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { type LoginSchema, loginSchema } from '../schemas/forms/auth';

export default function AdminConnection() {
    const { login, resetLogoutFlag } = useAuth();
    const navigate = useNavigate();
    
    const {
        register,
        handleSubmit,
        formState: { isSubmitting, errors },
    } = useForm<LoginSchema>({
        resolver: zodResolver(loginSchema),
    });

    useEffect(() => {
        resetLogoutFlag();
    }, [resetLogoutFlag]);

    const onSubmit = async (data: LoginSchema) => {
        try {
            const token = await AdminService.loginAdmin(data);
            login(token);
            toast.success('Login successful');
            navigate({ to: '/admin' });
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'An error occurred';
            toast.error(errorMessage);
        }
    };

    const handleBack = () => {
        // Always navigate to home page for consistency
        navigate({ to: '/contribution' });
    };

    return (
        <div className="min-h-screen flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div className="max-w-md w-full space-y-8">
                
                <div className="flex justify-start">
                    <Button onClick={handleBack}>
                        <ArrowLeft />
                    </Button>
                </div>

                <div>
                    <h2 className="mt-6 text-center text-3xl font-extrabold">
                        Admin Sign In
                    </h2>
                </div>

                <form className="mt-8 space-y-6" onSubmit={handleSubmit(onSubmit)}>
                    <div className="space-y-4">
                        <div className="grid w-full items-center gap-3">
                            <Label htmlFor="username">Username</Label>
                            <Input
                                id="username"
                                type="text"
                                {...register("username")}
                                placeholder="Enter username"
                                disabled={isSubmitting}
                            />
                            {errors.username && (
                                <p className="text-red-500 text-sm">{errors.username.message}</p>
                            )}
                        </div>

                        <div className="grid w-full items-center gap-3">
                            <Label htmlFor="password">Password</Label>
                            <Input
                                id="password"
                                type="password"
                                {...register("password")}
                                placeholder="Enter password"
                                disabled={isSubmitting}
                            />
                            {errors.password && (
                                <p className="text-red-500 text-sm">{errors.password.message}</p>
                            )}
                        </div>
                    </div>

                    <div className="w-full">
                        <Button
                            type="submit"
                            className="w-full"
                            disabled={isSubmitting}
                        >
                            {isSubmitting ? 'Signing in...' : 'Sign in'}
                        </Button>
                    </div>
                </form>
            </div>
        </div>
    );
}