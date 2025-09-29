import { OAuthProvider } from '@/types/oauth-provider';
import { call } from './call';

export async function sendVerificationCode(email: string): Promise<void> {
  await call('POST', '/v3/auth/verification/send-verification-code', {
    email,
  });
}

export async function verifyCode(email: string, code: string): Promise<void> {
  await call('POST', '/v3/auth/verification/verify-code', {
    email,
    code,
  });
}

export async function loginWithOAuth(
  provider: OAuthProvider,
  access_token: string,
): Promise<User> {
  const user: User = await call('POST', '/v3/auth/login', {
    provider,
    access_token,
  });

  return user;
}

export interface User {
  pk: string;
  sk: string;

  created_at: number; // i64 -> number (epoch)
  updated_at: number;

  display_name: string;
  profile_url: string;

  email: string;

  username: string;

  term_agreed: boolean;
  informed_agreed: boolean;

  // FIXME: use enum for user_type
  user_type: number;

  followers_count: number;
  followings_count: number;

  // profile contents
  description: string;

  password: string | null;

  // FIXME: use enum for membership
  membership: number;

  // FIXME: use enum for theme
  theme: number;
  points: number;
}
