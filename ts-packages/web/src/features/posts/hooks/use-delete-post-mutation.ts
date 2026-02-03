import { useMutation } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { FeedStatus } from '@/features/posts/types/post'; // FeedType 추가
import { showErrorToast } from '@/lib/toast';
import { optimisticListUpdate, removeQueries } from '@/lib/hook-utils';
import PostResponse, { ListPostResponse } from '../dto/list-post-response';
import { call } from '@/lib/api/ratel/call';

export function deletePost(postPk: string): Promise<void> {
  return call('DELETE', `/v3/posts/${encodeURIComponent(postPk)}`);
}

export function useDeletePostMutation(username: string, status: FeedStatus) {
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

      const rollbackPosts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (post) => (post.pk === postPk ? undefined : post),
      );

      let rollbackDrafts;

      if (status === FeedStatus.Draft) {
        const queryKey = feedKeys.drafts(username);

        rollbackDrafts = await optimisticListUpdate<ListPostResponse>(
          { queryKey },
          (post) => {
            const items = post.items.filter((p) => p.pk !== postPk);
            post.items = items;

            return post;
          },
        );
      }

      return { rollbackPostDetail, rollbackPosts, rollbackDrafts };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackPostDetail?.rollback();
      context?.rollbackPosts?.rollback();
      context?.rollbackDrafts?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: () => {
      // queryClient.invalidateQueries({ queryKey: feedKeys.lists() });
    },
  });
}
