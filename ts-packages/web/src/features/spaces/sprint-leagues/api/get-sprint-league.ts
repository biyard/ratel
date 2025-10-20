import { call } from '@/lib/api/ratel/call';
import SprintLeague from '../types/sprint-league';

export default function getSprintLeague(
  spacePk: string,
): Promise<SprintLeague> {
  return call(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/sprint-leagues`,
  );
}
