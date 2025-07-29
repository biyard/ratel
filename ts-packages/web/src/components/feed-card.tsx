'use client';
import React, { useState, useEffect } from 'react';
import { Col } from './ui/col';
import { Row } from './ui/row';
import { CommentIcon, Rewards, Shares, ThumbUp } from './icons';
import { convertNumberToString } from '@/lib/number-utils';
import TimeAgo from './time-ago';
import DOMPurify from 'dompurify';
import { Button } from './ui/button';
import { useRouter } from 'next/navigation';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { route } from '@/route';
import { SpaceType } from '@/lib/api/models/spaces';
import { Extra } from './icons';
import { Trash2 } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { showSuccessToast, showErrorToast } from '@/lib/toast';

export interface FeedCardProps {
  id: number;
  industry: string;
  title: string;
  contents: string;
  author_profile_url: string;
  author_name: string;
  author_type: UserType;
  url?: string;
  created_at: number;

  likes: number;
  is_liked: boolean;
  comments: number;
  rewards: number;
  shares: number;

  space_id?: number;
  space_type?: SpaceType;
  author_id: number;
  user_id: number;
  onboard: boolean;
  onLikeClick?: (value: boolean) => void;
  refetch?: () => void;
  isLikeProcessing?: boolean;
}

export default function FeedCard(props: FeedCardProps) {
  const router = useRouter();
  const { post } = useApiCall();

  const [localLikes, setLocalLikes] = useState(props.likes);
  const [localIsLiked, setLocalIsLiked] = useState(props.is_liked);
  const [isProcessing, setIsProcessing] = useState(false);

  // Sync with props when they change
  useEffect(() => {
    setLocalLikes(props.likes);
    setLocalIsLiked(props.is_liked);
  }, [props.likes, props.is_liked]);

  const handleLike = async (value: boolean) => {
    if (isProcessing) return; // Prevent multiple clicks

    // Set processing state and optimistic update
    setIsProcessing(true);
    setLocalIsLiked(value);
    setLocalLikes((prev) => (value ? prev + 1 : prev - 1));

    try {
      await post(ratelApi.feeds.likePost(props.id), {
        like: { value },
      });

      // Success - trigger callbacks
      props.onLikeClick?.(value);
      props.refetch?.();
    } catch (error) {
      // Revert optimistic update on error
      setLocalIsLiked(props.is_liked);
      setLocalLikes(props.likes);
      console.error('Failed to update like:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <Col
      className="cursor-pointer bg-component-bg rounded-[10px]"
      onClick={() => {
        router.push(route.threadByFeedId(props.id));
      }}
    >
      <FeedBody {...props} />
      <FeedFooter
        {...props}
        likes={localLikes}
        is_liked={localIsLiked}
        isLikeProcessing={isProcessing}
        onLikeClick={handleLike}
      />
    </Col>
  );
}

export function FeedBody({
  industry,
  title,
  contents,
  author_name,
  author_profile_url,
  url,
  created_at,
  author_type,
  user_id,
  author_id,
  space_type,
  space_id,
  onboard,
  id,
}: FeedCardProps) {
  const { post: apiPost } = useApiCall();

  const handleDeletePost = async () => {
    try {
      await apiPost(ratelApi.feeds.removeDraft(id), { delete: {} });
      router.refresh();
      showSuccessToast('Post deleted successfully');
    } catch (error) {
      console.error('Failed to delete post:', error);
      showErrorToast('Failed to delete post. Please try again.');
    }
  };
  const router = useRouter();
  return (
    <Col className="pt-5 pb-2.5">
      <Row className="justify-between px-5">
        <Row>
          <IndustryTag industry={industry} />
          {onboard && <OnboradingTag />}
        </Row>
        {/* {user_id === author_id && !space_id && (
          <Button
            variant="rounded_primary"
            className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
          >
            Create a Space
          </Button>
        )} */}

        {space_id && space_type ? (
          <Button
            variant="rounded_primary"
            className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
            onClick={(e) => {
              e.stopPropagation();
              if (space_type === SpaceType.Committee) {
                router.push(route.commiteeSpaceById(space_id));
              } else {
                router.push(route.deliberationSpaceById(space_id));
              }
            }}
          >
            Join
          </Button>
        ) : (
          <div />
        )}

        {/* Delete functionality in main feed for post authors */}
        {author_id === user_id && (
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger>
              <button
                onClick={(e) => e.stopPropagation()}
                className="p-1 hover:bg-gray-700 rounded-full focus:outline-none transition-colors"
                aria-haspopup="true"
                aria-label="Post options"
              >
                <Extra className="w-6 h-6 text-gray-400" />
              </button>
            </DropdownMenuTrigger>

            <DropdownMenuContent
              align="end"
              className="w-40 bg-[#404040] border-gray-700 transition ease-out duration-100"
            >
              <DropdownMenuItem asChild>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDeletePost();
                  }}
                  className="flex items-center w-full px-4 py-2 text-sm text-red-400 hover:bg-gray-700 cursor-pointer"
                >
                  <Trash2 className="w-4 h-4" />
                  Delete
                </button>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </Row>
      <h2 className="w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-white px-5">
        {title}
      </h2>
      <Row className="justify-between items-center px-5">
        <UserBadge
          profile_url={author_profile_url}
          name={author_name}
          author_type={author_type}
        />
        <TimeAgo timestamp={created_at} />
      </Row>
      <Row className="justify-between px-5"></Row>
      <FeedContents contents={contents} url={url} />
    </Col>
  );
}

export function FeedContents({
  contents,
  url,
}: {
  contents: string;
  url?: string;
}) {
  const html =
    typeof window !== 'undefined' ? DOMPurify.sanitize(contents) : contents;

  return (
    <Col className="text-white">
      <div
        className="feed-content font-normal text-[15px]/[24px] align-middle tracking-[0.5px] text-c-wg-30 px-5"
        dangerouslySetInnerHTML={{ __html: html }}
      />

      {url && (
        <div className="px-5">
          <div className="relative w-full max-h-80 aspect-video">
            <Image
              src={url}
              alt="Uploaded image"
              fill
              className="object-cover rounded-[8px]"
              sizes="100vw"
            />
          </div>
        </div>
      )}
    </Col>
  );
}

export function IconText({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement> & { children: React.ReactNode }) {
  return (
    <Row
      className={`justify-center items-center gap-1.25 text-white font-normal text-[15px] px-4 py-5 ${className || ''}`}
      {...props}
    >
      {children}
    </Row>
  );
}

export function UserBadge({
  author_type,
  profile_url,
  name,
}: {
  author_type: UserType;
  profile_url: string;
  name: string;
}) {
  return (
    <Row className="w-fit items-center med-16 text-white">
      <Image
        src={profile_url}
        alt="User Profile"
        width={24}
        height={24}
        className={
          author_type == UserType.Team
            ? 'w-6 h-6 rounded-sm object-cover'
            : 'w-6 h-6 rounded-full object-cover'
        }
      />
      <span>{name}</span>
    </Row>
  );
}

export function IndustryTag({ industry }: { industry: string }) {
  return (
    <span className="rounded-sm border border-c-wg-70 px-2 text-xs/[25px] font-semibold align-middle uppercase">
      {industry}
    </span>
  );
}

export function OnboradingTag() {
  return (
    <span className="rounded-sm bg-primary text-white px-2 text-xs/[25px] font-semibold align-middle uppercase">
      Onboard
    </span>
  );
}

export function FeedFooter({
  likes,
  comments,
  rewards,
  shares,
  is_liked,
  onLikeClick,
  isLikeProcessing,
}: FeedCardProps) {
  return (
    <Row className="items-center justify-around border-t w-full border-neutral-800">
      <IconText
        onClick={(evt) => {
          evt.stopPropagation();
          if (!isLikeProcessing) {
            onLikeClick?.(!is_liked);
          }
        }}
        className={
          isLikeProcessing ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'
        }
      >
        <ThumbUp
          className={
            is_liked
              ? '[&>path]:fill-primary [&>path]:stroke-primary'
              : undefined
          }
        />
        {convertNumberToString(likes)}
      </IconText>
      <IconText>
        <CommentIcon />
        {convertNumberToString(comments)}
      </IconText>
      <IconText>
        <Rewards />
        {convertNumberToString(rewards)}
      </IconText>
      <IconText>
        <Shares />
        {convertNumberToString(shares)}
      </IconText>
    </Row>
  );
}
