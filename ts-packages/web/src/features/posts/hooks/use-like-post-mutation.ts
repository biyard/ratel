import { useMutation } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { Feed } from '@/lib/api/models/feeds';

import { showErrorToast } from '@/lib/toast';
import { likePost } from '@/lib/api/ratel/posts.v3';
import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';

export function useLikePostMutation() {
  const queryClient = getQueryClient();

  return useMutation({
    mutationFn: async ({ feedId, like }: { feedId: string; like: boolean }) => {
      const feed = await likePost(feedId, like);
      return feed;
    },

    onMutate: async ({ feedId, like }) => {
      const detailQueryKey = feedKeys.detail(feedId);
      const listQueryKey = feedKeys.lists();

      const previousFeedDetail = await optimisticUpdate<Feed>(
        { queryKey: detailQueryKey },
        (old) => {
          if (!old || old.is_liked === like) return old;
          const delta = like ? 1 : -1;
          return {
            ...old,
            likes: Math.max(0, old.likes + delta),
            is_liked: like,
          };
        },
      );

      const previousFeedLists = await optimisticListUpdate<Feed>(
        { queryKey: listQueryKey },
        (post) => {
          if (post.id === feedId) {
            const likeCount = post.likes ?? post.likes ?? 0;
            const delta = like ? 1 : -1;
            return {
              ...post,
              is_liked: like,
              likes: Math.max(0, likeCount + delta),
              num_of_likes: Math.max(0, likeCount + delta),
            };
          }
          return post;
        },
      );

      return { previousFeedDetail, previousFeedLists };
    },

    onError: (error: Error, _variables, context) => {
      context?.previousFeedDetail.rollback();
      context?.previousFeedLists.rollback();

      //FIXME: i18n
      showErrorToast(error.message || 'Failed to like feed');
    },

    onSettled: (_data, _error, variables) => {
      const { feedId } = variables;
      const detailQueryKey = feedKeys.detail(feedId);

      queryClient.invalidateQueries({ queryKey: detailQueryKey });
    },
  });
}
