import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { MyRewardsResponse } from '../types';

export const QK_MY_REWARDS = 'my-rewards';

export async function getMyRewards(): Promise<MyRewardsResponse> {
  return call('GET', '/v3/me/points');
}

export function useMyRewards() {
  return useQuery({
    queryKey: [QK_MY_REWARDS],
    queryFn: getMyRewards,
  });
}
