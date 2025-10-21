import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { PollAnswer } from '@/features/spaces/polls/types/poll-question';
import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';

export function usePollResponseMutation() {
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
        {
          answers,
        },
      );
    },
    onSuccess: async (_, { spacePk, pollSk, answers }) => {
      const pollSpaceQK = spaceKeys.poll(spacePk, pollSk);
      await optimisticUpdate<Poll>({ queryKey: pollSpaceQK }, (poll) => {
        poll.user_response_count += 1;
        poll.myResponse = answers;

        return poll;
      });
    },
  });

  return mutation;
}
