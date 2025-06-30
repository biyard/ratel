import { QK_GET_SPACE_BY_SPACE_ID } from '@/constants';
import { Space } from '@/lib/api/models/spaces';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useSpaceBySpaceId(
  space_id: number,
): UseSuspenseQueryResult<Space> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_SPACE_BY_SPACE_ID, space_id],
    queryFn: () => get(ratelApi.spaces.getSpaceBySpaceId(space_id)),
    refetchOnWindowFocus: false,
  });

  return query;
}
