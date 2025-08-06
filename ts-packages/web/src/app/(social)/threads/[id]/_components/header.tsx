'use client';

import { useFeedByID } from '@/app/(social)/_hooks/feed';
import { ArrowLeft, Palace } from '@/components/icons';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import { Trash2, Edit } from 'lucide-react';
import Image from 'next/image';
import {
  BadgeIcon,
  Extra,
  UnlockPublic,
  ThumbUp,
  CommentIcon,
  Rewards,
  Shares,
  Lock,
} from '@/components/icons';
import Link from 'next/link';
import { route } from '@/route';
import { usePopup } from '@/lib/contexts/popup-service';
import SpaceCreateModal from './space-create-modal';
import { SpaceType } from '@/lib/api/models/spaces';
import { useRouter } from 'next/navigation';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useContext, useState, useEffect } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { showSuccessToast, showErrorToast } from '@/lib/toast';
import { usePostDraft } from '@/app/(social)/_components/create-post';
import { convertNumberToString } from '@/lib/number-utils';

export default function Header({ post_id }: { post_id: number }) {
  const { data: post } = useFeedByID(post_id);
  const popup = usePopup();
  const router = useRouter();
  const { teams } = useContext(TeamContext);
  const user = useSuspenseUserInfo();

  const author_id = post?.author[0].id;
  const [selectedTeam, setSelectedTeam] = useState<boolean>(false);
  const { post: apiPost } = useApiCall();
  const { loadDraft } = usePostDraft();

  const space_id = post?.spaces[0]?.id;

  const user_id = user.data ? user.data.id : 0;

  useEffect(() => {
    const index = teams.findIndex((t) => t.id === author_id);
    setSelectedTeam(index !== -1);
  }, [teams, author_id]);

  let target;
  if (space_id) {
    if (post.spaces[0].space_type === SpaceType.Deliberation) {
      target = route.deliberationSpaceById(space_id);
    } else {
      target = route.commiteeSpaceById(space_id);
    }
  }
  const handleCreateSpace = () => {
    popup
      .open(<SpaceCreateModal feed_id={post_id} />)
      .withoutBackdropClose()
      .withTitle('Select a Space Type');
  };

  const handleDeletePost = async () => {
    try {
      await apiPost(ratelApi.feeds.removeDraft(post_id), { delete: {} });
      showSuccessToast('Post deleted successfully');
      router.push('/'); // Navigate to homepage after successful deletion
    } catch (error) {
      console.error('Failed to delete post:', error);
      showErrorToast('Failed to delete post. Please try again.');
      // Remain on the feed page on failure
    }
  };

  const handleEditPost = async () => {
    try {
      await loadDraft(post_id);
    } catch (error) {
      console.error('Failed to load draft for editing:', error);
      showErrorToast('Failed to load post for editing. Please try again.');
    }
  };

  // Like functionality state and handlers
  const [localLikes, setLocalLikes] = useState(post?.likes || 0);
  const [localIsLiked, setLocalIsLiked] = useState(post?.is_liked || false);
  const [isLikeProcessing, setIsLikeProcessing] = useState(false);

  useEffect(() => {
    setLocalLikes(post?.likes || 0);
    setLocalIsLiked(post?.is_liked || false);
  }, [post?.likes, post?.is_liked]);

  const handleLike = async () => {
    if (isLikeProcessing || !post) return; // Prevent multiple clicks

    const newValue = !localIsLiked;

    // Set processing state and optimistic update
    setIsLikeProcessing(true);
    setLocalIsLiked(newValue);
    setLocalLikes((prev) => (newValue ? prev + 1 : prev - 1));

    try {
      await apiPost(ratelApi.feeds.likePost(post.id), {
        like: { value: newValue },
      });

      // Success - no notification needed, visual feedback is enough
    } catch (error) {
      // Revert optimistic update on error
      setLocalIsLiked(post.is_liked || false);
      setLocalLikes(post.likes || 0);
      console.error('Failed to update like:', error);
      showErrorToast('Failed to update like. Please try again.');
    } finally {
      setIsLikeProcessing(false);
    }
  };

  const isPostOwner = author_id === user_id || selectedTeam;

  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row justify-between items-center">
        <button onClick={router.back}>
          <ArrowLeft />
        </button>
        <div className="flex items-center space-x-2.5">
          {space_id ? (
            <Link href={target ?? ''}>
              <Button variant="rounded_secondary" className="max-tablet:hidden">
                Join Space
              </Button>
            </Link>
          ) : isPostOwner ? (
            <>
              <Button
                variant="rounded_secondary"
                className="rounded-md max-tablet:hidden text-lg px-3 py-1.5"
                onClick={handleEditPost}
              >
                <Edit className="!size-5" />
                Edit
              </Button>
              <Button
                variant="rounded_secondary"
                className="rounded-md max-tablet:hidden text-lg px-3 py-1.5"
              >
                <UnlockPublic className="!size-5 [&>path]:stroke-black" />
                Make Public
              </Button>
              <Button
                variant="rounded_primary"
                onClick={handleCreateSpace}
                className="max-tablet:hidden bg-[#FCB300] hover:bg-[#FCB300]/80 text-lg px-3 py-1.5"
              >
                <Palace className="!size-5" />
                Create a Space
              </Button>
            </>
          ) : (
            <></>
          )}

          {/* 3-dot overflow menu - only shown for post owners or when there's a space to join */}
          {(isPostOwner || space_id) && (
            <DropdownMenu modal={false}>
              <DropdownMenuTrigger asChild>
                <button
                  className="p-1 hover:bg-gray-700 rounded-full focus:outline-none transition-colors"
                  aria-haspopup="true"
                  aria-label="Post options"
                >
                  <Extra className="size-6 text-gray-400" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                align="end"
                className="w-40 bg-[#404040] border-gray-700 transition ease-out duration-100"
              >
                {/* Mobile-only menu items */}
                <div className="hidden max-tablet:block">
                  {space_id ? (
                    <DropdownMenuItem asChild>
                      <Link href={target ?? ''}>
                        <button className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer">
                          Join Space
                        </button>
                      </Link>
                    </DropdownMenuItem>
                  ) : isPostOwner ? (
                    <>
                      <DropdownMenuItem asChild>
                        <button
                          onClick={handleCreateSpace}
                          className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer"
                        >
                          <Palace className="w-4 h-4 [&>path]:stroke-white" />
                          Create a Space
                        </button>
                      </DropdownMenuItem>
                      <DropdownMenuItem asChild>
                        <button
                          onClick={handleEditPost}
                          className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer"
                        >
                          <Edit className="w-4 h-4" />
                          Edit
                        </button>
                      </DropdownMenuItem>
                      <DropdownMenuItem asChild>
                        <button className="flex items-center w-full px-4 py-2 text-sm text-white hover:bg-gray-700 cursor-pointer">
                          <UnlockPublic className="w-4 h-4 [&>path]:stroke-white" />
                          Make Public
                        </button>
                      </DropdownMenuItem>
                    </>
                  ) : null}
                </div>

                {/* Always visible delete option for post owners */}
                {isPostOwner && (
                  <DropdownMenuItem asChild>
                    <button
                      onClick={handleDeletePost}
                      className="flex items-center w-full px-4 py-2 text-sm text-red-400 hover:bg-gray-700 cursor-pointer"
                    >
                      <Trash2 className="w-4 h-4" />
                      Delete
                    </button>
                  </DropdownMenuItem>
                )}
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </div>
      </div>
      <div className="flex flex-row justify-between">
        <div>
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
        </div>
        <div className="flex items-center gap-4">
          {/* Feed Stats */}
          <button
            onClick={handleLike}
            disabled={isLikeProcessing}
            className={`flex items-center gap-1 transition-colors ${
              isLikeProcessing
                ? 'cursor-not-allowed opacity-50'
                : 'cursor-pointer'
            }`}
          >
            <ThumbUp
              className={
                localIsLiked
                  ? 'size-7 [&>path]:fill-primary [&>path]:stroke-primary'
                  : 'size-7 text-gray-400'
              }
            />
            <span className="text-base text-white">
              {convertNumberToString(localLikes)}
            </span>
          </button>
          <div className="flex items-center gap-1">
            <CommentIcon className="size-7 text-gray-400" />
            <span className="text-base text-white">
              {convertNumberToString(post?.comments || 0)}
            </span>
          </div>
          <div className="flex items-center gap-1">
            <Rewards className="size-7 text-gray-400" />
            <span className="text-base text-white">
              {convertNumberToString(post?.rewards || 0)}
            </span>
          </div>
          <div className="flex items-center gap-1">
            <Shares className="size-7 text-gray-400" />
            <span className="text-base text-white">
              {convertNumberToString(post?.shares || 0)}
            </span>
          </div>
          <div className="flex items-center gap-1">
            <Lock className="size-7 text-gray-400" />
            <span className="text-base text-white">Private</span>
          </div>
        </div>
      </div>

      <div>
        <h2 className="text-2xl font-bold">{post?.title}</h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={post?.author[0]?.profile_url ?? ''}
          proposerName={post?.author[0]?.nickname ?? ''}
          userType={post?.author[0]?.user_type || UserType.Individual}
        />
        <div className="font-light text-white text-sm/[14px]">
          {post?.created_at !== undefined ? getTimeAgo(post.created_at) : ''}
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
      <Image
        src={profileUrl || '/default-profile.png'}
        alt={proposerName}
        width={20}
        height={20}
        className={
          userType == UserType.Team
            ? 'rounded-[8px] object-cover object-top w-[25px] h-[25px]'
            : 'rounded-full object-cover object-top w-[25px] h-[25px]'
        }
      />
      <div className="font-semibold text-white text-sm/[20px]">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
