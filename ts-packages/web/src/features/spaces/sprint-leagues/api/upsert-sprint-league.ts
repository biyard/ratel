import { call } from '@/lib/api/ratel/call';
import SprintLeague from '../types/sprint-league';
import CreateSprintLeaguePlayer from '../types/create-sprint-league-player';

export default function updateSprintLeague(
  spacePk: string,
  players: CreateSprintLeaguePlayer[],
): Promise<SprintLeague> {
  return call(
    'PUT',
    `/v3/spaces/${encodeURIComponent(spacePk)}/sprint-leagues`,
    {
      players,
    },
  );
}
