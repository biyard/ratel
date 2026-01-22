import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { MyRewardsResponse } from '../types';

export const QK_MY_REWARDS = 'my-rewards';

export async function getMyRewards(month?: string): Promise<MyRewardsResponse> {
  const params = new URLSearchParams();
  if (month) {
    params.append('month', month);
  }
  const queryString = params.toString();
  const path = `/v3/me/points${queryString ? `?${queryString}` : ''}`;
  return call('GET', path);
}

export function useMyRewards(month?: string) {
  return useQuery({
    queryKey: [QK_MY_REWARDS, month],
    queryFn: () => getMyRewards(month),
  });
}
