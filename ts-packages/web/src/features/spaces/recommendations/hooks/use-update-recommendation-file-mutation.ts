import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { SpaceRecommendationResponse } from '../types/recommendation-response';
import { updateRecommendationFiles } from '@/lib/api/ratel/recommendation.spaces.v3';
import FileModel from '../../files/types/file';

export function useUpdateRecommendationFileMutation<
  T extends SpaceRecommendationResponse,
>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['update-recommendations-file'],
    mutationFn: async ({
      spacePk,
      files,
    }: {
      spacePk: string;
      files: FileModel[];
    }) => {
      await updateRecommendationFiles(spacePk, files);
    },
    onSuccess: async (_, { spacePk, files }) => {
      const spaceQK = spaceKeys.recommendations(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.files = files;
        return response;
      });
      qc.invalidateQueries({ queryKey: spaceKeys.files(spacePk) });
    },
  });

  return mutation;
}
