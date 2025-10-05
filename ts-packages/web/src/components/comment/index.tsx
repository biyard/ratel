'use client';

import { getTimeAgo } from '@/lib/time-utils';
import Image from 'next/image';
import { ChevronDown } from 'lucide-react';
import { BendArrowRight, ThumbUp } from '@/components/icons';
import { Comment as CommentType, FeedType } from '@/lib/api/models/feeds';
import LexicalHtmlViewer from '@/components/lexical/lexical-html-viewer';
import {
  LexicalHtmlEditor,
  LexicalHtmlEditorRef,
} from '../lexical/lexical-html-editor';
import { validateString } from '@/lib/string-filter-utils';
import { ChevronDoubleDownIcon } from '@heroicons/react/20/solid';
import { useEffect, useRef, useState } from 'react';
import { logger } from '@/lib/logger';
import { useTranslations } from 'next-intl';
import { useLikeFeedMutation } from '@/hooks/feeds/use-like-feed-mutation';

interface CommentProps {
  comment: CommentType;
  // TODO: Update to use v3 comment API with string IDs
  onSubmit?: (
    postId: string | number,
    commentId: string | number,
    content: string,
  ) => Promise<void>;
}

export default function Comment({ comment, onSubmit }: CommentProps) {
  const t = useTranslations('Threads');
  const [expand, setExpand] = useState(false);
  const [showReplies, setShowReplies] = useState(false);

  const { mutateAsync, isPending } = useLikeFeedMutation();

  const handleLike = async (next: boolean) => {
    if (!isPending) {
      await mutateAsync({
        next,
        feedId: comment.id,
        feedType: FeedType.Reply,
        parentId: comment.parent_id || undefined,
      });
    }
  };
  return (
    <div className="flex flex-col gap-[14px] pb-5 border-b border-b-divider">
      <div className="flex flex-row gap-2 items-center">
        {comment.author[0].profile_url ? (
          <Image
            alt={comment.author[0].nickname ?? ''}
            src={comment.author[0].profile_url ?? ''}
            width={40}
            height={40}
            className="rounded-full object-cover object-top"
          />
        ) : (
          <div className="w-[40px] h-[40px] rounded-full bg-profile-bg" />
        )}

        <div className="flex flex-col gap-[2px]">
          <div className="font-semibold text-title-text text-[15px]/[15px]">
            {comment.author[0].nickname ?? ''}
          </div>
          <div className="font-semibold text-xs/[20px] text-time-text">
            {getTimeAgo(comment.created_at)}
          </div>
        </div>
      </div>

      <div className="flex flex-col mx-10 gap-5">
        {/* Quote */}
        {comment.quote_comment && (
          <div className="flex flex-row bg-[#282828] px-5 py-2.5 gap-2.5">
            <div className="flex flex-row space-between">
              <div className="flex flex-row gap-2 items-center">
                {comment.quote_comment?.author?.[0]?.profile_url ? (
                  <Image
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
        )}

        {/* Content */}
        {comment.html_contents && (
          <LexicalHtmlViewer htmlString={comment.html_contents} />
        )}

        {/* Actions */}
        <div className="flex flex-row w-full justify-between items-center gap-2">
          <div className="flex flex-row gap-5">
            {/* Expand Reply Button */}
            <button
              className="gap-2 text-primary flex flex-row justify-center items-center disabled:cursor-not-allowed"
              disabled={comment.num_of_replies === 0}
              onClick={() => {
                setShowReplies(!showReplies);
              }}
            >
              {`${comment.num_of_replies ?? 0} ${comment.num_of_replies <= 1 ? t('reply') : t('replies')}`}
              {comment.num_of_replies > 0 && (
                <ChevronDown
                  width={24}
                  height={24}
                  className="[&>path]:stroke-primary"
                />
              )}
            </button>
            {/* Reply Button */}
            <div
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
            </div>
          </div>
          {/* Like Button */}
          <button
            className="flex flex-row gap-2 justify-center items-center"
            onClick={async () => {
              handleLike(!comment.is_liked);
            }}
          >
            <ThumbUp
              width={24}
              height={24}
              className={
                comment.is_liked
                  ? '[&>path]:fill-primary [&>path]:stroke-primary'
                  : '[&>path]:stroke-comment-icon'
              }
            />
            <div className="font-medium text-base/[24px] text-comment-icon-text ">
              {comment.num_of_likes ?? 0}
            </div>
          </button>
        </div>
        {showReplies && (
          <div className="flex flex-col gap-2.5">
            {comment.replies.map((reply) => (
              <div
                key={reply.id}
                className="flex flex-col gap-2 p-5 rounded-lg bg-reply-box border border-transparent"
              >
                <div className="flex flex-row gap-2 items-center">
                  {reply.author?.[0]?.profile_url ? (
                    <Image
                      alt={reply.author?.[0]?.nickname ?? ''}
                      src={reply.author?.[0]?.profile_url ?? ''}
                      width={40}
                      height={40}
                      className="rounded-full object-cover object-top"
                    />
                  ) : (
                    <div className="w-[40px] h-[40px] bg-profile-bg" />
                  )}

                  <div className="flex flex-col gap-[2px]">
                    <div className="font-semibold text-title-text text-[15px]/[15px]">
                      {reply.author?.[0]?.nickname ?? ''}
                    </div>
                  </div>
                </div>
                <LexicalHtmlViewer htmlString={reply.html_contents} />
              </div>
            ))}
          </div>
        )}
        {expand && (
          <NewComment
            className="min-h-30"
            onClose={() => setExpand(false)}
            onSubmit={async (content) => {
              if (onSubmit && comment.parent_id) {
                await onSubmit(comment.parent_id, comment.id, content);
              }
            }}
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
}: {
  className?: string;
  onClose: () => void;
  onSubmit?: (content: string) => Promise<void>;
}) {
  const t = useTranslations('Threads');
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
        onClose();
      } catch (error) {
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
        <button className="p-1 flex flex-row justify-center" onClick={onClose}>
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
