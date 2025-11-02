import { OAuthProvider } from '@/types/oauth-provider';
import { call } from './call';
import { User } from './users.v3';

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

export async function loginWithTelegram(telegram_raw: string): Promise<User> {
  const user: User = await call('POST', '/v3/auth/login', {
    telegram_raw,
  });

  return user;
}

export interface SignupRequest {
  display_name: string;
  username: string;
  profile_url: string;
  description: string;
  term_agreed: boolean;
  informed_agreed: boolean;

  email?: string;
  password?: string;
  code?: string;

  provider?: OAuthProvider;
  access_token?: string;

  evm_address?: string;
  telegram_raw?: string;
}

export async function signup(data: SignupRequest): Promise<User> {
  const user: User = await call('POST', '/v3/auth/signup', data);

  return user;
}

export async function resetPassword(
  email: string,
  code: string,
  password: string,
): Promise<User> {
  const user: User = await call('POST', '/v3/auth/reset', {
    email,
    code,
    password,
  });

  return user;
}
