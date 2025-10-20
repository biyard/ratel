import { call } from './call';

// UserDetailResponse has the user fields flattened at the root level
// because the backend uses #[serde(flatten)] on the user field
export interface UserDetailResponse {
  // Flattened user fields
  pk: string;
  email: string;
  nickname: string;
  profile_url: string;
  description: string;
  user_type: number;
  username: string;
  followers_count: number;
  followings_count: number;
  membership: number;
  theme: number;
  point: number;
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
