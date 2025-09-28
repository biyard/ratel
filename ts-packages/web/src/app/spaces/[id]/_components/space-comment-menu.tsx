'use client';

import { useMemo, useCallback } from 'react';
import { useParams } from 'next/navigation';
import { formatDistanceToNow } from 'date-fns';
import { useTranslations } from 'next-intl';
import CommentIcon from '@/assets/icons/comment.svg';
import SearchIcon from '@/assets/icons/search.svg';
import HamburgerIcon from '@/assets/icons/hamburger2.svg';
import Check from '@/assets/icons/check-dynamic.svg';
import CheckCircle from '@/assets/icons/check-circle.svg';

// Simple Skeleton component for loading state
const Skeleton = ({ className = '' }: { className?: string }) => (
  <div className={`animate-pulse bg-gray-200 rounded ${className}`} />
);

interface CommentAuthor {
  id: number;
  nickname: string;
  profile_url: string | null;
}

interface CommentReply {
  id: number;
  html_contents: string;
  created_at: number;
  author: CommentAuthor[];
}

interface Comment {
  id: number;
  html_contents: string;
  created_at: number;
  author: CommentAuthor[];
  replies: CommentReply[];
}

interface CommentBoxProps {
  id: number;
  author: string;
  mention: string;
  text: string;
  time: string;
  replies: number;
  avatarGroup: string[];
  status: 'done' | 'pending';
  highlighted: boolean;
}

