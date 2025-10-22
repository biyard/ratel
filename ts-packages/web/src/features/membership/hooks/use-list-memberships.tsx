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
        const ret: ListResponse<unknown> = await call('GET', '/m3/memberships');

        return {
          items: ret.items.map((item) => new MembershipResponse(item)),
          bookmark: ret.bookmark,
        };
      } catch (e) {
        logger.error('Failed to fetch memberships', e);
        throw new Error(e);
      }
    },
  });
}
