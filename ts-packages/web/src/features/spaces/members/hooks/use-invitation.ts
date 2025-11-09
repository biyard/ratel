import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListInvitationMemberResponse } from '../types/list-invitation-member-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.invitations(spacePk),
    queryFn: async () => {
      const discussion = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/members`,
      );
      return new ListInvitationMemberResponse(discussion);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useInvitationMember(
  spacePk: string,
): UseSuspenseQueryResult<ListInvitationMemberResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
