import { feedKeys } from '@/constants';
import { reply } from '@/lib/api/ratel/comments.v3';
import { getQueryClient } from '@/providers/getQueryClient';
import { useMutation } from '@tanstack/react-query';

export function useReplyCommentMutation() {
  return useMutation({
    mutationFn: async ({
      postPk,
      commentSk,
      content,
    }: {
      postPk: string;
      commentSk: string;
      content: string;
    }) => {
      const resp = await reply(postPk, commentSk, content);

      return { postPk, commentSk, comment: resp };
    },
    onSuccess: async ({ postPk, commentSk }) => {
      const queryKey = feedKeys.repliesOfComment(postPk, commentSk);
      const queryClient = getQueryClient();
      queryClient.invalidateQueries({ queryKey });
    },
  });
}
