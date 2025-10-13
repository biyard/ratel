import { PublishType } from '@/components/post-header/modals/publish-space';
import { Post } from '@/lib/api/ratel/posts.v3';
import {
  SpaceCommon,
  SpacePublishState,
  SpaceStatus,
  SpaceVisibility,
} from '@/types/space-common';
import { ReactNode } from 'react';
import { useSpaceHeaderStore } from './store';
import { useNavigate } from 'react-router';
import { usePublishSpaceMutation } from '@/features/spaces/hooks/use-publish-mutation';
import { useUpdateSpaceVisibilityMutation } from '@/features/spaces/hooks/use-update-visibility-mutation';
import { SpaceHeaderContext } from './context';

/**
 * @interface SpaceHeaderContextType
 */
export interface SpaceHeaderContextType {
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
  updateTitle: (newTitle: string) => void;
  updateContent: (newContent: string) => void;
}

export const SpaceHeaderProvider = ({
  children,
  post,
  space,
  hasEditPermission,
  onSave: handleSave,
}: {
  children: ReactNode;
  post: Post;
  space: SpaceCommon;
  hasEditPermission: boolean;
  onSave: (title: string, html_content: string) => Promise<void>;
}) => {
  const store = useSpaceHeaderStore();

  const updatePublishState = usePublishSpaceMutation<SpaceCommon>().mutateAsync;
  const updateVisibility =
    useUpdateSpaceVisibilityMutation<SpaceCommon>().mutateAsync;
  const navigate = useNavigate();

  const isEditable =
    space.status !== SpaceStatus.Waiting ||
    space.publish_state === SpacePublishState.Published;

  const title = store.isEditingMode ? store.title : post.title;
  const html_content = store.isEditingMode
    ? store.html_content
    : post.html_contents;

  const onSave = async () => {
    await handleSave(store.title, store.html_content);
    store.stopEdit();
  };

  const contextValue: SpaceHeaderContextType = {
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
    onStartEdit: () => store.startEdit(post),
    onStopEdit: store.stopEdit,
    onSave,
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
  };

  return (
    <SpaceHeaderContext.Provider value={contextValue}>
      {children}
    </SpaceHeaderContext.Provider>
  );
};
