import { useQuery, UseQueryResult } from '@tanstack/react-query';
import SpaceArtwork from '../types/space-artwork';
import SpaceArtworkResponse from '../dto/space-artwork-response';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

export const getSpaceArtwork = async (
  spacePk: string,
): Promise<SpaceArtwork> => {
  const response = await call<unknown, SpaceArtworkResponse>(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/artworks`,
  );
  return new SpaceArtwork(response);
};

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.art_nft(spacePk),
    queryFn: async () => {
      const post = await getSpaceArtwork(spacePk);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceArtwork(
  spacePk: string,
): UseQueryResult<SpaceArtwork> {
  const query = useQuery({ ...getOption(spacePk) });
  return query;
}
