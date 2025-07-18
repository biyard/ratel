'use client';

import {
  createContext,
  useContext,
  useState,
  useCallback,
  useEffect,
  useRef,
  Fragment,
} from 'react';
import { X, Loader2 } from 'lucide-react';

import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import UserCircleIcon from '@/assets/icons/user-circle.svg';
import Certified from '@/assets/icons/certified.svg';
import { cn } from '@/lib/utils';
import { useUserInfo } from '@/lib/api/hooks/users';
import { Button } from '@/components/ui/button';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import {
  LexicalEditor,
  EditorState,
  $getRoot,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { logger } from '@/lib/logger';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  updateDraftRequest,
  UrlType,
} from '@/lib/api/models/feeds/update-draft-request';
import { editPostRequest } from '@/lib/api/models/feeds/edit-post-request';
import { Feed, FeedStatus, FeedType } from '@/lib/api/models/feeds';
import Image from 'next/image';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import { useQueryClient } from '@tanstack/react-query';
import { postByUserIdQk } from '../_hooks/use-posts';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import ToolbarPlugin from '@/components/toolbar/toolbar';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { QK_GET_FEED_BY_FEED_ID } from '@/constants';

export const editorTheme = {
  ltr: 'text-left',
  rtl: 'text-right',
  paragraph: 'relative mb-1',
  text: {
    bold: 'font-bold',
    italic: 'italic',
    underline: 'underline',
    strikethrough: 'line-through',
    underlineStrikethrough: 'underline line-through',
  },
  placeholder:
    'absolute top-0 left-0 text-neutral-500 pointer-events-none select-none inline-block',
};

function EditorRefPlugin({
  setEditorRef,
}: {
  setEditorRef: (editor: LexicalEditor) => void;
}) {
  const [editor] = useLexicalComposerContext();
  useEffect(() => {
    setEditorRef(editor);
  }, [editor, setEditorRef]);
  return null;
}

