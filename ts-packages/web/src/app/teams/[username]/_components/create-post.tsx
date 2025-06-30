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
import {
  ImagePlus,
  Bold,
  Italic,
  Underline,
  Strikethrough,
  X,
  Loader2,
} from 'lucide-react';

import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import UserCircleIcon from '@/assets/icons/user-circle.svg';
import Certified from '@/assets/icons/certified.svg';
import { cn } from '@/lib/utils';
import { useUserInfo } from '@/lib/api/hooks/users';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import {
  $getSelection,
  $isRangeSelection,
  FORMAT_TEXT_COMMAND,
  LexicalEditor,
  EditorState,
  $getRoot,
  TextFormatType,
  COMMAND_PRIORITY_LOW,
  SELECTION_CHANGE_COMMAND,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import FileUploader from '@/components/file-uploader';
import { logger } from '@/lib/logger';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  updateDraftRequest,
  UrlType,
} from '@/lib/api/models/feeds/update-draft-request';
import { Feed, FeedStatus, FeedType } from '@/lib/api/models/feeds';
import Image from 'next/image';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import { useQueryClient } from '@tanstack/react-query';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { postByUserIdQk } from '@/app/(social)/_hooks/use-posts';
import { useTeamByUsername } from '../../_hooks/use-team';

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

function ToolbarPlugin({
  onImageUpload,
}: {
  onImageUpload: (url: string) => void;
}) {
  const [editor] = useLexicalComposerContext();
  const [activeEditor, setActiveEditor] = useState(editor);
  const [isBold, setIsBold] = useState(false);
  const [isItalic, setIsItalic] = useState(false);
  const [isUnderline, setIsUnderline] = useState(false);
  const [isStrikethrough, setIsStrikethrough] = useState(false);

  const updateToolbar = useCallback(() => {
    const selection = $getSelection();
    if ($isRangeSelection(selection)) {
      setIsBold(selection.hasFormat('bold'));
      setIsItalic(selection.hasFormat('italic'));
      setIsUnderline(selection.hasFormat('underline'));
      setIsStrikethrough(selection.hasFormat('strikethrough'));
    }
  }, []);

  useEffect(() => {
    return editor.registerCommand(
      SELECTION_CHANGE_COMMAND,
      (_payload, newEditor) => {
        updateToolbar();
        setActiveEditor(newEditor);
        return false;
      },
      COMMAND_PRIORITY_LOW,
    );
  }, [editor, updateToolbar]);

  useEffect(() => {
    return activeEditor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        updateToolbar();
      });
    });
  }, [activeEditor, updateToolbar]);

  const formatText = (format: TextFormatType) => {
    activeEditor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  };

  return (
    <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700">
      <button
        onClick={() => formatText('bold')}
        className={cn(isBold && 'bg-neutral-600 text-white')}
        aria-label="Format text as bold"
      >
        <Bold />
      </button>
      <button
        onClick={() => formatText('italic')}
        className={cn(isItalic && 'bg-neutral-600 text-white')}
        aria-label="Format text as italics"
      >
        <Italic />
      </button>
      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 text-white')}
        aria-label="Format text to underlined"
      >
        <Underline />
      </button>
      <button
        onClick={() => formatText('strikethrough')}
        className={cn(isStrikethrough && 'bg-neutral-600 text-white')}
        aria-label="Format text with a strikethrough"
      >
        <Strikethrough />
      </button>
      <FileUploader onUploadSuccess={onImageUpload}>
        <ImagePlus />
      </FileUploader>
    </div>
  );
}

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
    status,
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
      <div className="w-full bg-neutral-900 border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden">
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
                    className="absolute -top-1.5 -right-1.5 w-5 h-5 bg-red-600 rounded-full flex items-center justify-center text-white text-xs hover:bg-red-700 border-2 border-neutral-900"
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
              {/* 상태 표시 */}
              {status === 'saving' && (
                <div className="flex items-center gap-2 text-sm text-neutral-400">
                  <Loader2 className="animate-spin" size={16} />
                  <span>Saving...</span>
                </div>
              )}
              {status === 'error' && (
                <span className="text-sm text-red-500">Error saving!</span>
              )}

              <button
                onClick={publishPost}
                disabled={isSubmitDisabled}
                className={cn(
                  'flex items-center gap-2 p-3 rounded-full font-medium text-sm transition-all',
                  !isSubmitDisabled
                    ? 'bg-primary text-black hover:bg-primary/50'
                    : 'bg-neutral-700 text-neutral-500 cursor-not-allowed',
                )}
              >
                {status === 'publishing' ? (
                  <Loader2 className="animate-spin" />
                ) : (
                  <UserCircleIcon />
                )}
              </button>
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
  loadDraft: (id: number) => Promise<void>;
  newDraft: () => void;
}

