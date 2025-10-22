import { QK_MEMBERSHIPS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';
import { useQuery } from '@tanstack/react-query';
import { MembershipResponse } from '../dto/membership-response';

/**
 * List all memberships (Admin only)
 */
export function useListMemberships() {
  return useQuery({
    queryKey: [QK_MEMBERSHIPS],
    queryFn: async (): Promise<ListResponse<MembershipResponse>> => {
      try {
        const ret: ListResponse<MembershipResponse> = await call(
          'GET',
          '/m3/memberships',
        );

        return ret;
      } catch (e) {
        logger.error('Failed to fetch memberships', e);
        throw new Error(e);
      }
    },
  });
}
