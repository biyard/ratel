import { QK_GET_PERMISSION } from '@/constants';
import { useQuery, UseQueryResult } from '@tanstack/react-query';
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import { TeamGroupPermission } from '@/lib/api/ratel/teams.v3';

// Helper function to check team permission and return boolean
async function checkPermission(teamPk: string, permission: TeamGroupPermission): Promise<boolean> {
  const result = await teamsV3Api.checkTeamPermission(teamPk, permission);
  return result.has_permission;
}

// Individual permission hooks following the established pattern
export function useTeamPermission(
  teamPk: string, 
  permission: TeamGroupPermission
): UseQueryResult<boolean> {
  return useQuery({
    queryKey: [QK_GET_PERMISSION, teamPk, permission],
    queryFn: () => checkPermission(teamPk, permission),
    enabled: !!teamPk,
    refetchOnWindowFocus: false,
  });
}

// Convenience hooks for specific permissions
export function useCanCreateGroup(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.TeamEdit);
}

export function useCanUpdateGroup(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.GroupEdit);
}

export function useCanDeleteGroup(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.TeamEdit);
}

export function useCanInviteMembers(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.GroupEdit);
}

export function useCanWritePosts(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.PostWrite);
}

export function useCanDeletePosts(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.PostDelete);
}

export function useCanManageTeam(teamPk: string): UseQueryResult<boolean> {
  return useTeamPermission(teamPk, TeamGroupPermission.TeamEdit);
}