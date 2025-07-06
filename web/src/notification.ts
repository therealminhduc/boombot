export class NotificationService {
    private static messageDiv: HTMLElement;

    static initialize(): void {
        this.messageDiv = document.getElementById("message")!;
    }

    static show(text: string, type: 'success' | 'error'): void {
        this.messageDiv.textContent = text;
        this.messageDiv.className = type;
        this.messageDiv.style.display = 'block';

        setTimeout(() => {
            this.messageDiv.style.display = 'none';
        }, 3000);
    }
}