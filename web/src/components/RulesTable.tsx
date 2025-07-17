import { ApiService, type DomainRule } from '../services/api';
import { Table, TableHeader, TableBody, TableHead, TableRow, TableCell } from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { toast } from 'sonner';

interface RulesTableProps {
    title: string;
    rules: DomainRule[];
    type: 'pending' | 'approved';
    onAction?: () => void;
    isAdmin?: boolean;
}

export default function RulesTable({ title, rules, type, onAction, isAdmin = false }: RulesTableProps) {
    const handleApprove = async (id: number) => {
        try {
            await ApiService.approveRule(id);
            onAction?.();
        } catch (error) {
            console.error('Failed to approve rule:', error);
            toast.error('Failed to approve rule');
        }
    }

    const handleReject = async (id: number) => {
        try {
            await ApiService.rejectRule(id);
            onAction?.();
        } catch (error) {
            console.error('Failed to reject rule:', error);
            toast.error('Failed to reject rule');
        }
    }

    return (
        <section className="mb-8">
            <h2 className="text-2xl font-semibold mb-4">{title}</h2>
            <div className="border rounded-lg overflow-hidden">
                <Table>
                    <TableHeader>
                        <TableRow>
                            {type === 'pending' && 
                                <TableHead>Contributor</TableHead>
                            }

                            <TableHead>Domain</TableHead>
                            <TableHead>Keys</TableHead>
                            <TableHead>Starts With</TableHead>
                            
                            {type === 'pending' && isAdmin && 
                                <TableHead>Actions</TableHead>
                            }
                            {type === 'approved' && 
                                <TableHead>Contributor</TableHead>
                            }
                        </TableRow>
                    </TableHeader>

                    <TableBody>
                        {rules.map((rule) => (
                            <TableRow key={rule.id}>
                                {type === 'pending' && (
                                    <TableCell>
                                        {(rule.contributors && rule.contributors.length > 0
                                            ? rule.contributors.join(', ')
                                            : "Anonymous"
                                        )}
                                    </TableCell>
                                )}
                                <TableCell>{rule.domain}</TableCell>
                                <TableCell>{rule.keys.join(', ')}</TableCell>
                                <TableCell>{rule.starts_with.join(', ')}</TableCell>
                                {type === 'pending' && isAdmin && (
                                    <TableCell>
                                        <div className="flex gap-2">
                                            <Button
                                                onClick={() => handleApprove(rule.id)}
                                                size="sm"
                                                className="bg-green-600 hover:bg-green-700"
                                            >
                                                Approve
                                            </Button>
                                            <Button
                                                onClick={() => handleReject(rule.id)}
                                                size="sm"
                                                variant="destructive"
                                            >
                                                Reject
                                            </Button>
                                        </div>
                                    </TableCell>
                                )}
                                {type === 'approved' && (
                                    <TableCell>{rule.contributors || 'Anonymous'}</TableCell>
                                )}
                            </TableRow>
                        ))}
                    </TableBody>
                    
                </Table>
            </div>
        </section>
    );
}