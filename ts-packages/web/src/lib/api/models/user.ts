import type { Follower } from './network';
import type { Team } from './team';

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

  theme?: ThemeType;
  membership: MembershipType;

  html_contents: string;

  evm_address?: string;

  followers_count: number;
  followings_count: number;

  followers: Follower[];
  followings: Follower[];

  referral_code?: string;
}

export const UserType = {
  Individual: 1,
  Team: 2,
  Bot: 3,
  Anonymous: 99,
} as const;

export type UserType = typeof UserType[keyof typeof UserType];

export const ThemeType = {
  Light: 1,
  Dark: 2,
  SystemDefault: 3,
} as const;

export type ThemeType = typeof ThemeType[keyof typeof ThemeType];

export const MembershipType = {
  Free: 1,
  Paid1: 2,
  Paid2: 3,
  Paid3: 4,
  Admin: 99,
} as const;

export type MembershipType = typeof MembershipType[keyof typeof MembershipType];

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

export const Scope = {
  Global: 1,
  Space: 2,
  Team: 3,
} as const;

export type Scope = typeof Scope[keyof typeof Scope];
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
  html_contents: string;
  username: string;
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
