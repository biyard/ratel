import { QK_USERS_GET_INFO } from '@/constants';
import { QueryClient, useQuery, type UseQueryResult } from '@tanstack/react-query';

import { getUserInfo, type UserResponse } from '@/lib/api/ratel/me.v3';

export function getKey(): [string] {
  return [QK_USERS_GET_INFO];
}

export function useUserInfo(): UseQueryResult<UserResponse | null> {
  const query = useQuery({
    queryKey: getKey(),
    queryFn: async () => {
      const data = await getUserInfo();
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
        const data = await getUserInfo();
        return data;
      },
      retry: false,
    });
  } catch (error) {
    console.warn(`Failed to prefetch user info:`, error);
  }
}
