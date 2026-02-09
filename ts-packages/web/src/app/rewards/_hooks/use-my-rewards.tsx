import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { MyRewardsResponse } from '../types';
import { userKeys } from '@/constants';

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
    queryKey: userKeys.point(month),
    queryFn: () => getMyRewards(month),
  });
}
