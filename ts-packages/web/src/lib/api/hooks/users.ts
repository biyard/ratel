import {
  QueryClient,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { QK_USERS_GET_INFO } from '@/constants';
import { getUserInfo, UserResponse } from '../ratel/me.v3';

export function useSuspenseUserInfo(): UseSuspenseQueryResult<UserResponse | null> {
  const query = useSuspenseQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: async () => {
      try {
        const user = await getUserInfo();
        return user;
      } catch {
        return null;
      }
    },
    // refetchOnWindowFocus: false,
  });

  return query;
}

export function removeUserInfo(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: [QK_USERS_GET_INFO] });
}

export function refetchUserInfo(queryClient: QueryClient) {
  queryClient.refetchQueries({ queryKey: [QK_USERS_GET_INFO] });
}

export function useLoggedIn(): boolean {
  const { data: user } = useSuspenseUserInfo();
  return user !== undefined && user !== null;
}
