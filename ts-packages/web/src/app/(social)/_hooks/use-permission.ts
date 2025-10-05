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

const EMPTY_PERMISSION: Permission = { has_permission: false } as Permission;

export function usePermission(
  teamUsername: string,
  permission: GroupPermission,
): UseSuspenseQueryResult<Permission> {
  const { get } = useApiCall();

  return useSuspenseQuery({
    queryKey: [QK_GET_PERMISSION, teamUsername, permission],
    queryFn: async () => {
      const maxAttempts = 3;
      const delayMs = 500;

      let last: Permission = EMPTY_PERMISSION;

      for (let i = 1; i <= maxAttempts; i++) {
        const p = (await get(
          ratelApi.permissions.getPermissions(teamUsername, permission),
        )) as Permission | null | undefined;

        last = p ?? EMPTY_PERMISSION;
        if (p?.has_permission === true) return p;
        if (i < maxAttempts) await sleep(delayMs);
      }

      return last;
    },
    retry: 0,
    refetchOnWindowFocus: false,
    staleTime: 0,
  });
}
