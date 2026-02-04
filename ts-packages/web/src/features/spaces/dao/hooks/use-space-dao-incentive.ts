import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

type SpaceDaoIncentiveRawResponse = {
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

export type SpaceDaoIncentiveResponse = {
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

type SpaceDaoIncentiveRawResponseBody = {
  item?: SpaceDaoIncentiveRawResponse | null;
  remaining_count?: number;
  total_count?: number;
};

export type SpaceDaoIncentiveResponseBody = {
  item?: SpaceDaoIncentiveResponse | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceDaoIncentive(
  spacePk: string,
  enabled = true,
): UseQueryResult<SpaceDaoIncentiveResponseBody> {
  return useQuery({
    queryKey: spaceDaoKeys.incentiveBase(spacePk),
    queryFn: async () => {
      const data = await call<void, SpaceDaoIncentiveRawResponseBody>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/reward`,
      );
      const item = data.item
        ? {
            ...data.item,
            incentive_distributed: data.item.reward_distributed,
          }
        : null;
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
