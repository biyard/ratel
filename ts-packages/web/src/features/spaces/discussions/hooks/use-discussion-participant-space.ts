import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListDiscussionParticipantResponse } from '../types/list-discussion-participant-response';

export function getOption(spacePk: string, discussionPk: string) {
  return {
    queryKey: spaceKeys.discussion_participants(spacePk, discussionPk),
    queryFn: async () => {
      const participants = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/participants`,
      );
      return new ListDiscussionParticipantResponse(participants);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDiscussionParticipantSpace(
  spacePk: string,
  discussionPk: string,
): UseSuspenseQueryResult<ListDiscussionParticipantResponse> {
  const query = useSuspenseQuery(getOption(spacePk, discussionPk));
  return query;
}
