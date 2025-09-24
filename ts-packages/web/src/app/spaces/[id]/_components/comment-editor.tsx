// 'use client';

// import { useState, useRef } from 'react';
// import { X } from 'lucide-react';
// import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
// import { Button } from '@/components/ui/button';
// import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
// import { showSuccessToast, showErrorToast } from '@/components/custom-toast/toast';
// import { useMutation } from '@tanstack/react-query';
// import { ratelApi } from '@/lib/api/ratel_api';
// import CommentIcon from '@/assets/icons/comment.svg';
// import { Editor } from '@tiptap/core';
// import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
// import { cn } from '@/lib/utils';
// import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
// import { Loader } from '@/components/icons';
// import UserCircle from '@/assets/icons/user-circle.svg';
// import SaveIcon from '@/assets/icons/save.svg';

// interface SpaceCommentEditorProps {
//     showCommentEditor: boolean;
//     setShowCommentEditor: (show: boolean) => void;
//     commentCount: number;
//     t: (key: string) => string;
//     spaceId?: number;
//     onCommentPosted?: () => void;
// }

// export default function SpaceCommentEditor({
//     showCommentEditor,
//     setShowCommentEditor,
//     commentCount,
//     t,
//     spaceId = 0,
//     onCommentPosted = () => { }
// }: SpaceCommentEditorProps) {
//     const [content, setContent] = useState('');
//     const [showUrlInput, setShowUrlInput] = useState(false);
//     const [url, setUrl] = useState('');
//     const { data: user } = useSuspenseUserInfo();
//     const editorRef = useRef<Editor | null>(null);
//     const [editorReady, setEditorReady] = useState(false);

//     const { mutate: createComment, isPending: isSubmitting } = useMutation({
//         mutationFn: async (commentData: { post_id: number; content: string }) => {
//             const response = await fetch('/api/feeds/comment', {
//                 method: 'POST',
//                 headers: {
//                     'Content-Type': 'application/json',
//                 },
//                 body: JSON.stringify(commentData),
//             });

//             if (!response.ok) {
//                 throw new Error('Failed to post comment');
//             }

//             return response.json();
//         },
//         onSuccess: () => {
//             showSuccessToast('Comment posted successfully');
//             setContent('');
//             onCommentPosted();
//             setShowCommentEditor(false);
//         },
//         onError: (error: Error) => {
//             console.error('Failed to post comment:', error);
//             showErrorToast('Failed to post comment');
//         },
//     });

//     const handleSubmit = async () => {
//         if (!content.trim() || !spaceId) return;

//         try {
//             await createComment({
//                 post_id: spaceId,
//                 content: content,
//             });
//         } catch (error) {
//             console.error('Error submitting comment:', error);
//             showErrorToast('Failed to post comment');
//         }
//     };

//     const handleInsertUrl = () => {
//         const urlToInsert = url?.trim();
//         if (!urlToInsert) return;

//         editorRef.current?.chain().focus().insertContent(urlToInsert).run();
//         setShowUrlInput(false);
//         setUrl('');
//     };

//     return (
//         <div className="relative">
//             <div
//                 className="flex items-center gap-1 cursor-pointer"
//                 onClick={() => setShowCommentEditor(true)}
//             >
//                 <CommentIcon className="w-5 h-5" />
//                 <span className="text-sm font-medium text-foreground">
//                     {commentCount}
//                 </span>
//             </div>

//             {showCommentEditor && (
//                 <div
//                     className="fixed inset-0 z-50 flex items-end justify-center "
//                     onClick={() => setShowCommentEditor(false)}
//                 >
//                     <div
//                         className="w-full bg-comment-box-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden max-w-6xl"
//                         onClick={e => e.stopPropagation()}
//                     >
//                         <div className="p-4 flex justify-between items-center">
//                             <h3 className=" font-medium text-foreground">Add a comment</h3>
//                             <button
//                                 onClick={() => setShowCommentEditor(false)}
//                                 className=""
//                             >
//                                 <DoubleArrowDown className="[&>path]:stroke-text-primary" />
//                             </button>
//                         </div>

