import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import getSprintLeague from '../api/get-sprint-league';
import SprintLeague from '../types/sprint-league';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.sprint_leagues(spacePk),
    queryFn: async () => {
      return await getSprintLeague(spacePk);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSprintLeague(
  spacePk: string,
): UseSuspenseQueryResult<SprintLeague> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
