import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';

export function useUpsertAnalyzeMutation<T extends SpaceAnalyze>() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({
      spacePk,
      ldaTopics,
    }: {
      spacePk: string;
      ldaTopics: number;
    }) => {
      await call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`, {
        lda_topics: ldaTopics,
      });

      return { spacePk };
    },

    onSuccess: async (_, { spacePk }) => {
      await optimisticUpdate<T>(
        { queryKey: spaceKeys.topics(spacePk) },
        (response) => {
          return response;
        },
      );
      qc.invalidateQueries({
        queryKey: spaceKeys.topics(spacePk),
      });
    },
  });
}
