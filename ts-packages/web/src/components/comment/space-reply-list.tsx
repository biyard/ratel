import { TiptapEditor } from '../text-editor';
import { useSpaceReplies } from '@/features/spaces/boards/hooks/use-space-replies';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { ThumbUp } from '../icons';
import { TFunction } from 'i18next';
import PostAdminMenu from '@/features/spaces/boards/components/post-admin-menu';
import { useRef, useState } from 'react';
import type { Editor } from '@tiptap/core';
import { Button } from '../ui/button';

export type SpaceReplyListProp = {
  spacePk: string;
  postPk: string;
  commentSk: string;
  isLoggedIn: boolean;
  onLike?: (commentId: string, like: boolean) => Promise<void>;
  onDelete?: (commentSk: string) => Promise<void>;
  onUpdate?: (commentSk: string, content: string) => Promise<void>;
  t: TFunction<'Thread', undefined>;
};

export function SpaceReplyList({
  spacePk,
  postPk,
  commentSk,
  isLoggedIn,
  onLike,
  onDelete,
  onUpdate,
  t,
}: SpaceReplyListProp) {
  const { data: user } = useSuspenseUserInfo();
  const { replies, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useSpaceReplies(spacePk, postPk, commentSk);

  const [editingSk, setEditingSk] = useState<string | null>(null);
  const [editingContent, setEditingContent] = useState('');
  const [savingEdit, setSavingEdit] = useState(false);
  const editEditorRef = useRef<Editor | null>(null);

  const handleEditReply = (replySk: string, content: string) => {
    setEditingSk(replySk);
    setEditingContent(content ?? '');
  };

  const handleCancelEdit = () => {
    setEditingSk(null);
    setEditingContent('');
    setSavingEdit(false);
  };

  const handleSaveEdit = async (replySk: string) => {
    if (!onUpdate) return;
    if (!editingContent.trim()) return;

    try {
      setSavingEdit(true);
      await onUpdate(replySk, editingContent);
      setEditingSk(null);
    } finally {
      setSavingEdit(false);
    }
  };

  const handleEditContainerClick = () => {
    editEditorRef.current?.commands.focus();
  };

  return (
    <div className="flex flex-col gap-2.5">
      {replies.map((reply) => {
        const isEditing = editingSk === reply.sk;

        return (
          <div
            key={reply.sk}
            className="flex flex-col gap-2 p-5 rounded-lg bg-reply-box border border-transparent"
          >
            <div className="flex flex-row w-full justify-between items-center">
              <div className="flex flex-row gap-2 items-center">
                <img
                  alt={reply.author_display_name}
                  src={reply.author_profile_url}
                  width={40}
                  height={40}
                  className="rounded-full object-cover object-top"
                />
                <div className="flex flex-col gap-[2px]">
                  <div className="font-semibold text-title-text text-[15px]/[15px]">
                    {reply.author_display_name ?? ''}
                  </div>
                </div>

                {reply.updated_at !== reply.created_at && (
                  <div className="font-medium text-title-text text-[15px]">
                    {t('update')}
                  </div>
                )}
              </div>

              <PostAdminMenu
                t={t}
                canDelete={reply?.author_pk === user?.pk}
                canEdit={reply?.author_pk === user?.pk}
                handleEditPost={async () => {
                  handleEditReply(reply.sk, reply.content);
                }}
                handleDeletePost={async () => {
                  if (onDelete) {
                    await onDelete(reply.sk);
                  }
                }}
              />
            </div>

            {!isEditing && (
              <TiptapEditor
                isMe={user?.pk === reply.author_pk}
                content={reply.content}
                editable={false}
                showToolbar={false}
                isFoldable={true}
              />
            )}

            {isEditing && (
              <>
                <div
                  className="flex-1 w-full cursor-text hover:bg-foreground/5 transition-colors rounded-md bg-comment-box-bg border border-primary"
                  onClick={handleEditContainerClick}
                  role="button"
                  tabIndex={-1}
                  aria-label="Click to focus editor"
                >
                  <TiptapEditor
                    ref={editEditorRef}
                    isMe={user?.pk === reply.author_pk}
                    content={editingContent}
                    editable={true}
                    showToolbar={false}
                    minHeight="80px"
                    onUpdate={(content) => {
                      setEditingContent(content);
                    }}
                    className="border-none"
                  />
                </div>
                <div className="flex flex-row justify-end items-center gap-2 pt-2">
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={handleCancelEdit}
                  >
                    Cancel
                  </Button>
                  <Button
                    type="button"
                    variant="primary"
                    size="sm"
                    onClick={() => handleSaveEdit(reply.sk)}
                    disabled={savingEdit || !editingContent.trim()}
                  >
                    Save
                  </Button>
                </div>
              </>
            )}

            {!isEditing && (
              <div className="flex flex-row w-full justify-end">
                {isLoggedIn && (
                  <button
                    aria-label="Like Comment"
                    className="flex flex-row gap-2 justify-center items-center"
                    onClick={() => {
                      if (onLike) {
                        onLike(reply.sk, !reply.liked);
                      } else {
                        throw new Error('onLike is not set');
                      }
                    }}
                  >
                    <ThumbUp
                      width={24}
                      height={24}
                      className={
                        reply.liked
                          ? '[&>path]:fill-primary [&>path]:stroke-primary'
                          : '[&>path]:stroke-comment-icon'
                      }
                    />
                    <div className="font-medium text-base/[24px] text-comment-icon-text ">
                      {reply.likes ?? 0}
                    </div>
                  </button>
                )}
              </div>
            )}
          </div>
        );
      })}

      {hasNextPage && (
        <div className="flex justify-center mt-1 w-full">
          <button
            type="button"
            onClick={() => fetchNextPage()}
            disabled={isFetchingNextPage}
            className="px-2 py-1 text-xs rounded-md border border-border-subtle hover:bg-bg-elevated disabled:opacity-60"
          >
            {t('reply_more')}
          </button>
        </div>
      )}
    </div>
  );
}
