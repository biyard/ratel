import { Post } from '@/lib/api/ratel/posts.v3';
import { create } from 'zustand';

export interface SpaceHeaderStore {
  title: string;
  html_content: string;
  isEditingMode: boolean;
  hasUnsavedChanges: boolean;

  // 편집 시작 시 React-Query 데이터를 복사
  startEdit: (post: Post) => void;
  stopEdit: () => void;
  updateTitle: (newTitle: string) => void;
  updateContent: (newContent: string) => void;
  onModifyContent: () => void;
}

export const useSpaceHeaderStore = create<SpaceHeaderStore>((set) => ({
  title: '',
  html_content: '',
  hasUnsavedChanges: false,
  isEditingMode: false,

  startEdit: (post: Post) =>
    set({
      isEditingMode: true,
      title: post.title,
      html_content: post.html_contents,
      hasUnsavedChanges: false,
    }),
  stopEdit: () => set({ isEditingMode: false, hasUnsavedChanges: false }),

  updateTitle: (newTitle) => set({ title: newTitle, hasUnsavedChanges: true }),
  updateContent: (newContent) =>
    set({ html_content: newContent, hasUnsavedChanges: true }),
  onModifyContent: () => set({ hasUnsavedChanges: true }),
}));
