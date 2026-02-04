import {
  InfiniteData,
  useInfiniteQuery,
  UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type SpaceDaoTokenResponse = {
  pk: string;
  sk: string;
  token_address: string;
  symbol: string;
  decimals: number;
  balance: string;
  updated_at?: number;
};

export type SpaceDaoTokenListResponse = {
  items: SpaceDaoTokenResponse[];
  bookmark?: string | null;
};

export function useSpaceDaoTokens(
  spacePk: string,
  limit = 50,
  enabled = true,
): UseInfiniteQueryResult<InfiniteData<SpaceDaoTokenListResponse>> {
  return useInfiniteQuery({
    queryKey: spaceDaoKeys.tokens(spacePk),
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      if (pageParam) {
        params.set('bookmark', String(pageParam));
      }
      const query = params.toString();
      return call<void, SpaceDaoTokenListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/tokens${
          query ? `?${query}` : ''
        }`,
      );
    },
    getNextPageParam: (lastPage) => lastPage.bookmark ?? undefined,
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
    initialPageParam: null,
  });
}
