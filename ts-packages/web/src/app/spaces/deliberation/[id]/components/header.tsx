import React from 'react';
import Badge from '@/assets/icons/badge.svg';
import Image from 'next/image';
import { Input } from '@/components/ui/input';
import { ArrowLeft, Save } from 'lucide-react';
import ArrowUp from '@/assets/icons/arrow-up.svg';
import {
  Edit1,
  Unlock2,
  Lock,
  //   Expand,
  ThumbUp,
  Share2,
  CommentIcon,
  Rewards,
  Internet,
  Extra,
} from '@/components/icons';
// import { useUserInfo } from '@/app/(social)/_hooks/user';
import { getTimeAgo } from '@/lib/time-utils';
import { usePopup } from '@/lib/contexts/popup-service';
import { FeedV2 } from '@/lib/api/models/feeds';
import { useTranslations } from 'next-intl';
import { PublishingScope } from '@/lib/api/models/notice';
import { useDeliberationSpaceByIdContext } from '../providers.client';
import PublishForm from '@/app/spaces/[id]/notice/_components/modal/publish-form';
import {
  DeliberationSpace,
  SpacePublishState,
} from '@/lib/api/ratel/spaces/deliberation-spaces.v3';
import { useUserInfo } from '@/hooks/use-user-info';
import GoPublicModal from '@/app/spaces/[id]/notice/_components/modal/go-public-modal';
import { useDropdown } from '@/app/spaces/[id]/_components/dropdown/dropdown-service';
import DropdownMenu from '@/app/spaces/[id]/_components/dropdown/dropdown-menu';
import DeleteSpacePopup from '@/app/spaces/[id]/_components/modal/confirm-delete';
// import GoPublicModal from '@/app/spaces/[id]/notice/_components/modal/go-public-modal';
// import DeleteSpacePopup from '@/app/spaces/[id]/_components/modal/confirm-delete';
// import { useDropdown } from '@/app/spaces/[id]/_components/dropdown/dropdown-service';

