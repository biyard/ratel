import { useMutation } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';

import { showErrorToast } from '@/lib/toast';
import { likePost, PostDetailResponse } from '@/lib/api/ratel/posts.v3';
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

      const previousFeedDetail = await optimisticUpdate<PostDetailResponse>(
        { queryKey: detailQueryKey },
        (old) => {
          if (!old || old.is_liked === like) return old;
          const delta = like ? 1 : -1;
          return {
            ...old,
            likes: Math.max(0, old.post.likes + delta),
            is_liked: like,
          };
        },
      );

      const previousFeedLists = await optimisticListUpdate<PostDetailResponse>(
        { queryKey: listQueryKey },
        (post) => {
          if (post.post.pk === feedId) {
            const likeCount = post.post.likes ?? post.post.likes ?? 0;
            const delta = like ? 1 : -1;
            return {
              ...post,
              is_liked: like,
              likes: Math.max(0, likeCount + delta),
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
