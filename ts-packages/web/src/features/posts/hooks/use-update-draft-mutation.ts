import { useMutation, useQueryClient } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';

import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import Post from '../types/post';
import { call } from '@/lib/api/ratel/call';
import PostResponse from '../dto/list-post-response';
import { PostDetailResponse } from '../dto/post-detail-response';

export function updatePostWithTitleAndContents(
  postPk: string,
  title: string,
  content: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    title,
    content,
  });
}

export function useUpdateDraftMutation() {
  const { data: user } = useSuspenseUserInfo();
  const queryClient = useQueryClient();

  const username = user?.username;

  return useMutation({
    mutationKey: ['update-draft'],
    mutationFn: async ({
      postPk,
      title,
      content,
    }: {
      postPk: string;
      title: string;
      content: string;
    }) => {
      await updatePostWithTitleAndContents(postPk, title, content);
      return { postPk };
    },

    onMutate: async ({ postPk, title, content }) => {
      const queryKey = feedKeys.detail(postPk);
      const listQueryKey = feedKeys.drafts(username!);
      const draft: Partial<Post> = {
        title,
        html_contents: content,
      };
      const rollbackDraft = await optimisticUpdate<PostDetailResponse>(
        { queryKey },
        (prev) => {
          return {
            ...prev!,
            post: {
              ...prev!.post,
              ...draft,
            },
          };
        },
      );

      const rollbackDrafts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (post) => {
          if (post.pk !== postPk) return post;

          return {
            ...post,
            ...draft,
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
      // Invalidate all feed list queries to refresh drafts/posts lists
      queryClient.invalidateQueries({
        queryKey: feedKeys.lists(),
      });
    },
  });
}
