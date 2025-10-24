import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceRecommendationResponse } from '../types/recommendation-response';
import { updateRecommendationFiles } from '@/lib/api/ratel/recommendation.spaces.v3';
import { File } from '@/lib/api/models/feeds';

export function useUpdateRecommendationFileMutation<
  T extends SpaceRecommendationResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['update-recommendations-file'],
    mutationFn: async ({
      spacePk,
      files,
    }: {
      spacePk: string;
      files: File[];
    }) => {
      await updateRecommendationFiles(spacePk, files);
    },
    onSuccess: async (_, { spacePk, files }) => {
      const spaceQK = spaceKeys.recommendation(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.files = files;
        return response;
      });
    },
  });

  return mutation;
}
