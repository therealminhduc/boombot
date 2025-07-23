import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { AdminService, type CreateAdminRequest } from '../services';
import { toast } from 'sonner';

export default function CreateAdminForm() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        
        if (!username.trim() || !password.trim()) {
            toast.error('Username and password are required');
            return;
        }

        if (password !== confirmPassword) {
            toast.error('Passwords do not match');
            return;
        }

        if (password.length < 6) {
            toast.error('Password must be at least 6 characters long');
            return;
        }

        setIsLoading(true);

        try {
            const request: CreateAdminRequest = { username, password };
            const result = await AdminService.createAdmin(request);
            toast.success(result);
            
            // Reset form after successful creation
            setUsername('');
            setPassword('');
            setConfirmPassword('');
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'An error occurred';
            toast.error(errorMessage);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
            <h3 className="text-lg font-semibold mb-4">Create New Admin</h3>
            
            <form onSubmit={handleSubmit} className="space-y-4">
                <div className="grid w-full items-center gap-2">
                    <Label htmlFor="new-username">Username</Label>
                    <Input
                        id="new-username"
                        name="username"
                        type="text"
                        required
                        value={username}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUsername(e.target.value)}
                        placeholder="Enter username"
                        disabled={isLoading}
                    />
                </div>

                <div className="grid w-full items-center gap-2">
                    <Label htmlFor="new-password">Password</Label>
                    <Input
                        id="new-password"
                        name="password"
                        type="password"
                        required
                        value={password}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
                        placeholder="Enter password"
                        disabled={isLoading}
                    />
                </div>

                <div className="grid w-full items-center gap-2">
                    <Label htmlFor="new-confirm-password">Confirm Password</Label>
                    <Input
                        id="new-confirm-password"
                        name="confirmPassword"
                        type="password"
                        required
                        value={confirmPassword}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)}
                        placeholder="Confirm password"
                        disabled={isLoading}
                    />
                </div>

                <Button
                    type="submit"
                    className="w-full"
                    disabled={isLoading}
                >
                    {isLoading ? 'Creating...' : 'Create Admin'}
                </Button>
            </form>

            <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-md">
                <div className="flex">
                    <div className="flex-shrink-0">
                        <svg className="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
                        </svg>
                    </div>
                    <div className="ml-3 flex-1">
                        <p className="text-sm text-blue-700 dark:text-blue-300 leading-relaxed">
                            <strong>Admin Account:</strong> Create new admin accounts to manage URL cleaning rules. 
                            Choose a strong password as admin accounts have full access to approve and reject submissions.
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
} 