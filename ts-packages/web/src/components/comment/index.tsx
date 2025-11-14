import { getTimeAgo } from '@/lib/time-utils';
import { ChevronDown } from 'lucide-react';
import { BendArrowRight, CommentIcon, ThumbUp } from '@/components/icons';

import { validateString } from '@/lib/string-filter-utils';
import { ChevronDoubleDownIcon } from '@heroicons/react/20/solid';
import { useRef, useState } from 'react';
import { logger } from '@/lib/logger';
import { ReplyList } from './reply-list';
import { TFunction } from 'i18next';
import PostComment from '@/features/posts/types/post-comment';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { Button } from '../ui/button';
import { TiptapEditor } from '../text-editor';
import type { Editor } from '@tiptap/core';
import { useLocation } from 'react-router';
import { SpaceReplyList } from './space-reply-list';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';

interface CommentProps {
  spacePk?: string;
  isLoggedIn?: boolean;
  comment: PostComment;
  // TODO: Update to use v3 comment API with string IDs
  onComment?: (commentId: string, content: string) => Promise<void>;
  onLike?: (commentId: string, like: boolean) => Promise<void>;
  t: TFunction<'Thread', undefined>;
}

export function Comment({
  spacePk,
  comment,
  onComment,
  onLike,
  t,
  isLoggedIn = true,
}: CommentProps) {
  const user = useSuspenseUserInfo();
  const location = useLocation();
  const boards = /\/boards\/posts(\/|$)/.test(location.pathname);
  const [expand, setExpand] = useState(false);
  const [showReplies, setShowReplies] = useState(false);
  const rootRef = useRef<HTMLDivElement | null>(null);

  const scrollToRoot = () => {
    rootRef.current?.scrollIntoView({
      behavior: 'smooth',
      block: 'start',
    });
  };

  return (
    <div
      ref={rootRef}
      className="flex flex-col gap-3 pb-4 border-b border-b-divider"
    >
      <div className="flex flex-row gap-2 items-center">
        <img
          alt={comment.author_display_name}
          src={comment.author_profile_url}
          className="w-10 h-10 rounded-full object-cover object-top"
        />

        <div className="flex flex-col gap-[2px]">
          <div className="font-semibold text-title-text text-[15px]/[15px]">
            {comment.author_display_name}
          </div>
          <div className="font-semibold text-xs/[20px] text-time-text">
            {getTimeAgo(comment.updated_at * 1000)}
          </div>
        </div>
      </div>

      <div className="flex flex-col ml-12 gap-3">
        {/* TODO: Quote */}
        {/* {comment.quote_comment && (
          <div className="flex flex-row bg-[#282828] px-5 py-2.5 gap-2.5">
            <div className="flex flex-row space-between">
              <div className="flex flex-row gap-2 items-center">
                {comment.quote_comment?.author?.[0]?.profile_url ? (
                  <img
                    alt={comment.quote_comment?.author?.[0]?.nickname ?? ''}
                    src={comment.quote_comment?.author?.[0]?.profile_url ?? ''}
                    width={40}
                    height={40}
                    className="rounded-full object-cover object-top"
                  />
                ) : (
                  <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
                )}

                <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
                  {comment.quote_comment?.author?.[0]?.nickname ?? ''}
                </div>
              </div>
            </div>
            <LexicalHtmlViewer
              htmlString={comment.quote_comment.html_contents}
            />
          </div>
        )} */}

        {/* Content */}
        <TiptapEditor
          isMe={user.data.pk === comment.author_pk}
          content={comment.content}
          editable={false}
          showToolbar={false}
        />

        {/* Actions */}
        <div className="flex flex-row w-full justify-between items-center gap-2">
          <div className="flex flex-row gap-5">
            {/* Expand Reply Button */}
            <button
              aria-label="Expand Replies"
              className="gap-2 text-primary flex flex-row justify-center items-center disabled:cursor-not-allowed"
              disabled={comment.replies === 0}
              onClick={() => {
                setShowReplies((prev) => {
                  const next = !prev;
                  if (!prev) {
                    scrollToRoot();
                  }
                  return next;
                });
              }}
            >
              {`${comment.replies ?? 0} ${comment.replies <= 1 ? t('reply') : t('replies')}`}
              {comment.replies > 0 && (
                <ChevronDown
                  width={24}
                  height={24}
                  className="[&>path]:stroke-primary"
                />
              )}
            </button>
            {/* Reply Button */}
            {isLoggedIn && (
              <button
                aria-label="Reply to Comment"
                onClick={() => {
                  setExpand((prev) => !prev);
                  // setShowReplies(true);
                }}
                className="flex gap-2 cursor-pointer justify-center items-center text-text-primary"
              >
                <BendArrowRight
                  width={24}
                  height={24}
                  className="[&>path]:stroke-text-primary"
                />
                {t('reply')}
              </button>
            )}
          </div>
          {/* Like Button */}
          {isLoggedIn && (
            <button
              aria-label="Like Comment"
              className="flex flex-row gap-2 justify-center items-center"
              onClick={() => {
                if (onLike) {
                  onLike(comment.sk, !comment.liked);
                } else {
                  throw new Error('onLike is not set');
                }
              }}
            >
              <ThumbUp
                width={24}
                height={24}
                className={
                  comment.liked
                    ? '[&>path]:fill-primary [&>path]:stroke-primary'
                    : '[&>path]:stroke-comment-icon'
                }
              />
              <div className="font-medium text-base/[24px] text-comment-icon-text ">
                {comment.likes ?? 0}
              </div>
            </button>
          )}
        </div>
        {showReplies && comment.replies > 0 && boards && (
          <SpaceReplyList
            t={t}
            spacePk={spacePk ?? ''}
            postPk={comment.pk}
            commentSk={comment.sk}
            isLoggedIn={isLoggedIn}
            onLike={onLike}
          />
        )}
        {showReplies && comment.replies > 0 && !boards && (
          <ReplyList postPk={comment.pk} commentSk={comment.sk} />
        )}
        {expand && (
          <NewComment
            className="min-h-30"
            onClose={() => setExpand(false)}
            onSubmit={async (content) => {
              if (onComment && comment.sk) {
                await onComment(comment.sk, content);
              }
            }}
            t={t}
          />
        )}
      </div>
    </div>
  );
}

