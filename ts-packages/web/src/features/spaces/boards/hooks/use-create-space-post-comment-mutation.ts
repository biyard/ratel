import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpacePostResponse } from '../types/space-post-response';
import { optimisticUpdate } from '@/lib/hook-utils';

export function commentSpacePost(
  spacePk: string,
  postPk: string,
  content: string,
): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}/comments`,
    {
      content,
    },
  );
}

export function useCommentSpacePostMutation<T extends SpacePostResponse>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['comment-space-post'],
    mutationFn: async ({
      spacePk,
      postPk,
      content,
    }: {
      spacePk: string;
      postPk: string;
      content: string;
    }) => {
      await commentSpacePost(spacePk, postPk, content);
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
