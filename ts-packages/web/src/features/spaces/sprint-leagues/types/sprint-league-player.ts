export default interface SprintLeaguePlayer {
  pk: string;
  sk: string;

  name: string;
  description: string;

  player_image: PlayerImage;
  votes: number;
}

export interface PlayerImage {
  select: SpriteSheet;
  run: SpriteSheet;
  win: string;
  lose: string;
}

export interface SpriteSheet {
  json: string;
  image: string;
}

export const defaultPlayer: () => SprintLeaguePlayer = () => ({
  pk: crypto.randomUUID(),
  sk: crypto.randomUUID(),
  name: '',
  description: '',
  player_image: {
    select: {
      json: '',
      image: '',
    },
    run: {
      json: '',
      image: '',
    },
    win: '',
    lose: '',
  },
  votes: 0,
});
