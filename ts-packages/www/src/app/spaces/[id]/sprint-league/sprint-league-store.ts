import { create } from 'zustand';

import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';
import { useEditCoordinatorStore } from '../space-store';

type SprintLeagueState = {
  players: Record<number, SprintLeaguePlayer>;
};

type SprintLeagueActions = {
  initialize: (initialPlayers: SprintLeaguePlayer[]) => void;
  updatePlayer: (playerId: number, player: SprintLeaguePlayer) => void;
  reset: () => void;
};

const initialState: SprintLeagueState = {
  players: {},
};

export const useSprintLeagueStore = create<
  SprintLeagueState & SprintLeagueActions
>((set) => ({
  ...initialState,
  initialize: (initialPlayers = []) => {
    const playersRecord = initialPlayers.reduce(
      (acc, player) => {
        acc[player.id] = player;
        return acc;
      },
      {} as Record<number, SprintLeaguePlayer>,
    );
    set({ players: playersRecord });
  },
  updatePlayer: (playerId, player) => {
    useEditCoordinatorStore.getState().setModified();
    set((state) => ({
      players: {
        ...state.players,
        [playerId]: player,
      },
    }));
  },
  reset: () => set(initialState),
}));
