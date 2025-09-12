import { ratelApi } from '@/lib/api/ratel_api';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import { getQueryClient } from '@/providers/getQueryClient';

const DEFAULT_SIZE = 10;

export async function getFeeds(
  user_id: number,
  size: number,
  page: number,
  status?: FeedStatus,
): Promise<Feed[]> {
  const { data } = await apiFetch<Feed[]>(
    `${config.api_url}${ratelApi.feeds.getFeeds(page, size, user_id, status)}`,
    {
      method: 'GET',
    },
  );
  if (!data) {
    throw new Error('Failed to fetch posts');
  }
  return data;
}

export function getOptions(
  userId: number,
  status: FeedStatus,
  size = DEFAULT_SIZE,
) {
  return {
    queryKey: feedKeys.list({ userId, status }),
    queryFn: async ({ pageParam = 1 }) => {
      return getFeeds(userId, size, pageParam as number, status);
    },
    getNextPageParam: (lastPage: Feed[], allPages: Feed[][]) => {
      return lastPage.length === size ? allPages.length + 1 : undefined;
    },
    initialPageParam: 1,
    refetchOnWindowFocus: false,
  };
}

export async function prefetchInfiniteFeeds(
  userId: number,
  status: FeedStatus = FeedStatus.Published,
  size = DEFAULT_SIZE,
) {
  const queryClient = getQueryClient();
  const options = getOptions(userId, status, size);

  await queryClient.prefetchInfiniteQuery(options);
}

/*
 userId 0 means anonymous
*/
export default function useInfiniteFeeds(
  userId: number,
  status: FeedStatus = FeedStatus.Published,
  size = DEFAULT_SIZE,
) {
  return useSuspenseInfiniteQuery({
    ...getOptions(userId, status, size),
  });
}
