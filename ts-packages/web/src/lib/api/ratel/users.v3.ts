import { call } from './call';

export interface UserResponse {
  pk: string;
  email: string;
  nickname: string;
  profile_url: string;
  description: string;
  user_type: UserType;
  username: string;
  followers_count: number;
  followings_count: number;
  theme: ThemeType;
  point: number;
  membership: MembershipType;
}

export interface UserDetailResponse extends UserResponse {
  // Additional metadata fields
  referral_code?: string;
  phone_number?: string;
  principal?: string;
  evm_address?: string;
  teams?: Array<{
    team_pk: string;
    nickname: string;
    profile_url: string;
    username: string;
  }>;
  is_identified: boolean;
  has_billing_key: boolean;
}

export type FindUserQueryType = 'email' | 'username' | 'phone-number';

export async function findUser(
  type: FindUserQueryType,
  value: string,
): Promise<UserDetailResponse> {
  const params = new URLSearchParams({
    type,
    value,
  });

  return await call('GET', `/v3/users?${params.toString()}`);
}

// Helper functions for different search types
export async function findUserByEmail(
  email: string,
): Promise<UserDetailResponse> {
  return await findUser('email', email);
}

export async function findUserByUsername(
  username: string,
): Promise<UserDetailResponse> {
  return await findUser('username', username);
}

export async function findUserByPhoneNumber(
  phoneNumber: string,
): Promise<UserDetailResponse> {
  return await findUser('phone-number', phoneNumber);
}

export interface User {
  pk: string;
  sk: string;

  created_at: number; // i64 -> number (epoch)
  updated_at: number;

  display_name: string;
  profile_url: string;
  nickname: string;

  email: string;

  username: string;

  term_agreed: boolean;
  informed_agreed: boolean;

  user_type: UserType;

  followers_count: number;
  followings_count: number;

  // profile contents
  description: string;

  password: string | null;
  membership: MembershipType;

  theme: ThemeType;
  points: number;
}

export enum UserType {
  Individual = 1,
  Team = 2,
  Bot = 3,
  AnonymousSpaceUser = 4,
  Admin = 98,
  Anonymous = 99,
}
// FIXME: Use Membership model in `features/membership`
export enum MembershipType {
  Free = 1,
  Paid1 = 2,
  Paid2 = 3,
  Paid3 = 4,
  Admin = 99,
}

export enum ThemeType {
  Light = 1,
  Dark = 2,
  SystemDefault = 3,
}
