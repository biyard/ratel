import { QK_GET_TEAM_BY_ID, QK_GET_TEAM_BY_USERNAME } from '@/constants';
import { Team } from '@/lib/api/models/team';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function useTeamById(id: number): UseSuspenseQueryResult<Team> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_ID, id],
    queryFn: () => get(ratelApi.teams.getTeamById(id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useTeamByUsername(
  username: string,
): UseSuspenseQueryResult<Team> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_TEAM_BY_USERNAME, username],
    queryFn: () => get(ratelApi.teams.getTeamByUsername(username)),
    refetchOnWindowFocus: false,
  });

  return query;
}
