import { UserType } from '../ratel/users.v3';

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

export type Scope = (typeof Scope)[keyof typeof Scope];
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
