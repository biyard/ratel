import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';

const POLL_MS = 3000;

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
    enabled: !!spacePk,
    refetchOnWindowFocus: false,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    refetchInterval: (query: any) => {
      const data = query?.state?.data as SpaceAnalyze | undefined;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const finished = !!(data as any)?.analyze_finish;
      return finished ? false : POLL_MS;
    },
    refetchIntervalInBackground: true,
    staleTime: 0,
  };
}

export default function useTopic(
  spacePk: string,
): UseSuspenseQueryResult<SpaceAnalyze> {
  return useSuspenseQuery(getOption(spacePk));
}
