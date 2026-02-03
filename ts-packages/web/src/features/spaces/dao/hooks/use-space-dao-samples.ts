import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type SpaceDaoSampleResponse = {
  pk: string;
  sk: string;
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
  reward_distributed: boolean;
  created_at?: number;
  updated_at?: number;
};

export type SpaceDaoSampleListResponse = {
  items: SpaceDaoSampleResponse[];
  bookmark?: string | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceDaoSamples(
  spacePk: string,
  bookmark: string | null | undefined,
  limit = 1,
  enabled = true,
): UseQueryResult<SpaceDaoSampleListResponse> {
  return useQuery({
    queryKey: spaceDaoKeys.samples(spacePk, bookmark ?? null, limit),
    queryFn: async () => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      if (bookmark) {
        params.set('bookmark', bookmark);
      }
      const query = params.toString();
      return call<void, SpaceDaoSampleListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/selected${
          query ? `?${query}` : ''
        }`,
      );
    },
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
  });
}
