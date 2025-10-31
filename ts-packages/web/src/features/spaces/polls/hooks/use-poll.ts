import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { ListPollResponse } from '../types/list-poll-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.polls(spacePk),
    queryFn: async () => {
      const poll = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls`,
      );
      return new ListPollResponse(poll);
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePoll(
  spacePk: string,
): UseSuspenseQueryResult<ListPollResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
