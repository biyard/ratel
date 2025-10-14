import { useState, useEffect } from 'react';
import { Col } from './ui/col';
import { Row } from './ui/row';
import { CommentIcon, Palace, Rewards, Shares, ThumbUp } from './icons';
import { convertNumberToString } from '@/lib/number-utils';
import TimeAgo from './time-ago';
import DOMPurify from 'dompurify';
import { NavLink, useNavigate } from 'react-router';
import { UserType } from '@/lib/api/models/user';
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
import { useTranslation } from 'react-i18next';
import { usePostEditorContext } from '@/app/(social)/_components/post-editor';
import { likePost, PostResponse } from '@/lib/api/ratel/posts.v3';
import { BoosterType } from '@/types/booster-type';

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

  const p = usePostEditorContext();

  const [localLikes, setLocalLikes] = useState(post.likes);
  const [localIsLiked, setLocalIsLiked] = useState(post.liked);
  const [isProcessing, setIsProcessing] = useState(false);
  const [localShares, setLocalShares] = useState(post.shares);

  // const { t } = useTranslation('Feeds');

  const r = useRepostDraft();

  // Sync with props when they change
  useEffect(() => {
    setLocalLikes(post.likes);
    setLocalIsLiked(post.liked);
    setLocalShares(post.shares);
  }, [post.likes, post.liked, post.shares]);

  const handleLike = async (value: boolean) => {
    if (isProcessing) return; // Prevent multiple clicks

    // Set processing state and optimistic update
    setIsProcessing(true);
    setLocalIsLiked(value);
    setLocalLikes((prev) => (value ? prev + 1 : prev - 1));

    try {
      await likePost(post.pk, value);

      // Success - trigger callbacks
      props.onLikeClick?.(value);
    } catch (error) {
      // Revert optimistic update on error
      setLocalIsLiked(post.liked);
      setLocalLikes(post.likes);
      console.error('Failed to update like:', error);
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

      setLocalShares((prev) => prev + 1);
      showSuccessToast('Reposted successfully');
    } catch (error) {
      logger.error('Failed to repost:', error);
      showErrorToast('Failed to repost');
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRepostThought = () => {
    if (!r) {
      showErrorToast('Incorrectly set up repost provider');
      return;
    }
    const {
      setAuthorName,
      setAuthorProfileUrl,
      setFeedContent,
      setFeedImageUrl,
      setOriginalFeedId,
      setExpand,
      setAuthorId,
    } = r;
    setAuthorId(post.author_pk);
    setAuthorName(post.author_display_name);
    setAuthorProfileUrl(post.author_profile_url);
    setFeedContent(post.html_contents);
    setFeedImageUrl(post.urls[0] || null);
    setOriginalFeedId(post.pk);
    setExpand(true);
  };

  const handleEditPost = (postId: string) => async (e: React.MouseEvent) => {
    e?.preventDefault();
    e?.stopPropagation();
    try {
      await p?.openPostEditorPopup(postId);
    } catch (error) {
      console.error('Error editing post:', error);
    }
  };

  const href =
    post.space_pk == null
      ? route.threadByFeedId(post.pk)
      : post?.space_pk?.includes('DELIBERATION')
      ? route.deliberationSpaceById(post.space_pk)
      : route.pollSpaceByPk(post.space_pk);

  return (
    <Col className="relative rounded-[10px] bg-card-bg-secondary border border-card-enable-border">
      <NavLink to={href} className="block">
        <FeedBody post={post} onEdit={handleEditPost(post.pk)} />
      </NavLink>
      <FeedFooter
        space_id={post.space_pk}
        space_type={post.space_type}
        booster_type={post.booster}
        likes={localLikes}
        comments={post.comments}
        rewards={post.rewards || 0}
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
        <div className="flex flex-row justify-start items-center gap-2.5">
          {space_pk && space_type ? <SpaceTag /> : <></>}
        </div>

        <div>{user?.pk === author_pk && <EditButton onClick={onEdit} />}</div>
      </Row>
      <h2 className="w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-text-primary px-5">
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

  return (
    <div className="text-desc-text">
      <p
        className="feed-content font-normal text-[15px]/[24px] align-middle tracking-[0.5px] text-c-wg-30 px-5"
        dangerouslySetInnerHTML={{ __html: sanitized }}
      ></p>

      {urls.length > 0 && urls[0] !== '' && (
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
      )}
    </div>
  );
}
export function IconText({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement> & { children: React.ReactNode }) {
  return (
    <Row
      className={`inline-flex items-center gap-1.5 whitespace-nowrap leading-none text-text-primary text-[15px] px-3 py-3 ${
        className || ''
      }`}
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
    <Row className="w-fit items-center med-16 text-text-primary">
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
    <span className="flex flex-row justify-start items-center px-2 border border-label-color-border bg-label-color-bg gap-1 rounded-sm">
      <Palace className="w-3.5 h-3.5 [&>path]:stroke-label-color-text [&_g>path:nth-child(n+2)]:stroke-web-bg" />
      <div className="font-semibold text-xs/[25px] text-label-color-text">
        SPACE
      </div>
    </span>
  );
}

export function IndustryTag({ industry }: { industry: string }) {
  return (
    <span className="rounded-sm border border-label-color-border-secondary bg-label-color-bg-secondary text-label-text px-2 text-xs/[25px] font-semibold align-middle uppercase">
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
      className="rounded-full p-1.5 hover:bg-gray-100 dark:hover:bg-gray-800"
    >
      <Edit1 className="w-4 h-4" />
    </button>
  );
}

export function OnboardingTag() {
  return (
    <span className="rounded-sm bg-label-color-bg border border-label-color-border text-label-color-text px-2 text-xs/[25px] font-semibold align-middle uppercase">
      Onboard
    </span>
  );
}

export function JoinNowButton({ onClick }: { onClick: () => void }) {
  const { t } = useTranslation('Home');
  return (
    <Button
      variant="rounded_primary"
      className="cursor-pointer bg-enable-button-bg hover:bg-enable-button-bg/80 flex my-2.5 flex-row w-fit px-5 py-3 rounded-[10px] font-bold text-enable-button-text text-[15px]"
      onClick={(e) => {
        e.stopPropagation();
        e.preventDefault();
        onClick();
      }}
    >
      {t('join_now')}
    </Button>
  );
}

interface FeedFooterProps {
  space_id?: string;
  space_type?: SpaceType;
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
  space_id,
  space_type,
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
      className={`items-center justify-between border-t w-full px-5 ${
        space_id && space_type ? 'border-divider' : 'border-divider'
      } `}
    >
      {space_id && space_type ? (
        <div className="max-tablet:!hidden">
          <JoinNowButton
            onClick={() => {
              nav(route.space(space_id));
            }}
          />
        </div>
      ) : (
        <div></div>
      )}
      <div
        className={`flex flex-row ${
          space_id && space_type
            ? 'w-fit items-center max-tablet:!w-full max-tablet:!justify-between max-tablet:!items-center'
            : 'w-full justify-between items-center'
        }`}
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
                className="flex flex-row w-fit justify-center items-center gap-1.25"
                onClick={(e) => e.stopPropagation()}
              >
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
                  className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-hover transition-colors text-text-primary text-lg font-semibold"
                >
                  {isReposting ? <Loader2 /> : <Edit1 />}
                  {t('repost_with_your_thoughts')}
                </button>
              </DropdownMenuItem>

              <DropdownMenuItem asChild>
                <button
                  onClick={handleRepost}
                  disabled={isReposting}
                  className="flex items-center gap-3 w-full px-4 py-2 rounded hover:bg-hover transition-colors text-text-primary text-lg font-semibold"
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
