import { call } from '@/lib/api/ratel/call';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import SpaceArtworkResponse from '../dto/space-artwork-response';
import SpaceArtwork from '../types/space-artwork';
import { spaceKeys } from '@/constants';

export const mintSpaceArtwork = async (
  spacePk: string,
): Promise<SpaceArtwork> => {
  const response = await call<unknown, SpaceArtworkResponse>(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/artworks/mint`,
    {},
  );
  return new SpaceArtwork(response);
};

const useMintSpaceArtworkMutation = () => {
  const queryClient = useQueryClient();

  return useMutation<SpaceArtwork, Error, { spacePk: string }>({
    mutationFn: ({ spacePk }) => mintSpaceArtwork(spacePk),
    onSuccess: (_, variables) => {
      const queryKey = spaceKeys.art_nft(variables.spacePk);
      // Invalidate artwork and history queries to refetch the data
      queryClient.invalidateQueries({
        queryKey,
      });
    },
  });
};

export default useMintSpaceArtworkMutation;
