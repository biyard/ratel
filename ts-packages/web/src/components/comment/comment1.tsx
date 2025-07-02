// 'use client';

// import { getTimeAgo } from '@/lib/time-utils';
// import Image from 'next/image';
// import { ChevronDown, ChevronUp, Loader2 } from 'lucide-react';
// import { BendArrowRight, CommentIcon, ThumbUp } from '@/components/icons';
// import { Comment as CommentType } from '@/lib/api/models/feeds';
// import LexicalHtmlViewer from '@/components/lexical/lexical-html-viewer';
// import {
//   LexicalHtmlEditor,
//   LexicalHtmlEditorRef,
// } from '../lexical/lexical-html-editor';
// import { validateString } from '@/lib/string-filter-utils';
// import { ChevronDoubleDownIcon } from '@heroicons/react/20/solid';
// import { cn } from '@/lib/utils';
// import { useEffect, useRef, useState } from 'react';
// import { logger } from '@/lib/logger';
// import { TrashArrowUp } from '@/components/icons';
// import { ChatIcon } from '@/components/icons';

// import RichTextEditor from './rich-text-editor';

// interface CommentProps {
//   comment: CommentType;
//   onSubmit?: (comment_id: number, content: string) => Promise<void>;
//   onLike?: (comment_id: number, value: boolean) => Promise<void>;
// }

// export default function Comment1({ comment, onSubmit, onLike }: CommentProps) {
//   const [expand, setExpand] = useState(false);
//   const [showReplies, setShowReplies] = useState(false);

//   const ChevronIcon =
//     comment.num_of_replies > 0 ? (
//       showReplies ? (
//         <ChevronUp
//           width={26}
//           height={26}
//           className={` ${showReplies ? '[&>path]:stroke-neutral-700' : ''}`}
//         />
//       ) : (
//         <ChevronDown
//           width={24}
//           height={24}
//           className={` ${!showReplies ? '[&>path]:stroke-primary' : ''}`}
//         />
//       )
//     ) : null;

//   return (
//     <div className="flex flex-col gap-[14px] pb-5 border-b border-b-neutral-800 relative">
//       <div className="flex flex-row gap-2 items-center">
//         {comment.author[0].profile_url ? (
//           <Image
//             alt={comment.author[0].nickname ?? ''}
//             src={comment.author[0].profile_url ?? ''}
//             width={40}
//             height={40}
//             className="rounded-full object-cover object-top"
//           />
//         ) : (
//           <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
//         )}

//         <div className="flex flex-col gap-[2px]">
//           <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
//             {comment.author[0].nickname ?? ''}
//           </div>
//           <div className="font-semibold text-xs/[20px] text-[#6d6d6d]">
//             {getTimeAgo(comment.created_at)}
//           </div>
//         </div>
//       </div>

//       <div className="flex flex-col mx-10 gap-5">
//         {/* Quote */}
//         {comment.quote_comment && (
//           <div className="flex flex-row bg-[#282828] px-5 py-2.5 gap-2.5">
//             <div className="flex flex-row space-between">
//               <div className="flex flex-row gap-2 items-center">
//                 {comment.quote_comment?.author?.[0]?.profile_url ? (
//                   <Image
//                     alt={comment.quote_comment?.author?.[0]?.nickname ?? ''}
//                     src={comment.quote_comment?.author?.[0]?.profile_url ?? ''}
//                     width={40}
//                     height={40}
//                     className="rounded-full object-cover object-top"
//                   />
//                 ) : (
//                   <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
//                 )}

//                 <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
//                   {comment.quote_comment?.author?.[0]?.nickname ?? ''}
//                 </div>
//               </div>
//             </div>
//             <LexicalHtmlViewer
//               htmlString={comment.quote_comment.html_contents}
//             />
//           </div>
//         )}

//         {/* Content */}
//         {comment.html_contents && (
//           <LexicalHtmlViewer htmlString={comment.html_contents} />
//         )}

//         {/* Actions */}
//         <div className="flex flex-row w-full justify-between items-center gap-2">
//           <div className="flex flex-row gap-5">
//             {/* Number Reply Button */}
//             <div className="relative">
//               <button
//                 className={`gap-2  flex flex-row justify-center items-center disabled:cursor-not-allowed ${showReplies ? 'bg-primary p-2 rounded-md text-neutral-800' : 'text-primary'}`}
//                 disabled={comment.num_of_replies === 0}
//                 onClick={() => {
//                   setShowReplies(!showReplies);
//                 }}
//               >
//                 {`${comment.num_of_replies ?? 0} ${comment.num_of_replies <= 1 ? 'Reply' : 'Replies'}`}
//                 {ChevronIcon}
//               </button>

