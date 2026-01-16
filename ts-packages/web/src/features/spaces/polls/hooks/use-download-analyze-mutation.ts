import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';
import { getOption } from './use-topic';

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

      const openUrl = (url: string) => {
        const a = document.createElement('a');
        a.href = url;
        a.target = '_blank';
        a.rel = 'noreferrer';
        a.download = '';
        document.body.appendChild(a);
        a.click();
        a.remove();
      };

      if (metadataUrl && metadataUrl.startsWith('http')) {
        openUrl(metadataUrl);
        return;
      }

      const startedAt = Date.now();
      const pollIntervalMs = 3000;
      const maxWaitMs = 600000;

      const timer = window.setInterval(async () => {
        if (Date.now() - startedAt > maxWaitMs) {
          window.clearInterval(timer);
          return;
        }

        try {
          const data = await qc.fetchQuery(getOption(spacePk));
          const url = (data as SpaceAnalyze | undefined)?.metadata_url ?? '';
          if (url && url.startsWith('http')) {
            window.clearInterval(timer);
            openUrl(url);
          }
        } catch {
          // ignore polling errors
        }
      }, pollIntervalMs);
    },
  });
}
