import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import {
  getPollSurveySummaries,
  PollSurveySummariesResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { spaceKeys } from '@/constants';

export function getOption(spacePk: string, pollSk: string) {
  return {
    queryKey: spaceKeys.poll_summary(spacePk, pollSk),
    queryFn: async () => {
      const survey = await getPollSurveySummaries(spacePk, pollSk);
      return survey;
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePollSpaceSummaries(
  spacePk: string,
  pollSk: string,
): UseSuspenseQueryResult<PollSurveySummariesResponse> {
  const query = useSuspenseQuery(getOption(spacePk, pollSk));
  return query;
}
