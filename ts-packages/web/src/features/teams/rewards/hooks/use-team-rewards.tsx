import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { TeamRewardsResponse } from '../types';
import { teamKeys } from '@/constants';

export async function getTeamRewards(
  teamPk: string,
  month?: string,
): Promise<TeamRewardsResponse> {
  const params = new URLSearchParams();
  if (month) {
    params.append('month', month);
  }
  const queryString = params.toString();

  const path = `/v3/teams/${encodeURIComponent(teamPk)}/points${queryString ? `?${queryString}` : ''}`;
  return call('GET', path);
}

export function useTeamRewards(teamPk: string, month?: string) {
  return useQuery({
    queryKey: teamKeys.reward(teamPk, month),
    queryFn: () => getTeamRewards(teamPk, month),
  });
}
