import { QK_USERS_GET_INFO } from '@/constants';
import { ratelApi } from '@/lib/api/ratel_api';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { QueryClient, useQuery, UseQueryResult } from '@tanstack/react-query';
import { config } from '@/config';

import { User } from '@/lib/api/models/user';

export function getKey(): [string] {
  return [QK_USERS_GET_INFO];
}

export function useUserInfo(): UseQueryResult<User | null> {
  const query = useQuery({
    queryKey: getKey(),
    queryFn: async () => {
      const { data } = await requestUserInfo();
      return data;
    },
    retry: false,
    refetchOnWindowFocus: false,
  });

  return query;
}

export async function prefetchUserInfo(queryClient: QueryClient) {
  try {
    await queryClient.prefetchQuery({
      queryKey: getKey(),
      queryFn: async () => {
        const { data } = await requestUserInfo();
        return data;
      },
      retry: false,
    });
  } catch (error) {
    console.warn(`Failed to prefetch user info:`, error);
  }
}

export function requestUserInfo(): Promise<FetchResponse<User | null>> {
  return apiFetch<User | null>(
    `${config.api_url}${ratelApi.users.getUserInfo()}`,
    {
      ignoreError: true,
    },
  );
}
