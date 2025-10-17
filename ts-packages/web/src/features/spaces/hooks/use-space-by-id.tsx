import { spaceKeys } from '@/constants';
import { getSpaceByPostPk } from '@/lib/api/ratel/spaces.v3';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { logger } from '@/lib/logger';
import { Space } from '../types/space';

export function useSpaceById(spacePk: string): UseSuspenseQueryResult<Space> {
  return useSuspenseQuery({
    queryKey: spaceKeys.detail(spacePk),
    queryFn: async () => {
      try {
        const space = await getSpaceByPostPk(spacePk);
        return Space.fromJson(space);
      } catch (e) {
        logger.error('Failed to fetch space', e);
      }
    },
  });
}
