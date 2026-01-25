import { useState, useEffect } from 'react';
import { Col } from './ui/col';
import { Row } from './ui/row';
import { CommentIcon, Palace, Rewards, Shares, ThumbUp } from './icons';
import { convertNumberToString } from '@/lib/number-utils';
import TimeAgo from './time-ago';
import DOMPurify from 'dompurify';
import { NavLink, useNavigate } from 'react-router';
import { UserType } from '@/lib/api/ratel/users.v3';

import { route } from '@/route';
import { Button } from './ui/button';
import {
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenu,
  DropdownMenuItem,
} from './ui/dropdown-menu';
import { Edit1 } from './icons';
import { showSuccessToast, showErrorToast } from './custom-toast/toast';
import { useSuspenseUserInfo, useUserInfo } from '@/hooks/use-user-info';
import { Loader2 } from 'lucide-react';
import { logger } from '@/lib/logger';
import { useTranslation } from 'react-i18next';
import { BoosterType } from '@/features/spaces/types/booster-type';
import PostResponse from '@/features/posts/dto/list-post-response';
import { useLikePostMutation } from '@/features/posts/hooks/use-like-post-mutation';
import { SpaceType } from '@/features/spaces/types/space-type';
import { PostEditor } from '@/features/posts/components/post-editor';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginModal } from './popup/login-popup';

export interface FeedCardProps {
  post: PostResponse;

  onRepostThought?: () => void;
  onRepost?: (e: React.MouseEvent) => void;
  onLikeClick?: (value: boolean) => void;
  isLikeProcessing?: boolean;
  onEdit?: (e: React.MouseEvent) => void | Promise<void>;
}

