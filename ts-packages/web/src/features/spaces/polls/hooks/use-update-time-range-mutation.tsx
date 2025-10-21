import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';

export function useUpdateTimeRangeMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      pollSk,
      started_at,
      ended_at,
    }: {
      spacePk: string;
      pollSk: string;
      started_at: number;
      ended_at: number;
    }) => {
      await call(
        'PUT',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollSk)}`,
        {
          started_at,
          ended_at,
        },
      );

      return { spacePk };
    },

    onMutate: async ({ pollSk, spacePk, started_at, ended_at }) => {
      const rollbackSpace = await optimisticUpdate<Poll>(
        { queryKey: spaceKeys.poll(spacePk, pollSk) },
        (poll) => {
          poll.started_at = started_at;
          poll.ended_at = ended_at;

          return poll;
        },
      );

      return { rollbackSpace };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackSpace?.rollback();

      showErrorToast(error.message || 'Failed to delete feed');
    },

    onSettled: () => {
      // TODO: Run after completed, as invalidation
      // const queryClient = getQueryClient();
      // queryClient.invalidateQueries({ queryKey });
    },
  });
}
