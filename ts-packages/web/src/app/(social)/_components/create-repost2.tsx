// "use client"
// import { useState, useRef } from "react"
// import {
//   Bold,
//   Italic,
//   UnderlineIcon,
//   Strikethrough,
//   Type,
//   Highlighter,
//   ImageIcon,
//   LinkIcon,
//   Code,
//   TableIcon,
//   AlignLeft,
//   AlignCenter,
//   AlignRight,
//   List,
//   ListOrdered,
//   Quote,
//   ChevronDown,
//   Sparkles,
// } from "lucide-react"
// import { Button } from "@/components/ui/button"
// import { cn } from "@/lib/utils"

// interface SocialPostEditorProps {
//   className?: string
// }

// export default function SocialPostEditor({ className }: SocialPostEditorProps) {
//   const [title, setTitle] = useState("Here is a title line. What do you think about......")
//   const [topicUrl, setTopicUrl] = useState("")
//   const [commentQuote, setCommentQuote] = useState("")
//   const [isExpanded, setIsExpanded] = useState(true)
//   const editorRef = useRef<HTMLDivElement>(null)

//   const execCommand = (command: string, value?: string) => {
//     document.execCommand(command, false, value)
//     editorRef.current?.focus()
//   }

//   const toggleBold = () => execCommand("bold")
//   const toggleItalic = () => execCommand("italic")
//   const toggleUnderline = () => execCommand("underline")
//   const toggleStrike = () => execCommand("strikeThrough")
//   const setTextAlign = (alignment: string) =>
//     execCommand(`justify${alignment.charAt(0).toUpperCase() + alignment.slice(1)}`)
//   const toggleBulletList = () => execCommand("insertUnorderedList")
//   const toggleOrderedList = () => execCommand("insertOrderedList")
//   const insertLink = () => {
//     const url = prompt("Enter URL:")
//     if (url) execCommand("createLink", url)
//   }
//   const insertImage = () => {
//     const url = prompt("Enter image URL:")
//     if (url) execCommand("insertImage", url)
//   }

//   if (!isExpanded) {
//     return (
//       <div className={cn("w-full", className)}>
//         <div
//           className="bg-neutral-900 border border-neutral-700 rounded-lg p-4 cursor-pointer"
//           onClick={() => setIsExpanded(true)}
//         >
//           <div className="flex items-center gap-3">
//             <div className="w-8 h-8 rounded-full bg-neutral-700 flex items-center justify-center">
//               <span className="text-sm">👤</span>
//             </div>
//             <span className="text-neutral-400">Start a new post...</span>
//           </div>
//         </div>
//       </div>
//     )
//   }

//   return (
//     <div
//       className={cn(
//         "w-full bg-neutral-900 border-t-4 border-orange-500 border-x border-b border-neutral-700 rounded-t-lg overflow-hidden",
//         className,
//       )}
//     >
//       {/* Header */}
//       <div className="flex items-center justify-between p-4 border-b border-neutral-700">
//         <div className="flex items-center gap-3">
//           <div className="w-8 h-8 rounded-full bg-neutral-700 flex items-center justify-center">
//             <span className="text-sm">👤</span>
//           </div>
//           <div className="flex items-center gap-2">
//             <span className="text-white font-medium">Politician name</span>
//             <span className="text-yellow-500">👑</span>
//           </div>
//         </div>
//         <div className="flex items-center gap-4">
//           <div className="flex items-center gap-2 text-neutral-400">
//             <span>Crypto</span>
//             <ChevronDown size={16} />
//           </div>
//           <button onClick={() => setIsExpanded(false)} className="text-neutral-400 hover:text-white">
//             <ChevronDown size={20} />
//           </button>
//         </div>
//       </div>

//       {/* User Info Section */}
//       <div className="p-4 border-b border-neutral-700">
//         <div className="flex items-center gap-3 mb-3">
//           <div className="w-8 h-8 rounded-full bg-neutral-700 flex items-center justify-center">
//             <span className="text-sm">👤</span>
//           </div>
//           <div className="flex items-center gap-2">
//             <span className="text-white font-medium">Politician name</span>
//             <span className="text-yellow-500">👑</span>
//           </div>
//           <button className="text-neutral-400 hover:text-white ml-auto">
//             <ChevronDown size={16} />
//           </button>
//         </div>
//         <p className="text-neutral-400 text-sm">
//           Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings...{" "}
//           <span className="text-blue-400 cursor-pointer hover:underline">See more</span>
//         </p>
//       </div>