//                         <div className="p-4">
//                             <div className="">
//                                 <div className="min-h-[80px] text-text-primary rounded p-2">
//                                     <TiptapEditor
//                                         content={content}
//                                         onUpdate={setContent}
//                                         editable={!isSubmitting}
//                                         ref={editorRef}

//                                         onCreate={() => setEditorReady(true)}
//                                     />
//                                 </div>

//                                 <div className='flex items-center justify-between gap-4 m-2'>
//                                     {
//                                         editorReady && (
//                                             <ToolbarPlugin
//                                                 editor={editorRef.current}
//                                                 onImageUpload={(url: string) => {
//                                                     editorRef.current?.commands.insertContent(
//                                                         `<img src="${url}" alt="Uploaded image" />`
//                                                     );
//                                                 }}
//                                                 onUrlInsert={() => setShowUrlInput(true)}
//                                             />
//                                         )
//                                     }

//                                     <div className='flex flex-row gap-4'>
//                                     <button
//                                         onClick={handleSubmit}
//                                         className="shrink-0  text-foreground rounded-full  px-4 py-2 font-bold flex items-center gap-x-2"
//                                     >
//                                         {isSubmitting ? (
//                                             <Loader className="animate-spin" />
//                                         ) : (
//                                             <SaveIcon />
//                                         )}
//                                         {isSubmitting ? '' : 'Save'}
//                                     </button>

//                                     <button
//                                         onClick={handleSubmit}
//                                         className="shrink-0 bg-primary text-text-third rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2"
//                                     >
//                                         {isSubmitting ? (
//                                             <Loader className="animate-spin" />
//                                         ) : (
//                                             <UserCircle />
//                                         )}
//                                         {isSubmitting ? '' : 'Post'}
//                                     </button>

//                                     </div>

//                                 </div>

//                                 {showUrlInput && (
//                                     <div className="p-4 border-t border-gray-200 dark:border-gray-700">
//                                         <div className="flex gap-2">
//                                             <input
//                                                 type="text"
//                                                 value={url}
//                                                 onChange={(e) => setUrl(e.target.value)}
//                                                 placeholder="Paste URL here"
//                                                 className="flex-1 p-2 border rounded dark:bg-gray-700 dark:border-gray-600"
//                                                 onKeyDown={(e) => {
//                                                     if (e.key === 'Enter') {
//                                                         e.preventDefault();
//                                                         handleInsertUrl();
//                                                     }
//                                                 }}
//                                             />
//                                             <Button
//                                                 onClick={handleInsertUrl}
//                                                 variant="outline"
//                                                 size="sm"
//                                             >
//                                                 Insert
//                                             </Button>
//                                         </div>
//                                     </div>
//                                 )}

//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             )}
//         </div>
//     );
// }

// 'use client';

// import { useState, useRef } from 'react';
// import { X } from 'lucide-react';
// import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
// import { Button } from '@/components/ui/button';
// import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
// import { showSuccessToast, showErrorToast } from '@/components/custom-toast/toast';
// import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
// import CommentIcon from '@/assets/icons/comment.svg';
// import { Editor } from '@tiptap/core';
// import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
// import { cn } from '@/lib/utils';
// import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
// import { Loader } from '@/components/icons';
// import UserCircle from '@/assets/icons/user-circle.svg';
// import SaveIcon from '@/assets/icons/save.svg';

// interface SpaceCommentEditorProps {
//     showCommentEditor: boolean;
//     setShowCommentEditor: (show: boolean) => void;
//     commentCount: number;
//     t: (key: string) => string;
//     spaceId?: number;
//     onCommentPosted?: () => void;
// }

// export default function SpaceCommentEditor({
//     showCommentEditor,
//     setShowCommentEditor,
//     commentCount,
//     t,
//     spaceId = 0,
//     onCommentPosted = () => { }
// }: SpaceCommentEditorProps) {
//     const [content, setContent] = useState('');
//     const [showUrlInput, setShowUrlInput] = useState(false);
//     const [url, setUrl] = useState('');
//     const { data: user } = useSuspenseUserInfo();
//     const editorRef = useRef<Editor | null>(null);
//     const [editorReady, setEditorReady] = useState(false);

