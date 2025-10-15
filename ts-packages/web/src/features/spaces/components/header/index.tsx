import { useNavigate } from 'react-router';

import { openModal as openPublishSpaceModal } from '@/components/post-header/modals/publish-space';
import { openModal as openMakePublicModal } from '@/components/post-header/modals/make-public';
import { openModal as openUnsaveAlertModal } from '@/components/post-header/modals/unsave-alert-modal';
import {
  SpacePublishState,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';
import { usePopup } from '@/lib/contexts/popup-service';
import { Post } from '@/lib/api/ratel/posts.v3';

import {
  BackButton,
  EditButton,
  MakePublicButton,
  PublishSpaceButton,
  SaveButton,
} from '@/components/post-header/buttons';
import { useTranslation } from 'react-i18next';

export type SpaceHeaderProps = {
  post: Post;
  title: string;
  isEditable: boolean;
  hasEditPermission: boolean;

  isEditingMode: boolean;
  hasUnsavedChanges: boolean;

  visibility: SpaceVisibility;
  publishState: SpacePublishState;

  onStartEdit: () => void;
  onStopEdit: () => void;
  onSave: () => Promise<void>;
  onMakePublic: () => Promise<void>;
  onPublish: (type: string) => Promise<void>;

  updateTitle: (newTitle: string) => void;
};

export default function SpaceHeader({
  post,
  title,
  isEditable,
  hasEditPermission,

  isEditingMode,
  hasUnsavedChanges,

  visibility,
  publishState,

  onStartEdit,
  onSave,
  onStopEdit,

  onMakePublic,
  onPublish,

  // onShare,
  // onLike,

  updateTitle,
}: SpaceHeaderProps) {
  const { t } = useTranslation('SpaceHeader');

  const navigate = useNavigate();
  const popup = usePopup();

  const handleGoBack = () => {
    if (isEditingMode) {
      onStopEdit();
    } else {
      navigate(-1);
    }
  };

  const handlePublish = () => {
    if (hasUnsavedChanges) {
      openUnsaveAlertModal(
        popup,
        onSave,
        () => {
          openPublishSpaceModal(popup, onPublish, t('publish_modal_title'));
        },
        t('unsave_notice_modal'),
      );
    } else {
      openPublishSpaceModal(popup, onPublish, t('publish_modal_title'));
    }
  };

  const handleMakePublic = () => {
    openMakePublicModal(popup, onMakePublic, t('make_public_modal_title'));
  };
  return (
    <div className="flex flex-col w-full gap-2.5">
      <SpaceModifySection
        isEditable={isEditable}
        hasEditPermission={hasEditPermission}
        isEditingMode={isEditingMode}
        hasUnsavedChanges={hasUnsavedChanges}
        isPublished={publishState === SpacePublishState.Published}
        canMakePublic={visibility.type !== 'Public'}
        onGoBack={handleGoBack}
        onStartEdit={onStartEdit}
        onSave={onSave}
        onMakePublicButtonClick={handleMakePublic}
        onPublishButtonClick={handlePublish}
      />
      <PostInfoSection
        likes={post.likes}
        shares={post.shares}
        comments={post.comments}
        rewards={post.rewards ?? 0}
        isDraft={publishState === SpacePublishState.Draft}
        isPublic={visibility.type === 'Public'}
      />
      <TitleSection
        isEdit={isEditingMode}
        title={title}
        setTitle={(newTitle) => updateTitle(newTitle)}
        handleShare={async () => {
          console.error('handleShare not implemented');
        }}
      />
      <AuthorSection
        type={post.author_type}
        profileImage={post.author_profile_url}
        name={post.author_display_name}
        isCertified={true}
        createdAt={post.created_at}
      />
    </div>
  );
}

interface SpaceModifySectionProps {
  isEditable: boolean;
  isPublished: boolean;

  canMakePublic: boolean;

  isEditingMode: boolean;
  hasUnsavedChanges: boolean;
  hasEditPermission: boolean;

  onGoBack: () => void;
  onStartEdit: () => void;
  onSave: () => void;

  onPublishButtonClick: () => void;
  onMakePublicButtonClick: () => void;
}

function SpaceModifySection({
  isEditable,
  isPublished,
  canMakePublic,

  isEditingMode,
  hasUnsavedChanges,
  hasEditPermission,

  onGoBack,
  onStartEdit,
  onSave,

  onPublishButtonClick,
  onMakePublicButtonClick,
}: SpaceModifySectionProps) {
  return (
    <div className="flex flex-row justify-between items-center w-full">
      <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
        <BackButton onClick={onGoBack} />
      </div>

      {hasEditPermission && (
        <div className="flex flex-row items-center gap-2 text-sm text-white">
          {isEditable ? (
            isEditingMode ? (
              <SaveButton onClick={onSave} disabled={!hasUnsavedChanges} />
            ) : (
              <EditButton onClick={onStartEdit} />
            )
          ) : (
            <></>
          )}

          {!isPublished && (
            <PublishSpaceButton onClick={onPublishButtonClick} />
          )}
          {canMakePublic && (
            <MakePublicButton onClick={onMakePublicButtonClick} />
          )}
        </div>
      )}
    </div>
  );
}