//               {/*first  Vertical orange line */}
//               {showReplies && expand && (comment.num_of_replies ?? 0) > 0 ? (
//                 <div className="absolute  left-10 w-0.5 h-6 bg-primary rounded-t-md" />
//               ) : (
//                 ' '
//               )}
//             </div>

//             {/* Reply Button */}
//             <div
//               onClick={() => {
//                 setExpand((prev) => !prev);
//                 setShowReplies(true);
//               }}
//               className="flex gap-2 cursor-pointer justify-center items-center"
//             >
//               <BendArrowRight width={24} height={24} />
//               Reply
//             </div>
//           </div>
//           {/* Like Button */}
//           <button
//             className="flex flex-row gap-2 justify-center items-center"
//             onClick={async () => {
//               if (onLike) {
//                 await onLike(comment.id, !comment.is_liked);
//               }
//             }}
//           >
//             <ThumbUp
//               width={24}
//               height={24}
//               className={
//                 comment.is_liked
//                   ? '[&>path]:fill-primary [&>path]:stroke-primary'
//                   : '[&>path]:stroke-[#aeaaab]'
//               }
//             />
//             <div className="font-medium text-base/[24px] text-[#aeaaab] ">
//               {comment.num_of_likes ?? 0}
//             </div>
//           </button>
//         </div>

//         {/* To show div used to reply comment */}
//         {expand && (
//           <div className="relative">
//             <NewComment1
//               className="min-h-30"
//               onClose={() => setExpand(false)}
//               onSubmit={async (content) => {
//                 if (onSubmit) {
//                   await onSubmit(comment.id, content);
//                 }
//               }}
//             />
//             {/* Second Vertical orange line */}
//             {showReplies && expand && (comment.num_of_replies ?? 0) > 0 ? (
//               <div className="absolute  left-10 w-0.5 h-10 z-12 bg-primary rounded-t-md" />
//             ) : (
//               ' '
//             )}
//           </div>
//         )}

//         {/* To show replies */}
//         {showReplies && (
//           <div className="flex flex-col gap-2.5">
//             {comment.replies.map((reply) => (
//               <div
//                 key={reply.id}
//                 className="flex flex-col gap-2 p-5  bg-neutral-background rounded-lg"
//               >
//                 <div className="flex flex-row gap-2 items-center">
//                   {reply.author?.[0]?.profile_url ? (
//                     <Image
//                       alt={reply.author?.[0]?.nickname ?? ''}
//                       src={reply.author?.[0]?.profile_url ?? ''}
//                       width={40}
//                       height={40}
//                       className="rounded-full object-cover object-top"
//                     />
//                   ) : (
//                     <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
//                   )}

//                   <div className="flex flex-col gap-[2px]">
//                     <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
//                       {reply.author?.[0]?.nickname ?? ''}
//                     </div>
//                   </div>

//                   {/* Two extreeme arrows at the right */}
//                   <div className="flex flex-row ml-auto justify-end space-x-4">
//                     <ChevronDown className="[&>path]:stroke-neutral-600" />

//                     <TrashArrowUp />
//                   </div>
//                 </div>

//                 <div className=" text-center">{reply.html_contents}</div>

//                 {/* <LexicalHtmlViewer  htmlString={reply.html_contents} /> */}
//               </div>
//             ))}
//           </div>
//         )}

//         {/* bottom vertical orange  line */}
//         {showReplies && expand && (comment.num_of_replies ?? 0) > 0 ? (
//           <div className="absolute  left-20 bottom-12 w-0.5 h-21 z-12 bg-primary rounded-t-md" />
//         ) : (
//           ' '
//         )}

//         {showReplies && expand && (comment.num_of_replies ?? 0) > 0 ? (
//           <div className=" inline-block">
//             <button
//               className="ml-6 inline-flex items-center justify-center p-2 rounded-md bg-primary text-neutral-800"
//               style={{ width: 'auto' }}
//               disabled={comment.num_of_replies === 0}
//               onClick={() => setShowReplies(!showReplies)}
//             >
//               <ChevronUp className="w-6 h-6" />
//             </button>
//           </div>
//         ) : null}

//       </div>
//     </div>
//   );
// }

'use client';
import { useState } from 'react';
import Image from 'next/image';
import { ChevronDown, ChevronUp } from 'lucide-react';
import { BendArrowRight, ThumbUp, TrashArrowUp } from '@/components/icons';
import { getTimeAgo } from '@/lib/time-utils';
import { Comment as CommentType } from '@/lib/api/models/feeds';
import RichTextEditor from './rich-text-editor';

