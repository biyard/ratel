import { EmailOperation } from '../types/email-operation';
import { NotificationStatus } from '../types/notification-status';

export class NotificationResponse {
  pk: string;
  sk: string;
  created_at: number;
  readed_at: number | null;
  status: NotificationStatus;
  operation: EmailOperation;

  constructor(json: unknown) {
    const data = json as Record<string, unknown>;
    this.pk = data.pk as string;
    this.sk = data.sk as string;
    this.created_at = data.created_at as number;
    this.readed_at = (data.readed_at as number | null) ?? null;
    this.status = data.status as NotificationStatus;
    this.operation = data.operation as EmailOperation;
  }

  isRead(): boolean {
    return this.status === NotificationStatus.Read;
  }

  isUnread(): boolean {
    return this.status === NotificationStatus.Unread;
  }

  getNotificationId(): string {
    // Extract ID from sk which is in format "NOTIFICATION#uuid"
    const parts = this.sk.split('#');
    if (parts.length >= 2) {
      return parts.slice(1).join('#'); // Handle cases with multiple # in the ID
    }
    return this.sk;
  }
}
