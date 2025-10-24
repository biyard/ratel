import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { FileResponse } from '../types/file-response';

import File from '@/features/spaces/files/types/file';
import { call } from '@/lib/api/ratel/call';

export function updateSpaceFiles(
  spacePk: string,
  files: File[],
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}/files`, {
    files,
  });
}

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
