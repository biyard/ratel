// 'use client';

// import { useState } from 'react';
// import { X, MessageSquare } from 'lucide-react';
// import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
// import { Button } from '@/components/ui/button';
// import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
// import { showSuccessToast, showErrorToast } from '@/components/custom-toast/toast';
// import { useMutation } from '@tanstack/react-query';
// import { ratelApi } from '@/lib/api/ratel_api';
// import CommentIcon from '@/assets/icons/comment.svg';

// interface SpaceCommentEditorProps {
//   showCommentEditor: boolean;
//   setShowCommentEditor: (show: boolean) => void;
//   commentCount: number;
//   t: (key: string) => string;
//   spaceId?: number; // Make optional for backward compatibility
//   onCommentPosted?: () => void; // Make optional for backward compatibility
// }

// export default function SpaceCommentEditor({ 
//   showCommentEditor, 
//   setShowCommentEditor, 
//   commentCount, 
//   t,
//   spaceId = 0, // Default value for backward compatibility
//   onCommentPosted = () => {} // Default empty function for backward compatibility
// }: SpaceCommentEditorProps) {
//   const [content, setContent] = useState('');
//   const { data: user } = useSuspenseUserInfo();

//   const { mutate: createComment, isPending: isSubmitting } = useMutation({
//     mutationFn: async (commentData: { post_id: number; content: string }) => {
//       const response = await fetch('/api/feeds/comment', {
//         method: 'POST',
//         headers: {
//           'Content-Type': 'application/json',
//         },
//         body: JSON.stringify(commentData),
//       });

//       if (!response.ok) {
//         throw new Error('Failed to post comment');
//       }

//       return response.json();
//     },
//     onSuccess: () => {
//       showSuccessToast('Comment posted successfully');
//       setContent('');
//       onCommentPosted?.();
//       setShowCommentEditor(false);
//     },
//     onError: (error: Error) => {
//       console.error('Failed to post comment:', error);
//       showErrorToast('Failed to post comment');
//     },
//   });

//   const handleSubmit = async () => {
//     if (!content.trim() || !spaceId) return;

//     try {
//       await createComment({
//         post_id: spaceId,
//         content: content,
//       });

//       // Refresh comments count
//       onCommentPosted();
//       setShowCommentEditor(false);
//       setContent('');
//     } catch (error) {
//       console.error('Error submitting comment:', error);
//       showErrorToast('Failed to post comment');
//     }
//   };

//   return (
//     <div className="relative">
//       <div className="flex items-center gap-1 cursor-pointer" onClick={() => setShowCommentEditor(true)}>
//         <CommentIcon className="w-5 h-5" />
//         <span className="text-sm font-medium">
//           {commentCount}
//         </span>
//       </div>

//       {showCommentEditor && (
//         <div className="fixed inset-0 bg-black/50 z-50 flex items-end justify-center p-4" onClick={() => setShowCommentEditor(false)}>
//           <div className="w-full max-w-2xl bg-white dark:bg-gray-900 rounded-t-lg shadow-xl" onClick={e => e.stopPropagation()}>
//             <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
//               <h3 className="text-lg font-medium">Add a comment</h3>
//               <button 
//                 onClick={() => setShowCommentEditor(false)}
//                 className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
//               >
//                 <X className="w-5 h-5" />
//               </button>
//             </div>
//             <div className="p-4">
//               <div className="bg-white dark:bg-gray-800 rounded-lg p-4">
//                 <div className="min-h-[120px] border rounded p-2">
//                   <TiptapEditor
//                     content={content}
//                     onUpdate={(value) => setContent(value)}
//                     editable={!isSubmitting}
//                   />
//                 </div>

