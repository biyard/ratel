import { QK_MY_MEMBERSHIPS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { logger } from '@/lib/logger';
import { useQuery } from '@tanstack/react-query';
import { UserMembershipResponse } from '../dto/user-membership-response';

/**
 * List all memberships (Admin only)
 */
export function useGetMyMemberships() {
  return useQuery({
    queryKey: [QK_MY_MEMBERSHIPS],
    queryFn: async (): Promise<UserMembershipResponse> => {
      try {
        const ret = await call('GET', '/v3/memberships');

        return new UserMembershipResponse(ret);
      } catch (e) {
        logger.error('Failed to get my memberships', e);
        throw new Error(e);
      }
    },
  });
}
