import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceRecommendationResponse } from '../types/recommendation-response';
import { updateRecommendationContents } from '@/lib/api/ratel/recommendation.spaces.v3';

export function useUpdateRecommendationContentMutation<
  T extends SpaceRecommendationResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['update-recommendations-content'],
    mutationFn: async ({
      spacePk,
      htmlContents,
    }: {
      spacePk: string;
      htmlContents: string;
    }) => {
      await updateRecommendationContents(spacePk, htmlContents);
    },
    onSuccess: async (_, { spacePk, htmlContents }) => {
      const spaceQK = spaceKeys.recommendations(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.html_contents = htmlContents;
        return response;
      });
    },
  });

  return mutation;
}
