import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

type UpdateSpaceCommentParams = {
  spacePk: string;
  postPk: string;
  commentSk: string;
  content: string;
};

export function updateSpaceComment(
  spacePk: string,
  postPk: string,
  commentSk: string,
  content: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(
      postPk,
    )}/comments/${encodeURIComponent(commentSk)}`,
    { content },
  );
}

export function useUpdateSpaceCommentMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-comment'],
    mutationFn: async ({
      spacePk,
      postPk,
      commentSk,
      content,
    }: UpdateSpaceCommentParams) => {
      await updateSpaceComment(spacePk, postPk, commentSk, content);
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
