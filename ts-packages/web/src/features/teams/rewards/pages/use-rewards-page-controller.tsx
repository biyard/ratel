import { useTeamRewardsData } from '@/features/teams/rewards/hooks/use-rewards-data';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';

export function useTeamRewardsPageController(username: string) {
  const { data: team } = useSuspenseFindTeam(username);
  const data = useTeamRewardsData(team.pk);
  const permissions = new TeamGroupPermissions(team.permissions || 0n);

  // Admin check for token exchange functionality
  const canExchangeTokens = permissions.isAdmin();

  return {
    ...data,
    team,
    permissions,
    canExchangeTokens,
  };
}
