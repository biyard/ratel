import { call } from './call';

// Types based on the v3 API DTOs
export interface TeamGroupPermission {
  PostRead: 0;
  PostWrite: 1;
  PostEdit: 2;
  PostDelete: 3;
  SpaceRead: 10;
  SpaceWrite: 11;
  SpaceEdit: 12;
  SpaceDelete: 13;
  TeamAdmin: 20;
  TeamEdit: 21;
  GroupEdit: 22;
}

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
  return await call('GET', `/v3/teams/${teamPk}`);
}

export async function updateTeam(
  teamPk: string,
  request: UpdateTeamRequest,
): Promise<TeamDetailResponse> {
  return await call('POST', `/v3/teams/${teamPk}`, request);
}

export async function deleteTeam(teamPk: string): Promise<void> {
  return await call('POST', `/v3/teams/${teamPk}`, {});
}

// Group management functions
export async function createGroup(
  teamPk: string,
  request: CreateGroupRequest,
): Promise<CreateGroupResponse> {
  return await call('POST', `/v3/teams/${teamPk}/groups`, request);
}

export async function updateGroup(
  teamPk: string,
  groupSk: string,
  request: UpdateGroupRequest,
): Promise<void> {
  return await call('POST', `/v3/teams/${teamPk}/groups/${groupSk}`, request);
}

// Member management functions
export async function addGroupMember(
  teamPk: string,
  groupSk: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'POST',
    `/v3/teams/${teamPk}/groups/${groupSk}/member`,
    request,
  );
}

export async function removeGroupMember(
  teamPk: string,
  groupSk: string,
  request: AddMemberRequest,
): Promise<AddMemberResponse> {
  return await call(
    'POST',
    `/v3/teams/${teamPk}/groups/${groupSk}/member`,
    request,
  );
}
