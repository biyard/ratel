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
import PostAdminMenu from '@/features/spaces/boards/components/post-admin-menu';

interface CommentProps {
  spacePk?: string;
  isLoggedIn?: boolean;
  canDelete?: boolean;
  canEdit?: boolean;
  comment: PostComment;
  onComment?: (commentId: string, content: string) => Promise<void>;
  onLike?: (commentId: string, like: boolean) => Promise<void>;
  onDelete?: (commentSk: string) => Promise<void>;
  onUpdate?: (commentSk: string, content: string) => Promise<void>;
  t: TFunction<'Thread', undefined>;
}

export function Comment({
  spacePk,
  comment,
  onComment,
  onDelete,
  onLike,
  onUpdate,
  t,
  isLoggedIn = true,
  canDelete = false,
  canEdit = false,
}: CommentProps) {
  const user = useSuspenseUserInfo();
  const location = useLocation();
  const boards = /\/boards\/posts(\/|$)/.test(location.pathname);

  const [expand, setExpand] = useState(false);
  const [showReplies, setShowReplies] = useState(false);
  const rootRef = useRef<HTMLDivElement | null>(null);

  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(comment.content ?? '');
  const [savingEdit, setSavingEdit] = useState(false);
  const editEditorRef = useRef<Editor | null>(null);

  const scrollToRoot = () => {
    rootRef.current?.scrollIntoView({
      behavior: 'smooth',
      block: 'start',
    });
  };

  const handleEditPost = () => {
    setEditContent(comment.content ?? '');
    setIsEditing(true);
    scrollToRoot();
  };

  const handleConfirmEdit = async () => {
    if (!onUpdate) return;
    if (!editContent.trim()) return;

    try {
      setSavingEdit(true);
      await onUpdate(comment.sk, editContent);
      setIsEditing(false);
    } catch (e) {
      logger.error('failed to update comment', e);
    } finally {
      setSavingEdit(false);
    }
  };

  const handleEditContainerClick = () => {
    editEditorRef.current?.commands.focus();
  };

  return (
    <div
      ref={rootRef}
      className="flex flex-col gap-3 pb-4 border-b border-b-divider"
    >
      <div className="flex flex-row justify-between items-start w-full">
        <div className="flex flex-row gap-2 items-center">
          <img
            alt={comment.author_display_name}
            src={comment.author_profile_url}
            className="object-cover object-top w-10 h-10 rounded-full"
          />

          <div className="flex flex-col gap-[2px]">
            <div className="font-semibold text-title-text text-[15px]/[15px]">
              {comment.author_display_name}
            </div>
            <div className="font-semibold text-xs/[20px] text-time-text">
              {getTimeAgo(comment.updated_at * 1000)}
            </div>
          </div>

          {comment.updated_at !== comment.created_at && (
            <div className="font-medium text-title-text text-[15px]">
              {t('update')}
            </div>
          )}
        </div>
        <PostAdminMenu
          t={t}
          canDelete={canDelete}
          canEdit={canEdit}
          handleEditPost={async () => {
            handleEditPost();
          }}
          handleDeletePost={async () => {
            if (onDelete) {
              await onDelete(comment.sk);
            }
          }}
        />
      </div>

      <div className="flex flex-col gap-3 ml-12">
        {!isEditing && (
          <TiptapEditor
            isMe={user.data.pk === comment.author_pk}
            content={comment.content}
            editable={false}
            showToolbar={false}
            minHeight="80px"
            isFoldable={true}
          />
        )}

        {isEditing && (
          <>
            <div
              className="flex-1 w-full rounded-md border transition-colors cursor-text bg-comment-box-bg border-primary hover:bg-foreground/5"
              onClick={handleEditContainerClick}
              role="button"
              tabIndex={-1}
              aria-label="Click to focus editor"
            >
              <TiptapEditor
                ref={editEditorRef}
                isMe={user.data.pk === comment.author_pk}
                content={editContent}
                editable={true}
                showToolbar={false}
                minHeight="80px"
                onUpdate={(content) => {
                  setEditContent(content);
                }}
                className="border-none"
              />
            </div>
            <div className="flex flex-row gap-2 justify-end items-center pt-2">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => {
                  setIsEditing(false);
                  setEditContent(comment.content ?? '');
                }}
              >
                Cancel
              </Button>
              <Button
                type="button"
                variant="primary"
                size="sm"
                onClick={handleConfirmEdit}
                disabled={savingEdit || !editContent.trim()}
              >
                Save
              </Button>
            </div>
          </>
        )}

        {!isEditing && (
          <>
            <div className="flex flex-row gap-2 justify-between items-center w-full">
              <div className="flex flex-row gap-5">
                <button
                  aria-label="Expand Replies"
                  className="flex flex-row gap-2 justify-center items-center disabled:cursor-not-allowed text-primary"
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
                  {`${comment.replies ?? 0} ${
                    comment.replies <= 1 ? t('reply') : t('replies')
                  }`}
                  {comment.replies > 0 && (
                    <ChevronDown
                      width={24}
                      height={24}
                      className="[&>path]:stroke-primary"
                    />
                  )}
                </button>
                {isLoggedIn && (
                  <button
                    aria-label="Reply to Comment"
                    onClick={() => {
                      setExpand((prev) => !prev);
                    }}
                    className="flex gap-2 justify-center items-center cursor-pointer text-text-primary"
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
                  <div className="font-medium text-base/[24px] text-comment-icon-text">
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
                onUpdate={onUpdate}
                onDelete={onDelete}
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
          </>
        )}
      </div>
    </div>
  );
}

export function NewComment({
  className,
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
    <div
      className={`flex flex-col w-full bg-comment-box-bg border rounded-lg border-primary max-w-desktop ${
        className ?? ''
      }`}
    >
      <div className="flex flex-row justify-between items-center px-3 pt-3">
        <span className="text-sm font-medium text-text-primary">
          {t('write_comment')}
        </span>
        <button
          aria-label="Close"
          className="p-1 rounded transition-colors hover:bg-foreground/10"
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
        className="flex-1 w-full rounded-md transition-colors cursor-text hover:bg-foreground/5"
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

      <div className="flex flex-row gap-2 justify-end items-center px-3 pt-3 pb-3 border-t border-divider">
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
