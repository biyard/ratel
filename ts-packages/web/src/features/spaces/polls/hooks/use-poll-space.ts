import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { Poll } from '../types/poll';

export function getOption(spacePk: string, pollPk: string) {
  return {
    queryKey: spaceKeys.poll(spacePk, pollPk),
    queryFn: async () => {
      const poll = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/polls/${encodeURIComponent(pollPk)}`,
      );
      return new Poll(poll);
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePollSpace(
  spacePk: string,
  pollPk: string,
): UseSuspenseQueryResult<Poll> {
  const query = useSuspenseQuery(getOption(spacePk, pollPk));
  return query;
}
