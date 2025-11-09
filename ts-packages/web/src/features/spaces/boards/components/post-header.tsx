import { ArrowLeft, BadgeIcon } from '@/components/icons';

import { UserType } from '@/lib/api/ratel/users.v3';

import { getTimeAgo } from '@/lib/time-utils';

import { TFunction } from 'i18next';
import PostAdminMenu from './post-admin-menu';
import { SpacePostResponse } from '../types/space-post-response';

export type PostHeaderProps = {
  t: TFunction<'SpaceBoardsEditorDetail', undefined>;
  post: SpacePostResponse;
  canDelete: boolean;
  canEdit: boolean;
  handleEditPost: () => Promise<void>;
  handleDeletePost: () => Promise<void>;
  goBack: () => void;
};

export default function PostHeader(props: PostHeaderProps) {
  const { post, canDelete, canEdit, goBack } = props;
  const categoryName = post?.category_name ?? '';

  return (
    <div className="flex flex-col gap-2.5 w-full">
      <div className="flex flex-row justify-between items-center">
        <div className="flex flex-row w-fit gap-3">
          <button aria-label="Go back" onClick={goBack}>
            <ArrowLeft className="[&>path]:stroke-back-icon" />
          </button>

          <span className="inline-flex items-center rounded-md border border-neutral-700 light:bg-neutral-300 light:text-black light:border-none bg-neutral-800 px-2 py-0.5 text-xs text-neutral-200">
            Post
          </span>
          {categoryName && (
            <span className="inline-flex items-center rounded-md border border-neutral-700 light:bg-neutral-300 light:text-black light:border-none bg-neutral-800 px-2 py-0.5 text-xs text-neutral-200">
              {categoryName}
            </span>
          )}
        </div>

        {(canEdit || canDelete) && <PostAdminMenu {...props} />}
      </div>

      <div>
        <h2 className="text-xl font-bold text-text-primary">{post?.title}</h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={post?.author_profile_url ?? ''}
          proposerName={post?.author_display_name ?? ''}
          userType={UserType.Individual}
        />
        <div className="font-light text-text-primary text-sm/[14px]">
          {post?.created_at !== undefined
            ? getTimeAgo(post?.created_at / 1000)
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