const CommentBox = ({
  id,
  author,
  mention,
  text,
  time,
  replies,
  avatarGroup,
  status,
  highlighted,
}: CommentBoxProps) => {
  const t = useTranslations('Common');

  // Truncate long text
  const truncatedText =
    text.length > 100 ? `${text.substring(0, 100)}...` : text;

  const handleActivate = useCallback(() => {
    const element = document.getElementById(`comment-${id}`);
    if (!element) return;

    // Scroll to comment
    element.scrollIntoView({ behavior: 'smooth' });

    // Add highlight class
    element.classList.add('bg-blue-50', 'transition-colors', 'duration-1000');

    // Remove highlight class after animation completes
    const timeoutId = setTimeout(() => {
      element.classList.remove('bg-blue-50');
    }, 1000);

    // Cleanup function to clear timeout if component unmounts
    return () => clearTimeout(timeoutId);
  }, [id]);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      // Handle both Space and Enter keys
      if (e.key === ' ' || e.key === 'Enter') {
        e.preventDefault();
        handleActivate();
      }
    },
    [handleActivate],
  );

  return (
    <div
      role="button"
      tabIndex={0}
      className={`p-3 rounded-lg border ${
        highlighted ? 'bg-blue-50 border-blue-200' : 'border-gray-200'
      } hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 transition-colors cursor-pointer`}
      onClick={handleActivate}
      onKeyDown={handleKeyDown}
      aria-label={`Comment by ${author}: ${truncatedText}`}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-1">
            <span className="font-medium text-sm truncate">{author}</span>
            <span className="text-xs text-gray-500 truncate">{mention}</span>
          </div>
          <p className="text-sm mt-1 text-gray-800 line-clamp-2">
            {truncatedText}
          </p>
          <div className="flex items-center justify-between mt-2">
            <span className="text-xs text-gray-500">{time}</span>
            {replies > 0 && (
              <span className="text-xs text-blue-500">
                {replies} {replies === 1 ? t('reply') : t('replies')}
              </span>
            )}
          </div>
        </div>
        {avatarGroup.length > 0 && (
          <div className="flex -space-x-2 ml-2 flex-shrink-0">
            {avatarGroup.slice(0, 3).map((avatar, idx) => (
              <div key={idx} className="relative">
                <img
                  src={avatar}
                  alt=""
                  aria-hidden="true"
                  className="w-6 h-6 rounded-full border-2 border-white bg-gray-100"
                  loading="lazy"
                  decoding="async"
                  onError={(e) => {
                    const target = e.target as HTMLImageElement;
                    // Only replace if not already set to default avatar
                    if (!target.src.endsWith('/default-avatar.png')) {
                      target.src = '/default-avatar.png';
                      // Prevent infinite loop if default avatar fails
                      target.onerror = null;
                    }
                  }}
                />
                {idx === 2 && avatarGroup.length > 3 && (
                  <div className="absolute inset-0 rounded-full bg-black bg-opacity-50 flex items-center justify-center">
                    <span className="text-white text-xs font-medium">
                      +{avatarGroup.length - 3}
                    </span>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
      <div className="mt-2 flex items-center justify-end">
        {status === 'done' ? (
          <CheckCircle className="w-4 h-4 text-green-500" />
        ) : (
          <Check className="w-4 h-4 text-gray-300" />
        )}
      </div>
    </div>
  );
};

export default function SideCommentMenu() {
  const params = useParams();
  // Will be used when implementing the API call
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const spaceId = Number(params.id);
  const t = useTranslations('Threads');

  // For now, using empty data - to replace with actual API call in production
  const { data: feed, isLoading } = {
    data: {
      comment_list: [] as Comment[],
    },
    isLoading: false,
  };

  // to be used later
  // const { data: feed, isLoading } = useSuspenseQuery<FeedData>({
  //   queryKey: ['space-feed', spaceId],
  //   queryFn: async () => {
  //     const response = await fetch(`/api/spaces/${spaceId}/feed`);
  //     if (!response.ok) throw new Error('Failed to fetch feed');
  //     return response.json();
  //   },
  // });

  // Extract and format comments from feed
  const comments = useMemo(() => {
    if (!feed?.comment_list?.length) return [];

    return feed.comment_list.map((comment) => ({
      id: comment.id,
      html_contents: comment.html_contents,
      created_at: comment.created_at,
      author: comment.author,
      replies: comment.replies || [],
    }));
  }, [feed]);

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-2.5">
      <div className="border border-card-border rounded-[10px] bg-card-bg-secondary">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="font-semibold flex items-center gap-2 text-text-primary">
            <CommentIcon /> {t('comments')}
          </h2>
          <HamburgerIcon />
        </div>

        {/* Search */}
        <div className="px-4 py-2">
          <div className="flex items-center bg-write-comment-box-bg rounded-lg px-3 py-2">
            <SearchIcon className="mr-2" />
            <input
              type="text"
              placeholder={t('searchPlaceholder')}
              aria-label={t('searchComments')}
              className="bg-write-comment-box-bg outline-none text-sm w-full placeholder:text-modal-label-text text-modal-label-text"
            />
          </div>
        </div>

        {/* Comments */}
        <div className="flex-1 overflow-y-auto space-y-2 p-4 max-h-[calc(100vh-200px)]">
          {isLoading ? (
            // Loading state
            Array.from({ length: 3 }).map((_, i) => (
              <div key={i} className="p-3 space-y-2">
                <div className="flex items-center space-x-2">
                  <Skeleton className="h-8 w-8 rounded-full" />
                  <Skeleton className="h-4 w-24" />
                </div>
                <Skeleton className="h-4 w-full" />
                <Skeleton className="h-4 w-3/4" />
              </div>
            ))
          ) : comments.length === 0 ? (
            // Empty state
            <div className="flex flex-col items-center justify-center py-8 text-center text-muted-foreground">
              <CommentIcon className="w-8 h-8 mb-2 opacity-50" />
              <p>{t('noComments')}</p>
            </div>
          ) : (
            // Comments list
            comments.map((comment) => {
              const author = comment.author[0]?.nickname || 'Anonymous';
              const mention = `@${author}`;
              const text = comment.html_contents.replace(/<[^>]*>?/gm, '');
              const time = formatDistanceToNow(
                new Date(comment.created_at * 1000),
                {
                  addSuffix: true,
                },
              );
              const repliesCount = comment.replies?.length || 0;
              const avatarGroup = [
                comment.author[0]?.profile_url || '',
                ...(comment.replies?.map(
                  (reply) => reply.author[0]?.profile_url || '',
                ) || []),
              ].filter(Boolean);
              const status =
                repliesCount > 0 ? ('done' as const) : ('pending' as const);

              return (
                <CommentBox
                  key={comment.id}
                  id={comment.id}
                  author={author}
                  mention={mention}
                  text={text}
                  time={time}
                  replies={repliesCount}
                  avatarGroup={avatarGroup}
                  status={status}
                  highlighted={false}
                />
              );
            })
          )}
        </div>
      </div>
    </div>
  );
}
