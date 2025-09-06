import { create } from 'zustand';

import Artwork from '@/lib/api/models/artwork';
import { useEditCoordinatorStore } from '../space-store';
import { FileInfo } from '@/lib/api/models/feeds';

export enum Tab {
  Content = 1,
  Artwork = 2,
}

type DagitState = {
  artworks: Artwork[];
  insertedArtworks: Artwork[];
  activeTab: Tab;
};

type DagitActions = {
  initialize: (initial: Artwork[]) => void;
  changeTab: (tab: Tab) => void;
  insertArtwork: (
    title: string,
    description: string | null,
    file: FileInfo,
  ) => void;
  clearInsertedArtworks: () => void;
  reset: () => void;
};

const initialState: DagitState = {
  artworks: [],
  insertedArtworks: [],
  activeTab: Tab.Content,
};

export const useDagitStore = create<DagitState & DagitActions>((set) => ({
  ...initialState,
  initialize: (initial = []) => {
    set({
      artworks: initial,
    });
  },

  insertArtwork: (
    title: string,
    description: string | null,
    file: FileInfo,
  ) => {
    useEditCoordinatorStore.getState().setModified();
    set((state) => ({
      insertedArtworks: [
        ...state.insertedArtworks,
        {
          id: Date.now(),
          title,
          description,
          file,
        } as Artwork,
      ],
    }));
  },

  changeTab: (activeTab) => {
    set({ activeTab });
  },
  clearInsertedArtworks: () => {
    set({ insertedArtworks: [] });
  },
  reset: () => set(initialState),
}));
