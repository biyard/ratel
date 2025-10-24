import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { deleteSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';
import { ListDiscussionResponse } from '../types/list-discussion-response';

export function useDeleteDiscussionMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-discussion'],
    mutationFn: async ({
      spacePk,
      discussionPk,
    }: {
      spacePk: string;
      discussionPk: string;
    }) => {
      await deleteSpaceDiscussion(spacePk, discussionPk);
      return { spacePk, discussionPk };
    },

    onMutate: async ({ spacePk, discussionPk }) => {
      const qk = spaceKeys.discussions(spacePk);
      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListDiscussionResponse>(qk);

      qc.setQueryData<ListDiscussionResponse>(qk, (old) => {
        if (!old) return old;
        return new ListDiscussionResponse({
          discussions: old.discussions.filter((d) => d.pk !== discussionPk),
          bookmark: old.bookmark,
        });
      });

      return { qk, prev };
    },

    onError: (_err, _vars, ctx) => {
      if (ctx?.qk && ctx?.prev) {
        qc.setQueryData(ctx.qk, ctx.prev);
      }
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.discussions(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
