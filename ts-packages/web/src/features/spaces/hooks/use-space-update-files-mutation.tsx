import { spaceKeys } from '@/constants';
import { updateSpaceFiles } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Space } from '../types/space';
import FileModel from '../files/types/file';

export function useSpaceUpdateFilesMutation<T extends Space>() {
  const queryClient = useQueryClient();
  
  const mutation = useMutation({
    mutationKey: ['update-files-space'],
    mutationFn: async ({
      spacePk,
      files,
    }: {
      spacePk: string;
      files: FileModel[];
    }) => {
      const response = await updateSpaceFiles(spacePk, files);
      return response.files;
    },
    onSuccess: async (updatedFiles, { spacePk }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.files = updatedFiles;
        return space;
      });
      
      // Invalidate file links and files list
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.file_links(spacePk),
      });
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.files(spacePk),
      });
    },
  });

  return mutation;
}
