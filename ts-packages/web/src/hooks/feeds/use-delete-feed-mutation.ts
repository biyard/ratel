import { useMutation, InfiniteData } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { Feed, FeedStatus, FeedType } from '@/lib/api/models/feeds'; // FeedType 추가
import { showErrorToast } from '@/lib/toast';
import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { removePostRequest } from '@/lib/api/models/feeds/remove-post';
import { deletePost } from '@/lib/api/ratel/posts.v3';

export async function deleteFeed(feedId: number): Promise<void> {
  const { data } = await apiFetch<void>(
    `${config.api_url}${ratelApi.feeds.removeDraft(feedId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(removePostRequest()),
    },
  );
  if (!data) {
    throw new Error('Failed to delete post');
  }
}

export function useDeleteFeedMutation(status: FeedStatus, targetId: number) {
  const queryClient = getQueryClient();

  return useMutation({
    mutationFn: async ({
      feedId,
    }: {
      feedId: string;
      feedType: FeedType;
      parentId?: number;
    }) => {
      await deletePost(feedId);
      return { feedId };
    },

    onMutate: async ({ feedId, feedType, parentId }) => {
      const isReply = feedType === FeedType.Reply;

      if (isReply && parentId) {
        const detailQueryKey = feedKeys.detail(parentId);
        await queryClient.cancelQueries({ queryKey: detailQueryKey });
        const previousFeedDetail =
          queryClient.getQueryData<Feed>(detailQueryKey);

        queryClient.setQueryData<Feed>(detailQueryKey, (old) => {
          if (!old) return undefined;
          const updatedComments = old.comment_list.filter(
            (comment) => comment.id !== feedId,
          );
          return { ...old, comment_list: updatedComments };
        });

        return { previousFeedDetail };
      }

      const detailQueryKey = feedKeys.detail(feedId);
      const listQueryKey = feedKeys.list({
        userId: targetId,
        status,
      });

      await queryClient.cancelQueries({ queryKey: detailQueryKey });
      await queryClient.cancelQueries({ queryKey: listQueryKey });

      const previousFeedDetail = queryClient.getQueryData<Feed>(detailQueryKey);
      const previousFeedLists = queryClient.getQueriesData<
        InfiniteData<Feed[]>
      >({ queryKey: listQueryKey });

      queryClient.removeQueries({ queryKey: detailQueryKey });

      queryClient.setQueriesData<InfiniteData<Feed[]>>(
        { queryKey: listQueryKey },
        (oldData) => {
          if (!oldData) return oldData;
          const newPages = oldData.pages.map((page) =>
            page.filter((post) => post.id !== feedId),
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
          feedType === FeedType.Reply ? parentId! : feedId,
        );
        queryClient.setQueryData(detailQueryKey, context.previousFeedDetail);
      }
      if (context?.previousFeedLists) {
        context.previousFeedLists.forEach(([key, data]) => {
          queryClient.setQueryData(key, data);
        });
      }
      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: (data, error, variables) => {
      const { feedType, parentId } = variables;
      if (feedType === FeedType.Reply) {
        queryClient.invalidateQueries({ queryKey: feedKeys.detail(parentId!) });
      } else {
        queryClient.invalidateQueries({ queryKey: feedKeys.lists() });
      }
    },
  });
}
