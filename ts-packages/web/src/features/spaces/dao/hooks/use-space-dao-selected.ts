import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type SpaceDaoSelectedResponse = {
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

export type SpaceDaoSelectedListResponse = {
  items: SpaceDaoSelectedResponse[];
  bookmark?: string | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceDaoSelected(
  spacePk: string,
  bookmark: string | null | undefined,
  limit = 1,
  enabled = true,
): UseQueryResult<SpaceDaoSelectedListResponse> {
  return useQuery({
    queryKey: spaceDaoKeys.selected(spacePk, bookmark ?? null, limit),
    queryFn: async () => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      if (bookmark) {
        params.set('bookmark', bookmark);
      }
      const query = params.toString();
      return call<void, SpaceDaoSelectedListResponse>(
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
