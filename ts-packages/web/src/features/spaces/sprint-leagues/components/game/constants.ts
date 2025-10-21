import { PlayerImage } from '../../types/sprint-league-player';

export const VIEWPORT_WIDTH = 360;
export const VIEWPORT_HEIGHT = 640;
export const CHARACTER_SIZE = 200;

export const BasePlayerImages: PlayerImage[] = [
  {
    run: {
      json: 'https://metadata.ratel.foundation/assets/lee_jun_run.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jun_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/lee_jun_win.png',
    lose: 'https://metadata.ratel.foundation/assets/lee_jun_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/lee_jun_selected.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jun_selected.webp',
    },
  },
  {
    run: {
      json: 'https://metadata.ratel.foundation/assets/kim_moon_run.json',
      image: 'https://metadata.ratel.foundation/assets/kim_moon_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/kim_moon_win.png',
    lose: 'https://metadata.ratel.foundation/assets/kim_moon_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/kim_moon_selected.json',
      image: 'https://metadata.ratel.foundation/assets/kim_moon_selected.webp',
    },
  },
  {
    run: {
      json: 'https://metadata.ratel.foundation/assets/lee_jae_run.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jae_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/lee_jae_win.png',
    lose: 'https://metadata.ratel.foundation/assets/lee_jae_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/lee_jae_selected.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jae_selected.webp',
    },
  },
];
