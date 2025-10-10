import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { pollSpaceKeys } from '@/constants';
import {
  getPollSpace,
  PollSpaceResponse,
} from '@/lib/api/ratel/poll.spaces.v3';

export function getOption(spacePk: string) {
  return {
    queryKey: pollSpaceKeys.detail(spacePk),
    queryFn: async () => {
      const post = await getPollSpace(spacePk);
      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePollSpace(
  spacePk: string,
): UseSuspenseQueryResult<PollSpaceResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
