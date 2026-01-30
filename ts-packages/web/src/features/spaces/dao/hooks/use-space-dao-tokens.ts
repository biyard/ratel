import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';

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
): UseQueryResult<SpaceDaoTokenListResponse> {
  return useQuery({
    queryKey: ['space-dao-tokens', spacePk, limit],
    queryFn: async () => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      const query = params.toString();
      return call<void, SpaceDaoTokenListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/tokens${
          query ? `?${query}` : ''
        }`,
      );
    },
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
  });
}
