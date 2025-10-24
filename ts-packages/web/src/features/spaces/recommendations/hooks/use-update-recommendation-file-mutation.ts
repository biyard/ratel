import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceRecommendationResponse } from '../types/recommendation-response';
import { updateRecommendationFiles } from '@/lib/api/ratel/recommendation.spaces.v3';
import FileType from '../../files/types/file';

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
      files: FileType[];
    }) => {
      await updateRecommendationFiles(spacePk, files);
    },
    onSuccess: async (_, { spacePk, files }) => {
      const spaceQK = spaceKeys.recommendations(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.files = files;
        return response;
      });
    },
  });

  return mutation;
}
