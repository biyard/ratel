import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { FileLinkInfo, FileLinkTarget } from '../types/file-link-target';
import { spaceKeys } from '@/constants';

// List all file links in a space
export function useListFileLinks(spacePk: string) {
  return useQuery({
    queryKey: spaceKeys.file_links(spacePk),
    queryFn: async () => {
      const result = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/links`,
      );
      return result as { file_links: FileLinkInfo[] };
    },
  });
}

// Get files by target
export function useFilesByTarget(spacePk: string, target: FileLinkTarget) {
  return useQuery({
    queryKey: [...spaceKeys.files(spacePk), 'by-target', target],
    queryFn: async (): Promise<{
      target: FileLinkTarget;
      file_urls: string[];
    }> => {
      return await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/by-target?target=${target}`,
      );
    },
  });
}
