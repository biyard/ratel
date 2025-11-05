import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';
import { optimisticUpdate } from '@/lib/hook-utils';

export function commentReplySpacePost(
  spacePk: string,
  postPk: string,
  commentSk: string,
  content: string,
): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}`,
    {
      content,
    },
  );
}

export function useCommentReplySpacePostMutation<
  T extends SpacePostResponse,
>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['comment-reply-space-post'],
    mutationFn: async ({
      spacePk,
      postPk,
      commentSk,
      content,
    }: {
      spacePk: string;
      postPk: string;
      commentSk: string;
      content: string;
    }) => {
      await commentReplySpacePost(spacePk, postPk, commentSk, content);
    },
    onSuccess: async (_, { spacePk, postPk }) => {
      const spaceQK = spaceKeys.boards_posts(spacePk);
      const spacePostQk = spaceKeys.boards_post(spacePk, postPk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });
      qc.invalidateQueries({
        queryKey: spacePostQk,
      });
    },
  });

  return mutation;
}
