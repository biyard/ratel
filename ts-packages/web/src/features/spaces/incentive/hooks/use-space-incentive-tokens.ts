import {
  InfiniteData,
  useInfiniteQuery,
  UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceIncentiveKeys } from '@/constants';

export type SpaceIncentiveTokenResponse = {
  pk: string;
  sk: string;
  token_address: string;
  symbol: string;
  decimals: number;
  balance: string;
  updated_at?: number;
};

export type SpaceIncentiveTokenListResponse = {
  items: SpaceIncentiveTokenResponse[];
  bookmark?: string | null;
};

export function useSpaceIncentiveTokens(
  spacePk: string,
  limit = 50,
  enabled = true,
): UseInfiniteQueryResult<InfiniteData<SpaceIncentiveTokenListResponse>> {
  return useInfiniteQuery({
    queryKey: spaceIncentiveKeys.tokens(spacePk),
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      if (pageParam) {
        params.set('bookmark', String(pageParam));
      }
      const query = params.toString();
      return call<void, SpaceIncentiveTokenListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/tokens${
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
