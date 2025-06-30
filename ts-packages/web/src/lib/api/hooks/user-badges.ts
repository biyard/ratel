import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { UserBadge } from '../models/user-badge';
import { useApiCall } from '../use-send';
import { QK_GET_USER_BADGE } from '@/constants';
import { ratelApi } from '../ratel_api';
import { QueryResponse } from '@/lib/api/models/common';

export function useUserBadge(
  spaceId: number,
  page: number,
  size: number,
): UseSuspenseQueryResult<QueryResponse<UserBadge[]>> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_USER_BADGE, spaceId],
    queryFn: () => get(ratelApi.spaces.getUserBadge(spaceId, page, size)),
    refetchOnWindowFocus: false,
  });

  return query;
}
