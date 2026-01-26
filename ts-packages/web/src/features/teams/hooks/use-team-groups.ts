import {
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { TeamGroup } from '../types/team_group';

export interface ListGroupsResponse {
  items: TeamGroup[];
  bookmark?: string;
}

async function getTeamGroups(
  teamPk: string,
  bookmark?: string,
): Promise<ListGroupsResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }

  const queryString = params.toString() ? `?${params.toString()}` : '';
  return await call(
    'GET',
    `/v3/teams/${encodeURIComponent(teamPk)}/groups${queryString}`,
  );
}

export function getTeamGroupsOption(teamPk: string, bookmark?: string) {
  return {
    queryKey: bookmark
      ? ([...teamKeys.groups(teamPk), bookmark] as const)
      : teamKeys.groups(teamPk),
    queryFn: async () => {
      return await getTeamGroups(teamPk, bookmark);
    },
    refetchOnWindowFocus: false,
  };
}

export function useSuspenseTeamGroups(
  teamPk: string,
  bookmark?: string,
): UseSuspenseQueryResult<ListGroupsResponse> {
  return useSuspenseQuery(getTeamGroupsOption(teamPk, bookmark));
}

export function useTeamGroups(
  teamPk?: string,
  bookmark?: string,
): UseQueryResult<ListGroupsResponse> {
  return useQuery({
    ...getTeamGroupsOption(teamPk!, bookmark),
    enabled: !!teamPk,
  });
}
