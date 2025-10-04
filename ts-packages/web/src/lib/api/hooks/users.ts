import {
  QueryClient,
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { QK_USERS_GET_INFO } from '@/constants';
import { getUserInfo, UserResponse } from '../ratel/me.v3';

/**
 * @deprecated Use `useUserInfo` in '_hooks/user.ts'.
 */

export function useUserInfo(): UseQueryResult<UserResponse | undefined> {
  const query = useQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: () => getUserInfo(),
    // enabled: !!principalText,
    // refetchOnWindowFocus: false,
  });

  return query;
}

export function useSuspenseUserInfo(): UseSuspenseQueryResult<UserResponse> {
  const query = useSuspenseQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: () => getUserInfo(),
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
