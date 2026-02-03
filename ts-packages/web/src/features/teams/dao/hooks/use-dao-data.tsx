import { useMemo } from 'react';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';
import { useSuspenseTeamMembers } from '@/features/teams/hooks/use-team-members';
import { TeamMember } from '@/features/teams/types/team_member';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';

export interface EligibleAdmin extends TeamMember {
  evm_address: string;
}

export function useDaoData(username: string) {
  const { data: team } = useSuspenseFindTeam(username);
  const { data: membersResponse } = useSuspenseTeamMembers(username);

  const members = useMemo(
    () => membersResponse?.items ?? [],
    [membersResponse?.items],
  );

  const eligibleAdmins = useMemo(() => {
    return members.filter((member): member is EligibleAdmin => {
      if (!member.evm_address) {
        return false;
      }

      if (member.is_owner) {
        return true;
      }

      // FIXME: Refactor `list_members_handler`. We should include group permissions in the response
      const hasAdminPermission = member.groups.some((_group) => {
        return true;
      });

      return hasAdminPermission;
    });
  }, [members]);

  const permissions = team.permissions
    ? new TeamGroupPermissions(team.permissions)
    : null;

  return {
    team,
    members,
    eligibleAdmins,
    permissions,
  };
}
