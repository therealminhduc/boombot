import { ApiService } from './api';
import type { DomainRule } from './api';
import { TableRenderer } from './table';

export class RulesState {
    private pendingRules: DomainRule[] = [];
    private approvedRules: DomainRule[] = [];
    private pendingContainer: HTMLElement;
    private approvedContainer: HTMLElement;

    constructor() {
        this.pendingContainer = document.getElementById('pendingBody')!;
        this.approvedContainer = document.getElementById('approvedBody')!;
    }

    async loadPendingRules(): Promise<void> {
        try {
            this.pendingRules = await ApiService.getPendingRules();
            TableRenderer.renderPendingTable(this.pendingRules, this.pendingContainer);
        } catch (error) {
            console.error('Failed to load pending rules:', error);
        }
    }

    async loadApprovedRules(): Promise<void> {
        try {
            this.approvedRules = await ApiService.getApprovedRules();
            TableRenderer.renderApprovedTable(this.approvedRules, this.approvedContainer);
        } catch (error) {
            console.error('Failed to load approved rules:', error);
        }
    }

    async approveRule(id: number): Promise<void> {
        try {
            await ApiService.approveRule(id);
            await this.loadPendingRules();
            await this.loadApprovedRules();
        } catch (error) {
            console.error('Failed to approve rule:', error);
        }
    }

    async rejectRule(id: number): Promise<void> {
        try {
            await ApiService.rejectRule(id);
            await this.loadPendingRules();
        } catch (error) {
            console.error('Failed to reject rule:', error);
        }
    }

    getPendingRules(): DomainRule[] {
        return this.pendingRules;
    }

    getApprovedRules(): DomainRule[] {
        return this.approvedRules;
    }
}