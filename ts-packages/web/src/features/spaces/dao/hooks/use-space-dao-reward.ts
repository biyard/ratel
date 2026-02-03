import {
  InfiniteData,
  useInfiniteQuery,
  UseInfiniteQueryResult,
} from '@tanstack/react-query';
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

export type SpaceDaoRewardListResponse = {
  items: SpaceDaoRewardResponse[];
  bookmark?: string | null;
  remaining_count?: number;
  total_count?: number;
};

export function useSpaceDaoReward(
  spacePk: string,
  limit = 1,
  enabled = true,
): UseInfiniteQueryResult<InfiniteData<SpaceDaoRewardListResponse>> {
  return useInfiniteQuery({
    queryKey: spaceDaoKeys.rewardBase(spacePk),
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams();
      params.set('limit', String(limit));
      if (pageParam) {
        params.set('bookmark', String(pageParam));
      }
      const query = params.toString();
      return call<void, SpaceDaoRewardListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/reward${
          query ? `?${query}` : ''
        }`,
      );
    },
    getNextPageParam: (lastPage) => {
      return lastPage.bookmark ?? undefined;
    },
    enabled: Boolean(spacePk) && enabled,
    refetchOnWindowFocus: false,
    initialPageParam: null,
  });
}
