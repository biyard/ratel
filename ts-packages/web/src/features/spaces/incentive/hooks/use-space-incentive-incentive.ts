import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceIncentiveKeys } from '@/constants';

type SpaceIncentiveIncentiveRawResponse = {
  pk: string;
  sk: string;
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
  incentive_distributed: boolean;
  created_at?: number;
  updated_at?: number;
};

export type SpaceIncentiveIncentiveResponse = {
  pk: string;
  sk: string;
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
  incentive_distributed: boolean;
  created_at?: number;
  updated_at?: number;
};

type SpaceIncentiveIncentiveRawResponseBody = {
  item?: SpaceIncentiveIncentiveRawResponse | null;
  remaining_count?: number;
  total_count?: number;
};

export type SpaceIncentiveIncentiveResponseBody = {
  item?: SpaceIncentiveIncentiveResponse | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceIncentiveIncentive(
  spacePk: string,
  enabled = true,
): UseQueryResult<SpaceIncentiveIncentiveResponseBody> {
  return useQuery({
    queryKey: spaceIncentiveKeys.incentiveBase(spacePk),
    queryFn: async () => {
      const data = await call<void, SpaceIncentiveIncentiveRawResponseBody>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/user`,
      );
      const item = data.item ?? null;
      return {
        item,
        remaining_count: data.remaining_count,
        total_count: data.total_count,
      };
    },
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
  });
}
