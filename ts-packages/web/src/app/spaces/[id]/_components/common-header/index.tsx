'use client';

import React, { Suspense, useContext } from 'react';
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
import { usePopup } from '@/lib/contexts/popup-service';
import useSpaceById, {
  useMakePublicSpace,
  usePublishSpace,
  useShareSpace,
} from '@/hooks/use-space-by-id';
import useFeedById from '@/hooks/use-feed-by-id';
import { useParams, useRouter } from 'next/navigation';
import useSpaceStore from '../../store';

import {
  openModal as openPublishSpaceModal,
  PublishType,
} from './modal/publish-space';
import { openModal as openMakePublicModal } from './modal/make-public';

import { openModal as openMakePublicWithSaveModal } from './modal/make-public-with-save';
import { Button } from '@/components/ui/button';
import { PublishingScope } from '@/lib/api/models/notice';
import { ErrorBoundary } from 'next/dist/client/components/error-boundary';

function ErrorComponent() {
  return <div>Error</div>;
}

export default function CommonHeader() {
  return (
    <ErrorBoundary errorComponent={ErrorComponent}>
      <Suspense fallback={<div>Loading...</div>}>
        {/* <Header spaceId={spaceId} /> */}
        <Header />
      </Suspense>
    </ErrorBoundary>
  );
}

function Header() {
  const { id } = useParams();
  const spaceId = Number(id);

  const popup = usePopup();
  const router = useRouter();
  const {
    isEdit,
    isModified,
    saveHandler,
    startEditing,
    title: editableTitle,
    setTitle,
  } = useSpaceStore();

  const { data: space } = useSpaceById(spaceId);
  const { status, publishing_scope, title } = space;
  const {
    data: { likes, shares, comments, rewards },
  } = useFeedById(space.feed_id);
  const { data: userInfo } = useUserInfo();
  const userId = userInfo?.id ?? 0;

  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;
  const selectedTeam = teams.some((t) => t.id === authorId);

  const userType = space.author[0].user_type;
  const proposerImage = space.author[0].profile_url;
  const proposerName = space.author[0].nickname;
  const createdAt = space.created_at;

  const publishSpace = usePublishSpace(spaceId);
  const shareSpace = useShareSpace(spaceId);
  const makeSpacePublic = useMakePublicSpace(spaceId);

  const handleShareSpace = async () => {
    await shareSpace.mutateAsync();
  };
  const handlePublish = async (type: PublishType) => {
    await publishSpace.mutateAsync(type);
  };

  const handleSave = async () => {
    if (!isModified) return;
    await saveHandler();
  };

  const handleMakePublic = async () => {
    await makeSpacePublic.mutateAsync();
  };

  const handleMakePublicModal = () => {
    if (!isModified) {
      openMakePublicWithSaveModal(popup, handleMakePublic, handleSave);
    } else {
      openMakePublicModal(popup, handleMakePublic);
    }
  };

  const handlePublishSpaceModal = () => {
    openPublishSpaceModal(popup, handlePublish);
  };

  const handleGoBack = () => {
    router.back();
  };

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
              <Button
                // className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                variant="default"
                onClick={async () => {
                  await saveHandler();
                }}
              >
                <Save className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">Save</div>
              </Button>
            ) : (
              <Button
                // className="flex flex-row w-fit px-3.5 py-2 rounded-md bg-white gap-1"
                variant="default"
                onClick={() => {
                  startEditing(space);
                }}
              >
                <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">Edit</div>
              </Button>
            )}

            {status === SpaceStatus.Draft && (
              <Button variant="default" onClick={handlePublishSpaceModal}>
                <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                <div className="font-bold text-zinc-900 text-sm">Publish</div>
              </Button>
            )}
            {status === SpaceStatus.InProgress &&
              publishing_scope !== PublishingScope.Public && (
                <Button variant="default" onClick={handleMakePublicModal}>
                  <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2 w-5 h-5" />
                  <div className="font-bold text-zinc-900 text-sm">
                    Make Public
                  </div>
                </Button>
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

          {publishing_scope == PublishingScope.Public ? (
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
              value={editableTitle || ''}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="Input title."
            />
          </>
        ) : (
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-white text-[20px]/[30px]">
              {title}
            </div>

            <div
              className="cursor-pointer w-fit h-fit"
              onClick={() => handleShareSpace()}
            >
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
