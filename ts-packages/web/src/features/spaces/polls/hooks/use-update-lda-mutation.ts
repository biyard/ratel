import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';

export function useUpdateLdaMutation<T extends SpaceAnalyze = SpaceAnalyze>() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({
      spacePk,
      topics,
      keywords,
    }: {
      spacePk: string;
      topics: string[];
      keywords: string[][];
    }) => {
      await call(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`,
        {
          topics,
          keywords,
        },
      );

      return { spacePk };
    },

    onSuccess: async (_, { spacePk }) => {
      await optimisticUpdate<T>(
        { queryKey: spaceKeys.topics(spacePk) },
        (response) => response,
      );

      qc.invalidateQueries({
        queryKey: spaceKeys.topics(spacePk),
      });
    },
  });
}
