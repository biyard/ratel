'use client';

import { Suspense, useState } from 'react';
import Badge from '@/assets/icons/badge.svg?react';
import { UserType } from '@/lib/api/ratel/users.v3';

import { Input } from '@/components/ui/input';

import { getTimeAgo } from '@/lib/time-utils';

import { Unlock2, Lock2 } from '@/assets/icons/security';
import { File } from '@/assets/icons/file';
import { Repost } from '@/assets/icons/arrows';
import { RoundBubble } from '@/assets/icons/chat';
import { RewardCoin } from '@/assets/icons/money-payment';
import { ThumbsUp } from '@/assets/icons/emoji';
import Loading from '@/app/loading';
import { useTranslation } from 'react-i18next';
import { executeOnKeyStroke } from '@/utils/key-event-handle';
import { Row } from '../ui/row';
import { Edit1, Save } from '@/components/icons';

export function SuspenseWrapper({ children }: { children: React.ReactNode }) {
  return (
    <div className="w-full">
      <Suspense fallback={<Loading />}>{children}</Suspense>
    </div>
  );
}

interface TitleSectionProps {
  title: string | undefined;
  canEdit: boolean;
  setTitle: (title: string) => void;
}

export function TitleSection({
  title,
  canEdit: canEdit,
  setTitle,
}: TitleSectionProps) {
  const { t } = useTranslation('SprintSpace');
  const [editMode, setEditMode] = useState(false);
  const [internalTitle, setInternalTitle] = useState(title || '');
  const Icon = !editMode ? Edit1 : Save;
  const handleSave = () => {
    setTitle(internalTitle);
    setEditMode(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    executeOnKeyStroke(e, handleSave, () => setEditMode(false));
  };

  return (
    <Row className="items-center">
      {editMode && canEdit ? (
        <Input
          className="border-b border-transparent border-b-white! focus:border-transparent! focus:rounded-md font-bold text-text-primary text-[24px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
          value={internalTitle}
          onChange={(e) => {
            setInternalTitle(e.target.value);
          }}
          onKeyDown={onKeyDown}
          placeholder={t('title_hint')}
        />
      ) : (
        <div className="flex flex-row justify-between items-center w-full overflow-ellipsis">
          <div
            className="font-bold text-text-primary text-[28px]/[32px]"
            onClick={() => {
              if (canEdit) setEditMode(true);
            }}
          >
            {title}
          </div>
        </div>
      )}
      {canEdit && (
        <Icon
          className="cursor-pointer w-fit h-fit"
          onClick={() => {
            if (editMode) {
              handleSave();
            } else {
              setEditMode(true);
            }
          }}
        />
      )}
    </Row>
  );
}

interface PostInfoSectionProps {
  likes: number;
  shares: number;
  comments: number;
  rewards: number;
  isDraft: boolean;
  isPublic: boolean;
  hasRewards?: boolean;
}

export function PostInfoSection({
  likes,
  comments,
  rewards,
  shares,
  isDraft,
  isPublic,
  hasRewards = false,
}: PostInfoSectionProps) {
  const { t } = useTranslation('SprintSpace');
  return (
    <div className="flex flex-row items-center max-mobile:flex-col">
      <div className="flex flex-row gap-x-5 flex-nowrap [&>*>svg>*]:stroke-neutral-500 [&>*>svg]:size-5">
        <div className="flex flex-row gap-1">
          <ThumbsUp className="[&>path]:stroke-neutral-500" />
          <div className="font-medium text-[15px] text-text-primary">
            {likes}
          </div>
        </div>

        <div className="flex flex-row gap-1">
          <RoundBubble />
          <div className="font-medium text-[15px] text-text-primary">
            {comments}
          </div>
        </div>

        {hasRewards && (
          <div className="flex flex-row gap-1">
            <RewardCoin />
            <div className="font-medium text-[15px] text-text-primary">
              {rewards}
            </div>
          </div>
        )}

        <div className="flex flex-row gap-1">
          <Repost />
          <div className="font-medium text-[15px] text-text-primary">
            {shares}
          </div>
        </div>
        <div className="flex flex-row gap-1">
          {isDraft ? (
            <>
              <File />
              <div className="font-normal text-text-primary text-[15px]">
                {t('draft')}
              </div>
            </>
          ) : isPublic ? (
            <>
              <Unlock2 />
              <div className="font-normal text-text-primary text-[15px]">
                {t('public')}
              </div>
            </>
          ) : (
            <>
              <Lock2 />
              <div className="font-normal text-text-primary text-[15px]">
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
    <div className="flex flex-row gap-4 items-center w-full text-sm text-c-wg-50">
      <div className="flex gap-2 items-center">
        <img
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
        <span className="font-medium text-text-primary">{name}</span>
        {isCertified && <Badge />}
      </div>
      <div className="text-sm font-light text-text-primary">
        {getTimeAgo(createdAt)}
      </div>
    </div>
  );
}

// function Onboard() {
//   const { t } = useTranslation('SprintSpace');
//   return (
//     <div className="flex flex-row gap-1 items-center px-2 rounded-sm border opacity-50 w-fit border-[#05df72]">
//       <Play className="size-2.5 stroke-[#00d492]-[#00d492]" />
//       <div className="font-semibold text-sm/[25px] text-[#05df72]">
//         {t('onboard')}
//       </div>
//     </div>
//   );
// }

// FIXME: use Industry ID.
// function SpaceType() {
//   return (
//     <div className="flex flex-row px-2 font-semibold text-white bg-transparent rounded-sm border w-fit h-fit border-c-wg-70 text-xs/[25px]">
//       Crypto
//     </div>
//   );
// }
