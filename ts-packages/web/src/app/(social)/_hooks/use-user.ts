import { QK_GET_USER_BY_EMAIL } from '@/constants';
import { TotalUser } from '@/lib/api/models/user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useUserByEmail(
  email: string,
): UseSuspenseQueryResult<TotalUser> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_USER_BY_EMAIL],
    queryFn: () => get(ratelApi.users.getUserByEmail(email)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useUserByUsername(
  username: string,
): UseSuspenseQueryResult<TotalUser> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_USER_BY_EMAIL],
    queryFn: () => get(ratelApi.users.getUserByUsername(username)),
    refetchOnWindowFocus: false,
  });

  return query;
}
