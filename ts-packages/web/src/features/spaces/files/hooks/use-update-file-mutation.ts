import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { FileResponse } from '../types/file-response';

import File from '@/features/spaces/files/types/file';
import { call } from '@/lib/api/ratel/call';

export function updateSpaceFiles(
  spacePk: string,
  files: File[],
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}/files`, {
    files,
    link_target: 'Files', // Files uploaded directly to Files tab
  });
}

export function useUpdateFileMutation<T extends FileResponse>() {
  const queryClient = useQueryClient();
  
  const mutation = useMutation({
    mutationKey: ['update-files-file'],
    mutationFn: async ({
      spacePk,
      files,
    }: {
      spacePk: string;
      files: File[];
    }) => {
      await updateSpaceFiles(spacePk, files);
    },
    onSuccess: async (_, { spacePk, files }) => {
      // Update the files cache
      const spaceQK = spaceKeys.files(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.files = files;
        return response;
      });
      
      // Invalidate file links to refresh the file origins
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.file_links(spacePk),
      });
      
      // Invalidate space detail to refresh overview files
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.detail(spacePk),
      });
      
      // Invalidate board posts to refresh board files
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.boards_posts(spacePk),
      });
    },
  });

  return mutation;
}
