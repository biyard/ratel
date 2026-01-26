import {
  useQuery,
  UseQueryResult,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { TeamMember } from '../types/team_member';
export interface ListMembersResponse {
  items: TeamMember[];
  bookmark?: string;
}

async function getTeamMembers(
  teamUsername: string,
  bookmark?: string,
): Promise<ListMembersResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  const queryString = params.toString() ? `?${params.toString()}` : '';
  return await call(
    'GET',
    `/v3/teams/${encodeURIComponent(teamUsername)}/members${queryString}`,
  );
}

export function getTeamMembersOption(teamUsername: string, bookmark?: string) {
  return {
    queryKey: bookmark
      ? ([...teamKeys.members(teamUsername), bookmark] as const)
      : teamKeys.members(teamUsername),
    queryFn: async () => {
      return await getTeamMembers(teamUsername, bookmark);
    },
    refetchOnWindowFocus: false,
  };
}

export function useSuspenseTeamMembers(
  teamUsername: string,
  bookmark?: string,
): UseSuspenseQueryResult<ListMembersResponse> {
  return useSuspenseQuery(getTeamMembersOption(teamUsername, bookmark));
}

export function useTeamMembers(
  teamUsername?: string,
  bookmark?: string,
): UseQueryResult<ListMembersResponse> {
  return useQuery({
    ...getTeamMembersOption(teamUsername!, bookmark),
    enabled: !!teamUsername,
  });
}
