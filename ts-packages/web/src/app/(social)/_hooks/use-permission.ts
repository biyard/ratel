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

  const query = useSuspenseQuery({
    queryKey: [QK_GET_PERMISSION],
    queryFn: () => get(ratelApi.permissions.getPermissions(teamId, permission)),
    refetchOnWindowFocus: false,
  });

  return query;
}
