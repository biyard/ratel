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
      tfIdfKeywords,
      networkTopNodes,

      removeTopics,
    }: {
      spacePk: string;
      ldaTopics: number;
      tfIdfKeywords: number;
      networkTopNodes: number;
      removeTopics: string[];
    }) => {
      await call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`, {
        lda_topics: ldaTopics,
        tf_idf_keywords: tfIdfKeywords,
        network_top_nodes: networkTopNodes,

        remove_topics: removeTopics,
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
