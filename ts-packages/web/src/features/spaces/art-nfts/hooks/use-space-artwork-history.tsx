import { useInfiniteQuery } from '@tanstack/react-query';
import { ListSpaceArtworkTradeResponse } from '../dto/space-artwork-trade-response';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

export const listSpaceArtworkTrades = async (
  spacePk: string,
  bookmark?: string,
): Promise<ListSpaceArtworkTradeResponse> => {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }

  const response = await call<null, ListSpaceArtworkTradeResponse>(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/artworks/trades?${params.toString()}`,
  );
  return response;
};

export function getOptions(spacePk: string) {
  return {
    queryKey: spaceKeys.art_nft_trades(spacePk),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListSpaceArtworkTradeResponse> =>
      listSpaceArtworkTrades(spacePk, pageParam),
    getNextPageParam: (last: ListSpaceArtworkTradeResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteSpaceArtworkTrade(spacePk: string) {
  return useInfiniteQuery(getOptions(spacePk));
}