export function CreatePost() {
  const {
    expand,
    setExpand,
    title,
    setTitle,
    content,
    setContent,
    image,
    setImage,
    publishPost,
    savePost,
    status,
    isPublishedPost,
  } = usePostDraft();

  const { data: userInfo, isLoading } = useUserInfo();
  const editorRef = useRef<LexicalEditor | null>(null);
  const isLoadingContent = useRef(false);

  const handleLexicalChange = (
    editorState: EditorState,
    editor: LexicalEditor,
  ) => {
    editorRef.current = editor;
    editorState.read(() => {
      const html = $generateHtmlFromNodes(editor, null);
      if (html !== content) {
        setContent(html);
      }
    });
  };

  const removeImage = () => {
    setImage(null);
  };

  const isSubmitDisabled =
    !title.trim() ||
    checkString(title) ||
    checkString(content ?? '') ||
    status !== 'idle';

  const createEditorStateFromHTML = useCallback(
    (editor: LexicalEditor, htmlString: string) => {
      if (!htmlString) {
        const root = $getRoot();
        root.clear();
        root.append($createParagraphNode());
        return;
      }
      try {
        const parser = new DOMParser();
        const dom = parser.parseFromString(htmlString, 'text/html');
        const nodes = $generateNodesFromDOM(editor, dom);
        const root = $getRoot();
        root.clear();
        root.append(...nodes);
      } catch (error) {
        logger.error('Error parsing HTML:', error);
      }
    },
    [],
  );

  const editorConfig = {
    namespace: 'CreatePostEditor',
    theme: editorTheme,
    onError(error: Error) {
      console.error(error);
    },
  };

  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;

    const currentHtml = editor
      .getEditorState()
      .read(() => $generateHtmlFromNodes(editor, null));
    if (!content || content !== currentHtml) {
      isLoadingContent.current = true;

      editor.update(
        () => {
          createEditorStateFromHTML(editor, content ?? '');
        },
        {
          onUpdate: () => {
            setTimeout(() => {
              isLoadingContent.current = false;
            }, 0);
          },
        },
      );
    }
  }, [editorRef, content, createEditorStateFromHTML]);

  if (isLoading || !expand) {
    return <Fragment />;
  }

  return (
    <LexicalComposer initialConfig={editorConfig}>
      <div className="w-full bg-component-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden">
        {/* Header */}
        <div className="flex items-center p-4 justify-between">
          <div className="flex items-center gap-3">
            <div className="size-6 rounded-full">
              <Image
                width={40}
                height={40}
                src={userInfo?.profile_url || '/default-profile.png'}
                alt="Profile"
                className="w-full h-full object-cover"
              />
            </div>
            <div className="flex items-center gap-2">
              <span className="text-white font-medium text-lg">
                {userInfo?.nickname || 'Anonymous'}
              </span>
            </div>
            <Certified className="size-5" />
          </div>
          <div
            className={cn('cursor-pointer')}
            onClick={() => setExpand(!expand)}
          >
            <DoubleArrowDown />
          </div>
        </div>

        <>
          {/* Title input */}
          <div className="px-4 pt-4">
            <input
              type="text"
              placeholder="Write a title..."
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="w-full bg-transparent text-white text-xl font-semibold placeholder-neutral-500 outline-none border-none"
            />
          </div>

          {/* Lexical Content Area */}
          <div className="px-4 pt-2 min-h-[80px] relative text-neutral-300 text-[15px] leading-relaxed">
            <RichTextPlugin
              contentEditable={
                <ContentEditable className="outline-none resize-none w-full min-h-[60px]" />
              }
              placeholder={
                <div className="absolute top-0 text-neutral-500 pointer-events-none select-none">
                  Type here, Use Markdown, BB code, or HTML to format.
                </div>
              }
              ErrorBoundary={LexicalErrorBoundary}
            />
            <OnChangePlugin onChange={handleLexicalChange} />

            <HistoryPlugin />
            <EditorRefPlugin
              setEditorRef={(editor) => (editorRef.current = editor)}
            />
          </div>

          {/* Image previews */}
          {image && (
            <div className="px-4 pt-2">
              <div className="flex flex-wrap gap-2">
                <div className="relative size-16">
                  <Image
                    width={64}
                    height={64}
                    src={image}
                    alt={`Uploaded image`}
                    className="object-cover rounded-lg border border-neutral-600"
                  />
                  <button
                    onClick={removeImage}
                    className="absolute -top-1.5 -right-1.5 w-5 h-5 bg-red-600 rounded-full flex items-center justify-center text-white text-xs hover:bg-red-700 border-2 border-component-bg"
                    aria-label={`Remove uploaded image`}
                  >
                    <X size={12} />
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Bottom toolbar */}
          <div className="flex items-center justify-between p-4 text-neutral-400">
            <ToolbarPlugin onImageUpload={(url) => setImage(url)} />

            <div className="flex items-center gap-4">
              {/* Status indicator */}
              {status === 'saving' && (
                <div className="flex items-center gap-2 text-sm text-neutral-400">
                  <Loader2 className="animate-spin" size={16} />
                  <span>Saving...</span>
                </div>
              )}
              {status === 'error' && (
                <span className="text-sm text-red-500">Save failed</span>
              )}

              {isPublishedPost ? (
                // Save button for published posts
                <Button
                  variant="rounded_primary"
                  size="default"
                  onClick={savePost}
                  disabled={!title.trim() || status !== 'idle'}
                  className="gap-2"
                >
                  {status === 'saving' ? (
                    <Loader2 className="animate-spin" />
                  ) : (
                    <UserCircleIcon />
                  )}
                </Button>
              ) : (
                // Publish button for drafts
                <Button
                  variant="rounded_primary"
                  size="default"
                  onClick={publishPost}
                  disabled={isSubmitDisabled}
                  className="gap-2"
                >
                  {status === 'publishing' ? (
                    <Loader2 className="animate-spin" />
                  ) : (
                    <UserCircleIcon />
                  )}
                </Button>
              )}
            </div>
          </div>
        </>
      </div>
    </LexicalComposer>
  );
}

