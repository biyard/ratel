import useInfiniteNotifications from '@/features/notification/hooks/use-list-notifications';
import { NotificationResponse } from '@/features/notification/dto/notification-response';

export interface NotificationsData {
  notifications: NotificationResponse[] | undefined;
  isLoading: boolean;
  error: Error | null;
  hasNextPage: boolean;
  fetchNextPage: () => void;
  isFetchingNextPage: boolean;
}

export function useNotificationsData(): NotificationsData {
  const {
    data,
    isLoading,
    error,
    hasNextPage,
    fetchNextPage,
    isFetchingNextPage,
  } = useInfiniteNotifications();

  const notifications = data?.pages.flatMap((page) =>
    page.items.map((item) => new NotificationResponse(item)),
  );

  return {
    notifications,
    isLoading,
    error,
    hasNextPage: hasNextPage ?? false,
    fetchNextPage,
    isFetchingNextPage,
  };
}
