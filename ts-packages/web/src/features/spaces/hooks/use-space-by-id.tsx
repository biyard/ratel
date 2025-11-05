import { spaceKeys } from '@/constants';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { logger } from '@/lib/logger';
import { Space } from '../types/space';
import { call } from '@/lib/api/ratel/call';

export function useSpaceById(spacePk: string): UseSuspenseQueryResult<Space> {
  return useSuspenseQuery({
    queryKey: spaceKeys.detail(spacePk),
    queryFn: async () => {
      try {
        const space = await call(
          'GET',
          `/v3/spaces/${encodeURIComponent(spacePk)}`,
        );
        return new Space(space);
      } catch (e) {
        logger.error('Failed to fetch space', e);
      }
    },
  });
}
