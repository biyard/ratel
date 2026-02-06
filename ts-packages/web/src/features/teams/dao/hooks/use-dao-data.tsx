import { useMemo } from 'react';
import { useFindTeam } from '@/features/teams/hooks/use-find-team';
import { useTeamMembers } from '@/features/teams/hooks/use-team-members';
import { TeamMember } from '@/features/teams/types/team_member';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';

export interface EligibleAdmin extends TeamMember {
  evm_address: string;
}

export function useDaoData(username: string, enabled = true) {
  const { data: team } = useFindTeam(enabled ? username : undefined);
  const { data: membersResponse } = useTeamMembers(
    enabled ? username : undefined,
  );

  const members = useMemo(
    () => (enabled ? (membersResponse?.items ?? []) : []),
    [membersResponse?.items, enabled],
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

  const permissions = team?.permissions
    ? new TeamGroupPermissions(team.permissions)
    : null;

  return {
    team,
    members,
    eligibleAdmins,
    permissions,
  };
}