//     // Use the useDraftMutations hook
//     const { createComment } = useDraftMutations(user?.id || 0);

//     const handleSubmit = async () => {
//         if (!content.trim() || !spaceId || !user?.id) return;

//         try {
//             await createComment.mutateAsync({
//                 userId: user.id,
//                 parentId: spaceId, // Changed from postId to parentId to match the expected parameter name
//                 postId: spaceId,
//                 content: content,
//             });

//             // Clear the editor and close it
//             setContent('');
//             setShowCommentEditor(false);
//             onCommentPosted?.();
//             showSuccessToast('Comment posted successfully');
//         } catch (error) {
//             console.error('Error submitting comment:', error);
//             showErrorToast('Failed to post comment');
//         }
//     };

//     const handleInsertUrl = () => {
//         const urlToInsert = url?.trim();
//         if (!urlToInsert) return;

//         editorRef.current?.chain().focus().insertContent(urlToInsert).run();
//         setShowUrlInput(false);
//         setUrl('');
//     };

//     return (
//         <div className="relative">
//             <div
//                 className="flex items-center gap-1 cursor-pointer"
//                 onClick={() => setShowCommentEditor(true)}
//             >
//                 <CommentIcon className="w-5 h-5" />
//                 <span className="text-sm font-medium text-foreground">
//                     {commentCount}
//                 </span>
//             </div>

//             {showCommentEditor && (
//                 <div
//                     className="fixed inset-0 z-50 flex items-end justify-center"
//                     onClick={() => setShowCommentEditor(false)}
//                 >
//                     <div
//                         className="w-full bg-comment-box-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden max-w-6xl"
//                         onClick={e => e.stopPropagation()}
//                     >
//                         <div className="p-4 flex justify-between items-center">
//                             <h3 className="font-medium text-foreground">Add a comment</h3>
//                             <button
//                                 onClick={() => setShowCommentEditor(false)}
//                                 className=""
//                             >
//                                 <DoubleArrowDown className="[&>path]:stroke-text-primary" />
//                             </button>
//                         </div>

//                         <div className="p-4">
//                             <div className="">
//                                 <div className="min-h-[80px] text-text-primary rounded p-2">
//                                     <TiptapEditor
//                                         content={content}
//                                         onUpdate={setContent}
//                                         editable={!createComment.isPending}
//                                         ref={editorRef}
//                                         onCreate={() => setEditorReady(true)}
//                                     />
//                                 </div>

//                                 <div className='flex items-center justify-between gap-4 m-2'>
//                                     {editorReady && (
//                                         <ToolbarPlugin
//                                             editor={editorRef.current}
//                                             onImageUpload={(url: string) => {
//                                                 editorRef.current?.commands.insertContent(
//                                                     `<img src="${url}" alt="Uploaded image" />`
//                                                 );
//                                             }}
//                                             onUrlInsert={() => setShowUrlInput(true)}
//                                         />
//                                     )}

//                                     <div className='flex flex-row gap-4'>
//                                         <button
//                                             onClick={handleSubmit}
//                                             disabled={!content.trim() || createComment.isPending}
//                                             className="shrink-0 bg-primary text-text-third rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2"
//                                         >
//                                             {createComment.isPending ? (
//                                                 <Loader className="animate-spin" />
//                                             ) : (
//                                                 <UserCircle />
//                                             )}
//                                             {createComment.isPending ? '' : 'Post'}
//                                         </button>
//                                     </div>
//                                 </div>

