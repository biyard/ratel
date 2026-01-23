import { useMutation, useQueryClient } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';
import { optimisticUpdate } from '@/lib/hook-utils';
import { buildReportHtmlDocument } from '../utils/report-html';
import { buildReportPdfBlob, uploadReportPdf } from '../utils/report-pdf';

type DownloadAnalyzeResponse = {
  presigned_url: string;
  metadata_url: string;
  html_contents: string;
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

      const htmlDocument = buildReportHtmlDocument(res.html_contents ?? '');
      const pdfBlob = await buildReportPdfBlob(htmlDocument);
      await uploadReportPdf(res.presigned_url, pdfBlob);

      await call(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`,
        { metadata_url: res.metadata_url },
      );

      return { spacePk, metadataUrl: res.metadata_url };
    },

    onSuccess: async ({ spacePk }) => {
      await optimisticUpdate<T>(
        { queryKey: spaceKeys.topics(spacePk) },
        (response) => response,
      );

      qc.invalidateQueries({ queryKey: spaceKeys.topics(spacePk) });
    },
  });
}
