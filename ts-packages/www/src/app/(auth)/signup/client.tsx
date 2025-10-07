'use client';
import { useRouter, useSearchParams } from 'next/navigation';
import SignupForm from '../_components/signup-form';
import { useAuthStore } from '../store';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { emailSignupRequest } from '@/lib/api/models/users/email-signup-request';
import { signupRequest } from '@/lib/api/models/users/signup-request';
import { route } from '@/route';

interface SignupRequest {
  email: string;
  nickname: string;
  username: string;
  profileImage: string;

  agreed: boolean;
  announcementAgreed: boolean;

  password?: string;
  evmAddress?: string;
  telegramRaw?: string;
}
async function signup({
  email,
  nickname,
  username,
  profileImage,
  agreed,
  announcementAgreed,
  password,
  evmAddress,
  telegramRaw,
}: SignupRequest) {
  let req;
  if (password) {
    req = emailSignupRequest(
      nickname,
      email,
      profileImage,
      agreed,
      announcementAgreed,
      username,
      password,
      telegramRaw || '',
    );
  } else {
    req = signupRequest(
      nickname,
      email,
      profileImage,
      agreed,
      announcementAgreed,
      username,
      evmAddress || '',
      telegramRaw || '',
    );
  }
  return apiFetch<void>(`${config.api_url}${ratelApi.users.signup()}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(req),
  });
}

export default function ClientPage() {
  const searchParams = useSearchParams();

  const source = searchParams?.get('source');
  const redirectUrl = searchParams?.get('redirectUrl');

  const { clearState } = useAuthStore();

  const router = useRouter();
  // Wrap to enforce required fields and return a Promise
  const handleUserInfo = async ({
    email,
    nickname,
    username,
    profileImage,
    password,
    agreed,
    announcementAgreed,
  }: {
    email: string;
    nickname: string;
    username: string;
    profileImage: string;
    password: string;
    agreed: boolean;
    announcementAgreed: boolean;
  }) => {
    await signup({
      email,
      nickname,
      username,
      profileImage,
      password,
      agreed,
      announcementAgreed,
      evmAddress: undefined,
      telegramRaw: undefined,
    });
    clearState();

    if (source) {
      router.push(route.connect());
    } else if (redirectUrl) {
      router.push(redirectUrl);
    }
    router.push(route.home());
  };

  return (
    <div className="flex flex-col items-center w-full py-2">
      <div className="max-w-160 w-full">
        <SignupForm updateUserInfo={handleUserInfo} />
      </div>
    </div>
  );
}
