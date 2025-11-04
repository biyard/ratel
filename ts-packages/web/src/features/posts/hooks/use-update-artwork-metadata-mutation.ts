import { useMutation, useQueryClient } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { call } from '@/lib/api/ratel/call';
import Post from '../types/post';
import { ArtworkTrait } from '../types/post-artwork';

export function updateArtworkMetadata(
  postPk: string,
  metadata: ArtworkTrait[],
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    metadata,
  });
}

export function useUpdateArtworkMetadataMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: ['update-artwork-metadata'],
    mutationFn: async ({
      postPk,
      metadata,
    }: {
      postPk: string;
      metadata: ArtworkTrait[];
    }) => {
      await updateArtworkMetadata(postPk, metadata);
      return { postPk };
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to update artwork metadata');
    },

    onSettled: (_data, _error, variables) => {
      // Invalidate the specific post detail query
      queryClient.invalidateQueries({
        queryKey: feedKeys.detail(variables.postPk),
      });
    },
  });
}
