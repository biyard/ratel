import { call } from './call';
import { File } from '@/lib/api/models/feeds';

export function updateRecommendationContents(
  spacePk: string,
  html_contents: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/recommendations`,
    {
      Content: {
        html_contents: html_contents,
      },
    },
  );
}

export function updateRecommendationFiles(
  spacePk: string,
  files: File[],
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/recommendations`,
    {
      File: {
        files: files,
      },
    },
  );
}
