import { ratelApi } from '@/lib/api/ratel_api';
import { QueryResponse } from '@/lib/api/models/common';
import { useApiCall } from '@/lib/api/use-send';
import { useInfiniteQuery } from '@tanstack/react-query';
import { QK_GET_NOTIFICATIONS } from '@/constants';
import { Notification, NotificationsFilter } from '@/app/notifications/types';

export const NOTIFICATIONS_SIZE = 10;

export const useNotificationsInfinite = (
  filterType: NotificationsFilter = 'all',
) => {
  const { get } = useApiCall();

  const toApiFilterType = (t: NotificationsFilter): string | undefined => {
    if (t === 'all') return undefined;
    return String(t);
  };

  return useInfiniteQuery<QueryResponse<Notification>, Error>({
    queryKey: [QK_GET_NOTIFICATIONS, filterType],
    queryFn: async ({ pageParam = 1 }) => {
      const filterParam = toApiFilterType(filterType);
      return get(
        ratelApi.notifications.getNotifications(
          pageParam as number,
          NOTIFICATIONS_SIZE,
          filterParam,
        ),
      );
    },
    getNextPageParam: (lastPage, allPages) => {
      return lastPage.items.length === NOTIFICATIONS_SIZE
        ? allPages.length + 1
        : undefined;
    },
    initialPageParam: 1,
    refetchInterval: 7000, // Refetch every 7 seconds for notifications
    refetchOnWindowFocus: true,
  });
};
