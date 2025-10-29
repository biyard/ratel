import { useMutation } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticListUpdate, removeQueries } from '@/lib/hook-utils';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import PostResponse from '@/features/posts/dto/list-post-response';
import Post, { Visibility } from '../types/post';
import { call } from '@/lib/api/ratel/call';
import { getQueryClient } from '@/providers/getQueryClient';
import { PostDetailResponse } from '../dto/post-detail-response';

export function publishPost(
  postPk: string,
  title: string,
  content: string,
  imageUrls: string[] = [],
  visibility?: Visibility,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    publish: true,
    title,
    content,
    image_urls: imageUrls,
    visibility,
  });
}
export function usePublishDraftMutation() {
  const { data: user } = useSuspenseUserInfo();

  const username = user?.username;

  return useMutation({
    mutationKey: ['publish-draft'],
    mutationFn: async ({
      postPk,
      title,
      content,
      imageUrls,
      visibility,
    }: {
      postPk: string;
      title: string;
      content: string;
      imageUrls?: string[];
      visibility?: Visibility;
    }) => {
      const updatedPost = await publishPost(
        postPk,
        title,
        content,
        imageUrls,
        visibility,
      );
      return { postPk, updatedPost };
    },

    onMutate: async ({ postPk }) => {
      const listQueryKey = feedKeys.drafts(username!);

      const rollbackDrafts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (draft) => {
          if (draft.pk === postPk) return undefined;

          return draft;
        },
      );

      return { rollbackDrafts };
    },
    onSuccess: ({ postPk, updatedPost }) => {
      const queryClient = getQueryClient();
      console.log('Published post:', updatedPost);
      queryClient.setQueryData(
        feedKeys.detail(postPk),
        (oldData: PostDetailResponse) => {
          console.log('Old data:', oldData);
          if (!oldData) {
            return { post: updatedPost };
          }

          return {
            ...oldData,
            post: {
              ...oldData.post,
              ...updatedPost,
            },
          };
        },
      );

      queryClient.invalidateQueries({
        queryKey: feedKeys.my_posts(username!),
      });

      // Invalidate homepage feed to show newly published post
      queryClient.invalidateQueries({
        queryKey: feedKeys.lists(),
      });
    },
    onError: (error: Error, _variables, context) => {
      context?.rollbackDrafts?.rollback();

      showErrorToast(error.message || 'Failed to publish feed');
    },

    // onSettled: () => {
    //   const queryClient = getQueryClient();
    //   queryClient.invalidateQueries({
    //     queryKey: feedKeys.drafts(username!),
    //   });
    //   queryClient.invalidateQueries({
    //     queryKey: feedKeys.my_posts(username!),
    //   });
    // },
  });
}
