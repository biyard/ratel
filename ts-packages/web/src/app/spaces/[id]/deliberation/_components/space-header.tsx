import React, { useContext } from 'react';
import Badge from '@/assets/icons/badge.svg';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Input } from '@/components/ui/input';
import { SpaceStatus } from '@/lib/api/models/spaces';
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
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { getTimeAgo } from '@/lib/time-utils';
import {
  useDeliberationFeed,
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from '../provider.client';
import { usePopup } from '@/lib/contexts/popup-service';
import GoPublicPopup from './modal/go_public';

export default function SpaceHeader() {
  const popup = usePopup();
  const space = useDeliberationSpace();
  const feed = useDeliberationFeed(space.feed_id);
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
  } = useDeliberationSpaceContext();

  const handlePost = () => {
    popup
      .open(
        <GoPublicPopup
          onclose={() => {
            popup.close();
          }}
          onpublic={async () => {
            await handlePostingSpace();
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

  const authorId = space?.author[0].id;
  const likes = feed.likes;
  const shares = feed.shares;
  const comments = feed.comments;
  const rewards = feed.rewards;

  const { data: userInfo } = useUserInfo();
  const userId = userInfo ? userInfo.id : 0;
  const { teams } = useContext(TeamContext);
  const selectedTeam = teams.some((t) => t.id === authorId);

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
                <div className="font-bold text-zinc-900 text-sm">Save</div>
              </button>
            ) : (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={handleEdit}
              >
                <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">Edit</div>
              </button>
            )}

            {status === SpaceStatus.Draft && (
              <button
                className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                onClick={handlePost}
              >
                <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">
                  Make Public
                </div>
              </button>
            )}
          </div>
        )}
      </div>

      <div className="flex flex-row w-full justify-between items-center">
        <div className="flex flex-row w-fit gap-2.5 items-center">
          <SpaceType />
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
              <div className="font-normal text-white text-[15px]">Public</div>
            </div>
          ) : (
            <div className="flex flex-row w-fit gap-1 items-center">
              <Lock className="w-5 h-5" />
              <div className="font-normal text-white text-[15px]">Private</div>
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
              placeholder="Input title."
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
          <Image
            src={proposerImage || '/default-profile.png'}
            alt={proposerName}
            width={24}
            height={24}
            className={
              userType === UserType.Team
                ? 'rounded-lg object-cover object-top w-6 h-6'
                : 'rounded-full object-cover object-top w-6 h-6'
            }
          />
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
  return (
    <div className="flex flex-row items-center w-fit px-2 gap-1 border border-[#05df72] opacity-50 rounded-sm">
      <Play className="w-2.5 h-2.5 stroke-[#00d492] fill-[#00d492]" />
      <div className="font-semibold text-sm/[25px] text-[#05df72]">ONBOARD</div>
    </div>
  );
}

function SpaceType() {
  return (
    <div className="flex flex-row w-fit h-fit px-2 bg-transparent rounded-sm border border-c-wg-70 font-semibold text-white text-xs/[25px]">
      Crypto
    </div>
  );
}
