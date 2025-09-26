import React, { useContext } from 'react';
// import Shared from '@/assets/icons/share.svg';
// import Extra from '@/assets/icons/extra.svg';
// import Bookmark from '@/assets/icons/bookmark.svg';
import Badge from '@/assets/icons/badge.svg';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Input } from '@/components/ui/input';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { ArrowLeft, Play, Save } from 'lucide-react';
import ArrowUp from '@/assets/icons/arrow-up.svg';
import Internet from '@/assets/icons/internet.svg';
import {
  Edit1,
  Unlock2,
  Lock,
  Expand,
  ThumbUp,
  Share2,
  CommentIcon,
  Rewards,
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { getTimeAgo } from '@/lib/time-utils';
import { useTranslations } from 'next-intl';
import { convertNumberToString } from '@/lib/number-utils';

export interface SpaceHeaderProps {
  title: string;
  status: SpaceStatus;
  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;
  authorId: number;
  rewards: number;
  likes: number;
  shares: number;
  comments: number;
  isLiked?: boolean;
  isEdit?: boolean;
  // Whether the space has been published privately
  isPrivatelyPublished?: boolean;
  onback: () => void;
  onsave: () => void;
  onedit: () => void;
  onpost: () => void;
  onlike: () => void;
  onshare: () => void;
  onpublishwhileediting?: () => void;
  setTitle?: (title: string) => void;
}

export default function SpaceHeader({
  title,
  status,
  userType,
  proposerImage,
  proposerName,
  createdAt,
  authorId,
  likes,
  rewards,
  shares,
  comments,
  isEdit = false,
  isPrivatelyPublished = false,
  setTitle = () => {},
  onshare = () => {},
  onback = () => {},
  onsave = () => {},
  onedit = () => {},
  onpost = () => {},
  onpublishwhileediting = () => {},
}: SpaceHeaderProps) {
  const t = useTranslations('NoticeSpace');
  const { data: userInfo } = useUserInfo();
  const userId = userInfo ? userInfo.id : 0;
  const { teams } = useContext(TeamContext);
  const selectedTeam = teams.some((t) => t.id === authorId);

  return (
    <div className="flex flex-col w-full gap-2.5 mb-10">
      <div className="flex flex-row justify-between items-center w-full">
        <div className="flex flex-row items-center gap-1 text-sm text-c-wg-50 cursor-pointer">
          {isEdit ? (
            <div className="cursor-pointer w-fit h-fit" onClick={onback}>
              <ArrowLeft size={24} className="w-6 h-6 stroke-foreground" />
            </div>
          ) : (
            <></>
          )}
        </div>

        {(authorId === userId || selectedTeam) && (
          <div className="flex flex-row items-center gap-2 text-sm text-white">
            {isEdit ? (
              <>
                <button
                  className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                  onClick={onsave}
                >
                  <Save className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                  <div className="font-bold text-zinc-900 text-sm">
                    {t('save')}
                  </div>
                </button>
                {/* Removed publish button while editing for draft spaces - it should not show */}
                {/* Show make public button while editing if space is privately published */}
                {status === SpaceStatus.InProgress && isPrivatelyPublished && (
                  <button
                    className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                    onClick={onpublishwhileediting}
                  >
                    <Internet className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                    <div className="font-bold text-zinc-900 text-sm">
                      {t('make_public')}
                    </div>
                  </button>
                )}
              </>
            ) : (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={onedit}
              >
                <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('edit')}
                </div>
              </button>
            )}

            {/* 
              Show "Publish" when space is in Draft state but not in edit mode
              Show "Go Public" when space has been published privately but not in edit mode
              Show nothing when space has been published publicly or when in edit mode
            */}
            {!isEdit && status === SpaceStatus.Draft && (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={onpost}
              >
                <ArrowUp className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  {t('publish')}
                </div>
              </button>
            )}
            {!isEdit &&
              status === SpaceStatus.InProgress &&
              isPrivatelyPublished && (
                <button
                  className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                  onClick={onpost}
                >
                  <Internet className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                  <div className="font-bold text-zinc-900 text-sm">
                    {t('go_public')}
                  </div>
                </button>
              )}
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
            <div className="font-medium text-[15px] text-text-primary">
              {convertNumberToString(likes)}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <CommentIcon width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {convertNumberToString(comments)}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Rewards width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {convertNumberToString(rewards)}
            </div>
          </div>

          <div className="flex flex-row w-fit gap-1 items-center">
            <Share2 width={20} height={20} />
            <div className="font-medium text-[15px] text-text-primary">
              {convertNumberToString(shares)}
            </div>
          </div>

          {status === SpaceStatus.Draft ? (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Lock className="w-5 h-5" />
              <div className="font-normal text-text-primary text-[15px]">
                {t('draft')}
              </div>
            </div>
          ) : status === SpaceStatus.InProgress && isPrivatelyPublished ? (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Lock className="w-5 h-5" />
              <div className="font-normal text-text-primary text-[15px]">
                {t('private')}
              </div>
            </div>
          ) : (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Unlock2 className="w-5 h-5" />
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
              onChange={(e) => setTitle(e.target.value)}
              placeholder="Input title."
            />
          </>
        ) : (
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-text-primary text-[20px]/[30px]">
              {title}
            </div>

            <div
              className="cursor-pointer w-fit h-fit"
              onClick={() => {
                onshare();
              }}
            >
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
          <span className="text-text-primary font-medium">{proposerName}</span>
          <Badge />
        </div>

        <div className="font-light text-text-primary text-sm">
          {getTimeAgo(createdAt)}
        </div>
      </div>
    </div>
  );
}

function Onboard() {
  const t = useTranslations('NoticeSpace');
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
