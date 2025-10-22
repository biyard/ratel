import { call } from './call';
import type { ListPostResponse } from './posts.v3';

export async function getUserInfo(): Promise<UserResponse> {
  return call('GET', '/v3/me');
}

export async function listMyPosts(
  bookmark?: string,
): Promise<ListPostResponse> {
  let path = '/v3/me/posts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export async function listMyDrafts(
  bookmark?: string,
): Promise<ListPostResponse> {
  let path = '/v3/me/drafts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
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

// Update user request types matching backend UpdateUserRequest enum
// Note: Keys must be capitalized to match Rust enum variant names
export type UpdateUserProfileRequest = {
  Profile: {
    nickname: string;
    profile_url: string;
    description: string;
  };
};

export type UpdateUserThemeRequest = {
  Theme: {
    theme: 1 | 2 | 3; // 1 = Light, 2 = Dark, 3 = SystemDefault
  };
};

export type UpdateUserEvmAddressRequest = {
  EvmAddress: {
    evm_address: string;
  };
};

export type UpdateUserRequest =
  | UpdateUserProfileRequest
  | UpdateUserThemeRequest
  | UpdateUserEvmAddressRequest;

// Update user function
export async function updateUser(
  request: UpdateUserRequest,
): Promise<UserResponse> {
  return call('PATCH', '/v3/me', request);
}
