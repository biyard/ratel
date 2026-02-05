import { useQuery } from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceRewardResponse } from '../types';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';

export function useSpaceRewards(spacePk: string, entityType?: string) {
  return useQuery({
    queryKey: entityType
      ? spaceKeys.rewards_by_entityType(spacePk, entityType)
      : spaceKeys.rewards(spacePk),
    queryFn: async (): Promise<SpaceRewardResponse[]> => {
      try {
        // Build query string with action_key if entityType is provided
        const params = new URLSearchParams();
        if (entityType) {
          params.set('action_key', entityType);
        }
        const queryString = params.toString();
        const path = `/v3/spaces/${encodeURIComponent(spacePk)}/rewards${queryString ? `?${queryString}` : ''}`;

        const ret: ListResponse<SpaceRewardResponse> = await call('GET', path);
        return ret.items;
      } catch (e) {
        logger.error('Failed to fetch space rewards', e);
        throw new Error(e);
      }
    },
  });
}
