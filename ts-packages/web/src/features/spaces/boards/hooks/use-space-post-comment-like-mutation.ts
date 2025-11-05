import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';
import { optimisticUpdate } from '@/lib/hook-utils';

export function commentLikeSpacePost(
  spacePk: string,
  postPk: string,
  commentSk: string,
  like: boolean,
): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}/likes`,
    {
      like,
    },
  );
}

export function useCommentLikeSpacePostMutation<T extends SpacePostResponse>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['comment-like-space-post'],
    mutationFn: async ({
      spacePk,
      postPk,
      commentSk,
      like,
    }: {
      spacePk: string;
      postPk: string;
      commentSk: string;
      like: boolean;
    }) => {
      await commentLikeSpacePost(spacePk, postPk, commentSk, like);
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
