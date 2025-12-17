import { useState } from 'react';
import { useNotificationsData } from './use-notifications-data';
import { useMarkAsReadMutation } from '@/features/notification/hooks/use-mark-as-read-mutation';
import { useMarkAllAsReadMutation } from '@/features/notification/hooks/use-mark-all-as-read-mutation';
import { useDeleteNotificationMutation } from '@/features/notification/hooks/use-delete-notification-mutation';
import { showSuccessToast, showErrorToast } from '@/lib/toast';

export function useNotificationsPageController() {
  const data = useNotificationsData();

  const markAsReadMutation = useMarkAsReadMutation();
  const markAllAsReadMutation = useMarkAllAsReadMutation();
  const deleteNotificationMutation = useDeleteNotificationMutation();

  const [isMarkingAsRead, setIsMarkingAsRead] = useState(false);
  const [isMarkingAllAsRead, setIsMarkingAllAsRead] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleMarkAsRead = async (notificationId: string) => {
    setIsMarkingAsRead(true);
    try {
      await markAsReadMutation.mutateAsync({
        notification_ids: [notificationId],
      });
      showSuccessToast('Notification marked as read');
    } catch {
      showErrorToast('Failed to mark notification as read');
    } finally {
      setIsMarkingAsRead(false);
    }
  };

  const handleMarkAllAsRead = async () => {
    setIsMarkingAllAsRead(true);
    try {
      await markAllAsReadMutation.mutateAsync();
      showSuccessToast('All notifications marked as read');
    } catch {
      showErrorToast('Failed to mark all notifications as read');
    } finally {
      setIsMarkingAllAsRead(false);
    }
  };

  const handleDelete = async (notificationId: string) => {
    setIsDeleting(true);
    try {
      await deleteNotificationMutation.mutateAsync(notificationId);
      showSuccessToast('Notification deleted');
    } catch {
      showErrorToast('Failed to delete notification');
    } finally {
      setIsDeleting(false);
    }
  };

  return {
    ...data,
    handleMarkAsRead,
    handleMarkAllAsRead,
    handleDelete,
    isMarkingAsRead,
    isMarkingAllAsRead,
    isDeleting,
  };
}
