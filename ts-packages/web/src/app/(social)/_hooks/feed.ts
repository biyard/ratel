import { QK_GET_FEED_BY_FEED_ID } from '@/constants';
import { ratelApi } from '@/lib/api/ratel_api';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { QueryClient } from '@tanstack/react-query';
import { Feed } from '@/lib/api/models/feeds';
import { config } from '@/config';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function getKey(id: number): [string, number] {
  return [QK_GET_FEED_BY_FEED_ID, id];
}

export function useFeedByID(id: number): UseSuspenseQueryResult<Feed | null> {
  const query = useSuspenseQuery({
    queryKey: getKey(id),
    queryFn: async () => {
      const { data } = await requestFeedByID(id);
      return data;
    },
    refetchOnWindowFocus: false,
  });

  return query;
}

export async function prefetchFeedByID(queryClient: QueryClient, id: number) {
  try {
    await queryClient.prefetchQuery({
      queryKey: getKey(id),
      queryFn: async () => {
        const { data } = await requestFeedByID(id);
        return data;
      },
    });
  } catch (error) {
    console.warn(`Failed to prefetch feed ${id}:`, error);
  }
}

export function setInitialFeedByID(
  queryClient: QueryClient,
  id: number,
  data: Feed | null,
) {
  if (data) {
    queryClient.setQueryData(getKey(id), data);
  }
}

export function requestFeedByID(
  id: number,
): Promise<FetchResponse<Feed | null>> {
  return apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.getFeedsByFeedId(id)}`,
    {
      ignoreError: true,
    },
  );
}
