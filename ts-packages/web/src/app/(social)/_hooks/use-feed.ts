/**
 * @deprecated Use `useFeedById` in '_hooks/feed.ts'.
 */
import { QK_GET_FEED_BY_FEED_ID } from '@/constants';
import { Feed } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useFeedByID(id: number): UseSuspenseQueryResult<Feed> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_FEED_BY_FEED_ID, id],
    queryFn: () => get(ratelApi.feeds.getFeedsByFeedId(id)),
    refetchOnWindowFocus: false,
  });

  return query;
}
