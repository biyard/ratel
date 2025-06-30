import { QK_GET_TOTAL_USER, QK_GET_USER_BY_EMAIL } from '@/constants';
import { TotalUser } from '@/lib/api/models/user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { QueryResponse } from '@/lib/api/models/common';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useUser(
  page: number,
  size: number,
): UseSuspenseQueryResult<QueryResponse<TotalUser>> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_TOTAL_USER],
    queryFn: () => get(ratelApi.users.getTotalInfo(page, size)),
    refetchOnWindowFocus: false,
  });

  return query;
}

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
