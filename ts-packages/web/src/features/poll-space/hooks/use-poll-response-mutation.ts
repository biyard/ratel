import { pollSpaceKeys } from '@/constants';
import {
  PollSpaceResponse,
  submitPollSurveyResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SurveyAnswer } from '@/types/survey-type';
import { useMutation } from '@tanstack/react-query';

export function usePollResponseMutation() {
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
      const pollSpaceQK = pollSpaceKeys.detail(spacePk);
      await optimisticUpdate<PollSpaceResponse>(
        { queryKey: pollSpaceQK },
        (space) => {
          space.user_response_count += 1;
          space.my_response = answers;
          return space;
        },
      );
    },
  });

  return mutation;
}
