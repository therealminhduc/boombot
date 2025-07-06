import './style.css';
import { NotificationService } from './notification';
import { FormHandler } from './form';
import { RulesState } from './rules';

NotificationService.initialize();
const rulesState = new RulesState();

new FormHandler('addRuleForm', async () => {
    await rulesState.loadPendingRules();
});

(window as any).approveRule = async (id: number) => {
    try {
        await rulesState.approveRule(id);
        NotificationService.show('Rule approved successfully!', 'success');
    } catch (error) {
        NotificationService.show('Failed to approve rule', 'error');
    }
};

(window as any).rejectRule = async (id: number) => {
    try {
        await rulesState.rejectRule(id);
        NotificationService.show('Rule rejected successfully!', 'success');
    } catch (error) {
        NotificationService.show('Failed to reject rule', 'error');
    }
};

async function init() {
    await rulesState.loadPendingRules();
    await rulesState.loadApprovedRules();
}

init();