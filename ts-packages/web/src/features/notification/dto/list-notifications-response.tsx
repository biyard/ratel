import { NotificationResponse } from './notification-response';

export interface ListNotificationsResponse {
  items: NotificationResponse[];
  bookmark: string | null;
}