export type DraftStatus =
  | 'idle'
  | 'loading'
  | 'creating'
  | 'saving'
  | 'publishing'
  | 'error';

export interface PostDraftContextType {
  expand: boolean;
  setExpand: (expand: boolean) => void;
  draftId: number | null;
  title: string;
  setTitle: (title: string) => void;
  content: string | null;
  setContent: (content: string | null) => void;
  image: string | null;
  setImage: (image: string | null) => void;
  status: DraftStatus;
  publishPost: () => Promise<void>;
  savePost: () => Promise<void>;
  loadDraft: (id: number) => Promise<void>;
  newDraft: () => void;
  isPublishedPost: boolean;
}

export const PostDraftContext = createContext<PostDraftContextType | undefined>(
  undefined,
);

export const PostDraftProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const router = useRouter();
  const [expand, setExpand] = useState(false);
  const [draftId, setDraftId] = useState<number | null>(null);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(null);
  const [status, setStatus] = useState<DraftStatus>('idle');
  const [isPublishedPost, setIsPublishedPost] = useState(false);

  const lastSavedRef = useRef({
    title: '',
    content: null as string | null,
    image: null as string | null,
  });

  const { get, post } = useApiCall();
  const { data: user } = useUserInfo();
  const queryClient = useQueryClient();

  const refetchDrafts = useCallback(() => {
    if (!user) return;
    queryClient.invalidateQueries({
      queryKey: postByUserIdQk(user.id, 1, 20, FeedStatus.Draft),
    });
  }, [user, queryClient]);

  const resetState = useCallback(() => {
    setDraftId(null);
    setTitle('');
    setImage(null);
    setStatus('idle');
    setIsPublishedPost(false);
  }, []);

  const newDraft = useCallback(() => {
    resetState();
    setExpand(true);
  }, [resetState]);

  const loadDraft = useCallback(
    async (id: number) => {
      setStatus('loading');

      try {
        const draft: Feed = await get(ratelApi.feeds.getFeedsByFeedId(id));
        const draftTitle = draft.title || '';
        const draftContent = draft.html_contents || '';
        const draftImage =
          draft.url && draft.url_type === UrlType.Image ? draft.url : null;
        const isPublished = draft.status === FeedStatus.Published;

        // Reset content first to clear the editor
        setContent(null);

        // React 18+ automatically batches these state updates
        setDraftId(draft.id);
        setTitle(draftTitle);
        setImage(draftImage);
        setIsPublishedPost(isPublished);

        // Update the ref immediately since it doesn't trigger re-renders
        lastSavedRef.current = {
          title: draftTitle,
          content: draftContent,
          image: draftImage,
        };

        // Set content after other state is set - this will trigger the useEffect
        // that handles editor content updates
        setContent(draftContent);
        setExpand(true);
      } catch (error: unknown) {
        logger.error('Failed to load draft:', error);
        setStatus('error');
      } finally {
        setStatus('idle');
      }
    },
    [get],
  );

  const saveDraft = useCallback(
    async (
      currentTitle: string,
      currentContent: string,
      currentImage: string | null,
    ) => {
      if (status === 'saving' || status === 'creating' || !user) return;

      if (!currentTitle.trim()) return;

      const lastSaved = lastSavedRef.current;
      const hasChanges =
        currentTitle !== lastSaved.title ||
        currentContent !== lastSaved.content ||
        currentImage !== lastSaved.image;

      if (!hasChanges) return;

      const isCreating = !draftId;
      setStatus(isCreating ? 'creating' : 'saving');

      try {
        let currentDraftId = draftId;

        if (checkString(currentTitle) || checkString(currentContent)) {
          showErrorToast('Please remove the test keyword');
          return;
        }

        if (isCreating) {
          const data: Feed = await post(
            ratelApi.feeds.createDraft(),
            createDraftRequest(FeedType.Post, user.id),
          );
          currentDraftId = data.id;
          setDraftId(currentDraftId);
        }

        if (currentDraftId) {
          let url = '';
          let url_type = UrlType.None;
          if (currentImage) {
            url = currentImage;
            url_type = UrlType.Image;
          }

          if (isPublishedPost) {
            await post(
              ratelApi.feeds.editPost(currentDraftId),
              editPostRequest(
                currentContent,
                1,
                currentTitle,
                0,
                [],
                url,
                url_type,
              ),
            );
          } else {
            await post(
              ratelApi.feeds.updateDraft(currentDraftId),
              updateDraftRequest(
                currentContent,
                1,
                currentTitle,
                0,
                [],
                url,
                url_type,
              ),
            );
          }

          lastSavedRef.current = {
            title: currentTitle,
            content: currentContent,
            image: currentImage,
          };
        }

        refetchDrafts();
      } catch (error: unknown) {
        logger.error('Failed to save draft:', error);
        setStatus('error');
      } finally {
        setStatus('idle');
      }
    },
    [draftId, user, post, refetchDrafts, status, isPublishedPost],
  );

  useEffect(() => {
    // Only auto-save for drafts, not published posts
    if (!title.trim() && !content?.trim()) return;
    if (status !== 'idle' || isPublishedPost) return;

    const lastSaved = lastSavedRef.current;
    const hasChanges =
      title !== lastSaved.title ||
      content !== lastSaved.content ||
      image !== lastSaved.image;

    if (!hasChanges || content === null) return;

    const AUTO_SAVE_DELAY = 1500; // ms
    const timeoutId = setTimeout(() => {
      saveDraft(title, content, image);
    }, AUTO_SAVE_DELAY);

    return () => clearTimeout(timeoutId);
  }, [title, content, image, status, saveDraft, isPublishedPost]);

  const publishPost = useCallback(async () => {
    if (checkString(title) || checkString(content ?? '')) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    if (!draftId || !title.trim() || status !== 'idle' || content == null) {
      return;
    }

    setStatus('publishing');

    try {
      await saveDraft(title, content, image);

      await post(ratelApi.feeds.publishDraft(draftId), {
        publish: {},
      });

      router.push(route.threadByFeedId(draftId));
      resetState();
      setExpand(false);
      refetchDrafts();
    } catch (error: unknown) {
      logger.error('Failed to publish post:', error);
      setStatus('error');
    } finally {
      setStatus('idle');
    }
  }, [
    draftId,
    title,
    content,
    image,
    status,
    saveDraft,
    post,
    resetState,
    refetchDrafts,
    router,
  ]);

  const savePost = useCallback(async () => {
    if (checkString(title) || checkString(content ?? '')) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    if (!draftId || !title.trim() || status !== 'idle' || content == null) {
      return;
    }

    try {
      await saveDraft(title, content, image);

      // Invalidate the specific feed query to refetch updated data
      queryClient.invalidateQueries({
        queryKey: [QK_GET_FEED_BY_FEED_ID, draftId],
      });

      // Close the editor and reset state
      setExpand(false);
      resetState();
    } catch (error: unknown) {
      logger.error('Failed to save post changes:', error);
    }
  }, [
    draftId,
    title,
    content,
    image,
    status,
    saveDraft,
    queryClient,
    setExpand,
    resetState,
  ]);

  const contextValue = {
    expand,
    setExpand,
    draftId,
    title,
    setTitle,
    content,
    setContent,
    image,
    setImage,
    status,
    publishPost,
    savePost,
    loadDraft,
    newDraft,
    isPublishedPost,
  };

  return (
    <PostDraftContext.Provider value={contextValue}>
      {children}
    </PostDraftContext.Provider>
  );
};

export const usePostDraft = () => {
  const context = useContext(PostDraftContext);
  if (context === undefined) {
    throw new Error('usePostDraft must be used within a PostDraftProvider');
  }
  return context;
};
