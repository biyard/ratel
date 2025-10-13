import { QK_GET_TEAM_BY_ID, QK_GET_TEAM_BY_USERNAME, QK_GET_TEAM_BY_PK } from '@/constants';
import type { Team } from '@/lib/api/models/team';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
  useQuery,
} from '@tanstack/react-query';
import * as teamsV3Api from '@/lib/api/ratel/teams.v3';
import { getTeamByUsername } from '@/lib/api/ratel/teams.v3';
import type { TeamDetailResponse, FindTeamResponse } from '@/lib/api/ratel/teams.v3';
import { logger } from '@/lib/logger';

// Legacy hooks - deprecated, use v3 hooks instead
export function useTeamById(id: number): UseSuspenseQueryResult<Team> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_ID, id],
    queryFn: () => get(ratelApi.teams._legacy_getTeamById(id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useTeamByUsername(
  username: string,
): UseSuspenseQueryResult<Team> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME, username],
    queryFn: () => get(ratelApi.teams._legacy_getTeamByUsername(username)),
    refetchOnWindowFocus: false,
  });

  return query;
}

// V3 hooks - use these for new code
export function useTeamByPk(teamPk: string): UseSuspenseQueryResult<TeamDetailResponse> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_PK, teamPk],
    queryFn: () => teamsV3Api.getTeam(teamPk),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useTeamByUsernameV3(
  username: string,
): UseSuspenseQueryResult<FindTeamResponse> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME + '_v3', username],
    queryFn: () => teamsV3Api.findTeam(username),
    refetchOnWindowFocus: false,
  });

  return query;
}

// Helper hook to get a single team by username using v3 API
export function useTeamDetailByUsername(
  username: string,
): UseSuspenseQueryResult<TeamDetailResponse | null> {
  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME + '_detail_v3_fix2', username],
    queryFn: async () => {
      try {
        console.log('useTeamDetailByUsername: called with username parameter:', username);
        logger.debug('useTeamDetailByUsername: calling with username:', username);
        return await getTeamByUsername(username);
      } catch (error) {
        logger.error('Failed to get team by username:', error);
        throw error; // Let React Query handle the error instead of returning null
      }
    },
    retry: 1, // Only retry once
    refetchOnWindowFocus: false,
  });

  return query;
}

// REMOVED: No more legacy compatibility hooks

// Team permissions hooks
export interface TeamPermissions {
  canEditTeam: boolean;
  canAdminTeam: boolean;
  canEditGroups: boolean;
  canManageMembers: boolean;
  canWritePosts: boolean;
  canEditPosts: boolean;
  canDeletePosts: boolean;
}

export function useTeamPermissions(teamPk: string): TeamPermissions {
  
  // Check multiple permissions in parallel
  const adminQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'admin'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.TeamAdmin) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000, // Cache for 30 seconds
  });
  
  const editQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'edit'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.TeamEdit) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000,
  });
  
  const groupEditQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'group-edit'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.GroupEdit) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000,
  });
  
  const postWriteQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'post-write'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.PostWrite) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000,
  });
  
  const postEditQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'post-edit'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.PostEdit) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000,
  });
  
  const postDeleteQuery = useQuery({
    queryKey: ['team-permission', teamPk, 'post-delete'],
    queryFn: () => teamPk ? teamsV3Api.checkTeamPermission(teamPk, teamsV3Api.TeamGroupPermission.PostDelete) : Promise.resolve({ has_permission: false }),
    enabled: !!teamPk,
    staleTime: 30000,
  });
  
  return {
    canEditTeam: editQuery.data?.has_permission || false,
    canAdminTeam: adminQuery.data?.has_permission || false,
    canEditGroups: groupEditQuery.data?.has_permission || false,
    canManageMembers: groupEditQuery.data?.has_permission || adminQuery.data?.has_permission || false,
    canWritePosts: postWriteQuery.data?.has_permission || false,
    canEditPosts: postEditQuery.data?.has_permission || false,
    canDeletePosts: postDeleteQuery.data?.has_permission || false,
  };
}

// V3 Team Members Hook
export function useTeamMembers(teamUsername: string): UseSuspenseQueryResult<teamsV3Api.ListMembersResponse> {
  return useSuspenseQuery({
    queryKey: ['team-members-v3', teamUsername],
    queryFn: () => teamsV3Api.getTeamMembers(teamUsername),
  });
}
