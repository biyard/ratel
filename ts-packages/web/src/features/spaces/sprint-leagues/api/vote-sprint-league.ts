import { call } from '@/lib/api/ratel/call';
import SprintLeague from '../types/sprint-league';

export default function voteSprintLeague(
  spacePk: string,
  playerSk: string,
  referralCode?: string,
): Promise<SprintLeague> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/sprint-leagues/votes`,
    {
      player_sk: playerSk,
      referral_code: referralCode,
    },
  );
}
