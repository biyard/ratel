import { useQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { TeamRewardsResponse } from '../types';

export const QK_TEAM_REWARDS = 'team-rewards';

export async function getTeamRewards(
  teamPk: string,
  month?: string,
): Promise<TeamRewardsResponse> {
  const params = new URLSearchParams();
  if (month) {
    params.append('month', month);
  }
  const queryString = params.toString();
  const path = `/v3/teams/${teamPk}/points${queryString ? `?${queryString}` : ''}`;
  return call('GET', path);
}

export function useTeamRewards(teamPk: string, month?: string) {
  return useQuery({
    queryKey: [QK_TEAM_REWARDS, teamPk, month],
    queryFn: () => getTeamRewards(teamPk, month),
  });
}
