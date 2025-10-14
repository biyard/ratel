import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import {
  getPollSurveySummaries,
  PollSurveySummariesResponse,
} from '@/lib/api/ratel/poll.spaces.v3';
import { SpaceType } from '@/types/space-type';

export function getOption(spacePk: string) {
  return {
    queryKey: [...spaceKeys.detail(spacePk, SpaceType.Poll), 'summary'],
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
