import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { useRef } from 'react';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceAnalyze } from '../types/space-analyze';

const POLL_MS = 3000;
const MAX_POLL_MS = 60_000 * 10; //10분

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
  const option = getOption(spacePk);
  const startAtRef = useRef<number>(Date.now());

  return useSuspenseQuery({
    ...option,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    refetchInterval: (query: any) => {
      const data = query?.state?.data as SpaceAnalyze | undefined;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const finished = !!(data as any)?.analyze_finish;
      if (finished) return false;
      if (Date.now() - startAtRef.current >= MAX_POLL_MS) return false;
      return POLL_MS;
    },
  });
}
