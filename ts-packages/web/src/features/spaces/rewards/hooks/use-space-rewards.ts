import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListRewardsResponse } from '../types/list-rewards-response';

export function getOption(spacePk: string, entityType?: string) {
  return {
    queryKey: entityType
      ? [...spaceKeys.rewards(spacePk), entityType]
      : spaceKeys.rewards(spacePk),
    queryFn: async () => {
      const params = new URLSearchParams();
      if (entityType) {
        params.append('entity_type', entityType);
      }
      const queryString = params.toString();
      const url = `/v3/spaces/${encodeURIComponent(spacePk)}/rewards${queryString ? `?${queryString}` : ''}`;
      const response = await call<void, ListRewardsResponse>('GET', url);
      return new ListRewardsResponse(response);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceRewards(
  spacePk: string,
  entityType?: string,
): UseSuspenseQueryResult<ListRewardsResponse> {
  const query = useSuspenseQuery(getOption(spacePk, entityType));
  return query;
}
