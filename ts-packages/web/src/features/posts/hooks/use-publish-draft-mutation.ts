import { useMutation } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticListUpdate, removeQueries } from '@/lib/hook-utils';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { getQueryClient } from '@/providers/getQueryClient';
import PostResponse from '@/features/posts/dto/list-post-response';
import Post from '../types/post';
import { call } from '@/lib/api/ratel/call';

export function publishPost(
  postPk: string,
  title: string,
  content: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    publish: true,
    title,
    content,
  });
}
export function usePublishDraftMutation() {
  const { data: user } = useSuspenseUserInfo();

  const username = user?.username;

  return useMutation({
    mutationFn: async ({
      postPk,
      title,
      content,
    }: {
      postPk: string;
      title: string;
      content: string;
    }) => {
      await publishPost(postPk, title, content);
      return { postPk };
    },

    onMutate: async ({ postPk }) => {
      const queryKey = feedKeys.detail(postPk);
      const listQueryKey = feedKeys.drafts(username!);

      const rollbackDraft = await removeQueries({ queryKey });

      const rollbackDrafts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (draft) => {
          if (draft.pk === postPk) return undefined;

          return draft;
        },
      );

      return { rollbackDraft, rollbackDrafts };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackDraft?.rollback();
      context?.rollbackDrafts?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: () => {
      const queryClient = getQueryClient();
      queryClient.invalidateQueries({
        queryKey: feedKeys.drafts(username!),
      });
      queryClient.invalidateQueries({
        queryKey: feedKeys.my_posts(username!),
      });
    },
  });
}
