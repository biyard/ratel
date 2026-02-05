import { useInfiniteQuery } from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceRewardResponse } from '../types';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';

export function useSpaceRewards(spacePk: string, entityType?: string) {
  return useInfiniteQuery({
    queryKey: entityType
      ? spaceKeys.rewards_by_entityType(spacePk, entityType)
      : spaceKeys.rewards(spacePk),
    queryFn: async ({
      pageParam,
    }): Promise<ListResponse<SpaceRewardResponse>> => {
      try {
        const params = new URLSearchParams();
        if (pageParam) {
          params.set('bookmark', pageParam);
        }
        params.set('limit', '20');
        const queryString = params.toString();
        const path = `/v3/spaces/${encodeURIComponent(spacePk)}/rewards${queryString ? `?${queryString}` : ''}`;

        const ret: ListResponse<SpaceRewardResponse> = await call('GET', path);
        return ret;
      } catch (e) {
        logger.error('Failed to fetch global rewards', e);
        throw new Error(e);
      }
    },
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage) => lastPage.bookmark ?? undefined,
  });
}
