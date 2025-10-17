import { call } from './call';

// Types based on the v3 API DTOs - must match backend enum exactly
export const TeamGroupPermission = {
  // Post Permissions
  PostRead: 0,
  PostWrite: 1,
  PostEdit: 2,
  PostDelete: 3,

  // Space Permissions
  SpaceRead: 10,
  SpaceWrite: 11,
  SpaceEdit: 12,
  SpaceDelete: 13,

  // Team Permission
  TeamAdmin: 20, // Change Group Permissions + All Other Permissions
  TeamEdit: 21, // Edit Team Info, Add/Remove Group
  GroupEdit: 22, // Edit Group Members (Invite/Kick), Change Group Info

  // Admin
  ManagePromotions: 62,
  ManageNews: 63,
} as const;

export type TeamGroupPermission =
  (typeof TeamGroupPermission)[keyof typeof TeamGroupPermission];

export interface CreateTeamRequest {
  username: string;
  nickname: string;
  profile_url: string;
  description: string;
}

export interface CreateTeamResponse {
  team_pk: string;
}

export interface FindTeamResponse {
  teams: TeamResponse[];
}

export interface TeamResponse {
  id: string;
  created_at: number;
  updated_at: number;
  nickname: string;
  username: string;
  profile_url?: string;
  user_type: number;
  html_contents: string;
}

export interface TeamGroupResponse {
  sk: string;
  name: string;
  description: string;
  members: number;
  permissions: number;
}

export interface TeamOwnerResponse {
  user_pk: string;
  display_name: string;
  profile_url: string;
  username: string;
}

export interface TeamDetailResponse {
  id: string;
  created_at: number;
  updated_at: number;
  nickname: string;
  username: string;
  profile_url?: string;
  user_type: number;
  html_contents: string;
  groups?: TeamGroupResponse[];
  owner?: TeamOwnerResponse;
}

export interface CreateGroupRequest {
  name: string;
  description: string;
  image_url: string;
  permissions: TeamGroupPermission[];
}

export interface CreateGroupResponse {
  group_pk: string;
  group_sk: string;
}

export interface UpdateTeamRequest {
  nickname?: string;
  description?: string;
  profile_url?: string;
}

export interface UpdateGroupRequest {
  name?: string;
  description?: string;
  permissions?: TeamGroupPermission[];
}

export interface AddMemberRequest {
  user_pks: string[];
}

export interface AddMemberResponse {
  total_added: number;
  failed_pks: string[];
}

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
  groupSk: string,
  request: UpdateGroupRequest,
): Promise<void> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupSk)}`,
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
  groupSk: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'POST',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupSk)}/member`,
    request,
  );
}

export async function removeGroupMember(
  teamPk: string,
  groupSk: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'DELETE',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups/${encodeURIComponent(groupSk)}/member`,
    request,
  );
}

// Team permissions
export interface HasTeamPermissionResponse {
  has_permission: boolean;
}

export async function checkTeamPermission(
  teamPk: string,
  permission: TeamGroupPermission,
): Promise<HasTeamPermissionResponse> {
  return await call(
    'GET',
    `/v3/teams/permissions?team_pk=${encodeURIComponent(teamPk)}&permission=${permission}`,
  );
}

// Team members
export interface MemberGroup {
  group_id: string;
  group_name: string;
  description: string;
}

export interface TeamMember {
  user_id: string;
  username: string;
  display_name: string;
  profile_url: string;
  groups: MemberGroup[];
  is_owner: boolean;
}

export interface ListMembersResponse {
  items: TeamMember[];
  bookmark?: string;
}

export async function getTeamMembers(
  teamPk: string,
  bookmark?: string,
): Promise<ListMembersResponse> {
  const params = new URLSearchParams({ team_pk: teamPk });
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  return await call('GET', `/v3/teams/_/members?${params.toString()}`);
}
