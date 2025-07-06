import { ApiService } from './api';
import type { SubmissionRequest } from './api';
import { NotificationService } from './notification';

export class FormHandler {
    private form: HTMLFormElement;
    private onSuccess: () => void;

    constructor(formId: string, onSuccess: () => void) {
        this.form = document.getElementById(formId) as HTMLFormElement;
        this.onSuccess = onSuccess;
        this.initialize();
    }

    private initialize(): void {
        this.form.onsubmit = this.handleSubmit.bind(this);
    }

    private async handleSubmit(e: Event): Promise<void> {
        e.preventDefault();

        const contributor = (document.getElementById('contributor') as HTMLInputElement).value;
        const domain = (document.getElementById('domain') as HTMLInputElement).value;
        const keys = (document.getElementById('keys') as HTMLInputElement).value;
        const startsWith = (document.getElementById('startsWith') as HTMLInputElement).value;

        if (!contributor || !domain || !keys) {
            NotificationService.show('Please fill in all required fields', 'error');
            return;
        }

        const request: SubmissionRequest = {
            domain,
            keys: keys.split(',').map(k => k.trim()),
            starts_with: startsWith ? startsWith.split(',').map(s => s.trim()) : undefined,
            contributor,
        };

        try {
            await ApiService.submitRule(request);
            NotificationService.show('Rule submitted successfully', 'success');
            this.form.reset();
            this.onSuccess();
        } catch (error) {
            NotificationService.show(`Error submitting rule: ${error}`, 'error');
        }
    }
}