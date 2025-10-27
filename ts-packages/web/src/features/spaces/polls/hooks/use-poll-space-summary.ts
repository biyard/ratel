import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import {
  getPollSurveySummaries,
  PollSurveySummariesResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { pollSpaceKeys } from '@/constants';

export function getOption(spacePk: string, pollPk: string) {
  return {
    queryKey: pollSpaceKeys.summary(spacePk),
    queryFn: async () => {
      const post = await getPollSurveySummaries(spacePk, pollPk);
      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function usePollSpaceSummaries(
  spacePk: string,
  pollPk: string,
): UseSuspenseQueryResult<PollSurveySummariesResponse> {
  const query = useSuspenseQuery(getOption(spacePk, pollPk));
  return query;
}
