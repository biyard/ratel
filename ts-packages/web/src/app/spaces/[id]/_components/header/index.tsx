import React, { useContext } from 'react';
import Badge from '@/assets/icons/badge.svg';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Input } from '@/components/ui/input';
import { Space, SpaceStatus } from '@/lib/api/models/spaces';
import { ArrowLeft, Play, Save } from 'lucide-react';
import {
  Edit1,
  Unlock2,
  Lock,
  Expand,
  ThumbUp,
  Share2,
  CommentIcon,
  Rewards,
  Extra,
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { getTimeAgo } from '@/lib/time-utils';
import { usePopup } from '@/lib/contexts/popup-service';
import GoPublicPopup from '../modal/go-public';
import { Feed } from '@/lib/api/models/feeds';
import { useSpaceContext } from './provider';
import { useDropdown } from '../dropdown/dropdown-service';
import DropdownMenu from '../dropdown/dropdown-menu';
import DeleteSpacePopup from '../modal/confirm-delete';
import { useTranslations } from 'next-intl';

export default function SpaceHeader({
  space,
  feed,
}: {
  space: Space;
  feed: Feed;
}) {
  const t = useTranslations('Space');
  const context = useSpaceContext();
  if (!context)
    throw new Error('SpaceHeader must be used within SpaceHeaderProvider');

  const {
    isEdit,
    title,
    status,
    userType,
    proposerImage,
    proposerName,
    createdAt,
    handleGoBack,
    handleSave,
    handleEdit,
    handleShare,
    handlePostingSpace,
    handleUpdateTitle,
    handleDelete,
  } = context;

  const popup = usePopup();

  const handlePost = () => {
    popup
      .open(
        <GoPublicPopup
          onclose={() => popup.close()}
          onpublic={async () => {
            await handlePostingSpace();
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

  // Add this new handler function in SpaceHeader
  const handleDeleteClick = () => {
    popup
      .open(
        <DeleteSpacePopup
          spaceName={space.title || t('untitled_space')}
          onClose={() => popup.close()}
          onDelete={async () => {
            await handleDelete(); // Your existing delete handler
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

  const { data: userInfo } = useUserInfo();
  const userId = userInfo?.id ?? 0;
  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;
  const selectedTeam = teams.some((t) => t.id === authorId);

  const likes = feed.likes;
  const shares = feed.shares;
  const comments = feed.comments;
  const rewards = feed.rewards;
  const { isOpen, toggle, close, dropdownRef } = useDropdown();

  return (
    <div className="flex flex-col w-full gap-2.5 mb-10">
      <div className="flex flex-row justify-between items-center w-full">
        <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
          {isEdit ? (
            <div className="cursor-pointer w-fit h-fit" onClick={handleGoBack}>
              <ArrowLeft size={24} className="w-6 h-6 stroke-white" />
            </div>
          ) : (
            <></>
          )}
        </div>

        {(authorId === userId || selectedTeam) && (
          <div className="flex flex-row items-center gap-2 text-sm text-white">
            {isEdit ? (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={handleSave}
              >
                <Save className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('save')}
                </div>
              </button>
            ) : (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={handleEdit}
              >
                <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('edit')}
                </div>
              </button>
            )}

            {status === SpaceStatus.Draft && (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={handlePost}
              >
                <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('make_public')}
                </div>
              </button>
            )}

            <div className="relative" ref={dropdownRef}>
              <button
                onClick={toggle}
                aria-expanded={isOpen}
                aria-label="Space options menu"
                aria-haspopup="menu"
                className="w-fit p-2 rounded-md bg-neutral-800"
                onKeyDown={(e) => {
                  // if (
                  //   e.key === 'Enter' ||
                  //   e.key === ' ' ||
                  //   e.key === 'ArrowDown'
                  // ) {
                  //   e.preventDefault();
                  //   toggle();
                  //   if (!isOpen) {
                  //     setTimeout(() => {
                  //       const firstMenuItem =
                  //         dropdownRef.current?.querySelector(
                  //           '[role="menuitem"]:not([aria-disabled="true"])',
                  //         );
                  //       (firstMenuItem as HTMLElement)?.focus();
                  //     }, 0);
                  //   }
                  // }

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
                      const firstMenuItem = dropdownRef.current?.querySelector(
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
          </div>
        )}
      </div>

      <div className="flex flex-row w-full justify-between items-center">
        <div className="flex flex-row w-fit gap-2.5 items-center">
          {/* <SpaceType /> */}
          {status == SpaceStatus.InProgress ? <Onboard /> : <></>}
        </div>

        <div className="flex flex-row w-fit gap-5">
          <div className="flex flex-row w-fit gap-1 items-center">
            <ThumbUp width={20} height={20} />
            <div className="font-medium text-[15px] text-white">
              {likes ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <CommentIcon width={20} height={20} />
            <div className="font-medium text-[15px] text-white">
              {comments ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Rewards width={20} height={20} />
            <div className="font-medium text-[15px] text-white">
              {rewards ?? 0}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Share2 width={20} height={20} />
            <div className="font-medium text-[15px] text-white">
              {shares ?? 0}
            </div>
          </div>

          {status == SpaceStatus.InProgress ? (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Unlock2 className="w-5 h-5" />
              <div className="font-normal text-white text-[15px]">
                {t('public')}
              </div>
            </div>
          ) : (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Lock className="w-5 h-5" />
              <div className="font-normal text-white text-[15px]">
                {t('private')}
              </div>
            </div>
          )}
        </div>
      </div>

      <div className="w-full">
        {isEdit ? (
          <>
            <Input
              className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-white text-[24px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
              value={title}
              onChange={(e) => handleUpdateTitle(e.target.value)}
              placeholder={t('input_title')}
            />
          </>
        ) : (
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-white text-[20px]/[30px]">
              {title}
            </div>

            <div className="cursor-pointer w-fit h-fit" onClick={handleShare}>
              <Expand />
            </div>
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
                userType === UserType.Team
                  ? 'rounded-lg object-cover object-top w-6 h-6'
                  : 'rounded-full object-cover object-top w-6 h-6'
              }
            />
          ) : (
            <div className="w-6 h-6 rounded-full border border-neutral-500 bg-neutral-600" />
          )}
          <span className="text-white font-medium">{proposerName}</span>
          <Badge />
        </div>

        <div className="font-light text-white text-sm">
          {getTimeAgo(createdAt)}
        </div>
      </div>
    </div>
  );
}

function Onboard() {
  const t = useTranslations('Space');
  return (
    <div className="flex flex-row items-center w-fit px-2 gap-1 border border-[#05df72] opacity-50 rounded-sm">
      <Play className="w-2.5 h-2.5 stroke-[#00d492] fill-[#00d492]" />
      <div className="font-semibold text-sm/[25px] text-[#05df72]">
        {t('onboard')}
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
