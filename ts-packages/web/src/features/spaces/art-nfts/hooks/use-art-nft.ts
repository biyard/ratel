import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import ArtNft from '../types/art-nft';
import ArtNftResponse from '../dto/art-nft-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.art_nft(spacePk),
    queryFn: async () => {
      const nft = await call<ArtNftResponse, null>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/nfts`,
      );
      return new ArtNft(nft);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceNft(
  spacePk: string,
): UseSuspenseQueryResult<ArtNft> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
