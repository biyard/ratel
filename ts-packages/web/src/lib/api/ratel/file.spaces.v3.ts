import { call } from './call';
import { File } from '@/lib/api/models/feeds';

export function updateSpaceFiles(
  spacePk: string,
  files: File[],
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}/files`, {
    files,
  });
}
