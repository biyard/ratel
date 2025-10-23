import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { createSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';
import { ListDiscussionResponse } from '../types/list-discussion-response';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';

type Vars = {
  spacePk: string;
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  user_ids: string[];
};

export function useCreateDiscussionMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-discussion'],
    mutationFn: async (v: Vars) => {
      const { spacePk, started_at, ended_at, name, description, user_ids } = v;

      await createSpaceDiscussion(
        spacePk,
        started_at,
        ended_at,
        name,
        description,
        user_ids,
      );
      return v;
    },

    onMutate: async (vars) => {
      const { spacePk, started_at, ended_at, name, description } = vars;
      const qk = spaceKeys.discussions(spacePk);

      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListDiscussionResponse>(qk);

      const optimisticItem: SpaceDiscussionResponse = {
        pk: '' as unknown as string,
        name,
        description,
        started_at,
        ended_at,
        is_member: true,
      } as SpaceDiscussionResponse;

      qc.setQueryData<ListDiscussionResponse>(qk, (old) => {
        if (!old) {
          return new ListDiscussionResponse({
            discussions: [optimisticItem],
            bookmark: null,
          });
        }
        return new ListDiscussionResponse({
          discussions: [optimisticItem, ...old.discussions],
          bookmark: old.bookmark,
        });
      });

      return { qk, prev };
    },

    onError: (_err, _vars, ctx) => {
      if (ctx?.qk) {
        qc.setQueryData(ctx.qk, ctx.prev);
      }
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.discussions(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
