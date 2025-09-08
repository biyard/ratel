import { ratelApi } from '@/lib/api/ratel_api';
import {
  InfiniteData,
  useMutation,
  useSuspenseInfiniteQuery,
} from '@tanstack/react-query';
import { QK_GET_POSTS } from '@/constants';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import { getQueryClient } from '@/providers/getQueryClient';
import { removeDraftRequest } from '@/lib/api/models/feeds/update-draft-request';

const DEFAULT_SIZE = 10;
export async function listPost(
  user_id: number,
  size: number,
  page: number,
  status?: FeedStatus,
): Promise<Feed[]> {
  const { data } = await apiFetch<Feed[]>(
    `${config.api_url}${ratelApi.feeds.listPostsByUserId(page, size, user_id, status)}`,
    {
      method: 'GET',
    },
  );
  if (!data) {
    throw new Error('Failed to fetch posts');
  }
  return data;
}
export async function deletePost(post_id: number): Promise<void> {
  const { data } = await apiFetch<void>(
    `${config.api_url}${ratelApi.feeds.removeDraft(post_id)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(removeDraftRequest()),
    },
  );
  if (!data) {
    throw new Error('Failed to delete post');
  }
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
      return lastPage.length === size ? allPages.length + 1 : undefined;
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

export function usePostMutation(userId: number, status?: FeedStatus) {
  const queryClient = getQueryClient();

  const deleteMutation = useMutation({
    mutationFn: async (postId: number) => {
      await deletePost(postId);
    },

    onSuccess: (_, postId) => {
      const key = queryKey(userId, status);

      queryClient.setQueryData(
        key,
        (oldData: InfiniteData<Feed[]> | undefined) => {
          if (!oldData) {
            return oldData;
          }

          const newPages = oldData.pages.map((page: Feed[]) =>
            page.filter((post) => post.id !== postId),
          );

          return {
            ...oldData,
            pages: newPages,
          };
        },
      );
    },
  });

  return { delete: deleteMutation };
}
