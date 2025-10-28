import { QK_USERS_GET_INFO } from '@/constants';
import { getUserInfo } from '@/lib/api/ratel/me.v3';
import { UserDetailResponse } from '@/lib/api/ratel/users.v3';
import {
  QueryClient,
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useUserInfo(): UseQueryResult<UserDetailResponse | null> {
  const query = useQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: async () => {
      try {
        const user = await getUserInfo();
        return user;
      } catch {
        return null;
      }
    },
  });

  return query;
}

export function useSuspenseUserInfo(): UseSuspenseQueryResult<UserDetailResponse | null> {
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
  });

  return query;
}

export function invalidateUserInfo(queryClient: QueryClient) {
  queryClient.invalidateQueries({
    queryKey: [QK_USERS_GET_INFO],
  });
}

export function removeUserInfo(queryClient: QueryClient) {
  queryClient.removeQueries({ queryKey: [QK_USERS_GET_INFO] });
}

export function refetchUserInfo(queryClient: QueryClient) {
  queryClient.refetchQueries({ queryKey: [QK_USERS_GET_INFO] });
}

export function useLoggedIn(): boolean {
  const { data: user } = useUserInfo();
  return user !== undefined && user !== null;
}
