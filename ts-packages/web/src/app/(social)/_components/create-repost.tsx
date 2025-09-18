'use client';
import {
  createContext,
  useContext,
  useState,
  useCallback,
  useRef,
} from 'react';
import { Clear } from '@/components/icons';
import { Loader } from '@/components/icons';
import { cn } from '@/lib/utils';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { useUserInfo } from '../_hooks/user';
import Image from 'next/image';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import Certified from '@/assets/icons/certified.svg';
import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import { logger } from '@/lib/logger';
import { UserCircle } from '@/components/icons';
import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
import {
  showErrorToast,
  showSuccessToast,
} from '@/components/custom-toast/toast';
import DOMPurify from 'dompurify';
import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
import { Editor } from '@tiptap/core';
import LinkPaste from '@/assets/icons/editor/link-paste.svg';
import CommentPaste from '@/assets/icons/editor/comment-paste.svg';

export function CreateRePost() {
  const {
    expand,
    title,
    setTitle,
    content,
    setContent,
    image,
    setImage,
    feedcontent,
    feedImageUrl,
    setFeedImageUrl,
    resetDraft,
    authorName,
    industry,
    authorProfileUrl,
    originalFeedId,
    showUrlInput,
    repostUrl,
    setShowUrlInput,
    setRepostUrl,
    commentUrl,
    showCommentUrlInput,
    setCommentUrl,
    setShowCommentUrlInput,
    authorId,
  } = useRepostDraft();

  const { data: User } = useSuspenseUserInfo();
  const { data: userInfo } = useUserInfo();
  const { post } = useApiCall();
  const router = useRouter();
  const editorRef = useRef<Editor | null>(null);
  const [isReposting, setIsReposting] = useState(false);

  const handlePublish = async () => {
    if (!title.trim() || !content) return;

    setIsReposting(true);
    let didTimeout = false;
    const timeout = setTimeout(() => {
      didTimeout = true;
      setIsReposting(false);
      showErrorToast('Request timed out');
    }, 10000);

    try {
      const response = await post(ratelApi.feeds.repost(), {
        repost: {
          html_contents: editorRef.current!.getHTML(),
          quote_feed_id: originalFeedId,
          parent_id: authorId,
          user_id: User?.id,
        },
      });

      clearTimeout(timeout);
      if (didTimeout) return;

      showSuccessToast('Repost Successful!');
      resetDraft();
      router.push(route.threadByFeedId(response.id));
    } catch (error) {
      setIsReposting(false);
      logger.debug('Failed to publish repost:', error);
      showErrorToast('Failed to publish repost');
    } finally {
      clearTimeout(timeout);
      setIsReposting(false);
    }
  };

  const removeImage = () => setImage(null);
  const removeQuotedImage = () => setFeedImageUrl(null);

  const isSubmitDisabled = !title.trim() || !content?.trim() || isReposting;

  const handleInsertUrl = () => {
    const url = repostUrl?.trim();
    if (!url) return;

    editorRef.current?.chain().focus().insertContent(url).run();
    setShowUrlInput(false);
    setRepostUrl('');
  };

  const handleCommentUrl = () => {
    const url = commentUrl?.trim();
    if (!url) return;

    editorRef.current?.chain().focus().insertContent(url).run();
    setCommentUrl('');
    setShowCommentUrlInput(false);
  };

  return (
    <div className={`flex flex-col w-full ${!expand ? 'hidden' : 'block'}`}>
      <div className="w-full bg-component-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden">
        {/* Header */}
        <div className="flex items-center p-4 justify-between">
          <div className="flex items-center gap-3 relative">
            <div className="size-6 rounded-full">
              <Image
                width={40}
                height={40}
                src={userInfo?.profile_url || '/default-profile.png'}
                alt="Profile"
                className="w-full h-full object-cover rounded-full"
              />
            </div>
            <div className="flex items-center gap-2">
              <span className="text-white font-medium text-lg">
                {userInfo?.nickname || 'Anonymous'}
              </span>
            </div>
            <Certified className="size-5" />
          </div>

          <div className="flex items-center space-x-4">
            <div className="p-4 rounded-lg flex w-[320px] justify-between border-[0.5px] border-[#262626]">
              <p className="text-left text-white text-lg">{industry}</p>
            </div>

            <div className={cn('cursor-pointer')} onClick={resetDraft}>
              <DoubleArrowDown />
            </div>
          </div>
        </div>

        {/* Quoted Content Section */}
        {(feedcontent || feedImageUrl) && (
          <div className="px-4 pt-2 pb-3 bg-write-comment-box-bg rounded-md mx-4 my-4">
            <div className="flex items-center gap-3 relative">
              <div className="size-6 rounded-full">
                <Image
                  width={40}
                  height={40}
                  src={authorProfileUrl || '/default-profile.png'}
                  alt="Profile"
                  className="w-full h-full object-cover rounded-full"
                />
              </div>
              <div className="flex items-center gap-2">
                <span className="text-foreground font-medium text-lg">
                  {authorName || 'Anonymous'}
                </span>
              </div>
              <Certified className="size-5" />
            </div>

            {feedcontent && (
              <div
                className="prose prose-invert text-sm p-3 bg-write-comment-box-bg  mb-3 font-light"
                dangerouslySetInnerHTML={{
                  __html: DOMPurify.sanitize(feedcontent),
                }}
              />
            )}

            {feedImageUrl && (
              <div className="relative group">
                <div className="relative w-full aspect-video rounded-md overflow-hidden max-h-40">
                  <Image
                    src={feedImageUrl}
                    alt="Quoted content"
                    fill
                    className="object-cover"
                    sizes="100vw"
                  />
                </div>
                <button
                  onClick={removeQuotedImage}
                  className="absolute top-2 right-2 bg-black/70 rounded-full p-1.5 opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  <Clear className="w-4 h-4 text-white" />
                </button>
              </div>
            )}
          </div>
        )}

        {/* Title Input */}
        <div className="px-4 pt-4">
          <input
            type="text"
            placeholder="Here is a title line. What do you think about......."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full bg-transparent text-white text-xl font-semibold placeholder-neutral-500 outline-none border-none"
          />
        </div>

        {/* Editor Area */}
        <div className="px-4 pt-2 min-h-[80px] relative">
          <TiptapEditor
            ref={editorRef}
            content={content || ''}
            onUpdate={setContent}
            className="mb-2"
          />

          {/*  Toolbar + Post Button Row */}
          <div className="flex items-center justify-between gap-4 m-2 ">
            {/* Toolbar */}
            {editorRef.current && (
              <ToolbarPlugin
                editor={editorRef.current}
                onImageUpload={setImage}
                onTriggerLinkPaste={() => {
                  setShowUrlInput(true);
                  setRepostUrl('');
                }}
                onCommentPaste={() => {
                  setShowCommentUrlInput(true);
                  setCommentUrl('');
                }}
              />
            )}

            {/* Post Button */}
            <button
              onClick={handlePublish}
              disabled={isSubmitDisabled}
              className="shrink-0 bg-primary text-background rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2"
            >
              {isReposting ? (
                <Loader className="animate-spin" />
              ) : (
                <UserCircle />
              )}
              {isReposting ? '' : 'Post'}
            </button>
          </div>

          {/* URL Input Dialogs */}
          {showUrlInput && (
            <div className="absolute top-4 left-2 z-20 bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 flex items-center gap-2 w-[90%]">
              <button onClick={handleInsertUrl}>
                <LinkPaste />
              </button>

              <input
                autoFocus
                value={repostUrl}
                onChange={(e) => setRepostUrl(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleInsertUrl()}
                placeholder="Paste or search for the relevant discussion or topic URL"
                className="bg-transparent text-white text-sm placeholder-neutral-400 outline-none flex-1"
              />
              {/* <button
                onClick={handleInsertUrl}
                className="text-green-400 hover:text-white"
                aria-label="Insert URL"
              >
                <Check className="w-4 h-4" />
              </button> */}
              <button
                onClick={() => {
                  setShowUrlInput(false);
                  setRepostUrl('');
                }}
                className="text-neutral-400 hover:text-white"
                aria-label="Cancel"
              >
                <Clear className="w-4 h-4" />
              </button>
            </div>
          )}

          {showCommentUrlInput && (
            <div className="absolute top-2/5 left-2 z-20 bg-neutral-800 border border-neutral-600 rounded-md px-3 py-2 flex items-center gap-2 w-[90%]">
              <button onClick={handleCommentUrl}>
                <CommentPaste />
              </button>

              <input
                autoFocus
                value={commentUrl}
                onChange={(e) => setCommentUrl(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleCommentUrl()}
                placeholder="Please paste or search for the comment to quote"
                className="bg-transparent text-white text-sm placeholder-neutral-400 outline-none flex-1"
              />
              {/* <button
                onClick={handleCommentUrl}
                className="text-green-400 hover:text-white"
                aria-label="Insert URL"
              >
                <Check className="w-4 h-4" />
              </button> */}
              <button
                onClick={() => {
                  setShowCommentUrlInput(false);
                  setCommentUrl('');
                }}
                className="text-neutral-400 hover:text-white"
                aria-label="Cancel"
              >
                <Clear className="w-4 h-4" />
              </button>
            </div>
          )}

          {/* Uploaded Image Preview */}
          {image && (
            <div className="px-4 pt-2">
              <div className="relative w-full aspect-video rounded-lg overflow-hidden">
                <Image
                  src={image}
                  alt="Uploaded content"
                  fill
                  className="object-cover"
                />
                <button
                  onClick={removeImage}
                  className="absolute top-2 right-2 bg-black/70 rounded-full p-1.5"
                >
                  <Clear className="w-4 h-4 text-white" />
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface RePostDraftContextType {
  originalFeedId: number | null;
  setOriginalFeedId: (id: number | null) => void;
  authorName?: string;
  authorProfileUrl?: string;
  setAuthorName: (name: string) => void;
  setAuthorProfileUrl: (url: string) => void;
  authorId: number | null;
  setAuthorId: (id: number | null) => void;
  industry?: string;
  setIndustry: (name: string) => void;
  expand: boolean;
  setExpand: (expand: boolean) => void;
  title: string;
  setTitle: (title: string) => void;
  content: string | null;
  setContent: (content: string | null) => void;
  image: string | null;
  setImage: (image: string | null) => void;
  feedcontent: string;
  setFeedContent: (content: string) => void;
  feedImageUrl: string | null;
  setFeedImageUrl: (url: string | null) => void;
  showUrlInput?: boolean;
  setShowUrlInput: (show: boolean) => void;
  repostUrl?: string;
  setRepostUrl: (url: string) => void;
  showCommentUrlInput?: boolean;
  setShowCommentUrlInput: (show: boolean) => void;
  commentUrl?: string;
  setCommentUrl: (url: string) => void;
  resetDraft: () => void;
}

const RePostDraftContext = createContext<RePostDraftContextType | undefined>(
  undefined,
);

export const RePostDraftProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [originalFeedId, setOriginalFeedId] = useState<number | null>(null);
  const [authorId, setAuthorId] = useState<number | null>(null);
  const [authorName, setAuthorName] = useState<string>();
  const [authorProfileUrl, setAuthorProfileUrl] = useState<string>();
  const [industry, setIndustry] = useState<string>();
  const [expand, setExpand] = useState(false);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(null);
  const [feedcontent, setFeedContent] = useState('');
  const [feedImageUrl, setFeedImageUrl] = useState<string | null>(null);
  const [showUrlInput, setShowUrlInput] = useState(false);
  const [repostUrl, setRepostUrl] = useState('');
  const [showCommentUrlInput, setShowCommentUrlInput] = useState(false);
  const [commentUrl, setCommentUrl] = useState('');

  const resetDraft = useCallback(() => {
    setOriginalFeedId(null);
    setAuthorId(null);
    setTitle('');
    setContent(null);
    setImage(null);
    setFeedContent('');
    setFeedImageUrl(null);
    setShowUrlInput(false);
    setRepostUrl('');
    setShowCommentUrlInput(false);
    setCommentUrl('');
    setExpand(false);
  }, []);

  const contextValue = {
    originalFeedId,
    setOriginalFeedId,
    authorId,
    setAuthorId,
    authorName,
    authorProfileUrl,
    setAuthorName,
    setAuthorProfileUrl,
    industry,
    setIndustry,
    expand,
    setExpand,
    title,
    setTitle,
    content,
    setContent,
    image,
    setImage,
    feedcontent,
    setFeedContent,
    feedImageUrl,
    setFeedImageUrl,
    showUrlInput,
    setShowUrlInput,
    repostUrl,
    setRepostUrl,
    showCommentUrlInput,
    setShowCommentUrlInput,
    commentUrl,
    setCommentUrl,
    resetDraft,
  };

  return (
    <RePostDraftContext.Provider value={contextValue}>
      {children}
    </RePostDraftContext.Provider>
  );
};

export const useRepostDraft = () => {
  const context = useContext(RePostDraftContext);
  if (!context) {
    throw new Error('useRepostDraft must be used within a RePostDraftProvider');
  }
  return context;
};
