import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';
import { ListDiscussionResponse } from '../types/list-discussion-response';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';

type Vars = {
  spacePk: string;
  discussionPk: string;
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  user_ids: string[];
};

export function useUpdateDiscussionMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-discussion'],
    mutationFn: async (v: Vars) => {
      const {
        spacePk,
        discussionPk,
        started_at,
        ended_at,
        name,
        description,
        user_ids,
      } = v;

      await updateSpaceDiscussion(
        spacePk,
        discussionPk,
        started_at,
        ended_at,
        name,
        description,
        user_ids,
      );

      return v;
    },

    onMutate: async (vars) => {
      const { spacePk, discussionPk, ...patch } = vars;

      const qk = spaceKeys.discussions(spacePk);
      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListDiscussionResponse>(qk);

      qc.setQueryData<ListDiscussionResponse>(qk, (old) => {
        if (!old) return old;

        const updatedList = old.discussions.map(
          (d): SpaceDiscussionResponse => {
            if (d.pk !== discussionPk) return d;

            return {
              ...d,
              started_at: patch.started_at,
              ended_at: patch.ended_at,
              name: patch.name,
              description: patch.description,
              members: patch.user_ids,
            } as SpaceDiscussionResponse;
          },
        );

        return new ListDiscussionResponse({
          discussions: updatedList,
          bookmark: old.bookmark,
        });
      });

      return { qk, prev };
    },

    onError: (_err, _vars, ctx) => {
      if (ctx?.qk) qc.setQueryData(ctx.qk, ctx.prev);
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.discussions(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
