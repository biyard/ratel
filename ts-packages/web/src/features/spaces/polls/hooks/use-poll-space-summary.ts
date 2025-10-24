import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import {
  getPollSurveySummaries,
  PollSurveySummariesResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { spaceKeys } from '@/constants';

export function getOption(spacePk: string, pollSk: string = 'default') {
  return {
    queryKey: spaceKeys.poll_summary(spacePk, pollSk),
    queryFn: async () => {
      const post = await getPollSurveySummaries(spacePk);
      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePollSpaceSummaries(
  spacePk: string,
): UseSuspenseQueryResult<PollSurveySummariesResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
