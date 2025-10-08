import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { getQueryClient } from '@/providers/getQueryClient';
import { useMutation } from '@tanstack/react-query';
import { getKey } from '@/app/(social)/_hooks/user';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { emailSignupRequest } from '@/lib/api/models/users/email-signup-request';
import { signupRequest } from '@/lib/api/models/users/signup-request';

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

export async function signup({
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

export function useSignupMutation() {
  const queryClient = getQueryClient();

  const mutation = useMutation({
    mutationFn: async (req: SignupRequest) => signup(req),
    onSuccess: () => {
      queryClient.refetchQueries({ queryKey: getKey() });
      showSuccessToast('Signup successful! Please check your email to verify.');
    },

    onError: (error) => {
      showErrorToast(error.message || 'Failed to submit signup request');
    },
  });

  return mutation;
}
