import { Trash2, Edit } from 'lucide-react';

import {
  ArrowLeft,
  BadgeIcon,
  CommentIcon,
  Extra,
  Palace,
  Shares,
  ThumbUp,
} from '@/components/icons';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { UserType } from '@/lib/api/models/user';
import { convertNumberToString } from '@/lib/number-utils';
import { getTimeAgo } from '@/lib/time-utils';

import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import { TFunction } from 'i18next';
import ThreadAdminMenu from './thread-admin-menu';

export type ThreadHeaderProps = {
  t: TFunction<'Thread', undefined>;
  feed: PostDetailResponse;
  isPostOwner: boolean;
  canDelete: boolean;
  canEdit: boolean;
  handleCreateSpace: () => Promise<void>;
  handleEditPost: () => Promise<void>;
  handleDeletePost: () => Promise<void>;
  handleLikePost: () => Promise<void>;
  goBack: () => void;
};

export default function ThreadHeader(props: ThreadHeaderProps) {
  const { feed, isPostOwner, canDelete, canEdit, handleLikePost, goBack } =
    props;

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row justify-between items-center">
        <button onClick={goBack}>
          <ArrowLeft className="[&>path]:stroke-back-icon" />
        </button>
        {isPostOwner && (canEdit || canDelete) && (
          <ThreadAdminMenu {...props} />
        )}
      </div>
      <div className="flex flex-row justify-between">
        <div className="flex items-center justify-end w-full gap-4">
          {/* Feed Stats */}
          <button
            onClick={handleLikePost}
            className={`flex items-center gap-1 transition-colors cursor-pointer disabled:cursor-not-allowed disabled:opacity-50`}
          >
            <ThumbUp
              className={
                feed.is_liked
                  ? 'size-5 [&>path]:fill-primary [&>path]:stroke-primary'
                  : 'size-5 [&>path]:stroke-icon'
              }
            />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(feed.post.likes || 0)}
            </span>
          </button>
          <div className="flex items-center gap-1">
            <CommentIcon className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(feed.post.comments || 0)}
            </span>
          </div>

          <div className="flex items-center gap-1">
            <Shares className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(feed.post.shares || 0)}
            </span>
          </div>
          {/* <div className="flex items-center gap-1">
            <Lock className="size-7 text-gray-400" />
            <span className="text-base text-white">{t('private')}</span>
          </div> */}
        </div>
      </div>

      <div>
        <h2 className="text-xl font-bold text-text-primary">
          {feed.post.title}
        </h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={feed.post.author_profile_url ?? ''}
          proposerName={feed.post.author_display_name ?? ''}
          userType={feed.post.author_type || UserType.Individual}
        />
        <div className="font-light text-text-primary text-sm/[14px]">
          {feed.post.created_at !== undefined
            ? getTimeAgo(feed.post.created_at)
            : ''}
        </div>
      </div>
    </div>
  );
}

export function ProposerProfile({
  profileUrl = '',
  proposerName = '',
  userType = UserType.Individual,
}: {
  profileUrl: string;
  proposerName: string;
  userType: UserType;
}) {
  return (
    <div className="flex flex-row w-fit gap-2 justify-between items-center">
      {profileUrl && profileUrl !== '' ? (
        <img
          src={profileUrl}
          alt={proposerName}
          className={
            userType == UserType.Team
              ? 'rounded-lg object-cover object-top w-6.25 h-6.25'
              : 'rounded-full object-cover object-top w-6.25 h-6.25'
          }
        />
      ) : (
        <div className="w-6.25 h-6.25 rounded-full bg-profile-bg" />
      )}
      <div className="font-semibold text-text-primary text-sm/[20px]">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
