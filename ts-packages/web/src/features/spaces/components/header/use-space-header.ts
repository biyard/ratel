import { PublishType } from '@/components/post-header/modals/publish-space';
import {
  SpaceCommon,
  SpacePublishState,
  SpaceStatus,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import { useNavigate } from 'react-router';
import { usePublishSpaceMutation } from '@/features/spaces/hooks/use-publish-mutation';
import { useUpdateSpaceVisibilityMutation } from '@/features/spaces/hooks/use-update-visibility-mutation';

import { create } from 'zustand';
import Post from '@/features/posts/types/post';

export interface SpaceHeaderStore {
  title: string;
  html_content: string;
  isEditingMode: boolean;
  hasUnsavedChanges: boolean;

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

export interface SpaceHeaderController {
  post: Post;
  space: SpaceCommon;
  isEditingMode: boolean;
  hasUnsavedChanges: boolean;
  title: string;
  html_content: string;
  visibility: SpaceVisibility;
  publishState: SpacePublishState;
  isEditable: boolean;
  hasEditPermission: boolean;
  onStartEdit: () => void;
  onStopEdit: () => void;
  onSave: () => Promise<void>;
  onPublish: (type: PublishType) => Promise<void>;
  onMakePublic: () => Promise<void>;
  onGoBack: () => void;
  onModifyContent: () => void;
  updateTitle: (newTitle: string) => void;
  updateContent: (newContent: string) => void;
}

export const useSpaceHeader = (
  post: Post,
  space: SpaceCommon,
  hasEditPermission: boolean,
  onSave: (title: string, html_content: string) => Promise<void>,
  onStartEdit: () => void,
): SpaceHeaderController => {
  const store = useSpaceHeaderStore();

  const updatePublishState = usePublishSpaceMutation<SpaceCommon>().mutateAsync;
  const updateVisibility =
    useUpdateSpaceVisibilityMutation<SpaceCommon>().mutateAsync;
  const navigate = useNavigate();

  const isEditable =
    space.publish_state === SpacePublishState.Draft ||
    (space.publish_state === SpacePublishState.Published &&
      (space.status === null || space.status === SpaceStatus.Waiting));

  const title = store.isEditingMode ? store.title : post.title;
  const html_content = store.isEditingMode
    ? store.html_content
    : post.html_contents;

  const handleSave = async () => {
    await onSave(store.title, store.html_content);
    store.stopEdit();
  };

  return {
    post,
    space,
    isEditingMode: store.isEditingMode,
    hasUnsavedChanges: store.hasUnsavedChanges,
    title,
    html_content,
    visibility: space.visibility,
    publishState: space.publish_state,
    isEditable,
    hasEditPermission,
    onStartEdit: () => {
      store.startEdit(post);
      onStartEdit();
    },
    onStopEdit: store.stopEdit,
    onSave: handleSave,
    onGoBack: () => {
      if (store.isEditingMode) {
        store.stopEdit();
      } else {
        navigate(-1);
      }
    },
    updateTitle: store.updateTitle,
    updateContent: store.updateContent,
    onPublish: async (type: PublishType) => {
      const visibility: SpaceVisibility =
        type === PublishType.Private ? { type: 'Private' } : { type: 'Public' };
      await updatePublishState({ spacePk: space.pk, visibility });
    },
    onMakePublic: async () => {
      await updateVisibility({
        spacePk: space.pk,
        visibility: { type: 'Public' },
      });
    },
    onModifyContent: store.onModifyContent,
  };
};
