import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { deleteSpaceFile } from '@/lib/api/ratel/spaces.v3';

export function useDeleteFileMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ['delete-space-file'],
    mutationFn: async ({
      spacePk,
      fileUrl,
    }: {
      spacePk: string;
      fileUrl: string;
    }) => {
      return await deleteSpaceFile(spacePk, fileUrl);
    },
    onSuccess: async (_, { spacePk }) => {
      // Invalidate all file-related queries to refresh UI
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.files(spacePk),
      });

      // Invalidate file links
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.file_links(spacePk),
      });

      // Invalidate space detail (for overview files)
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.detail(spacePk),
      });

      // Invalidate board posts (for board files)
      await queryClient.invalidateQueries({
        queryKey: spaceKeys.boards_posts(spacePk),
      });
    },
  });
}
