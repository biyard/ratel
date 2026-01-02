import { call } from '@/lib/api/ratel/call';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';
import { useQuery } from '@tanstack/react-query';
import { GlobalRewardResponse } from '../types';

export const QK_GLOBAL_REWARDS = 'global-rewards';

export function useListGlobalRewards() {
  return useQuery({
    queryKey: [QK_GLOBAL_REWARDS],
    queryFn: async (): Promise<ListResponse<GlobalRewardResponse>> => {
      try {
        const ret: ListResponse<GlobalRewardResponse> = await call(
          'GET',
          '/v3/rewards',
        );
        return ret;
      } catch (e) {
        logger.error('Failed to fetch global rewards', e);
        throw new Error(e);
      }
    },
  });
}
