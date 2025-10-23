import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListDiscussionMemberResponse } from '../types/list-discussion-member-response';

export function getOption(spacePk: string, discussionPk: string) {
  return {
    queryKey: spaceKeys.discussion(spacePk, discussionPk),
    queryFn: async () => {
      const file = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/members`,
      );
      return new ListDiscussionMemberResponse(file);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDiscussionMemberSpace(
  spacePk: string,
  discussionPk: string,
): UseSuspenseQueryResult<ListDiscussionMemberResponse> {
  const query = useSuspenseQuery(getOption(spacePk, discussionPk));
  return query;
}