//                                 {showUrlInput && (
//                                     <div className="p-4 border-t border-gray-200 dark:border-gray-700">
//                                         <div className="flex gap-2">
//                                             <input
//                                                 type="text"
//                                                 value={url}
//                                                 onChange={(e) => setUrl(e.target.value)}
//                                                 placeholder="Paste URL here"
//                                                 className="flex-1 p-2 border rounded dark:bg-gray-700 dark:border-gray-600"
//                                                 onKeyDown={(e) => {
//                                                     if (e.key === 'Enter') {
//                                                         e.preventDefault();
//                                                         handleInsertUrl();
//                                                     }
//                                                 }}
//                                             />
//                                             <Button
//                                                 onClick={handleInsertUrl}
//                                                 variant="outline"
//                                                 size="sm"
//                                             >
//                                                 Insert
//                                             </Button>
//                                         </div>
//                                     </div>
//                                 )}
//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             )}
//         </div>
//     );
// }

// 'use client';

// import { useState, useRef, useEffect } from 'react';
// import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
// import { Button } from '@/components/ui/button';
// import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
// import {
//   showSuccessToast,
//   showErrorToast,
// } from '@/components/custom-toast/toast';
// import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
// import CommentIcon from '@/assets/icons/comment.svg';
// import { Editor } from '@tiptap/core';
// import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
// import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
// import { Loader } from '@/components/icons';
// import UserCircle from '@/assets/icons/user-circle.svg';
// import SaveIcon from '@/assets/icons/save.svg';

// interface SpaceCommentEditorProps {
//   showCommentEditor: boolean;
//   setShowCommentEditor: (show: boolean) => void;
//   commentCount: number;
//   t: (key: string) => string;
//   spaceId?: number;
//   onCommentPosted?: () => void;
// }

// export default function SpaceCommentEditor({
//   showCommentEditor,
//   setShowCommentEditor,
//   commentCount,
//   t,
//   spaceId = 0,
//   onCommentPosted = () => {},
// }: SpaceCommentEditorProps) {
//   const [content, setContent] = useState('');
//   const [showUrlInput, setShowUrlInput] = useState(false);
//   const [url, setUrl] = useState('');
//   const { data: user } = useSuspenseUserInfo();
//   const editorRef = useRef<Editor | null>(null);
//   const [editorReady, setEditorReady] = useState(false);

//   // Use the useDraftMutations hook
//   const { createComment } = useDraftMutations(user?.id || 0);

//   // Debug effect
//   useEffect(() => {
//     console.log('Comment editor mounted', {
//       spaceId,
//       userId: user?.id,
//       editorReady,
//       contentLength: content.length,
//     });
//   }, [spaceId, user?.id, editorReady, content]);

//   const handleSubmit = async () => {
//     console.log('Submit clicked', {
//       content: content.trim(),
//       spaceId,
//       userId: user?.id,
//     });

//     if (!content.trim() || !spaceId || !user?.id) {
//       console.log('Validation failed', {
//         hasContent: !!content.trim(),
//         hasSpaceId: !!spaceId,
//         hasUserId: !!user?.id,
//       });
//       return;
//     }

//     try {
//       console.log('Attempting to submit comment...');

//       const result = await createComment.mutateAsync({
//         userId: user.id,
//         parentId: spaceId,
//         postId: spaceId,
//         content: content,
//       });

//       console.log('Comment submitted successfully', result);

//       // Clear the editor and close it
//       setContent('');
//       setShowCommentEditor(false);
//       onCommentPosted();
//       showSuccessToast('Comment posted successfully');
//     } catch (error) {
//       console.error('Error submitting comment:', error);
//       showErrorToast('Failed to post comment');
//     }
//   };

//   const handleInsertUrl = () => {
//     const urlToInsert = url?.trim();
//     if (!urlToInsert) return;

//     editorRef.current?.chain().focus().insertContent(urlToInsert).run();
//     setShowUrlInput(false);
//     setUrl('');
//   };

//   return (
//     <div className="relative">
//       <div
//         className="flex items-center gap-1 cursor-pointer"
//         onClick={() => setShowCommentEditor(true)}
//       >
//         <CommentIcon className="w-5 h-5" />
//         <span className="text-sm font-medium text-foreground">
//           {commentCount}
//         </span>
//       </div>

