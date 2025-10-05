import { call } from './call';

export async function getUserInfo(): Promise<UserResponse> {
  return call('GET', '/v3/me');
}

export type UserResponse = {
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

  referral_code?: string;
  phone_number?: string;
  principal?: string;
  evm_address?: string;
  telegram_id?: number;
  teams?: UserTeam[];
};

export type UserTeam = {
  nickname: string;
  profile_url: string;
  username: string;
  user_type: number;
};