export const PostDraftContext = createContext<PostDraftContextType | undefined>(
  undefined,
);

export const PostDraftProvider: React.FC<{
  children: React.ReactNode;
  username: string;
}> = ({ children, username }) => {
  const query = useTeamByUsername(username);
  const [expand, setExpand] = useState(false);
  const [draftId, setDraftId] = useState<number | null>(null);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(null);
  const [status, setStatus] = useState<DraftStatus>('idle');
  const team = query.data;

  const lastSavedRef = useRef({
    title: '',
    content: null as string | null,
    image: null as string | null,
  });

  const { get, post } = useApiCall();
  const queryClient = useQueryClient();

  const refetchDrafts = useCallback(() => {
    queryClient.invalidateQueries({
      queryKey: postByUserIdQk(team.id, 1, 20, FeedStatus.Draft),
    });
  }, [queryClient, team.id]);

  const resetState = useCallback(() => {
    setDraftId(null);
    setTitle('');
    setContent('');
    setImage(null);
    setStatus('idle');
    lastSavedRef.current = {
      title: '',
      content: '',
      image: null,
    };
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

        setDraftId(draft.id);
        setTitle(draftTitle);
        setImage(draftImage);
        setContent(draftContent);
        lastSavedRef.current = {
          title: draftTitle,
          content: draftContent,
          image: draftImage,
        };
        setExpand(true);
        logger.debug('Draft loaded:', draft);
      } catch (error: unknown) {
        logger.error('LoadDraft error:', error);
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
      if (status === 'saving' || status === 'creating' || !team) return;

      if (!currentTitle.trim()) return;

      const lastSaved = lastSavedRef.current;
      if (
        currentTitle === lastSaved.title &&
        currentContent === lastSaved.content &&
        currentImage === lastSaved.image
      ) {
        return;
      }

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
            createDraftRequest(FeedType.Post, team.id),
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

          // 성공적으로 저장된 값들을 기록
          lastSavedRef.current = {
            title: currentTitle,
            content: currentContent,
            image: currentImage,
          };
        }
        refetchDrafts();
      } catch (error: unknown) {
        logger.error('Save draft error:', error);
        setStatus('error');
      } finally {
        setStatus('idle');
      }
    },
    [draftId, team, post, refetchDrafts, status],
  );

  useEffect(() => {
    if (!title.trim() && !content?.trim()) return;
    if (status !== 'idle') return;

    const lastSaved = lastSavedRef.current;
    if (
      (title === lastSaved.title &&
        content === lastSaved.content &&
        image === lastSaved.image) ||
      content === null
    ) {
      return;
    }

    const handler = setTimeout(() => {
      saveDraft(title, content, image);
    }, 1500);

    return () => {
      clearTimeout(handler);
    };
  }, [title, content, image, status, saveDraft]);

  const publishPost = useCallback(async () => {
    if (checkString(title) || checkString(content ?? '')) {
      showErrorToast('Please remove the test keyword');
      return;
    }

    if (!draftId || !title.trim() || status !== 'idle' || content == null)
      return;

    setStatus('publishing');
    try {
      await saveDraft(title, content, image);

      await post(ratelApi.feeds.publishDraft(draftId), { publish: {} });

      resetState();
      setExpand(false);
      refetchDrafts();
    } catch (error: unknown) {
      logger.error('Publish error:', error);
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
    loadDraft,
    newDraft,
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
