import { ArrowLeft, Palace } from '@/components/icons';
// import type { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import { Trash2, Edit } from 'lucide-react';

import {
  BadgeIcon,
  Extra,
  ThumbUp,
  CommentIcon,
  Rewards,
  Shares,
  // Lock,
} from '@/components/icons';
import { Link } from 'react-router';
import { route } from '@/route';
import { usePopup } from '@/lib/contexts/popup-service';
import SpaceCreateModal from './space-create-modal';
import { useNavigate } from 'react-router';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useContext } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { usePostEditorContext } from '@/app/(social)/_components/post-editor';
import { convertNumberToString } from '@/lib/number-utils';
import { useTranslation } from 'react-i18next';
import { BoosterType } from '@/lib/api/models/notice';

import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { FeedStatus } from '@/lib/api/models/feeds';
import { GroupPermission } from '@/lib/api/models/group';
import { usePermission } from '@/app/(social)/_hooks/use-permission';
import { useDeletePostMutation } from '@/hooks/feeds/use-delete-post-mutation';
import { useLikePostMutation } from '@/hooks/feeds/use-like-post-mutation';

export default function Header({ postId }: { postId: string }) {
  const { t } = useTranslation('Threads');
  const popup = usePopup();
  const navigate = useNavigate();
  const { teams } = useContext(TeamContext);
  const {
    data: { post, is_liked },
  } = useFeedById(postId);

  const { data: user } = useSuspenseUserInfo();
  const username = user?.username || '';

  const p = usePostEditorContext();

  const space_id = post.space_pk;
  const is_boost = post.booster && post.booster !== BoosterType.NoBoost;
  let target = '';
  if (post.space_pk) {
    target = route.space(space_id!);
  }

  const likeMutation = useLikePostMutation();
  const deleteMutation = useDeletePostMutation(username, FeedStatus.Published);

  const isPostOwner =
    post.author_username === username ||
    teams.find((team) => team.username === post.author_username);

  const handleCreateSpace = () => {
    popup
      .open(<SpaceCreateModal feed_id={postId} />)
      .withoutBackdropClose()
      .withTitle(t('select_space_type'));
  };

  const handleDelete = async () => {
    if (!deleteMutation.isPending) {
      await deleteMutation.mutateAsync(postId);
      navigate(route.home());
    }
  };

  const handleLike = async (next: boolean) => {
    if (!likeMutation.isPending) {
      await likeMutation.mutateAsync({
        feedId: postId,
        like: next,
      });
    }
  };

  const handleEditPost = async () => {
    await p?.openPostEditorPopup(postId);
  };

  const writeGroupPermission = usePermission(
    post.author_username,
    GroupPermission.WritePosts,
  ).data.has_permission;

  const deletePostPermission = usePermission(
    post.author_username,
    GroupPermission.DeletePosts,
  ).data.has_permission;

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row justify-between items-center">
        <button onClick={() => navigate(-1)}>
          <ArrowLeft className="[&>path]:stroke-back-icon" />
        </button>
        <div className="flex items-center space-x-2.5">
          {space_id ? (
            <Link to={target ?? ''}>
              <Button
                variant="rounded_secondary"
                className="max-tablet:hidden bg-submit-button-bg text-submit-button-text"
              >
                {t('join_space')}
              </Button>
            </Link>
          ) : isPostOwner && writeGroupPermission ? (
            <>
              <Button
                variant="rounded_secondary"
                className="rounded-md max-tablet:hidden text-sm px-3 py-1.5 text-button-text bg-button-bg hover:bg-button-bg/80"
                onClick={handleEditPost}
              >
                <Edit className="!size-5" />
                {t('edit')}
              </Button>
              {/* <Button
                variant="rounded_secondary"
                className="rounded-md max-tablet:hidden text-lg px-3 py-1.5"
              >
                <UnlockPublic className="!size-5 [&>path]:stroke-black" />
                {t('make_public')}
              </Button> */}
              <Button
                variant="rounded_primary"
                onClick={handleCreateSpace}
                className="max-tablet:hidden bg-submit-button-bg hover:bg-submit-button-bg/80 text-sm px-3 py-1.5 text-submit-button-text"
              >
                <Palace className="!size-5 [&>path]:stroke-black" />
                {t('create_space')}
              </Button>
            </>
          ) : (
            <></>
          )}

          {/* 3-dot overflow menu - only shown for post owners or when there's a space to join */}
          {(isPostOwner || space_id) && deletePostPermission && (
            <DropdownMenu modal={false}>
              <DropdownMenuTrigger asChild>
                <button
                  className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
                  aria-haspopup="true"
                  aria-label="Post options"
                >
                  <Extra className="size-6 text-gray-400" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                align="end"
                className="w-40 border-gray-700 transition ease-out duration-100"
              >
                {/* Mobile-only menu items */}
                <div className="hidden max-tablet:block">
                  {space_id ? (
                    <DropdownMenuItem>
                      <Link to={target ?? ''}>
                        <button className="flex items-center w-full px-4 py-2 text-sm max-tablet:hover:bg-transparent text-text-primary hover:bg-gray-700 cursor-pointer">
                          {t('join_space')}
                        </button>
                      </Link>
                    </DropdownMenuItem>
                  ) : isPostOwner ? (
                    <>
                      <DropdownMenuItem>
                        <button
                          onClick={handleCreateSpace}
                          className="flex items-center max-tablet:justify-start gap-1 max-tablet:hover:bg-transparent w-full py-2 text-sm text-text-primary hover:bg-gray-700 cursor-pointer"
                        >
                          <Palace className="w-4 h-4 [&>path]:stroke-text-primary" />
                          {t('create_space')}
                        </button>
                      </DropdownMenuItem>
                      <DropdownMenuItem>
                        <button
                          onClick={handleEditPost}
                          className="flex items-center max-tablet:justify-start gap-1 max-tablet:hover:bg-transparent w-full py-2 text-sm text-text-primary hover:bg-gray-700 cursor-pointer"
                        >
                          <Edit className="w-4 h-4 [&>path]:stroke-text-primary" />
                          {t('edit')}
                        </button>
                      </DropdownMenuItem>
                      {/* <DropdownMenuItem>
                        <button className="flex items-center max-tablet:justify-start gap-1 max-tablet:hover:bg-transparent w-full py-2 text-sm text-text-primary hover:bg-gray-700 cursor-pointer">
                          <UnlockPublic className="w-4 h-4 [&>path]:stroke-text-primary" />
                          {t('make_public')}
                        </button>
                      </DropdownMenuItem> */}
                    </>
                  ) : null}
                </div>

                {/* Always visible delete option for post owners */}
                {isPostOwner && (
                  <DropdownMenuItem>
                    <button
                      onClick={handleDelete}
                      className="flex items-center w-full px-4 max-tablet:justify-start max-tablet:gap-1 max-tablet:hover:bg-transparent max-tablet:px-0 py-2 text-sm text-red-400 hover:bg-gray-700 cursor-pointer"
                    >
                      <Trash2 className="w-4 h-4" />
                      {t('delete')}
                    </button>
                  </DropdownMenuItem>
                )}
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </div>
      </div>
      <div className="flex flex-row justify-between">
        {/* <div>
          {post?.industry?.map((industry) => (
            <Badge
              key={industry.id}
              variant="outline"
              className="border-c-wg-70 mr-2"
              size="lg"
            >
              {industry.name}
            </Badge>
          ))}
        </div> */}
        <div className="flex items-center justify-end w-full gap-4">
          {/* Feed Stats */}
          <button
            onClick={() => handleLike(!is_liked)}
            disabled={likeMutation.isPending}
            className={`flex items-center gap-1 transition-colors cursor-pointer disabled:cursor-not-allowed disabled:opacity-50`}
          >
            <ThumbUp
              className={
                is_liked
                  ? 'size-5 [&>path]:fill-primary [&>path]:stroke-primary'
                  : 'size-5 [&>path]:stroke-icon'
              }
            />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(post.likes || 0)}
            </span>
          </button>
          <div className="flex items-center gap-1">
            <CommentIcon className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(post.comments || 0)}
            </span>
          </div>
          {is_boost ? (
            <div className="flex items-center gap-1">
              <Rewards className="size-5 [&>path]:stroke-icon" />
              <span className="text-[15px] text-text-primary">
                {convertNumberToString(post.rewards || 0)}
              </span>
            </div>
          ) : (
            <></>
          )}
          <div className="flex items-center gap-1">
            <Shares className="size-5 [&>path]:stroke-icon" />
            <span className="text-[15px] text-text-primary">
              {convertNumberToString(post.shares || 0)}
            </span>
          </div>
          {/* <div className="flex items-center gap-1">
            <Lock className="size-7 text-gray-400" />
            <span className="text-base text-white">{t('private')}</span>
          </div> */}
        </div>
      </div>

      <div>
        <h2 className="text-xl font-bold text-text-primary">{post.title}</h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={post.author_profile_url ?? ''}
          proposerName={post.author_display_name ?? ''}
          userType={post.author_type || UserType.Individual}
        />
        <div className="font-light text-text-primary text-sm/[14px]">
          {post.created_at !== undefined ? getTimeAgo(post.created_at) : ''}
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
