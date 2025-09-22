import { QK_GET_PERMISSION } from '@/constants';
import { GroupPermission } from '@/lib/api/models/group';
import { Permission } from '@/lib/api/models/permission';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  UseSuspenseQueryResult,
  useSuspenseQuery,
} from '@tanstack/react-query';

export function usePermission(
  teamId: number,
  permission: GroupPermission,
): UseSuspenseQueryResult<Permission> {
  const { get } = useApiCall();

  return useSuspenseQuery({
    queryKey: [QK_GET_PERMISSION, teamId, permission],
    queryFn: async () => {
      const p = (await get(
        ratelApi.permissions.getPermissions(teamId, permission),
      )) as Permission | null | undefined;

      if (!p || p.has_permission === false) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const err: any = new Error('permission_not_ready');
        err.__retryable__ = true;
        throw err;
      }

      return p;
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    retry: (failureCount, error: any) =>
      Boolean(error?.__retryable__) && failureCount < 4,
    retryDelay: 500,
    refetchOnWindowFocus: false,
    staleTime: 0,
  });
}
