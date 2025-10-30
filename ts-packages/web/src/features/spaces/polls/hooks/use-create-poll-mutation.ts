import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';
import { createPollSpace } from '@/lib/api/ratel/poll.spaces.v3';
import { ListPollResponse } from '../types/list-poll-response';

export function useCreatePollSpaceMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({ spacePk }: { spacePk: string }) => {
      await createPollSpace(spacePk);
      return { spacePk };
    },

    onMutate: async ({ spacePk }) => {
      const queryKey = spaceKeys.polls(spacePk);
      const rollbackSpace = await optimisticUpdate<ListPollResponse>(
        { queryKey },
        (poll) => {
          return poll;
        },
      );

      return { rollbackSpace };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackSpace?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.polls(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
