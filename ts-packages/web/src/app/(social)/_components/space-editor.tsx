
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

  spaceId?: number;
}

// custom  function for handling creating of comments, to be implemented.

export default function SpaceCommentEditor1({
  spaceId = 0,
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
    <div className="relative mb-2">
  

<div
          className="z-100 flex items-end justify-center"
          
        >
          <div
            className="w-full bg-comment-box-bg  overflow-hidden max-w-6xl border border-space-box-border rounded-lg"
            onClick={(e) => e.stopPropagation()}
          >
         

            <div className="relative p-4">
              

              <div className="flex items-center justify-between ">
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
                  <p className="text-home-side">Let's start!</p>
                  <TiptapEditor
                    content={content}
                    onUpdate={setContent}
                    editable={!createComment.isPending}
                    ref={editorRef}
                    onCreate={() => setEditorReady(true)}
                  />
                </div>

          
            </div>
          </div>
        </div>

     
    </div>
  );
}
