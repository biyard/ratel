// Team Types
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
  id: string; // Updated from 'sk' - just the UUID, not the full EntityType
  name: string;
  description: string;
  members: number;
  permissions: number;
}

export interface TeamOwnerResponse {
  id: string; // Updated from 'user_pk' - just the UUID, not the full Partition
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
  dao_address?: string;
  user_type: number;
  html_contents: string;
  groups?: TeamGroupResponse[];
  owner?: TeamOwnerResponse;
  permissions?: bigint; // User's permissions bitmask for this team
}

// Team Members
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

// Request/Response Types
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

export interface CreateGroupRequest {
  name: string;
  description: string;
  image_url: string;
  permissions: number[]; // TeamGroupPermission values
}

export interface CreateGroupResponse {
  group_pk: string;
  group_sk: string;
}

export interface UpdateTeamRequest {
  nickname?: string;
  description?: string;
  profile_url?: string;
  dao_address?: string;
}

export interface UpdateGroupRequest {
  name?: string;
  description?: string;
  permissions?: number[];
}

export interface AddMemberRequest {
  user_pks: string[];
}

export interface AddMemberResponse {
  total_added: number;
  failed_pks: string[];
}

export interface ListMembersResponse {
  items: TeamMember[];
  bookmark?: string;
}
