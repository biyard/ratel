import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { PollAnswer } from '@/features/spaces/polls/types/poll-question';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';
import { optimisticUpdate } from '@/lib/hook-utils';

export function usePollResponseMutation() {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['poll-response'],

    mutationFn: async ({
      spacePk,
      pollSk,
      answers,
    }: {
      spacePk: string;
      pollSk: string;
      answers: PollAnswer[];
    }) => {
      await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollSk)}/responses`,
        { answers },
      );
    },

    onSuccess: async (_, { spacePk, pollSk, answers }) => {
      const qk = spaceKeys.poll(spacePk, pollSk);

      await optimisticUpdate<Poll>({ queryKey: qk }, (poll) => {
        if (!poll) return poll;
        return {
          ...poll,
          user_response_count: (poll.user_response_count ?? 0) + 1,
          myResponse: answers,
        };
      });

      queryClient.invalidateQueries({ queryKey: qk });
    },
  });

  return mutation;
}
