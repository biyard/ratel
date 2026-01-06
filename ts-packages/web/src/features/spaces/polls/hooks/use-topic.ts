import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.topics(spacePk),
    queryFn: async () => {
      const analyze = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/analyzes`,
      );
      return new SpaceAnalyze(analyze);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useTopic(
  spacePk: string,
): UseSuspenseQueryResult<SpaceAnalyze> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
