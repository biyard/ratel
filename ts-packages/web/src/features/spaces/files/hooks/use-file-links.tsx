import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { FileLinkInfo, FileLinkTarget } from '../types/file-link-target';
import { spaceKeys } from '@/constants';

// List all file links in a space
export function useListFileLinks(spacePk: string) {
  return useQuery({
    queryKey: [...spaceKeys.files(spacePk), 'links'],
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

// Link file to targets
export function useLinkFileMutation(spacePk: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: {
      file_url: string;
      targets: FileLinkTarget[];
    }) => {
      return await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/links`,
        params,
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: spaceKeys.files(spacePk) });
    },
  });
}

// Unlink file from targets
export function useUnlinkFileMutation(spacePk: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: {
      file_url: string;
      targets: FileLinkTarget[];
    }) => {
      return await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/files/unlink`,
        params,
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: spaceKeys.files(spacePk) });
    },
  });
}
