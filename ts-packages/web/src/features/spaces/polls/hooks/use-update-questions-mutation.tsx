import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';
import { PollQuestion } from '../types/poll-question';

export function useUpdateQuestionsMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      pollSk,
      questions,
    }: {
      spacePk: string;
      pollSk: string;
      questions: PollQuestion[];
    }) => {
      await call(
        'PUT',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollSk)}`,
        {
          questions,
        },
      );

      return { spacePk };
    },

    onMutate: async ({ pollSk, spacePk, questions }) => {
      const rollbackSpace = await optimisticUpdate<Poll>(
        { queryKey: spaceKeys.poll(spacePk, pollSk) },
        (poll) => {
          poll.questions = questions;

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
