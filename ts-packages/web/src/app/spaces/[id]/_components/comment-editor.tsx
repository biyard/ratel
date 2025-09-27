'use client';

import { useState, useRef } from 'react';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
import {
  showSuccessToast,
  showErrorToast,
} from '@/components/custom-toast/toast';
import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
import CommentIcon from '@/assets/icons/comment.svg';
import { Editor } from '@tiptap/core';
import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import { Loader } from '@/components/icons';
import UserCircle from '@/assets/icons/user-circle.svg';
import SaveIcon from '@/assets/icons/save.svg';
import LinkPaste from '@/assets/icons/editor/link-paste.svg';
import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';

interface SpaceCommentEditorProps {
  showCommentEditor: boolean;
  setShowCommentEditor: (show: boolean) => void;
  commentCount: number;
  t: (key: string) => string;
  spaceId?: number;
  onCommentPosted?: () => void;
}

export default function SpaceCommentEditor({
  showCommentEditor,
  setShowCommentEditor,
  commentCount,
  spaceId = 0,
  onCommentPosted = () => {},
}: SpaceCommentEditorProps) {
  const [content, setContent] = useState('');
  const [showUrlInput, setShowUrlInput] = useState(false);
  const [url, setUrl] = useState('');
  const [showCommentUrlInput, setShowCommentUrlInput] = useState(false);
  const [commentUrl, setCommentUrl] = useState('');
  const { data: user } = useSuspenseUserInfo();
  const editorRef = useRef<Editor | null>(null);
  const [editorReady, setEditorReady] = useState(false);

  const { createComment } = useDraftMutations(user?.id || 0);

  const handleSubmit = async () => {
    if (!content.trim() || !spaceId || !user?.id) return;
    try {
      await createComment.mutateAsync({
        userId: user.id,
        parentId: spaceId,
        postId: spaceId,
        content: content,
      });
      setContent('');
      setShowCommentEditor(false);
      onCommentPosted();
      showSuccessToast('Comment posted successfully');
    } catch (error) {
      logger.debug('Failed to post comment', error);
      showErrorToast('Failed to post comment');
    }
  };

  //  regular link
  const handleInsertUrl = () => {
    const urlToInsert = url?.trim();
    if (!urlToInsert) return;
    editorRef.current?.chain().focus().insertContent(urlToInsert).run();
    setShowUrlInput(false);
    setUrl('');
  };

  //  quoted comment link
  const handleInsertCommentUrl = () => {
    const commentToInsert = commentUrl?.trim();
    if (!commentToInsert) return;
    editorRef.current?.chain().focus().insertContent(commentToInsert).run();
    setShowCommentUrlInput(false);
    setCommentUrl('');
  };

  return (
    <div className="relative">
      <div
        className="flex items-center gap-1 cursor-pointer"
        onClick={() => setShowCommentEditor(true)}
      >
        <CommentIcon className="w-5 h-5" />
        <span className="text-sm font-medium text-foreground">
          {commentCount}
        </span>
      </div>

      {showCommentEditor && (
        <div
          className="fixed inset-0 z-50 flex items-end justify-center"
          onClick={() => setShowCommentEditor(false)}
        >
          <div
            className="w-full bg-comment-box-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden max-w-6xl"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="p-4 flex justify-between items-center">
              <h3 className="font-medium text-foreground">Add a comment</h3>
              <button onClick={() => setShowCommentEditor(false)}>
                <DoubleArrowDown className="[&>path]:stroke-text-primary" />
              </button>
            </div>

            <div className="p-4 relative">
              <div>
                <div className="min-h-[80px] text-text-primary rounded p-2">
                  <TiptapEditor
                    content={content}
                    onUpdate={setContent}
                    editable={!createComment.isPending}
                    ref={editorRef}
                    onCreate={() => setEditorReady(true)}
                  />
                </div>

                <div className="flex items-center justify-between gap-4 m-2">
                  {editorReady && (
                    <ToolbarPlugin
                      editor={editorRef.current}
                      onTriggerLinkPaste={() => setShowUrlInput(true)}
                      onCommentPaste={() => setShowCommentUrlInput(true)}
                    />
                  )}

                  <div className="flex flex-row gap-4">
                    {/* Save button (not implemented) */}
                    <button className="shrink-0 text-foreground rounded-full px-4 py-2 font-bold flex items-center gap-x-2">
                      <SaveIcon />
                      Save
                    </button>
                    <button
                      onClick={handleSubmit}
                      disabled={!content.trim() || createComment.isPending}
                      className={cn(
                        'shrink-0 bg-primary text-text-third rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2',
                        createComment.isPending && 'opacity-70',
                      )}
                    >
                      {createComment.isPending ? (
                        <Loader className="animate-spin" />
                      ) : (
                        <UserCircle />
                      )}
                      {createComment.isPending ? 'Posting...' : 'Post'}
                    </button>
                  </div>
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
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
