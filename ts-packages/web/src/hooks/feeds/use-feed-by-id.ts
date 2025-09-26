import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { Feed } from '@/lib/api/models/feeds';
import { feedKeys } from '@/constants';
import { getQueryClient } from '@/providers/getQueryClient';

export async function getFeedById(
  postId: number,
): Promise<FetchResponse<Feed | null>> {
  return apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.getFeed(postId)}`,
  );
}

export function getOption(postId: number) {
  return {
    queryKey: feedKeys.detail(postId),
    queryFn: async () => {
      const { data } = await getFeedById(postId);

      if (!data) {
        throw new Error('Feed not found');
      }
      return data;
    },
    refetchOnWindowFocus: false,
  };
}

export async function prefetchFeedById(postId: number) {
  const queryClient = getQueryClient();
  const options = getOption(postId);

  await queryClient.prefetchQuery(options);
}

export default function useFeedById(
  postId: number,
): UseSuspenseQueryResult<Feed> {
  const query = useSuspenseQuery(getOption(postId));
  return query;
}
