import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

type DeleteSpaceCommentParams = {
  spacePk: string;
  postPk: string;
  commentSk: string;
};

export function deleteSpaceComment(
  spacePk: string,
  postPk: string,
  commentSk: string,
): Promise<void> {
  return call(
    'DELETE',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(
      postPk,
    )}/comments/${encodeURIComponent(commentSk)}`,
  );
}

export function useDeleteSpaceCommentMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-space-comment'],
    mutationFn: async ({
      spacePk,
      postPk,
      commentSk,
    }: DeleteSpaceCommentParams) => {
      await deleteSpaceComment(spacePk, postPk, commentSk);
    },
    onSuccess: async (_, { spacePk, postPk }) => {
      await qc.invalidateQueries({
        queryKey: spaceKeys.boards_comments(spacePk, postPk),
      });
      await qc.invalidateQueries({
        queryKey: spaceKeys.boards_post(spacePk, postPk),
      });
      await qc.invalidateQueries({
        queryKey: spaceKeys.boards_posts(spacePk),
      });
    },
  });
}
