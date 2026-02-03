import { useMutation } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';

import { showErrorToast } from '@/lib/toast';

import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import { ListResponse } from '@/lib/api/ratel/common';
import { PostDetailResponse } from '../dto/post-detail-response';
import PostResponse from '../dto/list-post-response';
import { call } from '@/lib/api/ratel/call';

export type LikePostResponse = {
  like: boolean;
};

export async function likePost(
  postPk: string,
  like: boolean,
): Promise<LikePostResponse> {
  return call('POST', `/v3/posts/${encodeURIComponent(postPk)}/likes`, {
    like,
  });
}

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
      const delta = like ? 1 : -1;

      const previousFeedDetail = await optimisticUpdate<PostDetailResponse>(
        { queryKey: detailQueryKey },
        (old) => {
          // If the detail query doesn't exist or the state is already what we want, don't update
          if (!old || !old.post || old.is_liked === like) return old;
          return {
            ...old,
            post: {
              ...old.post,
              likes: Math.max(0, old.post.likes + delta),
            },
            is_liked: like,
          };
        },
      );

      const previousFeedLists = await optimisticListUpdate<
        ListResponse<PostResponse>
      >({ queryKey: listQueryKey }, (page) => {
        const items = page.items.map((post) => {
          if (post.pk === feedId) {
            return {
              ...post,
              likes: Math.max(0, post.likes + delta),
              liked: like,
            };
          }
          return post;
        });
        return { ...page, items };
      });

      return { previousFeedDetail, previousFeedLists };
    },

    onError: (error: Error, _variables, context) => {
      context?.previousFeedDetail.rollback();
      context?.previousFeedLists.rollback();

      //FIXME: i18n
      showErrorToast(error.message || 'Failed to like feed');
    },

    onSuccess: (_data, variables) => {
      // On success, we trust the optimistic update and don't need to refetch
      // The server has confirmed the like/unlike, so the cache is already correct
    },

    onSettled: (_data, error, variables) => {
      // Only invalidate queries if there was an error, to force a refetch
      if (error) {
        const { feedId } = variables;
        const detailQueryKey = feedKeys.detail(feedId);
        const listQueryKey = feedKeys.lists();

        queryClient.invalidateQueries({ queryKey: detailQueryKey });
        queryClient.invalidateQueries({ queryKey: listQueryKey });
      }
    },
  });
}