export default function SpaceHeader({
  space,
  feed,
}: {
  space: DeliberationSpace;
  feed: FeedV2;
}) {
  const { data: user } = useUserInfo();
  const t = useTranslations('Space');

  const {
    isEdit,
    isPrivatelyPublished,
    title,
    proposerImage,
    proposerName,
    createdAt,
    handleGoBack,
    handleSave,
    handleEdit,
    handlePublishWithScope,
    handleUpdateTitle,
    handleDelete,
  } = useDeliberationSpaceByIdContext();

  const popup = usePopup();

  const handlePost = async () => {
    if (space.publish_state === SpacePublishState.Draft.toUpperCase()) {
      popup
        .open(
          <PublishForm
            onPublish={async (scope: PublishingScope) => {
              popup.close();
              await handlePublishWithScope(scope);
            }}
            // currentScope={space.publishing_scope}
            currentScope={PublishingScope.Private}
          />,
        )
        .withoutBackdropClose();
    } else {
      popup
        .open(
          <GoPublicModal
            onCancel={() => popup.close()}
            onGoPublic={async () => {
              popup.close();
              await handlePublishWithScope(PublishingScope.Public);
            }}
          />,
        )
        .withoutBackdropClose();
    }
  };

  // Add this new handler function in SpaceHeader
  const handleDeleteClick = () => {
    popup
      .open(
        <DeleteSpacePopup
          spaceName={feed.title || t('untitled_space')}
          onClose={() => popup.close()}
          onDelete={async () => {
            await handleDelete(); // Your existing delete handler
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

  //   const { data: userInfo } = useUserInfo();
  //   const userId = userInfo?.id ?? 0;
  //   const { teams } = useContext(TeamContext);
  //   const authorId = space?.author.id;
  //   const selectedTeam = teams.some((t) => t.id === authorId);
  const authorId = space?.user_pk;
  const publish = space?.publish_state;

  const likes = feed.likes;
  const shares = feed.shares;
  const comments = feed.comments;
  const rewards = feed.rewards;
  const { isOpen, toggle, close, dropdownRef } = useDropdown();

  //   const writePostPermission = usePermission(
  //     space.author_pk,
  //     GroupPermission.WritePosts,
  //   ).data.has_permission;

  return (
    <div className="flex flex-col w-full gap-2.5 mb-10">
      <div className="flex flex-row justify-between items-center w-full">
        <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
          {isEdit ? (
            <div className="cursor-pointer w-fit h-fit" onClick={handleGoBack}>
              <ArrowLeft size={24} className="w-6 h-6 stroke-back-icon" />
            </div>
          ) : (
            <></>
          )}
        </div>

        {authorId === user?.pk && (
          <div className="flex flex-row items-center gap-2 text-sm text-white">
            {isEdit ? (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white light:border-[#e5e5e5] gap-1"
                onClick={handleSave}
              >
                <Save className="stroke-neutral-600 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('save')}
                </div>
              </button>
            ) : (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white light:border-[#e5e5e5] gap-1"
                onClick={handleEdit}
              >
                <Edit1 className="stroke-neutral-600 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('edit')}
                </div>
              </button>
            )}

            {!isEdit && publish === SpacePublishState.Draft.toUpperCase() && (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={async () => {
                  await handlePost();
                }}
              >
                <ArrowUp className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('publish')}
                </div>
              </button>
            )}
            {!isEdit &&
              publish !== SpacePublishState.Draft.toUpperCase() &&
              isPrivatelyPublished && (
                <button
                  className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                  onClick={async () => {
                    await handlePost();
                  }}
                >
                  <Internet className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                  <div className="font-bold text-zinc-900 text-sm">
                    {t('go_public')}
                  </div>
                </button>
              )}

            {authorId === user?.pk ? (
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
                        toggle(); // open
                        setTimeout(() => {
                          const firstMenuItem =
                            dropdownRef.current?.querySelector(
                              '[role="menuitem"]:not([aria-disabled="true"])',
                            );
                          (firstMenuItem as HTMLElement)?.focus();
                        }, 0);
                      } else {
                        // move focus into menu if already open
                        const firstMenuItem =
                          dropdownRef.current?.querySelector(
                            '[role="menuitem"]:not([aria-disabled="true"])',
                          );
                        (firstMenuItem as HTMLElement)?.focus();
                      }
                    } else if (e.key === 'ArrowDown') {
                      e.preventDefault();
                      if (!isOpen) {
                        toggle(); // open
                        setTimeout(() => {
                          const firstMenuItem =
                            dropdownRef.current?.querySelector(
                              '[role="menuitem"]:not([aria-disabled="true"])',
                            );
                          (firstMenuItem as HTMLElement)?.focus();
                        }, 0);
                      } else {
                        const firstMenuItem =
                          dropdownRef.current?.querySelector(
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
                    <DropdownMenu
                      onclose={close}
                      ondelete={handleDeleteClick}
                    />
                  </div>
                )}
              </div>
            ) : (
              <></>
            )}
          </div>
        )}
      </div>

      <div className="flex flex-row w-full justify-between max-tablet:justify-end items-center">
        <div className="flex flex-row w-fit gap-2.5 items-center max-tablet:hidden">
          {/* <SpaceType /> */}
        </div>

        <div className="flex flex-row w-fit gap-5">
          <div className="flex flex-row w-fit gap-1 items-center">
            <ThumbUp width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {likes ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <CommentIcon width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {comments.length ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Rewards width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {rewards ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Share2 width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {shares ?? 0}
            </div>
          </div>

          {isPrivatelyPublished ? (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Unlock2 className="w-5 h-5" />
              <div className="font-normal text-text-primary text-[15px]">
                {t('private')}
              </div>
            </div>
          ) : (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Lock className="w-5 h-5" />
              <div className="font-normal text-text-primary text-[15px]">
                {t('public')}
              </div>
            </div>
          )}
        </div>
      </div>

      <div className="w-full">
        {isEdit ? (
          <>
            <Input
              className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-text-primary text-[24px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
              value={title}
              onChange={(e) => handleUpdateTitle(e.target.value)}
              placeholder={t('input_title')}
            />
          </>
        ) : (
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-text-primary text-[20px]/[30px]">
              {title}
            </div>

            {/* <div className="cursor-pointer w-fit h-fit" onClick={handleShare}>
              <Expand />
            </div> */}
          </div>
        )}
      </div>

      <div className="flex flex-row justify-between items-center w-full text-sm text-c-wg-50">
        <div className="flex items-center gap-2">
          {proposerImage && proposerImage !== '' ? (
            <Image
              src={proposerImage}
              alt={proposerName}
              width={24}
              height={24}
              className={
                'rounded-full object-cover object-top w-6 h-6'
                // userType === UserType.Team
                //   ? 'rounded-lg object-cover object-top w-6 h-6'
                //   : 'rounded-full object-cover object-top w-6 h-6'
              }
            />
          ) : (
            <div className="w-6 h-6 rounded-full bg-profile-bg" />
          )}
          <span className="text-text-primary font-medium">{proposerName}</span>
          <Badge />
        </div>

        <div className="font-light text-text-primary text-sm">
          {getTimeAgo(createdAt / 1000)}
        </div>
      </div>
    </div>
  );
}

// function SpaceType() {
//   return (
//     <div className="flex flex-row w-fit h-fit px-2 bg-transparent rounded-sm border border-c-wg-70 font-semibold text-white text-xs/[25px]">
//       Crypto
//     </div>
//   );
// }
