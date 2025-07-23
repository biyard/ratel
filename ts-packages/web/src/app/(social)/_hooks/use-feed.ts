/**
 * @deprecated Use `useFeedById` in '_hooks/feed.ts'.
 */
import { QK_GET_FEED_BY_FEED_ID } from '@/constants';
import { Feed } from '@/lib/api/models/feeds';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export function useFeedByID(id?: number): UseQueryResult<Feed> {
  const { get } = useApiCall();

  const query = useQuery({
    queryKey: [QK_GET_FEED_BY_FEED_ID, id],
    queryFn: () => {
      if (id === undefined) throw new Error('Feed ID is undefined');
      return get(ratelApi.feeds.getFeedsByFeedId(id));
    },
    enabled: !!id,
    refetchOnWindowFocus: false,
  });

  return query;
}
