import { useMutation } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { Feed, FeedStatus } from '@/lib/api/models/feeds'; // FeedType 추가
import { showErrorToast } from '@/lib/toast';
import { deletePost } from '@/lib/api/ratel/posts.v3';
import { optimisticListUpdate, removeQueries } from '@/lib/hook-utils';

export function useDeletePostMutation(username: string, status: FeedStatus) {
  const queryClient = getQueryClient();

  return useMutation({
    mutationFn: async (postPk: string) => {
      await deletePost(postPk);
      return { postPk };
    },

    onMutate: async (postPk) => {
      const detailQueryKey = feedKeys.detail(postPk);
      const listQueryKey = feedKeys.list({
        username,
        status,
      });

      const rollbackPostDetail = await removeQueries({
        queryKey: detailQueryKey,
      });

      const rollbackPosts = await optimisticListUpdate<Feed>(
        { queryKey: listQueryKey },
        (post) => (post.id !== postPk ? undefined : post),
      );

      return { rollbackPostDetail, rollbackPosts };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackPostDetail?.rollback();
      context?.rollbackPosts?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: feedKeys.lists() });
    },
  });
}