//       {showCommentEditor && (
//         <div
//           className="fixed inset-0 z-50 flex items-end justify-center"
//           onClick={() => setShowCommentEditor(false)}
//         >
//           <div
//             className="w-full bg-comment-box-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden max-w-6xl"
//             onClick={(e) => e.stopPropagation()}
//           >
//             <div className="p-4 flex justify-between items-center">
//               <h3 className="font-medium text-foreground">Add a comment</h3>
//               <button onClick={() => setShowCommentEditor(false)} className="">
//                 <DoubleArrowDown className="[&>path]:stroke-text-primary" />
//               </button>
//             </div>

//             <div className="p-4">
//               <div className="">
//                 <div className="min-h-[80px] text-text-primary rounded p-2">
//                   <TiptapEditor
//                     content={content}
//                     onUpdate={setContent}
//                     editable={!createComment.isPending}
//                     ref={editorRef}
//                     onCreate={() => setEditorReady(true)}
//                   />
//                 </div>

//                 <div className="flex items-center justify-between gap-4 m-2">
//                   {editorReady && (
//                     <ToolbarPlugin
//                       editor={editorRef.current}
//                       onImageUpload={(url: string) => {
//                         editorRef.current?.commands.insertContent(
//                           `<img src="${url}" alt="Uploaded image" />`,
//                         );
//                       }}
//                       onUrlInsert={() => setShowUrlInput(true)}
//                     />
//                   )}

//                   <div className="flex flex-row gap-4">

//                     {/* TODO: To be updated with actuall draft save logic for comment */}

//                     <button className="shrink-0  text-foreground rounded-full  px-4 py-2 font-bold flex items-center gap-x-2">
//                      <SaveIcon />
//                       Save
//                     </button>
//                     <button
//                       onClick={handleSubmit}
//                       disabled={!content.trim() || createComment.isPending}
//                       className={`shrink-0 bg-primary text-text-third rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2 ${
//                         createComment.isPending ? 'opacity-70' : ''
//                       }`}
//                     >
//                       {createComment.isPending ? (
//                         <Loader className="animate-spin" />
//                       ) : (
//                         <UserCircle />
//                       )}
//                       {createComment.isPending ? 'Posting...' : 'Post'}
//                     </button>
//                   </div>
//                 </div>

//                 {showUrlInput && (
//                   <div className="p-4 border-t border-gray-200 dark:border-gray-700">
//                     <div className="flex gap-2">
//                       <input
//                         type="text"
//                         value={url}
//                         onChange={(e) => setUrl(e.target.value)}
//                         placeholder="Paste URL here"
//                         className="flex-1 p-2 border rounded dark:bg-gray-700 dark:border-gray-600"
//                         onKeyDown={(e) => {
//                           if (e.key === 'Enter') {
//                             e.preventDefault();
//                             handleInsertUrl();
//                           }
//                         }}
//                       />
//                       <Button
//                         onClick={handleInsertUrl}
//                         variant="outline"
//                         size="sm"
//                       >
//                         Insert
//                       </Button>
//                     </div>
//                   </div>
//                 )}
//               </div>
//             </div>
//           </div>
//         </div>
//       )}
//     </div>
//   );
// }





'use client';

import { useState, useRef } from 'react';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { Button } from '@/components/ui/button';
import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
import { showSuccessToast, showErrorToast } from '@/components/custom-toast/toast';
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
  t,
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
      showErrorToast('Failed to post comment');
    }
  };

  // Insert a regular link
  const handleInsertUrl = () => {
    const urlToInsert = url?.trim();
    if (!urlToInsert) return;
    editorRef.current?.chain().focus().insertContent(urlToInsert).run();
    setShowUrlInput(false);
    setUrl('');
  };

  // Insert a quoted comment (could be styled differently if needed)
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
            onClick={e => e.stopPropagation()}
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
                        "shrink-0 bg-primary text-text-third rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2",
                        createComment.isPending && "opacity-70"
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
                      onChange={e => setUrl(e.target.value)}
                      onKeyDown={e => e.key === 'Enter' && handleInsertUrl()}
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
                      onChange={e => setCommentUrl(e.target.value)}
                      onKeyDown={e => e.key === 'Enter' && handleInsertCommentUrl()}
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