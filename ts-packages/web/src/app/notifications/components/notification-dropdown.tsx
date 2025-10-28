'use client';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Extra } from '@/components/icons';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { ratelApi } from '@/lib/api/ratel_api';

export interface NotificationDropdownProps {
  notificationId: number;
  onDismiss?: () => void;
}

export default function NotificationDropdown({
  notificationId,
  onDismiss,
}: NotificationDropdownProps) {
  const { post: apiPost } = useApiCall();

  const handleDismissNotification = async () => {
    try {
      await apiPost(ratelApi.notifications.dismiss(notificationId), {
        dismiss: {},
      });
      showSuccessToast('Notification dismissed');
      onDismiss?.();
    } catch (error) {
      console.error('Failed to dismiss notification:', error);
      showErrorToast('Failed to dismiss notification. Please try again.');
    }
  };

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger
        onClick={(e) => e.stopPropagation()}
        className="p-1 max-mobile:p-0.5 hover:bg-gray-700 rounded-full focus:outline-none transition-colors"
        aria-haspopup="true"
        aria-label="Notification options"
      >
        <Extra className="w-4 h-4 max-mobile:w-3 max-mobile:h-3 text-gray-400" />
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        className="w-32 max-mobile:w-28 bg-[#404040] border-gray-700 transition ease-out duration-100"
      >
        <DropdownMenuItem asChild>
          <button
            onClick={(e) => {
              e.stopPropagation();
              handleDismissNotification();
            }}
            className="flex items-center w-full px-4 max-mobile:px-3 py-2 max-mobile:py-1.5 text-sm max-mobile:text-xs text-white hover:bg-gray-700 cursor-pointer"
          >
            Dismiss
          </button>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
