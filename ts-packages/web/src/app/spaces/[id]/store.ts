import { Space } from '@/lib/api/models/spaces';
import { create } from 'zustand';

interface SpaceFormState {
  isEdit: boolean;
  isModified: boolean;
  // Common fields for editing space
  title: string | null;
  content: string | null;
  started_at: number | null;
  ended_at: number | null;

  setTitle: (title: string) => void;
  setContent: (content: string) => void;
  setStartedAt: (started_at: number) => void;
  setEndedAt: (ended_at: number) => void;

  startEditing: (initialData: Space) => void;
  endEditing: () => void;

  saveHandler: () => Promise<Space | null>;
  setSaveHandler: (handler: () => Promise<Space | null>) => void;
}

const useSpaceStore = create<SpaceFormState>((set) => ({
  isEdit: false,
  isModified: false,

  title: null,
  content: null,
  started_at: null,
  ended_at: null,

  setTitle: (title: string) => {
    set({ title, isModified: true });
  },
  setContent: (content: string) => set({ content, isModified: true }),
  setStartedAt: (started_at: number) => set({ started_at, isModified: true }),
  setEndedAt: (ended_at: number) => set({ ended_at, isModified: true }),
  startEditing: (initialData: Space) => {
    set({
      title: initialData.title,
      content: initialData.html_contents,
      started_at: initialData.started_at || Date.now() / 1000,
      ended_at: initialData.ended_at || Date.now() / 1000,
      isEdit: true,
    });
  },

  endEditing: () => {
    set({
      isEdit: false,
      isModified: false,
      title: null,
      content: null,
      started_at: null,
      ended_at: null,
    });
  },
  setSaveHandler: (handler) => set({ saveHandler: handler }),
  saveHandler: async () => {
    throw new Error('"saveHandler" is not implemented');
  },
}));

export default useSpaceStore;
