import { QK_GET_TEAM_BY_USERNAME, QK_GET_TEAM_BY_PK } from '@/constants';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
  useQuery,
} from '@tanstack/react-query';
import { getTeamByUsername } from '../api';
import type { TeamDetailResponse, FindTeamResponse } from '../types';
import { logger } from '@/lib/logger';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';

// V3 hooks - use these for new code
export function useTeamByPk(
  teamPk: string,
): UseSuspenseQueryResult<TeamDetailResponse> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_PK + '_v3', teamPk],
    queryFn: async () => {
      const { getTeam } = await import('../api');
      return await getTeam(teamPk);
    },
  });

  return query;
}

export function useTeamFindByUsername(
  username: string,
): UseSuspenseQueryResult<FindTeamResponse> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME + '_v3', username],
    queryFn: async () => {
      const { findTeam } = await import('../api');
      return await findTeam(username);
    },
    refetchOnWindowFocus: false,
  });

  return query;
}

// Helper hook to get a single team by username using v3 API
export function useTeamDetailByUsername(username: string) {
  return useQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME + '_detail_v3_fix2', username],
    queryFn: async () => {
      try {
        logger.debug(
          'useTeamDetailByUsername: calling with username:',
          username,
        );
        return await getTeamByUsername(username);
      } catch (error) {
        logger.error('Failed to get team by username:', error);
        throw error; // Let React Query handle the error instead of returning null
      }
    },
    retry: 1, // Only retry once
    refetchOnWindowFocus: false,
  });
}

// REMOVED: No more legacy compatibility hooks

/**
 * NEW: Get team permissions from TeamDetailResponse (no API calls!)
 * Use this instead of useTeamPermissions to avoid multiple API calls.
 * The permissions are included in the team detail response.
 */
export function useTeamPermissionsFromDetail(
  teamDetail: TeamDetailResponse | undefined,
): TeamGroupPermissions | null {
  if (!teamDetail?.permissions) {
    return null;
  }
  // Convert to bigint - JSON deserializes numbers, but we need bigint
  const permissionsBigInt = BigInt(teamDetail.permissions);
  return new TeamGroupPermissions(permissionsBigInt);
}

// V3 Team Members Hook
export function useTeamMembers(teamPkOrUsername: string) {
  return useQuery({
    queryKey: ['team-members-v3', teamPkOrUsername],
    queryFn: async () => {
      const { getTeamMembers } = await import('../api');
      return await getTeamMembers(teamPkOrUsername);
    },
  });
}
