export interface SprintLeague {
  id: number;
  created_at: number;
  updated_at: number;
  space_id: number;
  players: SprintLeaguePlayer[];
  winner_id?: number;
  votes: number;
  is_voted: boolean;
  reward_amount: number;
}
export interface SprintLeaguePlayer {
  id: number;
  sprint_league_id: number;
  name: string;
  description: string;
  player_images: PlayerImages;
  votes: number;
}

export interface PlayerImages {
  select: SpriteSheet;
  run: SpriteSheet;
  win: string;
  lose: string;
}

export interface SpriteSheet {
  json: string;
  image: string;
}
