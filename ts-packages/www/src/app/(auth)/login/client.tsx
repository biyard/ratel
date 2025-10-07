'use client';

import { useRouter, useSearchParams } from 'next/navigation';
import { useEffect, useState } from 'react';
import { useAuthStore } from '../store';
import LoginForm from '../_components/login-form';
import { send } from '@/lib/api/send';
import { sha3 } from '@/lib/utils';
import { useAuth } from '@/lib/contexts/auth-context';
import { getQueryClient } from '@/providers/getQueryClient';
import { useMutation } from '@tanstack/react-query';
import { getKey as getUserKey } from '@/app/(social)/_hooks/user';
import type { AuthUserInfo } from '@/lib/service/firebase-service';
import { route } from '@/route';
import { useTranslation } from 'react-i18next';
import GoogleIcon from '@/assets/icons/google.svg?react';
import Logo from '@/assets/icons/logo.svg?react';
import { useApiCall } from '@/lib/api/use-send';
function LoginLoader({ type }: { type: LoginType }) {
  const { t } = useTranslation('SignIn');

  let logo = null;
  let description = '';
  switch (type) {
    case LoginType.Google:
      logo = <GoogleIcon className="size-5" />;
      description = t('GoogleLogin.loading_description');
      break;
    case LoginType.Email:
      logo = <Logo className="size-5" />;
      description = t('EmailLogin.loading_description');
      break;
  }

  return (
    <div className="absolute bg-background/80 w-full h-full flex flex-col gap-4 justify-center items-center">
      <div className="flex flex-col w-full justify-center items-center gap-35">
        <div className="border-6 border-t-6 w-20.5 h-20.5 border-primary border-t-background rounded-full animate-spin" />
        <div className="absolute flex-row w-16 h-16 bg-white rounded-full justify-center items-center flex">
          <div className="flex flex-row w-6 h-6 justify-center items-center">
            {logo}
          </div>
        </div>
      </div>

      <div className="justify-center text-center text-white font-bold text-base/6">
        {description}
      </div>
    </div>
  );
}

export default function Client() {
  const searchParams = useSearchParams();
  const { updateSearchParams } = useAuthStore();

  useEffect(() => {
    updateSearchParams(searchParams || new URLSearchParams());
  }, [searchParams, updateSearchParams]);
  const { isLoading, login, googleLogin } = useLoginMutation();
  return (
    <div className="flex flex-col items-center w-full p-2">
      <div className="relative max-w-160 w-full">
        {isLoading !== null && <LoginLoader type={isLoading} />}
        <LoginForm
          onGoogleLogin={async () => {
            googleLogin.mutateAsync();
          }}
          onTelegramLogin={async () => {
            // Handle Telegram login
          }}
          onLogin={async (email, password) => {
            login.mutateAsync({ email, password });
          }}
        />
      </div>
    </div>
  );
}

const LoginType = {
  Email: 'Email',
  Google: 'Google',
} as const;

type LoginType = typeof LoginType[keyof typeof LoginType];

function useLoginMutation() {
  const queryClient = getQueryClient();
  const { ed25519KeyPair, login } = useAuth();
  const [isLoading, setLoading] = useState<LoginType | null>(null);
  const router = useRouter();
  const { updateUserInfo, redirectUrl, service } = useAuthStore();
  const { post } = useApiCall();

  const handleOnSuccess = (signupRequired: boolean) => {
    const userQueryKey = getUserKey();

    queryClient.invalidateQueries({
      queryKey: userQueryKey,
    });
    if (signupRequired) {
      router.push(route.signup());
    } else if (service) {
      router.push(route.connect());
    } else {
      router.push(redirectUrl || route.home());
    }
  };

  const loginMutation = useMutation({
    mutationFn: async ({
      email,
      password,
    }: {
      email: string;
      password: string;
    }) => {
      setLoading(LoginType.Email);
      const hashedPassword = sha3(password);
      if (!ed25519KeyPair) {
        throw new Error('Ed25519 key pair is not available');
      }
      const res = await post('/v3/auth/login', {
        email,
        password: hashedPassword,
      });
      if (!res) {
        throw new Error('Login failed');
      }
      return false;
    },
    onSuccess: handleOnSuccess,
    onError: (error) => {
      console.error('Login Error:', error);
    },
    onSettled: () => {
      setLoading(null);
    },
  });

  const googleLoginMutation = useMutation({
    mutationFn: async () => {
      setLoading(LoginType.Google);

      if (!ed25519KeyPair) {
        throw new Error('Ed25519 key pair is not available');
      }
      const user: AuthUserInfo = await login(ed25519KeyPair);
      if (user.keyPair === null) {
        throw new Error('User key pair is not available');
      }
      updateUserInfo({
        email: user.email || '',
        username: user.displayName || '',
        profileImage: user.photoURL || '',
      });
      const info = await send(user.keyPair, '/api/login', '');
      return !info;
    },
    onSuccess: handleOnSuccess,
    onError: (error) => {
      console.error('Google Login Error:', error);
    },
    onSettled: () => {
      setLoading(null);
    },
  });

  return { isLoading, login: loginMutation, googleLogin: googleLoginMutation };
}
