import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListDiscussionResponse } from '../types/list-discussion-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.discussions(spacePk),
    queryFn: async () => {
      const discussion = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/discussions`,
      );
      return new ListDiscussionResponse(discussion);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDiscussionSpace(
  spacePk: string,
): UseSuspenseQueryResult<ListDiscussionResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
