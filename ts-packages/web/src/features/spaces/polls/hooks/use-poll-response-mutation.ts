import { pollSpaceKeys, spaceKeys } from '@/constants';
import {
  PollSpaceResponse,
  submitPollSurveyResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SurveyAnswer } from '@/features/spaces/polls/types/poll-question';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export function usePollResponseMutation() {
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationKey: ['poll-response'],
    mutationFn: async ({
      spacePk,
      answers,
    }: {
      spacePk: string;
      answers: SurveyAnswer[];
    }) => {
      await submitPollSurveyResponse(spacePk, answers);
    },
    onSuccess: async (_, { spacePk, answers }) => {
      const pollSpaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<PollSpaceResponse>(
        { queryKey: pollSpaceQK },
        (space) => {
          if (!space.my_response) {
            space.user_response_count += 1;
          }
          space.my_response = answers;
          return space;
        },
      );

      queryClient.invalidateQueries({
        queryKey: pollSpaceKeys.summary(spacePk),
      });
    },
  });

  return mutation;
}
