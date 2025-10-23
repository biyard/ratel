import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { DiscussionResponse } from '../types/get-discussion-response';

export function getOption(spacePk: string, discussionPk: string) {
  return {
    queryKey: spaceKeys.discussions(spacePk),
    queryFn: async () => {
      const discussion = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}
        `,
      );
      return new DiscussionResponse(discussion);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDiscussion(
  spacePk: string,
  discussionPk: string,
): UseSuspenseQueryResult<DiscussionResponse> {
  const query = useSuspenseQuery(getOption(spacePk, discussionPk));
  return query;
}
