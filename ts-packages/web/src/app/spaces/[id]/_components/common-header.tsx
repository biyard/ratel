'use client';

import {
  TitleSection,
  PostInfoSection,
  AuthorSection,
  SuspenseWrapper,
} from '@/components/post-header';
import useSpaceById, {
  useDeleteSpace,
  useMakePublicSpace,
  usePublishSpace,
  useShareSpace,
} from '@/hooks/use-space-by-id';
import { TeamContext } from '@/lib/contexts/team-context';
import { useContext } from 'react';
import { spaceDeleteRequest, SpaceStatus } from '@/lib/api/models/spaces';
import { PublishingScope } from '@/lib/api/models/notice';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { useParams, useRouter } from 'next/navigation';
import {
  openModal as openPublishSpaceModal,
  PublishType,
} from '@/components/post-header/modals/publish-space';
import { openModal as openMakePublicModal } from '@/components/post-header/modals/make-public';
import { openModal as openUnsaveAlertModal } from '@/components/post-header/modals/unsave-alert-modal';
import { usePopup } from '@/lib/contexts/popup-service';
import {
  BackButton,
  EditButton,
  MakePublicButton,
  PublishSpaceButton,
  SaveButton,
} from '@/components/post-header/buttons';
import { useEditCoordinatorStore } from '../space-store';
import { useTranslations } from 'next-intl';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { GroupPermission } from '@/lib/api/models/group';
import { usePermission } from '@/app/(social)/_hooks/use-permission';
import { useDropdown } from './dropdown/dropdown-service';
import { Extra } from '@/components/icons';
import DropdownMenu from './dropdown/dropdown-menu';
import DeleteSpacePopup from './modal/confirm-delete';

