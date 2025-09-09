'use client';

import React, { Suspense } from 'react';
import Badge from '@/assets/icons/badge.svg';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { Input } from '@/components/ui/input';

import { getTimeAgo } from '@/lib/time-utils';
import {
  ErrorBoundary,
  type ErrorComponent,
} from 'next/dist/client/components/error-boundary';

import { Unlock2, Lock2 } from '@/assets/icons/security';
import { File } from '@/assets/icons/file';
import { Repost } from '@/assets/icons/arrows';
import { RoundBubble } from '@/assets/icons/chat';
import { RewardCoin } from '@/assets/icons/money-payment';
import { ThumbsUp } from '@/assets/icons/emoji';
import { Play, Expand } from '@/assets/icons';
import Loading from '@/app/loading';
import { Button } from '../ui/button';
import { useTranslations } from 'next-intl';

const ErrorComponent: ErrorComponent = ({ error, reset }) => {
  console.error('Error occurred:', error);
  return (
    <div className="w-full min-h-20 flex items-center justify-center">
      {reset && (
        <Button variant="default" onClick={reset}>
          Refresh
        </Button>
      )}
    </div>
  );
};

export function SuspenseWrapper({ children }: { children: React.ReactNode }) {
  return (
    <div className="w-full">
      <ErrorBoundary errorComponent={ErrorComponent}>
        <Suspense fallback={<Loading />}>{children}</Suspense>
      </ErrorBoundary>
    </div>
  );
}

interface TitleSectionProps {
  title: string | undefined;
  isEdit: boolean;
  setTitle: (title: string) => void;
  handleShare: () => Promise<void>;
}

export function TitleSection({
  title,
  isEdit,
  setTitle,
  handleShare,
}: TitleSectionProps) {
  const t = useTranslations('SprintSpace');
  return (
    <div>
      {isEdit ? (
        <Input
          className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-foreground text-[24px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
          value={title || ''}
          onChange={(e) => setTitle(e.target.value)}
          placeholder={t('title_hint')}
        />
      ) : (
        <div className="flex flex-row w-full justify-between items-center overflow-ellipsis">
          <div className="font-bold text-foreground text-[20px]/[30px]">
            {title}
          </div>
          <div className="cursor-pointer w-fit h-fit" onClick={handleShare}>
            <Expand />
          </div>
        </div>
      )}
    </div>
  );
}

interface PostInfoSectionProps {
  likes: number;
  shares: number;
  comments: number;
  rewards: number;
  isDraft: boolean;
  isPublic: boolean;
}

export function PostInfoSection({
  likes,
  comments,
  rewards,
  shares,
  isDraft,
  isPublic,
}: PostInfoSectionProps) {
  const t = useTranslations('SprintSpace');
  return (
    <div className="flex flex-row w-full justify-between items-center">
      <div className="flex flex-row w-fit gap-2.5 items-center">
        {/* <SpaceType /> */}
        {!isDraft ? <Onboard /> : <></>}
      </div>
      <div className="flex flex-row w-fit gap-5 [&>*>svg>*]:stroke-neutral-500 [&>*>svg]:size-5">
        <div className="flex flex-row w-fit gap-1 items-center">
          <ThumbsUp />
          <div className="font-medium text-[15px] text-foreground">{likes}</div>
        </div>

        <div className="flex flex-row w-fit gap-1 items-center">
          <RoundBubble />
          <div className="font-medium text-[15px] text-foreground">
            {comments}
          </div>
        </div>

        <div className="flex flex-row w-fit gap-1 items-center">
          <RewardCoin />
          <div className="font-medium text-[15px] text-foreground">
            {rewards}
          </div>
        </div>

        <div className="flex flex-row w-fit gap-1 items-center">
          <Repost />
          <div className="font-medium text-[15px] text-foreground">
            {shares}
          </div>
        </div>
        <div className="flex flex-row w-fit gap-1 items-center">
          {isDraft ? (
            <>
              <File />
              <div className="font-normal text-foreground text-[15px]">
                {t('draft')}
              </div>
            </>
          ) : isPublic ? (
            <>
              <Unlock2 />
              <div className="font-normal text-foreground text-[15px]">
                {t('public')}
              </div>
            </>
          ) : (
            <>
              <Lock2 />
              <div className="font-normal text-foreground text-[15px]">
                {t('private')}
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

interface ProfileSectionProps {
  profileImage: string;
  name: string;
  type: UserType;
  isCertified: boolean;
  createdAt: number;
}

export function AuthorSection({
  profileImage,
  isCertified,
  name,
  type,
  createdAt,
}: ProfileSectionProps) {
  return (
    <div className="flex flex-row justify-between items-center w-full text-sm text-c-wg-50">
      <div className="flex items-center gap-2">
        <Image
          src={profileImage || '/default-profile.png'}
          alt={name}
          width={24}
          height={24}
          className={
            type === UserType.Team
              ? 'rounded-lg object-cover object-top w-6 h-6'
              : 'rounded-full object-cover object-top w-6 h-6'
          }
        />
        <span className="text-foreground font-medium">{name}</span>
        {isCertified && <Badge />}
      </div>
      <div className="font-light text-foreground text-sm">
        {getTimeAgo(createdAt)}
      </div>
    </div>
  );
}

function Onboard() {
  const t = useTranslations('SprintSpace');
  return (
    <div className="flex flex-row items-center w-fit px-2 gap-1 border border-[#05df72] opacity-50 rounded-sm">
      <Play className="size-2.5 stroke-[#00d492] fill-[#00d492]" />
      <div className="font-semibold text-sm/[25px] text-[#05df72]">
        {t('onboard')}
      </div>
    </div>
  );
}

// FIXME: use Industry ID.
// function SpaceType() {
//   return (
//     <div className="flex flex-row w-fit h-fit px-2 bg-transparent rounded-sm border border-c-wg-70 font-semibold text-white text-xs/[25px]">
//       Crypto
//     </div>
//   );
// }
