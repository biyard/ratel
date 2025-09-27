'use client';
import { useState, useRef, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
import { showErrorToast } from '@/components/custom-toast/toast';
import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
import { Editor } from '@tiptap/core';
import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import LinkPaste from '@/assets/icons/editor/link-paste.svg';
import CommentPaste from '@/assets/icons/editor/comment-paste.svg';

interface SpaceCommentEditorProps {
  spaceId?: number;
  postId?: number;
  parentId?: number;
  onSuccess?: () => void;
  onCancel?: () => void;
  placeholder?: string;
  autoFocus?: boolean;
}

export default function SpaceCommentEditor1({
  spaceId = 0,
  postId = 0,
  parentId = 0,
  onSuccess,
  onCancel,
  placeholder = "Let's start!",
}: SpaceCommentEditorProps) {
  const [content, setContent] = useState('');
  const [showUrlInput, setShowUrlInput] = useState(false);
  const [url, setUrl] = useState('');
  const [showCommentUrlInput, setShowCommentUrlInput] = useState(false);
  const [commentUrl, setCommentUrl] = useState('');
  const { data: user } = useSuspenseUserInfo();
  const editorRef = useRef<Editor | null>(null);
  const [editorReady, setEditorReady] = useState(false);

  const {
    createComment: { mutateAsync },
  } = useDraftMutations(user?.id || 0);

  const handleSubmit = useCallback(
    async (e?: React.FormEvent) => {
      e?.preventDefault();
      if (!content.trim()) return;

      try {
        await mutateAsync({
          userId: user?.id || 0,
          parentId,
          postId: postId || spaceId,
          content,
        });

        // Clear the editor after successful submission
        setContent('');
        editorRef.current?.commands.clearContent();

        // Call onSuccess callback if provided
        onSuccess?.();
      } catch (error) {
        console.error('Error submitting comment:', error);
        showErrorToast('Failed to post comment');
      }
    },
    [content, mutateAsync, onSuccess, parentId, postId, spaceId, user?.id],
  );

  const handleEditorCreate = useCallback(() => {
    setEditorReady(true);
  }, []);

  const handleInsertUrl = () => {
    const urlToInsert = url?.trim();
    if (!urlToInsert) return;
    editorRef.current?.chain().focus().insertContent(urlToInsert).run();
    setShowUrlInput(false);
    setUrl('');
  };

  const handleInsertCommentUrl = () => {
    const commentToInsert = commentUrl?.trim();
    if (!commentToInsert) return;
    editorRef.current?.chain().focus().insertContent(commentToInsert).run();
    setShowCommentUrlInput(false);
    setCommentUrl('');
  };

  return (
    <div className="relative mb-2">
      <div className="z-100 flex items-end justify-center">
        <div
          className="w-full bg-comment-box-bg  overflow-hidden max-w-6xl border border-space-box-border rounded-lg"
          onClick={(e) => e.stopPropagation()}
        >
          <div className="relative p-4">
            <div className="flex items-center justify-between">
              {editorReady && (
                <ToolbarPlugin
                  editor={editorRef.current}
                  onTriggerLinkPaste={() => setShowUrlInput(true)}
                  onCommentPaste={() => setShowCommentUrlInput(true)}
                />
              )}
            </div>

            {/* LinkPaste input dialog */}
            {showUrlInput && (
              <div className="absolute top-2 z-20 bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 flex items-center gap-2 w-[90%]">
                <button onClick={handleInsertUrl}>
                  <LinkPaste />
                </button>
                <input
                  autoFocus
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleInsertUrl()}
                  placeholder="Paste or search for the relevant discussion or topic URL"
                  className="bg-transparent text-white text-sm placeholder-neutral-400 outline-none flex-1"
                />
                <button
                  onClick={() => {
                    setShowUrlInput(false);
                    setUrl('');
                  }}
                  className="text-neutral-400 hover:text-white"
                  aria-label="Cancel"
                >
                  <DoubleArrowDown className="w-4 h-4" />
                </button>
              </div>
            )}

            {/* CommentPaste input dialog */}
            {showCommentUrlInput && (
              <div className="absolute top-2/5  z-20 bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 flex items-center gap-2 w-[90%]">
                <button onClick={handleInsertCommentUrl}>
                  <CommentPaste />
                </button>
                <input
                  autoFocus
                  value={commentUrl}
                  onChange={(e) => setCommentUrl(e.target.value)}
                  onKeyDown={(e) =>
                    e.key === 'Enter' && handleInsertCommentUrl()
                  }
                  placeholder="Please paste or search for the comment to quote"
                  className="bg-transparent text-white text-sm placeholder-neutral-400 outline-none flex-1"
                />
                <button
                  onClick={() => {
                    setShowCommentUrlInput(false);
                    setCommentUrl('');
                  }}
                  className="text-neutral-400 hover:text-white"
                  aria-label="Cancel"
                >
                  <DoubleArrowDown className="w-4 h-4" />
                </button>
              </div>
            )}
            <div className="min-h-[80px] text-text-primary rounded p-2 border border-write-comment-box-border">
              {placeholder && (
                <p className="text-text-primary">{placeholder}</p>
              )}
              <form onSubmit={handleSubmit}>
                <TiptapEditor
                  content={content}
                  onUpdate={setContent}
                  ref={editorRef}
                  onCreate={handleEditorCreate}
                  editable={true}
                />
                <div className="flex justify-end gap-2 mt-2">
                  {onCancel && (
                    <button
                      type="button"
                      onClick={onCancel}
                      className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-200 rounded-md transition-colors duration-200"
                    >
                      Cancel
                    </button>
                  )}
                  <button
                    type="submit"
                    disabled={!content.trim()}
                    className={cn(
                      'px-4 py-2 rounded-md text-sm font-medium',
                      'text-follow-button-text-secondary bg-enable-button-bg hover:bg-enable-button-bg/80 ',
                      'disabled:opacity-50 disabled:cursor-not-allowed',
                      'flex items-center gap-2',
                    )}
                  >
                    <span>Post</span>
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