//                 <div className="flex justify-end mt-4 space-x-3">
//                   <Button 
//                     variant="outline" 
//                     onClick={() => setShowCommentEditor(false)}
//                     disabled={isSubmitting}
//                   >
//                     Cancel
//                   </Button>
//                   <Button 
//                     onClick={handleSubmit}
//                     disabled={!content.trim() || isSubmitting}
//                     className="bg-primary hover:bg-primary/90"
//                   >
//                     {isSubmitting ? 'Posting...' : 'Post Comment'}
//                   </Button>
//                 </div>
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
import { X } from 'lucide-react';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { Button } from '@/components/ui/button';
import { TiptapEditor } from '@/components/text-editor/tiptap-editor';
import { showSuccessToast, showErrorToast } from '@/components/custom-toast/toast';
import { useMutation } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import CommentIcon from '@/assets/icons/comment.svg';
import { Editor } from '@tiptap/core';
import ToolbarPlugin from '@/components/toolbar/toolbar-repost';
import { cn } from '@/lib/utils';
import DoubleArrowDown from '@/assets/icons/double-arrow-down.svg';
import { Loader } from '@/components/icons';
import UserCircle from '@/assets/icons/user-circle.svg';

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
    onCommentPosted = () => { }
}: SpaceCommentEditorProps) {
    const [content, setContent] = useState('');
    const [showUrlInput, setShowUrlInput] = useState(false);
    const [url, setUrl] = useState('');
    const { data: user } = useSuspenseUserInfo();
    const editorRef = useRef<Editor | null>(null);

    const { mutate: createComment, isPending: isSubmitting } = useMutation({
        mutationFn: async (commentData: { post_id: number; content: string }) => {
            const response = await fetch('/api/feeds/comment', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(commentData),
            });

            if (!response.ok) {
                throw new Error('Failed to post comment');
            }

            return response.json();
        },
        onSuccess: () => {
            showSuccessToast('Comment posted successfully');
            setContent('');
            onCommentPosted();
            setShowCommentEditor(false);
        },
        onError: (error: Error) => {
            console.error('Failed to post comment:', error);
            showErrorToast('Failed to post comment');
        },
    });

    const handleSubmit = async () => {
        if (!content.trim() || !spaceId) return;

        try {
            await createComment({
                post_id: spaceId,
                content: content,
            });
        } catch (error) {
            console.error('Error submitting comment:', error);
            showErrorToast('Failed to post comment');
        }
    };

    const handleInsertUrl = () => {
        const urlToInsert = url?.trim();
        if (!urlToInsert) return;

        editorRef.current?.chain().focus().insertContent(urlToInsert).run();
        setShowUrlInput(false);
        setUrl('');
    };

    return (
        <div className="relative">
            <div
                className="flex items-center gap-1 cursor-pointer"
                onClick={() => setShowCommentEditor(true)}
            >
                <CommentIcon className="w-5 h-5" />
                <span className="text-sm font-medium">
                    {commentCount}
                </span>
            </div>

            {showCommentEditor && (
                <div
                    className="fixed inset-0 bg-black/50 z-50 flex items-end justify-center p-4"
                    onClick={() => setShowCommentEditor(false)}
                >
                    <div
                        className="w-full bg-component-bg border-t-6 border-x border-b border-primary rounded-t-lg overflow-hidden max-w-4xl"
                        onClick={e => e.stopPropagation()}
                    >
                        <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
                            <h3 className="text-lg font-medium">Add a comment</h3>
                            <button
                                onClick={() => setShowCommentEditor(false)}
                                className=""
                            >
                                <DoubleArrowDown className="w-5 h-5" />
                            </button>
                        </div>

                        <div className="p-4">
                            <div className="">
                                <div className="min-h-[120px] border rounded p-2">
                                    <TiptapEditor
                                        content={content}
                                        onUpdate={setContent}
                                        editable={!isSubmitting}
                                        ref={editorRef}
                                    />
                                </div>

                                <div className='flex items-center justify-between gap-4 m-2'>
                                  {
                                    editorRef.current && (
                                      <ToolbarPlugin
                                        editor={editorRef.current}
                                        onImageUpload={(url: string) => {
                                            editorRef.current?.commands.insertContent(
                                                `<img src="${url}" alt="Uploaded image" />`
                                            );
                                        }}
                                        onUrlInsert={() => setShowUrlInput(true)}
                                    />
                                    )
                                  }

                                    <button
                                        onClick={handleSubmit}
                                        className="shrink-0 bg-primary text-background rounded-full hover:bg-primary/70 px-4 py-2 font-bold flex items-center gap-x-2"
                                    >
                                        {isSubmitting ? (
                                            <Loader className="animate-spin" />
                                        ) : (
                                            <UserCircle />
                                        )}
                                        {isSubmitting ? '' : 'Post'}
                                    </button>


                                </div>


                                {showUrlInput && (
                                    <div className="p-4 border-t border-gray-200 dark:border-gray-700">
                                        <div className="flex gap-2">
                                            <input
                                                type="text"
                                                value={url}
                                                onChange={(e) => setUrl(e.target.value)}
                                                placeholder="Paste URL here"
                                                className="flex-1 p-2 border rounded dark:bg-gray-700 dark:border-gray-600"
                                                onKeyDown={(e) => {
                                                    if (e.key === 'Enter') {
                                                        e.preventDefault();
                                                        handleInsertUrl();
                                                    }
                                                }}
                                            />
                                            <Button
                                                onClick={handleInsertUrl}
                                                variant="outline"
                                                size="sm"
                                            >
                                                Insert
                                            </Button>
                                        </div>
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