function SpaceModifySection({
  title,
  spaceId,
  isDraft,
  isPublic,
  authorName,
  onEdit,
  onDelete,
}: {
  title: string | undefined;
  spaceId: number;
  isDraft: boolean;
  isPublic: boolean;
  authorId: number;
  authorName: string;
  onEdit: () => void;
  onDelete: () => void;
}) {
  const router = useRouter();
  const popup = usePopup();

  const {
    isEdit,
    isModified,
    stopEditing,
    triggerGlobalSave: onSave,
    spacePublishValidator,
  } = useEditCoordinatorStore();
  const { selectedTeam } = useContext(TeamContext);
  const { data: userInfo } = useSuspenseUserInfo();

  // TODO: Update to use v3 space API with proper username/pk fields
  const writePostPermission = usePermission(
    authorName ?? '',
    GroupPermission.WritePosts,
  ).data.has_permission;
  // TODO: Update permission check to use usernames instead of IDs after v3 migration
  const hasEditPermission =
    (userInfo?.username === authorName ||
      selectedTeam?.username === authorName) &&
    writePostPermission;

  const publishSpace = usePublishSpace(spaceId);
  const makeSpacePublic = useMakePublicSpace(spaceId);
  const t = useTranslations('SpaceHeader');
  // Save / Publish Flow
  // Before Publishing
  // -> If not modified, directly open Publish Modal
  // -> If modified, open Save Modal and then open Publish Modal

  // After Publishing ( Not Editable )
  // -> User Only Can Change from Private to Public

  const { isOpen, toggle, close, dropdownRef } = useDropdown();

  const handleSave = async () => {
    if (!isModified) return;
    await onSave();
  };

  const handlePublish = async (type: PublishType) => {
    try {
      if (!spacePublishValidator()) {
        return;
      }
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
      console.error('Failed to make space as public:', error);
    }
  };

  const handleDeleteClick = () => {
    popup
      .open(
        <DeleteSpacePopup
          spaceName={title || t('untitled_space')}
          onClose={() => popup.close()}
          onDelete={async () => {
            await onDelete();
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

  const openPublishModal = () => {
    if (isModified) {
      openUnsaveAlertModal(
        popup,
        handleSave,
        () => {
          openPublishSpaceModal(popup, handlePublish, t('publish_modal_title'));
        },
        t('unsave_notice_modal'),
      );
    } else {
      openPublishSpaceModal(popup, handlePublish, t('publish_modal_title'));
    }
  };

  const openPublicModal = () => {
    openMakePublicModal(popup, handleMakePublic, t('make_public_modal_title'));
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

          {isDraft && <PublishSpaceButton onClick={openPublishModal} />}
          {!isDraft && !isPublic && (
            <MakePublicButton onClick={openPublicModal} />
          )}
          {writePostPermission ? (
            <div className="relative" ref={dropdownRef}>
              <button
                onClick={toggle}
                aria-expanded={isOpen}
                aria-label="Space options menu"
                aria-haspopup="menu"
                className="w-fit p-2 rounded-md bg-neutral-800 light:bg-transparent"
                onKeyDown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    if (!isOpen) {
                      toggle();
                      setTimeout(() => {
                        const firstMenuItem =
                          dropdownRef.current?.querySelector(
                            '[role="menuitem"]:not([aria-disabled="true"])',
                          );
                        (firstMenuItem as HTMLElement)?.focus();
                      }, 0);
                    } else {
                      const firstMenuItem = dropdownRef.current?.querySelector(
                        '[role="menuitem"]:not([aria-disabled="true"])',
                      );
                      (firstMenuItem as HTMLElement)?.focus();
                    }
                  } else if (e.key === 'ArrowDown') {
                    e.preventDefault();
                    if (!isOpen) {
                      toggle();
                      setTimeout(() => {
                        const firstMenuItem =
                          dropdownRef.current?.querySelector(
                            '[role="menuitem"]:not([aria-disabled="true"])',
                          );
                        (firstMenuItem as HTMLElement)?.focus();
                      }, 0);
                    } else {
                      const firstMenuItem = dropdownRef.current?.querySelector(
                        '[role="menuitem"]:not([aria-disabled="true"])',
                      );
                      (firstMenuItem as HTMLElement)?.focus();
                    }
                  }
                }}
              >
                <Extra />
              </button>
              {isOpen && (
                <div
                  role="menu"
                  className="absolute top-full mt-2 right-0 z-50"
                  onBlur={(e) => {
                    if (!e.currentTarget.contains(e.relatedTarget as Node)) {
                      close();
                    }
                  }}
                >
                  <DropdownMenu onclose={close} ondelete={handleDeleteClick} />
                </div>
              )}
            </div>
          ) : (
            <></>
          )}
        </div>
      )}
    </div>
  );
}

export default function Header() {
  const router = useRouter();
  const { id } = useParams();
  const spaceId = Number(id);
  const { data: space } = useSpaceById(spaceId);
  // TODO: Update space API to use string feed_id in v3
  const { data: feed } = useFeedById(space.feed_id.toString());
  const author = space.author[0];

  const { isEdit, commonData, startEditing, updateCommonData } =
    useEditCoordinatorStore();

  const isDraft = space.status === SpaceStatus.Draft;
  const isPublic = space.publishing_scope === PublishingScope.Public;

  const shareSpace = useShareSpace(spaceId);
  const deleteSpace = useDeleteSpace(spaceId);
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
  const handleDelete = async () => {
    await deleteSpace.mutateAsync(spaceDeleteRequest(space.title ?? ''));
    router.push('/');
  };
  return (
    <div>
      <SuspenseWrapper>
        <div className="flex flex-col w-full gap-2.5">
          <SpaceModifySection
            title={isEdit ? (commonData?.title ?? '') : space.title}
            isDraft={isDraft}
            isPublic={isPublic}
            authorId={space.author[0]?.id}
            authorName={space.author[0]?.username}
            spaceId={spaceId}
            onEdit={handleStartEdit}
            onDelete={handleDelete}
          />
          <PostInfoSection
            likes={feed.post.likes}
            shares={feed.post.shares}
            comments={feed.post.comments}
            rewards={feed.post.rewards ?? 0}
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
