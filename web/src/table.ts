import type { DomainRule } from "./api";

export class TableRenderer {
    
    //  Render pending rules table
    static renderPendingTable(rules: DomainRule[], container: HTMLElement): void {
        container.innerHTML = rules.map(rule => `
            <tr>
                <td>${rule.contributor || 'Anonymous'}</td>
                <td>${rule.domain}</td>
                <td>${rule.keys.join(', ')}</td>
                <td>${rule.starts_with.join(', ')}</td>
                <td>
                    <button onclick="approveRule(${rule.id})" class="btn-approve">Approve</button>
                    <button onclick="rejectRule(${rule.id})" class="btn-reject">Reject</button>
                </td>
            </tr>
        `).join("");
    }

    // Render approved rules table
    static renderApprovedTable(rules: DomainRule[], container: HTMLElement): void {
        container.innerHTML = rules.map(rule => `
            <tr>
                <td>${rule.domain}</td>
                <td>${rule.keys.join(', ')}</td>
                <td>${rule.starts_with.join(', ')}</td>
                <td>${rule.contributor || 'Anonymous'}</td>
            </tr>
        `).join("");
    }
}