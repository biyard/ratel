'use client';

import {
  TitleSection,
  PostInfoSection,
  AuthorSection,
  SuspenseWrapper,
} from '@/components/post-header';
import useSpaceById, {
  useMakePublicSpace,
  usePublishSpace,
  useShareSpace,
} from '@/hooks/use-space-by-id';
import { TeamContext } from '@/lib/contexts/team-context';
import { useContext } from 'react';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { PublishingScope } from '@/lib/api/models/notice';
import useFeedById from '@/hooks/use-feed-by-id';
import { useParams, useRouter } from 'next/navigation';
import {
  openModal as openPublishSpaceModal,
  PublishType,
} from '@/components/post-header/modals/publish-space';
import { openModal as openMakePublicModal } from '@/components/post-header/modals/make-public';

import { openModal as openMakePublicWithSaveModal } from '@/components/post-header/modals/make-public-with-save';
import { usePopup } from '@/lib/contexts/popup-service';
import {
  BackButton,
  EditButton,
  MakePublicButton,
  PublishSpaceButton,
  SaveButton,
} from '@/components/post-header/buttons';
import { useEditCoordinatorStore } from '../space-store';

function SpaceModifySection({
  spaceId,
  isDraft,
  isPublic,
  authorId,
  onEdit,
}: {
  spaceId: number;
  isDraft: boolean;
  isPublic: boolean;
  authorId: number;
  onEdit: () => void;
}) {
  const router = useRouter();
  const popup = usePopup();

  const {
    isEdit,
    isModified,
    stopEditing,
    triggerGlobalSave: onSave,
  } = useEditCoordinatorStore();
  const { selectedTeam } = useContext(TeamContext);
  const { data: userInfo } = useUserInfo();
  const hasEditPermission =
    authorId === userInfo?.id || selectedTeam?.id === authorId;
  const handleSave = async () => {
    if (!isModified) return;
    await onSave();
  };

  const publishSpace = usePublishSpace(spaceId);
  const makeSpacePublic = useMakePublicSpace(spaceId);

  const handlePublish = async (type: PublishType) => {
    try {
      await publishSpace.mutateAsync(type);
      popup.close();
    } catch (error) {
      console.error('Failed to publish space:', error);
    }
  };

  const handleMakePublic = async () => {
    try {
      await makeSpacePublic.mutateAsync();
      popup.close();
    } catch (error) {
      console.error('Failed to make space public:', error);
    }
  };

  const handleMakePublicModal = () => {
    if (isModified) {
      openMakePublicWithSaveModal(popup, handleMakePublic, handleSave);
    } else {
      openMakePublicModal(popup, handleMakePublic);
    }
  };

  const handlePublishSpaceModal = () => {
    openPublishSpaceModal(popup, handlePublish);
  };

  const handleGoBack = () => {
    if (isEdit) {
      stopEditing();
    } else {
      router.back();
    }
  };

  return (
    <div className="flex flex-row justify-between items-center w-full">
      <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
        <BackButton onClick={handleGoBack} />
      </div>

      {hasEditPermission && (
        <div className="flex flex-row items-center gap-2 text-sm text-white">
          {isDraft ? (
            isEdit ? (
              <SaveButton onClick={handleSave} disabled={!isModified} />
            ) : (
              <EditButton onClick={onEdit} />
            )
          ) : (
            <></>
          )}

          {isDraft && <PublishSpaceButton onClick={handlePublishSpaceModal} />}
          {!isDraft && !isPublic && (
            <MakePublicButton onClick={handleMakePublicModal} />
          )}
        </div>
      )}
    </div>
  );
}

export default function Header() {
  const { id } = useParams();
  const spaceId = Number(id);
  const { data: space } = useSpaceById(spaceId);
  const { data: feed } = useFeedById(space.feed_id);
  const author = space.author[0];

  const { isEdit, commonData, startEditing, updateCommonData } =
    useEditCoordinatorStore();

  const isDraft = space.status === SpaceStatus.Draft;
  const isPublic = space.publishing_scope === PublishingScope.Public;

  const shareSpace = useShareSpace(spaceId);
  const handleShare = async () => {
    await shareSpace.mutateAsync();
  };
  const handleStartEdit = () => {
    startEditing({
      title: space.title,
      html_contents: space.html_contents,
      started_at: space.started_at,
      ended_at: space.ended_at,
    });
  };
  return (
    <div>
      <SuspenseWrapper>
        <div className="flex flex-col w-full gap-2.5 mb-10">
          <SpaceModifySection
            isDraft={isDraft}
            isPublic={isPublic}
            authorId={space.author[0]?.id}
            spaceId={spaceId}
            onEdit={handleStartEdit}
          />
          <PostInfoSection
            likes={feed.likes}
            shares={feed.shares}
            comments={feed.comments}
            rewards={feed.rewards}
            isDraft={isDraft}
            isPublic={isPublic}
          />
          <TitleSection
            isEdit={isEdit}
            title={isEdit ? (commonData?.title ?? '') : space.title}
            setTitle={(newTitle) => updateCommonData({ title: newTitle })}
            handleShare={handleShare}
          />
          <AuthorSection
            type={author.user_type}
            profileImage={author.profile_url}
            name={author.nickname}
            isCertified={true}
            createdAt={space.created_at}
          />
        </div>
      </SuspenseWrapper>
    </div>
  );
}