export default function FeedCard(props: FeedCardProps) {
  const { post } = props;

  const [isProcessing, setIsProcessing] = useState(false);
  // Local state for optimistic updates
  const [optimisticLiked, setOptimisticLiked] = useState(post.liked);
  const [optimisticLikes, setOptimisticLikes] = useState(post.likes);

  const { data: user } = useUserInfo();
  const isLoggedIn = user !== null;
  const popup = usePopup();
  // const { t } = useTranslation('Feeds');

  const likePost = useLikePostMutation().mutateAsync;

  // Sync local state with props when they change (from cache updates)
  useEffect(() => {
    setOptimisticLiked(post.liked);
    setOptimisticLikes(post.likes);
  }, [post.liked, post.likes]);

  const handleLike = async (value: boolean) => {
    if (!isLoggedIn) {
      popup
        .open(<LoginModal />)
        .withTitle('Join the Movement')
        .withoutBackdropClose();
      return;
    }
    if (isProcessing) return; // Prevent multiple clicks

    // Optimistic update - update UI immediately
    const previousLiked = optimisticLiked;
    const previousLikes = optimisticLikes;
    const delta = value ? 1 : -1;

    setOptimisticLiked(value);
    setOptimisticLikes(Math.max(0, optimisticLikes + delta));
    setIsProcessing(true);

    try {
      await likePost({
        feedId: post.pk,
        like: value,
      });

      // Success - trigger callbacks
      props.onLikeClick?.(value);
    } catch (error) {
      // Rollback optimistic update on error
      console.error('Failed to update like:', error);
      setOptimisticLiked(previousLiked);
      setOptimisticLikes(previousLikes);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRepost = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsProcessing(true);

    try {
      // TODO: Implement repost with v3
      /* await post(ratelApi.feeds.repost(), {
       *   repost: {
       *     parent_id: props.id,
       *     user_id: User.id,
       *     html_contents: '',
       *     quote_feed_id: null,
       *   },
       * }); */

      showSuccessToast('Reposted successfully');
    } catch (error) {
      logger.error('Failed to repost:', error);
      showErrorToast('Failed to repost');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRepostThought = () => {
    console.log('Move to repost page - postId:', post.pk);
  };

  const handleEditPost = (postId: string) => async (e: React.MouseEvent) => {
    console.log('Move to post edit page - postId:', postId);
    // e?.preventDefault();
    // e?.stopPropagation();
    // try {
    //   await p?.openPostEditorPopup(postId);
    // } catch (error) {
    //   console.error('Error editing post:', error);
    // }
  };

  const href = post.space_pk
    ? route.spaceByType(post.space_type, post.space_pk)
    : route.threadByFeedId(post.pk);

  return (
    <Col className="relative border rounded-[10px] bg-card-bg-secondary border-card-enable-border">
      <NavLink to={href} className="block">
        <FeedBody post={post} onEdit={handleEditPost(post.pk)} />
      </NavLink>
      <FeedFooter
        href={href}
        booster_type={post.booster}
        likes={optimisticLikes}
        comments={post.comments}
        rewards={post.rewards || 0}
        shares={post.shares}
        is_liked={optimisticLiked}
        isLikeProcessing={isProcessing}
        onLikeClick={handleLike}
        onRepostThought={handleRepostThought}
        onRepost={handleRepost}
      />
    </Col>
  );
}

interface FeedBodyProps {
  post: PostResponse;
  onEdit?: (e: React.MouseEvent) => void | Promise<void>;
}

export function FeedBody({ post, onEdit = () => {} }: FeedBodyProps) {
  const { data: user } = useSuspenseUserInfo();
  const {
    title,
    html_contents,
    urls,
    author_display_name: author_name,
    author_profile_url,
    author_type: author_user_type,
    author_pk,
    created_at,
    space_pk,
    space_type,
  } = post;

  return (
    <Col className="pt-5 pb-2.5">
      <Row className="justify-between px-5">
        <div className="flex flex-row gap-2.5 justify-start items-center">
          {space_pk && space_type ? <SpaceTag /> : <></>}
        </div>

        <div>{user?.pk === author_pk && <EditButton onClick={onEdit} />}</div>
      </Row>
      <h2 className="px-5 w-full font-bold align-middle line-clamp-2 text-xl/[25px] tracking-[0.5px] text-text-primary">
        {title}
      </h2>
      <Row className="justify-between items-center px-5">
        <UserBadge
          profile_url={author_profile_url}
          name={author_name}
          author_type={author_user_type}
        />
        <TimeAgo timestamp={created_at} />
      </Row>
      <Row className="justify-between px-5"></Row>
      <FeedContents contents={html_contents} urls={urls} />
    </Col>
  );
}

export function FeedContents({
  contents,
  urls,
}: {
  contents: string;
  urls: string[];
}) {
  const [sanitized, setSanitized] = useState<string>('');

  useEffect(() => {
    setSanitized(DOMPurify.sanitize(contents));
  }, [contents]);

  const url = urls.length > 0 ? urls[0] : null;
  return (
    <div className="break-all text-desc-text">
      <PostEditor
        editable={false}
        showToolbar={false}
        content={sanitized}
        className="border-none"
        minHeight="50px"
        maxHeight="200px"
        url={url}
      />
      {/* <p
        className="px-5 font-normal align-middle feed-content text-[15px]/[24px] tracking-[0.5px] text-c-wg-30"
        dangerouslySetInnerHTML={{ __html: sanitized }}
      ></p> */}

      {/* {urls.length > 0 && urls[0] !== '' && (
        <div className="px-5">
          <div className="relative w-full max-h-80 aspect-video">
            <img
              src={urls[0]}
              alt="Uploaded image"
              className="object-cover w-full rounded-[8px]"
              sizes="100vw"
            />
          </div>
        </div>
      )} */}
    </div>
  );
  // return (
  //   <div className="text-desc-text">
  //     <TiptapEditor
  //       editable={false}
  //       showToolbar={false}
  //       content={sanitized}
  //       className="border-none"
  //       minHeight="50px"
  //       maxHeight="200px"
  //     />

  //     {urls.length > 0 && urls[0] !== '' && (
  //       <div className="px-5">
  //         <div className="relative w-full max-h-80 aspect-video">
  //           <img
  //             src={urls[0]}
  //             alt="Uploaded image"
  //             className="object-cover w-full rounded-[8px]"
  //             sizes="100vw"
  //           />
  //         </div>
  //       </div>
  //     )}
  //   </div>
  // );
}
export function IconText({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement> & { children: React.ReactNode }) {
  return (
    <Row
      className={`inline-flex items-center gap-1.5 whitespace-nowrap leading-none text-text-primary text-[15px] px-3 py-3 ${className || ''}`}
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
    <Row className="items-center w-fit med-16 text-text-primary">
      {profile_url != '' ? (
        <img
          src={profile_url}
          alt="User Profile"
          className={
            author_type == UserType.Team
              ? 'w-6 h-6 rounded-sm object-cover'
              : 'w-6 h-6 rounded-full object-cover'
          }
        />
      ) : (
        <></>
      )}
      <span>{name}</span>
    </Row>
  );
}

export function SpaceTag() {
  return (
    <span className="flex flex-row gap-1 justify-start items-center px-2 rounded-sm border border-label-color-border bg-label-color-bg">
      <Palace className="w-3.5 h-3.5 [&>path]:stroke-label-color-text [&_g>path:nth-child(n+2)]:stroke-web-bg" />
      <div className="font-semibold text-xs/[25px] text-label-color-text">
        SPACE
      </div>
    </span>
  );
}

export function IndustryTag({ industry }: { industry: string }) {
  return (
    <span className="px-2 font-semibold uppercase align-middle rounded-sm border border-label-color-border-secondary bg-label-color-bg-secondary text-label-text text-xs/[25px]">
      {industry}
    </span>
  );
}

interface EditButtonProps {
  onClick?: (e: React.MouseEvent) => void;
}

export function EditButton({ onClick }: EditButtonProps) {
  return (
    <button
      onClick={(e) => {
        e.stopPropagation();
        e.preventDefault();
        onClick?.(e);
      }}
      className="p-1.5 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"
    >
      <Edit1 className="w-4 h-4" />
    </button>
  );
}

export function OnboardingTag() {
  return (
    <span className="px-2 font-semibold uppercase align-middle rounded-sm border bg-label-color-bg border-label-color-border text-label-color-text text-xs/[25px]">
      Onboard
    </span>
  );
}


interface FeedFooterProps {
  href: string;
  booster_type?: BoosterType;
  likes: number;
  comments: number;
  rewards: number;
  shares: number;
  is_liked: boolean;
  onLikeClick?: (value: boolean) => void;
  isLikeProcessing?: boolean;
  onRepost?: (e: React.MouseEvent) => void;
  onRepostThought?: () => void;
}

export function FeedFooter({
  href,
  booster_type,
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
  const nav = useNavigate();
  const { t } = useTranslation('Home');

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
      className="items-center justify-between border-t w-full px-5 border-divider"
    >
      <div className="flex flex-row w-full justify-between items-center">
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
        <IconText
          onClick={(evt) => {
            evt.stopPropagation();
            nav(`${href}#comments`);
          }}
          className="cursor-pointer"
        >
          <CommentIcon />
          {convertNumberToString(comments)}
        </IconText>
        {booster_type && (
          <IconText>
            <Rewards />
            {convertNumberToString(rewards)}
          </IconText>
        )}

        <IconText>
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <button
                className="flex flex-row justify-center items-center w-fit gap-1.25"
                onClick={(e) => e.stopPropagation()}
              >
                <Shares />
                {convertNumberToString(shares)}
              </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              align="end"
              className="py-4 px-2 border-0 transition duration-100 ease-out w-84"
            >
              <DropdownMenuItem asChild>
                <button
                  onClick={handleRepostWithThoughts}
                  disabled={isReposting}
                  className="flex gap-3 items-center py-2 px-4 w-full text-lg font-semibold rounded transition-colors text-text-primary hover:bg-hover"
                >
                  {isReposting ? <Loader2 /> : <Edit1 />}
                  {t('repost_with_your_thoughts')}
                </button>
              </DropdownMenuItem>

              <DropdownMenuItem asChild>
                <button
                  onClick={handleRepost}
                  disabled={isReposting}
                  className="flex gap-3 items-center py-2 px-4 w-full text-lg font-semibold rounded transition-colors text-text-primary hover:bg-hover"
                >
                  {isReposting ? <Loader2 /> : <Shares />}
                  {t('repost')}
                </button>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </IconText>
      </div>
    </Row>
  );
}
