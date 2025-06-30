import {
  QueryClient,
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { User } from '../models/user';
import { useApiCall } from '../use-send';
import { QK_USERS_GET_INFO } from '@/constants';
import { ratelApi } from '../ratel_api';

/**
 * @deprecated Use `useUserInfo` in '_hooks/user.ts'.
 */

export function useUserInfo(): UseQueryResult<User | undefined> {
  const { get } = useApiCall();
  // const auth = useAuth();
  // const principalText = auth.ed25519KeyPair?.getPrincipal().toText();

  const query = useQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: () => get(ratelApi.users.getUserInfo()),
    // enabled: !!principalText,
    // refetchOnWindowFocus: false,
  });

  return query;
}

export function useSuspenseUserInfo(): UseSuspenseQueryResult<User> {
  const { get } = useApiCall();
  // const auth = useAuth();
  // const principalText = auth.ed25519KeyPair?.getPrincipal().toText();

  const query = useSuspenseQuery({
    queryKey: [QK_USERS_GET_INFO],
    queryFn: () => get(ratelApi.users.getUserInfo()),
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
