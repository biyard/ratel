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
import { getPost, PostDetailResponse } from '@/lib/api/ratel/posts.v3';

export async function getFeedById(
  postId: number,
): Promise<FetchResponse<Feed | null>> {
  return apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.getFeed(postId)}`,
  );
}

export function getOption(postId: string) {
  return {
    queryKey: feedKeys.detail(postId),
    queryFn: async () => {
      const post = await getPost(postId);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export async function prefetchFeedById(postId: string) {
  const queryClient = getQueryClient();
  const options = getOption(postId);

  await queryClient.prefetchQuery(options);
}

export default function useFeedById(
  postId: string,
): UseSuspenseQueryResult<PostDetailResponse> {
  const query = useSuspenseQuery(getOption(postId));
  return query;
}
