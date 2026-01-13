import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';

type DownloadAnalyzeResponse = {
  metadata_url: string;
};

export function useDownloadAnalyzeMutation<
  T extends SpaceAnalyze = SpaceAnalyze,
>() {
  const qc = useQueryClient();

  return useMutation({
    mutationFn: async ({ spacePk }: { spacePk: string }) => {
      const res = await call<Record<string, never>, DownloadAnalyzeResponse>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes/download`,
        {},
      );

      return { spacePk, metadataUrl: res.metadata_url };
    },

    onSuccess: async ({ metadataUrl }, { spacePk }) => {
      await optimisticUpdate<T>(
        { queryKey: spaceKeys.topics(spacePk) },
        (response) => response,
      );

      qc.invalidateQueries({ queryKey: spaceKeys.topics(spacePk) });

      const a = document.createElement('a');
      a.href = metadataUrl;
      a.target = '_blank';
      a.rel = 'noreferrer';
      a.download = '';
      document.body.appendChild(a);
      a.click();
      a.remove();
    },
  });
}
