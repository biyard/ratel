import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';

export function useUpdateTfIdfMutation<
  T extends SpaceAnalyze = SpaceAnalyze,
>() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({
      spacePk,
      htmlContents,
    }: {
      spacePk: string;
      htmlContents?: string;
    }) => {
      await call(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`,
        {
          tf_idf_html_contents: htmlContents,
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
