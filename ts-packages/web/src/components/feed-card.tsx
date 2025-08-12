'use client';
import React, { useState, useEffect } from 'react';
import { Col } from './ui/col';
import { Row } from './ui/row';
import { CommentIcon, Palace, Rewards, Shares, ThumbUp } from './icons';
import { convertNumberToString } from '@/lib/number-utils';
import TimeAgo from './time-ago';
import DOMPurify from 'dompurify';
import { useRouter } from 'next/navigation';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { UserType } from '@/lib/api/models/user';
import Image from 'next/image';
import { route } from '@/route';
import { SpaceType } from '@/lib/api/models/spaces';
import { Button } from './ui/button';
import {
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenu,
  DropdownMenuItem,
} from './ui/dropdown-menu';
import { Edit1 } from './icons';
import { useRepostDraft } from '@/app/(social)/_components/create-repost';
import { showSuccessToast, showErrorToast } from './custom-toast/toast';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { Loader2 } from 'lucide-react';
import { logger } from '@/lib/logger';

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

  onRepostThought?: () => void;
  onRepost?: (e: React.MouseEvent) => void;
  onLikeClick?: (value: boolean) => void;
  refetch?: () => void;
  isLikeProcessing?: boolean;
}

export default function FeedCard(props: FeedCardProps) {
  const router = useRouter();
  const { post } = useApiCall();
  const { data: User } = useSuspenseUserInfo();

  const [localLikes, setLocalLikes] = useState(props.likes);
  const [localIsLiked, setLocalIsLiked] = useState(props.is_liked);
  const [isProcessing, setIsProcessing] = useState(false);
  const [localShares, setLocalShares] = useState(props.shares);

  const {
    setAuthorName,
    setIndustry,
    setAuthorProfileUrl,
    setFeedContent,
    setFeedImageUrl,
    setOriginalFeedId,
    setExpand,
    setAuthorId,
  } = useRepostDraft();

  // Sync with props when they change
  useEffect(() => {
    setLocalLikes(props.likes);
    setLocalIsLiked(props.is_liked);
    setLocalShares(props.shares);
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

  const handleRepost = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsProcessing(true);

    try {
      await post(ratelApi.feeds.repost(), {
        repost: {
          parent_id: props.id,
          user_id: User.id,
          html_contents: '',
          quote_feed_id: null,
        },
      });

      setLocalShares((prev) => prev + 1);
      showSuccessToast('Reposted successfully');
      props.refetch?.();
    } catch (error) {
      logger.error('Failed to repost:', error);
      showErrorToast('Failed to repost');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRepostThought = () => {
    setAuthorId(props.author_id);
    setAuthorName(props.author_name);
    setIndustry(props.industry);
    setAuthorProfileUrl(props.author_profile_url);
    setFeedContent(props.contents);
    setFeedImageUrl(props.url || null);
    setOriginalFeedId(props.id);
    setExpand(true);
  };

  return (
    <Col
      className={`cursor-pointer border rounded-[10px] ${props.space_id && props.space_type ? 'border-primary bg-primary/10' : 'border-neutral-700'}`}
      onClick={() => {
        router.push(route.threadByFeedId(props.id));
      }}
    >
      <FeedBody {...props} />
      <FeedFooter
        {...props}
        likes={localLikes}
        shares={localShares}
        is_liked={localIsLiked}
        isLikeProcessing={isProcessing}
        onLikeClick={handleLike}
        onRepostThought={handleRepostThought}
        onRepost={handleRepost}
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
  space_id,
  space_type,
  onboard,
}: FeedCardProps) {
  return (
    <Col className="pt-5 pb-2.5">
      <Row className="justify-between px-5">
        <div className="flex flex-row justify-start items-center gap-2.5">
          {space_id && space_type ? <SpaceTag /> : <></>}
          <IndustryTag industry={industry} />
          {onboard && <OnboradingTag />}
        </div>
        {/* {user_id === author_id && !space_id && (
          <Button
            variant="rounded_primary"
            className="text-[10px] font-semibold align-middle uppercase py-1 px-3"
          >
            Create a Space
          </Button>
        )} */}

        {/* {space_id && space_type ? (
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
        )} */}
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
  const c =
    typeof window !== 'undefined' ? DOMPurify.sanitize(contents) : contents;

  return (
    <Col className="text-white">
      <p
        className="feed-content font-normal text-[15px]/[24px] align-middle tracking-[0.5px] text-c-wg-30 px-5"
        dangerouslySetInnerHTML={{ __html: c }}
      ></p>

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

export function SpaceTag() {
  return (
    <span className="flex flex-row justify-start items-center px-2 border border-primary/50 bg-transparent gap-1 rounded-sm">
      <Palace className="w-3.5 h-3.5 [&_g>path:nth-child(n+2)]:stroke-primary" />
      <div className="font-semibold text-xs/[25px] text-primary">SPACE</div>
    </span>
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

export function JoinNowButton({ onClick }: { onClick: () => void }) {
  return (
    <Button
      variant="rounded_primary"
      className="cursor-pointer flex flex-row w-fit px-5 py-3 bg-primary rounded-[10px] font-bold text-[#000203] text-[15px]"
      onClick={(e) => {
        e.stopPropagation();
        e.preventDefault();
        onClick();
      }}
    >
      Join Now
    </Button>
  );
}

interface FeedFooterProps extends Omit<FeedCardProps, 'onRepostThought'> {
  onRepostThought?: () => void;
  // isLikeProcessing?: boolean;
}

export function FeedFooter({
  space_id,
  space_type,
  likes,
  comments,
  rewards,
  shares,
  is_liked,
  onLikeClick,
  isLikeProcessing,
  onRepost,
  onRepostThought,
}: FeedFooterProps) {
  const router = useRouter();

  const [isReposting, setIsReposting] = useState(false);

  const handleRepostWithThoughts = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsReposting(true);
    try {
      onRepostThought?.();
    } catch (error) {
      console.error('Failed to repost:', error);
    } finally {
      setIsReposting(false);
    }
  };

  const handleRepost = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsReposting(true);
    try {
      onRepost?.(e);
    } catch (error) {
      console.error('Failed to repost:', error);
    } finally {
      setIsReposting(false);
    }
  };

  return (
    <Row
      className={`items-center justify-between border-t w-full px-5 ${space_id && space_type ? 'border-primary/10' : 'border-neutral-800'} `}
    >
      {space_id && space_type ? (
        <JoinNowButton
          onClick={() => {
            if (space_type === SpaceType.Committee) {
              router.push(route.commiteeSpaceById(space_id ?? 0));
            } else {
              router.push(route.deliberationSpaceById(space_id ?? 0));
            }
          }}
        />
      ) : (
        <div></div>
      )}
      <div
        className={`flex flex-row ${space_id && space_type ? 'w-fit items-center' : 'w-full justify-between items-center'}`}
      >
        <IconText
          onClick={(evt) => {
            evt.stopPropagation();
            if (!isLikeProcessing) {
              onLikeClick?.(!is_liked);
            }
          }}
          className={
            isLikeProcessing
              ? 'cursor-not-allowed opacity-50'
              : 'cursor-pointer'
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
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <button onClick={(e) => e.stopPropagation()}>
                <Shares />
                {convertNumberToString(shares)}
              </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              align="end"
              className="w-84 border-0 transition ease-out duration-100 py-4 px-2"
            >
              <DropdownMenuItem asChild>
                <button
                  onClick={handleRepostWithThoughts}
                  disabled={isReposting}
                  className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700 transition-colors text-white text-lg font-semibold"
                >
                  {isReposting ? <Loader2 /> : <Edit1 />}
                  Repost with your thoughts
                </button>
              </DropdownMenuItem>

              <DropdownMenuItem asChild>
                <button
                  onClick={handleRepost}
                  disabled={isReposting}
                  className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-neutral-700 transition-colors text-white text-lg font-semibold"
                >
                  {isReposting ? <Loader2 /> : <Shares />}
                  Repost
                </button>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </IconText>
      </div>
    </Row>
  );
}
