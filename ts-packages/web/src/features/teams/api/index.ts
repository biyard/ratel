import { call } from '@/lib/api/ratel/call';
import type {
  CreateTeamRequest,
  CreateTeamResponse,
  FindTeamResponse,
  TeamDetailResponse,
  UpdateTeamRequest,
  CreateGroupRequest,
  CreateGroupResponse,
  UpdateGroupRequest,
  AddMemberRequest,
  AddMemberResponse,
  ListMembersResponse,
} from '../types';

// Re-export types for convenience
export * from '../types';
export { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';

// Team management functions
export async function createTeam(
  request: CreateTeamRequest,
): Promise<CreateTeamResponse> {
  return await call('POST', '/v3/teams', request);
}

export async function findTeam(username?: string): Promise<FindTeamResponse> {
  const params = username ? `?username=${encodeURIComponent(username)}` : '';
  return await call('GET', `/v3/teams${params}`);
}

export async function getTeam(teamPk: string): Promise<TeamDetailResponse> {
  return await call('GET', `/v3/teams/${encodeURIComponent(teamPk)}`);
}

export async function getTeamByUsername(
  username: string,
): Promise<TeamDetailResponse> {
  // First find the team to get its ID
  const findResult = await findTeam(username);

  if (!findResult.teams || findResult.teams.length === 0) {
    throw new Error(`Team with username '${username}' not found`);
  }

  const team = findResult.teams.find((t) => t.username === username);
  if (!team) {
    throw new Error(`Team with username '${username}' not found`);
  }

  // Now get full team details including groups using the team ID
  return await getTeam(team.id);
}

export async function updateTeam(
  teamPk: string,
  request: UpdateTeamRequest,
): Promise<TeamDetailResponse> {
  return await call(
    'PATCH',
    `/v3/teams/${encodeURIComponent(teamPk)}`,
    request,
  );
}

export async function deleteTeam(teamUsername: string): Promise<void> {
  return await call('DELETE', `/v3/teams/${encodeURIComponent(teamUsername)}`);
}

// Group management functions
export async function createGroup(
  teamPk: string,
  request: CreateGroupRequest,
): Promise<CreateGroupResponse> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups`,
    request,
  );
}

export async function updateGroup(
  teamPk: string,
  groupId: string,
  request: UpdateGroupRequest,
): Promise<void> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupId)}`,
    request,
  );
}

export async function deleteGroup(
  teamUsername: string,
  groupId: string,
): Promise<void> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamUsername)}/groups/${encodeURIComponent(groupId)}`,
    {},
  );
}

// Member management functions
export async function addGroupMember(
  teamPk: string,
  groupId: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupId)}/member`,
    request,
  );
}

export async function removeGroupMember(
  teamPk: string,
  groupId: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupId)}/member`,
    request,
  );
}

// Team members
export async function getTeamMembers(
  teamUsername: string,
  bookmark?: string,
): Promise<ListMembersResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  const queryString = params.toString() ? `?${params.toString()}` : '';
  return await call(
    'GET',
    `/v3/teams/${encodeURIComponent(teamUsername)}/members${queryString}`,
  );
}
