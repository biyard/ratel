import { useNavigate } from 'react-router';
import SpaceModifySection from './modify-section';

import { openModal as openPublishSpaceModal } from '@/components/post-header/modals/publish-space';
import { openModal as openMakePublicModal } from '@/components/post-header/modals/make-public';
import { openModal as openUnsaveAlertModal } from '@/components/post-header/modals/unsave-alert-modal';
import { SpacePublishState } from '@/types/space-common';
import {
  AuthorSection,
  PostInfoSection,
  TitleSection,
} from '@/components/post-header';
import { useTranslation } from 'react-i18next';
import { usePopup } from '@/lib/contexts/popup-service';
import { useSpaceHeaderContext } from './context';

// Set Deprecated

/**
 * @deprecated use SpaceHeader from '@/features/spaces/components/header' instead
 */

export default function SpaceHeader() {
  const {
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
  } = useSpaceHeaderContext();
  const navigate = useNavigate();
  const { t } = useTranslation('SpaceHeader');
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
