import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type SpaceDaoRewardResponse = {
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

export type SpaceDaoRewardResponseBody = {
  item?: SpaceDaoRewardResponse | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceDaoReward(
  spacePk: string,
  enabled = true,
): UseQueryResult<SpaceDaoRewardResponseBody> {
  return useQuery({
    queryKey: spaceDaoKeys.rewardBase(spacePk),
    queryFn: async () =>
      call<void, SpaceDaoRewardResponseBody>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/reward`,
      ),
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
  });
}
