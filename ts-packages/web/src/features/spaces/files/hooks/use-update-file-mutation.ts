import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { File } from '@/lib/api/models/feeds';
import { updateSpaceFiles } from '@/lib/api/ratel/file.spaces.v3';
import { FileResponse } from '../types/file-response';

export function useUpdateFileMutation<T extends FileResponse>() {
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
      const spaceQK = spaceKeys.files(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.files = files;
        return response;
      });
    },
  });

  return mutation;
}
