import { useMutation, InfiniteData } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { Feed, FeedType } from '@/lib/api/models/feeds';

// FIXME: Move to lib/api/feeds/like-feed.ts
import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { showErrorToast } from '@/lib/toast';

// TODO: Update to use v3 feed API with string IDs
export async function likeFeed(
  feedId: number | string,
  value: boolean,
): Promise<Feed> {
  const req = {
    like: {
      value,
    },
  };
  const { data } = await apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.likePost(feedId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
  if (!data) {
    throw new Error('Failed to like post');
  }
  return data;
}

export function useLikeFeedMutation() {
  const queryClient = getQueryClient();

  return useMutation({
    mutationFn: async ({
      feedId,
      next,
    }: {
      feedType: FeedType;
      feedId: number | string;
      parentId?: number | string;
      next: boolean;
    }) => {
      const feed = await likeFeed(feedId, next);
      return feed;
    },

    onMutate: async ({ feedType, feedId, parentId, next }) => {
      const isReply = feedType === FeedType.Reply;

      if (isReply) {
        // TODO: Update to use v3 feed API with string IDs
        const detailQueryKey = feedKeys.detail(String(parentId!));
        await queryClient.cancelQueries({ queryKey: detailQueryKey });
        const previousFeedDetail =
          queryClient.getQueryData<Feed>(detailQueryKey);

        queryClient.setQueryData<Feed>(detailQueryKey, (old) => {
          if (!old) return undefined;
          const updatedComments = old.comment_list.map((comment) => {
            if (comment.id === feedId) {
              if (comment.is_liked === next) return comment;
              const delta = next ? 1 : -1;
              return {
                ...comment,
                num_of_likes: Math.max(0, comment.num_of_likes + delta),
                is_liked: next,
              };
            }
            return comment;
          });
          return { ...old, comment_list: updatedComments };
        });

        return { previousFeedDetail };
      }

      const detailQueryKey = feedKeys.detail(String(feedId));
      const listQueryKey = feedKeys.lists();

      await queryClient.cancelQueries({ queryKey: detailQueryKey });
      await queryClient.cancelQueries({ queryKey: listQueryKey });

      const previousFeedDetail = queryClient.getQueryData<Feed>(detailQueryKey);
      const previousFeedLists = queryClient.getQueriesData<
        InfiniteData<Feed[]>
      >({ queryKey: listQueryKey });

      queryClient.setQueryData<Feed>(detailQueryKey, (old) => {
        if (!old || old.is_liked === next) return old;
        const delta = next ? 1 : -1;
        return {
          ...old,
          likes: Math.max(0, old.likes + delta),
          is_liked: next,
        };
      });

      // 목록 페이지 업데이트
      queryClient.setQueriesData<InfiniteData<Feed[]>>(
        { queryKey: listQueryKey },
        (oldData) => {
          if (!oldData) return oldData;
          const newPages = oldData.pages.map((page) =>
            page.map((post) => {
              if (post.id === feedId) {
                const likeCount = post.likes ?? post.likes ?? 0;
                const delta = next ? 1 : -1;
                return {
                  ...post,
                  is_liked: next,
                  likes: Math.max(0, likeCount + delta),
                  num_of_likes: Math.max(0, likeCount + delta),
                };
              }
              return post;
            }),
          );
          return { ...oldData, pages: newPages };
        },
      );

      return { previousFeedDetail, previousFeedLists };
    },

    onError: (error: Error, variables, context) => {
      if (context?.previousFeedDetail) {
        const { feedType, feedId, parentId } = variables;
        const detailQueryKey = feedKeys.detail(
          String(feedType === FeedType.Reply ? parentId! : feedId),
        );
        queryClient.setQueryData(detailQueryKey, context.previousFeedDetail);
      }
      if (context?.previousFeedLists) {
        context.previousFeedLists.forEach(([key, data]) => {
          queryClient.setQueryData(key, data);
        });
      }
      //FIXME: i18n
      showErrorToast(error.message || 'Failed to like feed');
    },

    onSettled: (data, error, variables) => {
      const { feedType, feedId, parentId } = variables;
      const detailQueryKey = feedKeys.detail(
        String(feedType === FeedType.Reply ? parentId! : feedId),
      );

      queryClient.invalidateQueries({ queryKey: detailQueryKey });
    },
  });
}
