import { getTimeAgo } from '@/lib/time-utils';
import { ChevronDown } from 'lucide-react';
import { BendArrowRight, ThumbUp } from '@/components/icons';
import LexicalHtmlViewer from '@/components/lexical/lexical-html-viewer';
import {
  LexicalHtmlEditor,
  LexicalHtmlEditorRef,
} from '../lexical/lexical-html-editor';
import { validateString } from '@/lib/string-filter-utils';
import { ChevronDoubleDownIcon } from '@heroicons/react/20/solid';
import { useEffect, useRef, useState } from 'react';
import { logger } from '@/lib/logger';
import { ReplyList } from './reply-list';
import { TFunction } from 'i18next';
import PostComment from '@/features/posts/types/post-comment';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

interface CommentProps {
  comment: PostComment;
  // TODO: Update to use v3 comment API with string IDs
  onComment?: (commentId: string, content: string) => Promise<void>;
  onLike?: (commentId: string, like: boolean) => Promise<void>;
  t: TFunction<'Thread', undefined>;
}

export function Comment({ comment, onComment, onLike, t }: CommentProps) {
  const [expand, setExpand] = useState(false);
  const [showReplies, setShowReplies] = useState(false);

  return (
    <div className="flex flex-col gap-[14px] pb-5 border-b border-b-divider">
      <div className="flex flex-row gap-2 items-center">
        <img
          alt={comment.author_display_name}
          src={comment.author_profile_url}
          className="w-16 h-16 rounded-full object-cover object-top"
        />

        <div className="flex flex-col gap-[2px]">
          <div className="font-semibold text-title-text text-[15px]/[15px]">
            {comment.author_display_name}
          </div>
          <div className="font-semibold text-xs/[20px] text-time-text">
            {getTimeAgo(comment.updated_at)}
          </div>
        </div>
      </div>

      <div className="flex flex-col mx-10 gap-5">
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
        <LexicalHtmlViewer htmlString={comment.content} />

        {/* Actions */}
        <div className="flex flex-row w-full justify-between items-center gap-2">
          <div className="flex flex-row gap-5">
            {/* Expand Reply Button */}
            <button
              aria-label="Expand Replies"
              className="gap-2 text-primary flex flex-row justify-center items-center disabled:cursor-not-allowed"
              disabled={comment.replies === 0}
              onClick={() => {
                setShowReplies(!showReplies);
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
            <button
              aria-label="Reply to Comment"
              onClick={() => {
                setExpand((prev) => !prev);
                setShowReplies(true);
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
          </div>
          {/* Like Button */}
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
        </div>
        {showReplies && comment.replies > 0 && (
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
  className = '',
  onClose,
  onSubmit,
  t,
}: {
  className?: string;
  onClose: () => void;
  onSubmit?: (content: string) => Promise<void>;
  t: TFunction<'Thread', undefined>;
}) {
  const [isLoading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(true);
  const editorRef = useRef<LexicalHtmlEditorRef>(null);
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    ref.current?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }, [ref]);

  const handleSubmit = async () => {
    const content = editorRef.current?.getContent() || '';
    if (
      onSubmit &&
      !isLoading &&
      content.trim() !== '' &&
      validateString(content)
    ) {
      setLoading(true);
      try {
        await onSubmit(content);
        editorRef.current?.clear();
        setDisabled(false);
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
    <div
      ref={ref}
      className="flex flex-col w-full justify-end items-end bg-comment-box-bg border rounded-lg border-primary max-w-desktop"
    >
      <div className="p-3 flex flex-col justify-between">
        <button
          aria-labe="Reply"
          className="p-1 flex flex-row justify-center"
          onClick={onClose}
        >
          <ChevronDoubleDownIcon
            width={24}
            height={24}
            className="[&>path]:light:stroke-write-comment-box-icon"
          />
        </button>
      </div>
      <div className="flex-1 w-full">
        <LexicalHtmlEditor
          placeholder={t('contents_hint')}
          className={className}
          ref={editorRef}
          onChange={(content) => {
            setDisabled(content.trim() === '' || !validateString(content));
          }}
          enableButton={true}
          handleSubmit={handleSubmit}
          disabled={disabled}
        />
      </div>
    </div>
  );
}
