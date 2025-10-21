import SprintLeaguePlayer from './sprint-league-player';

export default interface SprintLeague {
  pk: string;
  sk: string;

  players: SprintLeaguePlayer[];
  votes: number;
  winner: SprintLeaguePlayer | null;
  is_voted: boolean;
}
