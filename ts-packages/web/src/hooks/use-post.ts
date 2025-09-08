import { ratelApi } from '@/lib/api/ratel_api';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { QK_GET_POSTS } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import { getQueryClient } from '@/providers/getQueryClient';

const DEFAULT_SIZE = 10;
export async function listPost(
  user_id: number,
  size: number,
  page: number,
  status?: FeedStatus,
): Promise<FetchResponse<Feed[] | null>> {
  return apiFetch<Feed[] | null>(
    `${config.api_url}${ratelApi.feeds.listPostsByUserId(user_id, status, page, size)}`,
    {
      method: 'GET',
    },
  );
}

export const queryKey = (userId: number, status?: FeedStatus) => {
  if (status) {
    return [QK_GET_POSTS, userId, status];
  }
  return [QK_GET_POSTS, userId];
};
/*
 when userId 0 means anonymous
*/
export const usePostInfiniteQuery = (
  userId: number,
  status: FeedStatus = FeedStatus.Published,
  size = DEFAULT_SIZE,
) => {
  return useSuspenseInfiniteQuery({
    queryKey: queryKey(userId, status),
    queryFn: async ({ pageParam = 1 }) => {
      return listPost(userId, size, pageParam as number);
    },
    getNextPageParam: (lastPage, allPages) => {
      return lastPage.data?.length === size ? allPages.length + 1 : undefined;
    },
    initialPageParam: 1,
    refetchOnWindowFocus: false,
  });
};

export const invalidateQuery = (userId: number, status?: FeedStatus) => {
  const queryClient = getQueryClient();

  queryClient.invalidateQueries({
    queryKey: queryKey(userId, status),
    exact: true,
  });
};
