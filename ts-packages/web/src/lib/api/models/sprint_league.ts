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
  alias: string; //UUID
  select: SpriteSheet;
  run: SpriteSheet;
  win: string;
  lose: string;
}

export interface SpriteSheet {
  json: string;
  image: string;
}

export const createSprintLeagueRequest = (reward_amount: number = 10000) => {
  return {
    create: {
      reward_amount,
      players: [
        {
          sprint_league_id: 0,
          name: '',
          description: '',
          player_images: {
            alias: '',
            select: { json: '', image: '' },
            run: { json: '', image: '' },
            win: '',
            lose: '',
          },
        },
        {
          sprint_league_id: 0,
          name: '',
          description: '',
          player_images: {
            alias: '',
            select: { json: '', image: '' },
            run: { json: '', image: '' },
            win: '',
            lose: '',
          },
        },
        {
          sprint_league_id: 0,
          name: '',
          description: '',
          player_images: {
            alias: '',
            select: { json: '', image: '' },
            run: { json: '', image: '' },
            win: '',
            lose: '',
          },
        },
      ],
    },
  };
};
export interface CreateSprintLeagueRequest {
  players: SprintLeaguePlayerCreateRequest[];
}

export interface SprintLeaguePlayerCreateRequest {
  sprint_league_id: number;
  name: string;
  description: string;
  player_images: PlayerImages;
}

export interface UpdateSprintLeaguePlayerRequest {
  name?: string;
  description?: string;
  player_images?: PlayerImages;
}
