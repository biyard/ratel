import { QK_GET_PERMISSION } from '@/constants';
import { GroupPermission } from '@/lib/api/models/group';
import { Permission } from '@/lib/api/models/permission';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  UseSuspenseQueryResult,
  useSuspenseQuery,
} from '@tanstack/react-query';

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

export function usePermission(
  teamId: number,
  permission: GroupPermission,
): UseSuspenseQueryResult<Permission | null> {
  const { get } = useApiCall();

  return useSuspenseQuery({
    queryKey: [QK_GET_PERMISSION, teamId, permission],
    queryFn: async () => {
      const maxAttempts = 3;
      const delayMs = 500;

      let last: Permission | null = null;

      for (let i = 1; i <= maxAttempts; i++) {
        const p = (await get(
          ratelApi.permissions.getPermissions(teamId, permission),
        )) as Permission | null | undefined;

        last = p ?? null;

        if (p && p.has_permission === true) {
          return p;
        }

        if (i < maxAttempts) {
          await sleep(delayMs);
        }
      }

      return last;
    },

    retry: 0,

    refetchOnWindowFocus: false,
    staleTime: 0,
  });
}