export function NewComment({
  // className,
  onClose,
  onSubmit,
  t,
}: {
  className?: string;
  onClose: () => void;
  onSubmit?: (content: string) => Promise<void>;
  t: TFunction<'Thread', undefined>;
}) {
  const [loading, setLoading] = useState(false);
  const [content, setContent] = useState('');
  const editorRef = useRef<Editor | null>(null);

  const handleEditorContainerClick = () => {
    editorRef.current?.commands.focus();
  };

  const handleSubmit = async () => {
    if (
      onSubmit &&
      !loading &&
      content.trim() !== '' &&
      validateString(content)
    ) {
      try {
        setLoading(true);
        await onSubmit?.(content);
        showSuccessToast(t('success_create_comment'));
        onClose();
      } catch (error) {
        showErrorToast(t('failed_create_comment'));
        logger.debug('Error submitting comment:', error);
      } finally {
        setLoading(false);
      }
    }
  };

  return (
    <div className="flex flex-col w-full bg-comment-box-bg border rounded-lg border-primary max-w-desktop">
      <div className="px-3 pt-3 flex flex-row justify-between items-center">
        <span className="text-sm font-medium text-text-primary">
          {t('write_comment')}
        </span>
        <button
          aria-label="Close"
          className="p-1 hover:bg-foreground/10 rounded transition-colors"
          onClick={onClose}
        >
          <ChevronDoubleDownIcon
            width={20}
            height={20}
            className="[&>path]:stroke-text-primary"
          />
        </button>
      </div>

      <div
        className="flex-1 w-full cursor-text hover:bg-foreground/5 transition-colors rounded-md"
        onClick={handleEditorContainerClick}
        role="button"
        tabIndex={-1}
        aria-label="Click to focus editor"
      >
        <TiptapEditor
          ref={editorRef}
          placeholder={t('contents_hint')}
          content={content}
          editable={true}
          showToolbar={false}
          minHeight="80px"
          onUpdate={(content) => {
            setContent(content);
          }}
          data-pw="comment-editor"
          className="border-none"
        />
      </div>

      <div className="px-3 pb-3 flex flex-row justify-end items-center gap-2 border-t border-divider pt-3">
        <Button
          id="publish-comment-button"
          aria-label="Publish"
          variant="rounded_primary"
          size="sm"
          onClick={handleSubmit}
          disabled={loading || !content.trim()}
          className="gap-2"
        >
          <CommentIcon
            width={20}
            height={20}
            className="[&>path]:stroke-black [&>line]:stroke-black"
          />
          <span>{loading ? t('publishing') : t('publish')}</span>
        </Button>
      </div>
    </div>
  );
}
