import { spaceKeys } from '@/constants';
import { useMutation } from '@tanstack/react-query';
import { PollAnswer } from '@/features/spaces/polls/types/poll-question';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';
import { optimisticUpdate } from '@/lib/hook-utils';
import { Space } from '../../types/space';
import { SpaceRequirementType } from '../../requirments/types';

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
        { answers },
      );
    },

    onSuccess: async (_, { spacePk, pollSk, answers }) => {
      await optimisticUpdate<Poll>(
        { queryKey: spaceKeys.poll(spacePk, pollSk) },
        (poll) => {
          if (!poll) return poll;
          return {
            ...poll,
            user_response_count: (poll.user_response_count ?? 0) + 1,
            myResponse: answers,
          };
        },
      );

      await optimisticUpdate<Space>(
        { queryKey: spaceKeys.detail(spacePk) },
        (space) => {
          if (!space) return space;

          // Fix: Actually assign the mapped array back to requirements
          space.requirements = space.requirements.map((req) => {
            // NOTE: Only support a pre poll
            if (
              req.typ === SpaceRequirementType.PrePoll &&
              req.related_sk === pollSk &&
              req.related_pk === spacePk
            ) {
              return { ...req, responded: true };
            }
            return req;
          });

          return space;
        },
      );

      // Invalidate prerequisites query to allow access to space content
      // queryClient.invalidateQueries({
      //   queryKey: spaceKeys.prerequisites(spacePk),
      // });
    },

    onError: (error) => {
      console.error('Failed to submit poll response:', error);
      // The error will be available via mutation.error in the component
      // and can be displayed to the user
    },
  });

  return mutation;
}
