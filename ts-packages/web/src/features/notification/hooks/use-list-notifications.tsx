import { useInfiniteQuery } from '@tanstack/react-query';
import { QK_GET_NOTIFICATIONS, NOTIFICATION_POLL_INTERVAL } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { ListNotificationsResponse } from '../dto/list-notifications-response';

export async function listNotifications(
  bookmark?: string,
): Promise<ListNotificationsResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }

  const queryString = params.toString();
  const path = `/v3/notifications${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function getOptions() {
  return {
    queryKey: [QK_GET_NOTIFICATIONS],
    queryFn: async ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListNotificationsResponse> => {
      try {
        return await listNotifications(pageParam);
      } catch (e) {
        logger.error('Failed to fetch notifications', e);
        throw new Error(String(e));
      }
    },
    getNextPageParam: (last: ListNotificationsResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchInterval: NOTIFICATION_POLL_INTERVAL,
    refetchOnWindowFocus: true, // Refetch when user returns to the tab
  };
}

export default function useInfiniteNotifications() {
  return useInfiniteQuery(getOptions());
}
