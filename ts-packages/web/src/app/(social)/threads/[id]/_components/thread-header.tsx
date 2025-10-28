import {
  ArrowLeft,
  BadgeIcon,
  CommentIcon,
  Shares,
  ThumbUp,
} from '@/components/icons';

import { UserType } from '@/lib/api/ratel/users.v3';

import { convertNumberToString } from '@/lib/number-utils';
import { getTimeAgo } from '@/lib/time-utils';

import { TFunction } from 'i18next';
import ThreadAdminMenu from './thread-admin-menu';
import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';

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
    <div className="flex flex-col gap-2.5 w-full">
      <div className="flex flex-row justify-between items-center">
        <button aria-label="Go back" onClick={goBack}>
          <ArrowLeft className="[&>path]:stroke-back-icon" />
        </button>
        {(isPostOwner || canEdit || canDelete) && (
          <ThreadAdminMenu {...props} />
        )}
      </div>
      <div className="flex flex-row justify-between">
        <div className="flex gap-4 justify-end items-center w-full">
          {/* Feed Stats */}
          <button
            aria-label="Like Post"
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
          <div className="flex gap-1 items-center">
            <CommentIcon className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(feed.post.comments || 0)}
            </span>
          </div>

          <div className="flex gap-1 items-center">
            <Shares className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(feed.post.shares || 0)}
            </span>
          </div>
          {/* <div className="flex gap-1 items-center">
            <Lock className="text-gray-400 size-7" />
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
    <div className="flex flex-row gap-2 justify-between items-center w-fit">
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
        <div className="rounded-full w-6.25 h-6.25 bg-profile-bg" />
      )}
      <div className="font-semibold text-text-primary text-sm/[20px]">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
