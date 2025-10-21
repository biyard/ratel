import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';

export function useUpdateResponseEditableMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      pollSk,
      response_editable,
    }: {
      spacePk: string;
      pollSk: string;
      response_editable: boolean;
    }) => {
      await call(
        'PUT',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollSk)}`,
        {
          response_editable,
        },
      );

      return { spacePk };
    },

    onMutate: async ({ pollSk, spacePk, response_editable }) => {
      const rollbackSpace = await optimisticUpdate<Poll>(
        { queryKey: spaceKeys.poll(spacePk, pollSk) },
        (poll) => {
          poll.response_editable = response_editable;

          return poll;
        },
      );

      return { rollbackSpace };
    },

    onError: (error: Error, _variables, context) => {
      context?.rollbackSpace?.rollback();

      showErrorToast(
        error.message || 'Failed to update response editable setting',
      );
    },

    onSettled: () => {
      // TODO: Run after completed, as invalidation
      // const queryClient = getQueryClient();
      // queryClient.invalidateQueries({ queryKey });
    },
  });
}