//       {/* Main Image */}
//       <div className="px-4 pt-4">
//         <div className="relative w-full h-64 rounded-lg overflow-hidden bg-gradient-to-br from-blue-100 to-gray-200">
//           <img
//             src="/placeholder.svg?height=256&width=800"
//             alt="Modern architecture building"
//             className="w-full h-full object-cover"
//           />
//         </div>
//       </div>

//       {/* Title Input */}
//       <div className="px-4 pt-4">
//         <input
//           type="text"
//           value={title}
//           onChange={(e) => setTitle(e.target.value)}
//           className="w-full bg-transparent text-white text-lg font-medium placeholder-neutral-500 outline-none border-none"
//           placeholder="Write a title..."
//         />
//       </div>

//       {/* Content Editor */}
//       <div className="px-4 pt-2">
//         <div
//           ref={editorRef}
//           contentEditable
//           className="min-h-[200px] text-neutral-300 text-[15px] leading-relaxed outline-none"
//           style={{ wordBreak: "break-word" }}
//           dangerouslySetInnerHTML={{
//             __html: `
//               <p>Thank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.</p>
//               <p>What I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under "Other."</p>
//             `,
//           }}
//         />
//       </div>

//       {/* URL and Quote Inputs */}
//       <div className="px-4 pt-4 space-y-3">
//         <div className="flex items-center gap-3 text-neutral-500">
//           <LinkIcon size={16} />
//           <input
//             type="text"
//             value={topicUrl}
//             onChange={(e) => setTopicUrl(e.target.value)}
//             placeholder="Please paste or search for the relevant discussion or topic URL"
//             className="flex-1 bg-transparent outline-none placeholder-neutral-500"
//           />
//         </div>
//         <div className="flex items-center gap-3 text-neutral-500">
//           <Quote size={16} />
//           <input
//             type="text"
//             value={commentQuote}
//             onChange={(e) => setCommentQuote(e.target.value)}
//             placeholder="Please paste or search for the comment to quote"
//             className="flex-1 bg-transparent outline-none placeholder-neutral-500"
//           />
//         </div>
//       </div>

//       {/* Toolbar */}
//       <div className="flex items-center justify-between p-4 border-t border-neutral-700 mt-4">
//         <div className="flex items-center gap-1">
//           <Button
//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Sparkles size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleBold}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Bold size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleItalic}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Italic size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleUnderline}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <UnderlineIcon size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleStrike}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Strikethrough size={16} />
//           </Button>
//           <Button

//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Type size={16} />
//           </Button>
//           <Button
//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Highlighter size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={insertImage}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <ImageIcon size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={insertLink}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <LinkIcon size={16} />
//           </Button>
//           <Button
//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Code size={16} />
//           </Button>
//           <Button
//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <TableIcon size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={() => setTextAlign("left")}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <AlignLeft size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={() => setTextAlign("center")}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <AlignCenter size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={() => setTextAlign("right")}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <AlignRight size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleBulletList}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <List size={16} />
//           </Button>
//           <Button
//             size="sm"
//             onClick={toggleOrderedList}
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <ListOrdered size={16} />
//           </Button>
//           <Button
//             size="sm"
//             className="h-8 w-8 p-0 text-neutral-400 hover:text-white hover:bg-neutral-800"
//           >
//             <Quote size={16} />
//           </Button>
//         </div>

//         <div className="flex items-center gap-3">
//           <Button
//             variant="outline"
//             className="bg-transparent border-neutral-600 text-neutral-300 hover:bg-neutral-800 hover:text-white"
//           >
//             Save
//           </Button>
//           <Button className="bg-orange-500 hover:bg-orange-600 text-white">Post</Button>
//         </div>
//       </div>
//     </div>
//   )
// }

'use client';
import {
  createContext,
  useContext,
  useState,
  useCallback,
  useEffect,
  useRef,
} from 'react';
import { X, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUserInfo } from '../_hooks/user';
import { Button } from '@/components/ui/button';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import Certified from '@/assets/icons/certified.svg';
import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';

