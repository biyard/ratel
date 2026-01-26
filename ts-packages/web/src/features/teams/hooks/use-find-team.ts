import {
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { Team } from '../types/team';

async function findTeam(username: string): Promise<Team> {
  return await call(
    'GET',
    `/v3/teams?username=${encodeURIComponent(username)}`,
  );
}

export function getFindTeamOption(username: string) {
  return {
    queryKey: [...teamKeys.all, 'find', username] as const,
    queryFn: async () => {
      return await findTeam(username);
    },
    refetchOnWindowFocus: false,
  };
}

export function useSuspenseFindTeam(
  username: string,
): UseSuspenseQueryResult<Team> {
  return useSuspenseQuery(getFindTeamOption(username));
}

export function useFindTeam(username?: string): UseQueryResult<Team> {
  return useQuery({
    ...getFindTeamOption(username!),
    enabled: !!username,
  });
}
