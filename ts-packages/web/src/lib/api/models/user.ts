import { Follower } from './network';
import { Team } from './team';

export interface User {
  id: number;
  created_at: number;
  updated_at: number;

  nickname: string;
  principal: string;
  email: string;
  profile_url?: string;

  term_agreed: boolean;
  informed_agreed: boolean;

  user_type: UserType;
  parent_id?: number;
  username: string;

  groups: Group[];
  teams: Team[];
  badges: Badge[];

  html_contents: string;

  evm_address?: string;

  followers_count: number;
  followings_count: number;

  followers: Follower[];
  followings: Follower[];
}

export enum UserType {
  Individual = 1,
  Team = 2,
  Bot = 3,
  Anonymous = 99,
}

export interface Badge {
  id: number;
  created_at: number;
  updated_at: number;

  creator_id: number;

  name: string;
  scope: Scope;
  image_url: string;

  contract?: string;
  token_id?: number;
}

export enum Scope {
  Global = 1,
  Space = 2,
  Team = 3,
}
export interface Group {
  id: number;
  created_at: number;
  updated_at: number;

  name: string;
  description: string;
  image_url: string;

  creator_id: number;

  member_count: number;
  permissions: number;
}

export interface GroupMember {
  id: number;
  created_at: number;
  updated_at: number;

  nickname: string;
  username: string;
  profile_url: string;
}

export interface TotalUser {
  id: number;
  created_at: number;
  updated_at: number;

  nickname: string;
  username: string;
  email: string;
  profile_url: string;

  user_type: UserType;
}

export interface UserEditProfileRequest {
  edit_profile: {
    nickname: string;
    html_contents: string;
    profile_url: string;
  };
}

export function userEditProfileRequest(
  nickname: string,
  html_contents: string,
  profile_url: string,
): UserEditProfileRequest {
  return {
    edit_profile: {
      nickname,
      html_contents,
      profile_url,
    },
  };
}
