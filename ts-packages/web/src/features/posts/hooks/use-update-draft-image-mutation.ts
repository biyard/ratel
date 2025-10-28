import { useMutation } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { call } from '@/lib/api/ratel/call';
import Post from '../types/post';
import PostResponse from '../dto/list-post-response';

export function updatePostWithImage(
  postPk: string,
  image: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    images: [image],
  });
}

export function useUpdateDraftImageMutation() {
  const { data: user } = useSuspenseUserInfo();

  const username = user?.username;

  return useMutation({
    mutationFn: async ({
      postPk,
      image,
    }: {
      postPk: string;
      image: string;
    }) => {
      await updatePostWithImage(postPk, image);
      return { postPk };
    },

    onMutate: async ({ postPk, image }) => {
      const queryKey = feedKeys.detail(postPk);
      const listQueryKey = feedKeys.drafts(username!);

      const rollbackDraft = await optimisticUpdate<PostResponse>(
        { queryKey },
        (post) => {
          return {
            ...post!,
            urls: [image],
          };
        },
      );

      const rollbackDrafts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (post) => {
          if (post.pk !== postPk) return post;

          return {
            ...post,
            urls: [image],
          };
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
      // TODO: Run after completed, as invalidation
      // const queryClient = getQueryClient();
      // queryClient.invalidateQueries({ queryKey });
    },
  });
}
