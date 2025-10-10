import { Post } from '@/lib/api/ratel/posts.v3';
import { SpaceCommon, SpacePublishState } from '@/types/space-common';
import { createContext, useContext } from 'react';

/**
 * @interface SpaceHeaderContextType
 */
interface SpaceHeaderContextType {
  space: SpaceCommon;
  post: Post;

  isEditing: boolean;
  isModified: boolean;

  isDraft: boolean; // space.publish_state === 'Draft'
  isPublic: boolean; // space.visibility ===  'Public'

  isEditable: boolean; // space.status === 'Waiting' || space.publish_state === 'Draft'
  hasEditPermission: boolean; // permission check

  onEdit: () => void;
  onDelete: () => Promise<void>;
  onSave: () => Promise<void>;
  onPublish: (type: SpacePublishState) => Promise<void>;
  onMakePublic: () => Promise<void>;

  onGoBack: () => void;

  onLike: () => Promise<void>; // Post Like
  onShare: () => Promise<void>; // Post Share
  onComment: () => Promise<void>; // Post Comment

  updateTitle: (newTitle: string) => void;
  updateContent: (newContent: string) => void;
}

const SpaceHeaderContext = createContext<SpaceHeaderContextType | null>(null);

export const useSpaceHeaderContext = (): SpaceHeaderContextType => {
  const context = useContext(SpaceHeaderContext);
  if (!context) {
    throw new Error(
      'useSpaceHeaderContext must be used within a SpaceHeaderProvider',
    );
  }
  return context;
};

export const SpaceHeaderProvider = SpaceHeaderContext.Provider;
