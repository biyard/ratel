import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListRewardsResponse } from '../types/list-rewards-response';

export function getOption(spacePk: string, feature?: string) {
  return {
    queryKey: [...spaceKeys.rewards(spacePk), feature],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (feature) {
        params.append('feature', feature);
      }
      const queryString = params.toString();
      const url = `/v3/spaces/${encodeURIComponent(spacePk)}/rewards${queryString ? `?${queryString}` : ''}`;
      const response = await call<void, ListRewardsResponse>('GET', url);
      return new ListRewardsResponse(response);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useRewards(
  spacePk: string,
  feature?: string,
): UseSuspenseQueryResult<ListRewardsResponse> {
  const query = useSuspenseQuery(getOption(spacePk, feature));
  return query;
}