import {
  LexicalEditor,
  EditorState,
  $getRoot,
  $createParagraphNode,
} from 'lexical';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import Image from 'next/image';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { Feed, FeedStatus, FeedType } from '@/lib/api/models/feeds';
import { UrlType } from '@/lib/api/models/feeds/update-draft-request';
import { useQueryClient } from '@tanstack/react-query';
import { postByUserIdQk } from '../_hooks/use-posts';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import ToolbarPlugin from '@/components/toolbar/toolbar';
import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { QK_GET_FEED_BY_FEED_ID } from '@/constants';
import { Folder, CommentIcon } from '@/components/icons';
import { useQuery } from '@tanstack/react-query';

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

export function CreateRePost() {
  const {
    expand,
    setExpand,
    title,
    setTitle,
    content,
    setContent,
    image,
    setImage,
    feedcontent,
    feedImageUrl,
    setFeedImageUrl,
    publishPost,
    savePost,
    status,
    isPublishedPost,
    resetDraft,
    authorName,
    authorProfileUrl,
  } = useRepostDraft();

  const { data: userInfo } = useUserInfo();
  const editorRef = useRef<LexicalEditor | null>(null);

  const handleLexicalChange = (editorState: EditorState) => {
    editorState.read(() => {
      const html = $generateHtmlFromNodes(editorRef.current!, null);
      setContent(html);
    });
  };

  const removeImage = () => setImage(null);
  const removeQuotedImage = () => setFeedImageUrl(null);

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
        console.error('Error parsing HTML:', error);
      }
    },
    [],
  );

  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;

    const currentHtml = editor
      .getEditorState()
      .read(() => $generateHtmlFromNodes(editor, null));
    if (!content || content !== currentHtml) {
      editor.update(() => {
        createEditorStateFromHTML(editor, content ?? '');
      });
    }
  }, [editorRef, content, createEditorStateFromHTML]);

  return (
    <div className={`flex flex-col w-full ${!expand ? 'hidden' : 'block'}`}>
      <LexicalComposer
        initialConfig={{
          namespace: 'CreatePostEditor',
          theme: editorTheme,
          onError: (error) => console.error(error),
        }}
      >
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

            {/* <button
              onClick={resetDraft}
              className="text-neutral-400 hover:text-white"
            >
              <X size={20} />
            </button> */}
            <div className={cn('cursor-pointer')} onClick={resetDraft}>
              <DoubleArrowDown />
            </div>
          </div>

          {/* Quoted Content Section */}

          {(feedcontent || feedImageUrl) && (
            <div className="px-4 pt-2 pb-3   bg-neutral-800 rounded-md mx-4 my-4 ">
              {/* <div className="text-sm text-neutral-400 mb-2">Reposting:</div> */}

              <div className="flex items-center gap-3">
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
                  <span className="text-white font-medium text-lg">
                    {authorName|| 'Anonymous'}
                  </span>
                </div>
                <Certified className="size-5" />
              </div>

              {feedcontent && (
                <div
                  className="prose prose-invert text-sm p-3 bg-neutral-800 border-orange-500 mb-3"
                  dangerouslySetInnerHTML={{ __html: feedcontent }}
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
                    <X className="w-4 h-4 text-white" />
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
            <RichTextPlugin
              contentEditable={
                <ContentEditable className="outline-none min-h-[100px] text-neutral-300" />
              }
              placeholder={
                <div className="absolute top-0 text-neutral-500 pointer-events-none">
                  Type your commentary here...
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
                  <X className="w-4 h-4 text-white" />
                </button>
              </div>
            </div>
          )}

          {/* Toolbar */}
          <div className="flex items-center justify-between p-4 border-t border-neutral-700">
            <ToolbarPlugin onImageUpload={setImage} />

            <Button
              onClick={isPublishedPost ? savePost : publishPost}
              disabled={isSubmitDisabled}
            >
              {status === 'publishing' || status === 'saving' ? (
                <Loader2 className="animate-spin mr-2" />
              ) : null}
              {isPublishedPost ? 'Update' : 'Post'}
            </Button>
          </div>
        </div>
      </LexicalComposer>
    </div>
  );
}

// Context Types
type DraftStatus = 'idle' | 'saving' | 'publishing' | 'error';

interface RePostDraftContextType {
  originalFeedId: number | null;
  setOriginalFeedId: (id: number | null) => void;

  authorName?: string;
  authorProfileUrl?: string;
  setAuthorName: (name: string) => void;
  setAuthorProfileUrl: (url: string) => void;

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
  status: DraftStatus;
  isPublishedPost: boolean;
  publishPost: () => Promise<void>;
  savePost: () => Promise<void>;
  newDraft: () => void;

  resetDraft: () => void;
}

const RePostDraftContext = createContext<RePostDraftContextType | undefined>(
  undefined,
);

export const RePostDraftProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [originalFeedId, setOriginalFeedId] = useState<number | null>(null);

  const [authorName, setAuthorName] = useState<string | undefined>();
  const [authorProfileUrl, setAuthorProfileUrl] = useState<
    string | undefined
  >();

  const router = useRouter();
  const queryClient = useQueryClient();
  const { get, post } = useApiCall();
  const { data: user } = useUserInfo();

  const [expand, setExpand] = useState(false);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(null);
  const [feedcontent, setFeedContent] = useState('');
  const [feedImageUrl, setFeedImageUrl] = useState<string | null>(null);
  const [status, setStatus] = useState<DraftStatus>('idle');
  const [isPublishedPost, setIsPublishedPost] = useState(false);
  const [draftId, setDraftId] = useState<number | null>(null);

  const newDraft = useCallback(() => {
    setTitle('');
    setContent(null);
    setImage(null);
    // setFeedContent('');
    // setFeedImageUrl(null);
    setIsPublishedPost(false);
    setStatus('idle');
    setExpand(true);
  }, []);

  const saveDraft = useCallback(async () => {
    if (!title.trim() || !content || status !== 'idle') return;

    setStatus('saving');
    try {
      const url = image ? image : '';
      const urlType = image ? UrlType.Image : UrlType.None;

      if (isPublishedPost) {
        await post(ratelApi.feeds.editPost(draftId!), {
          type: FeedType.Repost, // Important: use Repost type
          html_contents: content,
          title,
          url,
          url_type: urlType,
          original_feed_id: originalFeedId, //  Add this
        });
      } else {
        const data = await post(ratelApi.feeds.createDraft(), {
          type: FeedType.Post,
          user_id: user?.id,
          html_contents: content,
          title,
          url,
          url_type: urlType,
        });
        setDraftId(data.id);
      }

      queryClient.invalidateQueries({
        queryKey: [QK_GET_FEED_BY_FEED_ID, draftId],
      });
    } catch (error) {
      console.error('Failed to save draft:', error);
      setStatus('error');
    } finally {
      setStatus('idle');
    }
  }, [
    title,
    content,
    image,
    draftId,
    isPublishedPost,
    post,
    user?.id,
    queryClient,
    status,
  ]);

  const publishPost = useCallback(async () => {
    if (!title.trim() || !content || status !== 'idle') return;

    setStatus('publishing');
    try {
      await saveDraft();
      await post(ratelApi.feeds.publishDraft(draftId!));
      router.push(route.threadByFeedId(draftId!));

      resetDraft();
    } catch (error) {
      console.error('Failed to publish post:', error);
      setStatus('error');
    } finally {
      setStatus('idle');
    }
  }, [title, content, draftId, saveDraft, post, router, status]);

  const savePost = useCallback(async () => {
    await saveDraft();
    setExpand(false);
  }, [saveDraft]);

  const resetDraft = () => {
    setTitle('');
    setContent(null);
    setImage(null);
    setFeedContent('');
    setFeedImageUrl(null);
    setDraftId(null);
    setIsPublishedPost(false);
    setStatus('idle');
    setExpand(false); // optionally collapse editor too
  };

  const contextValue = {
    originalFeedId,
    setOriginalFeedId,

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
    status,
    isPublishedPost,
    publishPost,
    savePost,
    newDraft,
    resetDraft, // <-- Add this

    authorName,
    authorProfileUrl,
    setAuthorName,
    setAuthorProfileUrl,
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
