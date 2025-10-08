'use client';
import { Check } from 'lucide-react';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useState, useEffect } from 'react';
import { ratelApi } from '@/lib/api/ratel_api';

export interface NotificationReadStatusProps {
  notificationId: number;
  isRead: boolean;
  onStatusChange?: (isRead: boolean) => void;
}

export default function NotificationReadStatus({
  notificationId,
  isRead,
  onStatusChange,
}: NotificationReadStatusProps) {
  const { post: apiPost } = useApiCall();
  const [isUpdating, setIsUpdating] = useState(false);
  const [localIsRead, setLocalIsRead] = useState(isRead);

  // Sync local state with prop changes (e.g., when "Mark All Read" is used)
  useEffect(() => {
    setLocalIsRead(isRead);
  }, [isRead]);

  const handleMarkAsRead = async () => {
    if (localIsRead || isUpdating) return;

    setIsUpdating(true);
    try {
      // Optimistically update the local state
      setLocalIsRead(true);

      await apiPost(ratelApi.notifications.markAsRead(notificationId), {
        update_status_to_read: {},
      });
      showSuccessToast('Notification marked as read');
      onStatusChange?.(true);
    } catch (error) {
      // Revert the optimistic update on error
      setLocalIsRead(false);
      console.error('Failed to mark notification as read:', error);
      showErrorToast('Failed to mark notification as read. Please try again.');
    } finally {
      setIsUpdating(false);
    }
  };

  // Don't render anything if the notification is read
  if (localIsRead) {
    return null;
  }

  return (
    <button
      onClick={(e) => {
        e.stopPropagation();
        handleMarkAsRead();
      }}
      disabled={isUpdating}
      className={`p-1.5 max-mobile:p-1 rounded-md border transition-all duration-200 bg-transparent border-neutral-600 hover:bg-neutral-700 hover:border-neutral-500 cursor-pointer ${
        isUpdating ? 'opacity-50 cursor-not-allowed' : ''
      }`}
      title="Mark as read"
    >
      <Check className="w-3 h-3 max-mobile:w-2.5 max-mobile:h-2.5 text-neutral-400 hover:text-neutral-300 transition-colors" />
    </button>
  );
}