interface CommentProps {
  comment: CommentType;
  onSubmit?: (comment_id: number, content: string) => Promise<void>;
  onLike?: (comment_id: number, value: boolean) => Promise<void>;
}

const HtmlViewer = ({ html }: { html: string }) => (
  <div
    className="prose prose-invert max-w-none"
    dangerouslySetInnerHTML={{ __html: html }}
  />
);

export default function Comment1({ comment, onSubmit, onLike }: CommentProps) {
  const [expand, setExpand] = useState(false);
  const [showReplies, setShowReplies] = useState(false);

  const ChevronIcon =
    comment.num_of_replies > 0 ? (
      showReplies ? (
        <ChevronUp
          width={26}
          height={26}
          className="[&>path]:stroke-neutral-700"
        />
      ) : (
        <ChevronDown
          width={24}
          height={24}
          className="[&>path]:stroke-primary"
        />
      )
    ) : null;

  return (
    <div className="flex flex-col gap-[14px] pb-5 border-b border-b-neutral-800 relative">
      {/* Author Header */}
      <div className="flex flex-row gap-2 items-center">
        {comment.author[0].profile_url ? (
          <Image
            alt={comment.author[0].nickname ?? ''}
            src={comment.author[0].profile_url ?? ''}
            width={40}
            height={40}
            className="rounded-full object-cover object-top"
          />
        ) : (
          <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
        )}
        <div className="flex flex-col gap-[2px]">
          <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
            {comment.author[0].nickname ?? ''}
          </div>
          <div className="font-semibold text-xs/[20px] text-[#6d6d6d]">
            {getTimeAgo(comment.created_at)}
          </div>
        </div>
      </div>

      <div className="flex flex-col mx-10 gap-5">
        {/* Quoted Comment */}
        {comment.quote_comment && (
          <div className="flex flex-row bg-[#282828] px-5 py-2.5 gap-2.5">
            <div className="flex flex-row gap-2 items-center">
              {comment.quote_comment?.author?.[0]?.profile_url ? (
                <Image
                  alt={comment.quote_comment?.author?.[0]?.nickname ?? ''}
                  src={comment.quote_comment?.author?.[0]?.profile_url ?? ''}
                  width={40}
                  height={40}
                  className="rounded-full object-cover object-top"
                />
              ) : (
                <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
              )}
              <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
                {comment.quote_comment?.author?.[0]?.nickname ?? ''}
              </div>
            </div>
            <HtmlViewer html={comment.quote_comment.html_contents} />
          </div>
        )}

        {/* Main Comment Content */}
        {comment.html_contents && <HtmlViewer html={comment.html_contents} />}

        {/* Actions */}
        <div className="flex flex-row w-full justify-between items-center gap-2">
          <div className="flex flex-row gap-5">
            <div className="relative">
              <button
                className={`gap-2 flex items-center disabled:cursor-not-allowed ${
                  showReplies
                    ? 'bg-primary p-2 rounded-md text-neutral-800'
                    : 'text-primary'
                }`}
                disabled={comment.num_of_replies === 0}
                onClick={() => setShowReplies(!showReplies)}
              >
                {`${comment.num_of_replies ?? 0} ${
                  comment.num_of_replies <= 1 ? 'Reply' : 'Replies'
                }`}
                {ChevronIcon}
              </button>
              {showReplies && expand && comment.num_of_replies > 0 && (
                <div className="absolute left-10 w-0.5 h-6 bg-primary rounded-t-md" />
              )}
            </div>

            <div
              onClick={() => {
                setExpand((prev) => !prev);
                setShowReplies(true);
              }}
              className="flex gap-2 cursor-pointer justify-center items-center"
            >
              <BendArrowRight width={24} height={24} />
              Reply
            </div>
          </div>

          <button
            className="flex items-center gap-2"
            onClick={async () =>
              onLike && (await onLike(comment.id, !comment.is_liked))
            }
          >
            <ThumbUp
              width={24}
              height={24}
              className={
                comment.is_liked
                  ? '[&>path]:fill-primary [&>path]:stroke-primary'
                  : '[&>path]:stroke-[#aeaaab]'
              }
            />
            <div className="font-medium text-base/[24px] text-[#aeaaab]">
              {comment.num_of_likes ?? 0}
            </div>
          </button>
        </div>

        {/* Reply Input */}
        {expand && (
          <div className="relative">
            <RichTextEditor
              onClose={() => setExpand(false)}
              onSubmit={async (content) => {
                if (onSubmit) await onSubmit(comment.id, content);
              }}
            />
            {showReplies && expand && comment.num_of_replies > 0 && (
              <div className="absolute left-10 w-0.5 h-10 z-12 bg-primary rounded-t-md" />
            )}
          </div>
        )}

        {/* Replies */}
        {showReplies && (
          <div className="flex flex-col gap-2.5">
            {comment.replies.map((reply) => (
              <div
                key={reply.id}
                className="flex flex-col gap-2 p-5 bg-neutral-background rounded-lg text-right-[20px]"
              >
                <div className="flex flex-row gap-2 items-center">
                  {reply.author?.[0]?.profile_url ? (
                    <Image
                      alt={reply.author?.[0]?.nickname ?? ''}
                      src={reply.author?.[0]?.profile_url ?? ''}
                      width={40}
                      height={40}
                      className="rounded-full object-cover object-top"
                    />
                  ) : (
                    <div className="w-[40px] h-[40px] rounded-full bg-neutral-500" />
                  )}

                  <div className="font-semibold text-neutral-300 text-[15px]/[15px]">
                    {reply.author?.[0]?.nickname ?? ''}
                  </div>

                  <div className="flex flex-row ml-auto space-x-4">
                    <ChevronDown className="[&>path]:stroke-neutral-600" />
                    <TrashArrowUp />
                  </div>
                </div>

                <div className="pl-12">
                  <HtmlViewer html={reply.html_contents} />
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Bottom vertical line */}
        {showReplies && expand && comment.num_of_replies > 0 && (
          <div className="absolute left-20 bottom-12 w-0.5 h-21 z-12 bg-primary rounded-t-md" />
        )}

        {/* Collapse Replies Button */}
        {showReplies && expand && comment.num_of_replies > 0 && (
          <div className="inline-block">
            <button
              className="ml-6 inline-flex items-center justify-center p-2 rounded-md bg-primary text-neutral-800"
              disabled={comment.num_of_replies === 0}
              onClick={() => setShowReplies(false)}
            >
              <ChevronUp className="w-6 h-6" />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

// export function NewComment1({
//   className = '',
//   onClose,
//   onSubmit,
// }: {
//   className?: string;
//   onClose: () => void;
//   onSubmit?: (content: string) => Promise<void>;
// }) {
//   const [isLoading, setLoading] = useState(false);
//   const [disabled, setDisabled] = useState(true);
//   const editorRef = useRef<LexicalHtmlEditorRef>(null);
//   const ref = useRef<HTMLDivElement>(null);
//   useEffect(() => {
//     ref.current?.scrollIntoView({ behavior: 'smooth', block: 'start' });
//   }, [ref]);

//   const handleSubmit = async () => {
//     const content = editorRef.current?.getContent() || '';
//     if (
//       onSubmit &&
//       !isLoading &&
//       content.trim() !== '' &&
//       validateString(content)
//     ) {
//       setLoading(true);
//       try {
//         await onSubmit(content);
//         editorRef.current?.clear();
//         setDisabled(false);
//         onClose();
//       } catch (error) {
//         logger.debug('Error submitting comment:', error);
//       } finally {
//         setLoading(false);
//       }
//     }
//   };
//   return (
//     <div
//       ref={ref}
//       className="flex w-full bg-neutral-900 border rounded-lg border-primary max-w-desktop"
//     >
//       <div className="flex-1">
//         <LexicalHtmlEditor
//           className={className}
//           ref={editorRef}
//           onChange={(content) => {
//             setDisabled(content.trim() === '' || !validateString(content));
//           }}
//         />
//       </div>
//       <div className="p-3 flex flex-col justify-between">
//         <button className="p-1 flex flex-row justify-center" onClick={onClose}>
//           <ChevronDoubleDownIcon width={24} height={24} />
//         </button>
//         <div>
//           <button
//             onClick={handleSubmit}
//             disabled={disabled}
//             className={cn(
//               'flex items-center gap-2 p-2 rounded-full font-medium text-sm transition-all',
//               !disabled
//                 ? 'bg-primary text-black hover:bg-primary/50'
//                 : 'bg-neutral-700 text-neutral-500 cursor-not-allowed',
//             )}
//           >
//             {isLoading ? (
//               <Loader2 className="animate-spin size-6" />
//             ) : (
//               <CommentIcon
//                 width={24}
//                 height={24}
//                 className="[&>path]:stroke-white [&>line]:stroke-white"
//               />

//               // <ChatIcon
//               //   width={24}
//               //   height={24}
//               //   className="[&>path]:stroke-white [&>line]:stroke-white"
//               // />
//             )}
//           </button>
//         </div>
//       </div>
//     </div>
//   );
// }
