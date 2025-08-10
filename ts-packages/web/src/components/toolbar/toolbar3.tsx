// // 'use client';
// // import { useEditor, EditorContent } from '@tiptap/react';
// // import StarterKit from '@tiptap/starter-kit';
// // import Underline from '@tiptap/extension-underline';
// // import TextAlign from '@tiptap/extension-text-align';

// // import BulletList from "@tiptap/extension-bullet-list";
// // import Link from '@tiptap/extension-link';
// // import { useCallback, useState } from 'react';
// // import { cn } from '@/lib/utils';
// // import FileUploader from '@/components/file-uploader';

// // // Import your icons (same as before)
// // import Capslock from '@/assets/icons/editor/capslock.svg';
// // import CodesPen from '@/assets/icons/editor/codespen.svg';
// // import BoldIcon from '@/assets/icons/editor/bold.svg';
// // import Italics from '@/assets/icons/editor/italics.svg';
// // import UnderlineIcon from '@/assets/icons/editor/underline.svg';
// // import Strike from '@/assets/icons/editor/strike.svg';
// // import Paint2 from '@/assets/icons/editor/paint2.svg';
// // import Paint from '@/assets/icons/editor/paint.svg';
// // import ImageUpload from '@/assets/icons/editor/upload-image.svg';
// // import LinkPaste from '@/assets/icons/editor/link-paste.svg';
// // import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
// // import TableCells from '@/assets/icons/editor/tabe-cells.svg';
// // import LeftAlign from '@/assets/icons/editor/misc3-part.svg';
// // import NumberBullet from '@/assets/icons/editor/misc2-part.svg';
// // import Bullet from '@/assets/icons/editor/misc-parts.svg';
// // import RightAlign from '@/assets/icons/editor/paragraph.svg';
// // import RightAlignLight from '@/assets/icons/editor/paragraph-light.svg';

// // export default function ToolbarPlugin({
// //   onImageUpload,
// //   enableImage = true,
// //   onTriggerLinkPaste,
// // }: {
// //   onImageUpload: (url: string) => void;
// //   enableImage?: boolean;
// //   onTriggerLinkPaste?: () => void;
// // }) {
// //   const editor = useEditor({
// //     extensions: [
// //       StarterKit,
// //       Underline,
// //       TextAlign.configure({
// //         types: ['heading', 'paragraph'],
// //       }),
// //       Link.configure({
// //         openOnClick: false,
// //       }),

// //     ],
// //     content: '',
// //   });

// //   const [isBold, setIsBold] = useState(false);
// //   const [isItalic, setIsItalic] = useState(false);
// //   const [isUnderline, setIsUnderline] = useState(false);
// //   const [isStrikethrough, setIsStrikethrough] = useState(false);

// //   // Update toolbar state when editor changes
// //   const updateToolbar = useCallback(() => {
// //     if (!editor) return;
    
// //     setIsBold(editor.isActive('bold'));
// //     setIsItalic(editor.isActive('italic'));
// //     setIsUnderline(editor.isActive('underline'));
// //     setIsStrikethrough(editor.isActive('strike'));
// //   }, [editor]);

// //   // Set up event listeners
// //   editor?.on('transaction', updateToolbar);
// //   editor?.on('selectionUpdate', updateToolbar);

// //   if (!editor) {
// //     return null;
// //   }

// //   return (
// //     <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700">
// //       <button
// //         onClick={() => editor.chain().focus().toggleUnderline().run()}
// //         className={cn(isUnderline && 'bg-neutral-600')}
// //       >
// //         <Capslock />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleCode().run()}
// //         className={cn(editor.isActive('code') && 'bg-neutral-600')}
// //       >
// //         <CodesPen />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleBold().run()}
// //         className={cn(isBold && 'bg-neutral-600')}
// //       >
// //         <BoldIcon />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleItalic().run()}
// //         className={cn(isItalic && 'bg-neutral-600')}
// //       >
// //         <Italics />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleUnderline().run()}
// //         className={cn(isUnderline && 'bg-neutral-600')}
// //       >
// //         <UnderlineIcon />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleStrike().run()}
// //         className={cn(isStrikethrough && 'bg-neutral-600')}
// //       >
// //         <Strike />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <Paint2 />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <Paint />
// //       </button>

// //       {enableImage ? (
// //         <FileUploader onUploadSuccess={onImageUpload}>
// //           <ImageUpload />
// //         </FileUploader>
// //       ) : null}

// //       <button
// //         onClick={onTriggerLinkPaste}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <LinkPaste />
// //       </button>

// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <CommentPaste />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <TableCells />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('left').run()}
// //         className={cn(editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600')}
// //       >
// //         <LeftAlign />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleOrderedList().run()}
// //         className={cn(editor.isActive('orderedList') && 'bg-neutral-600')}
// //       >
// //         <NumberBullet />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleBulletList().run()}
// //         className={cn(editor.isActive('bulletList') && 'bg-neutral-600')}
// //       >
// //         <Bullet />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('right').run()}
// //         className={cn(editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600')}
// //       >
// //         <RightAlign />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('center').run()}
// //         className={cn(editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600')}
// //       >
// //         <RightAlignLight />
// //       </button>
// //     </div>
// //   );
// // }



// // 'use client';
// // import { Editor } from '@tiptap/react';
// // import { cn } from '@/lib/utils';
// // import FileUploader from '@/components/file-uploader';
// // import { useCallback, useState, useEffect } from 'react';

// // // Import your icons
// // import Capslock from '@/assets/icons/editor/capslock.svg';
// // import CodesPen from '@/assets/icons/editor/codespen.svg';
// // import BoldIcon from '@/assets/icons/editor/bold.svg';
// // import Italics from '@/assets/icons/editor/italics.svg';
// // import UnderlineIcon from '@/assets/icons/editor/underline.svg';
// // import Strike from '@/assets/icons/editor/strike.svg';
// // import Paint2 from '@/assets/icons/editor/paint2.svg';
// // import Paint from '@/assets/icons/editor/paint.svg';
// // import ImageUpload from '@/assets/icons/editor/upload-image.svg';
// // import LinkPaste from '@/assets/icons/editor/link-paste.svg';
// // import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
// // import TableCells from '@/assets/icons/editor/tabe-cells.svg';
// // import LeftAlign from '@/assets/icons/editor/misc3-part.svg';
// // import NumberBullet from '@/assets/icons/editor/misc2-part.svg';
// // import Bullet from '@/assets/icons/editor/misc-parts.svg';
// // import RightAlign from '@/assets/icons/editor/paragraph.svg';
// // import RightAlignLight from '@/assets/icons/editor/paragraph-light.svg';

// // interface ToolbarPluginProps {
// //   editor: Editor | null;
// //   onImageUpload: (url: string) => void;
// //   enableImage?: boolean;
// //   onTriggerLinkPaste?: () => void;
// // }

// // export default function ToolbarPlugin({
// //   editor,
// //   onImageUpload,
// //   enableImage = true,
// //   onTriggerLinkPaste,
// // }: ToolbarPluginProps) {
// //   const [isBold, setIsBold] = useState(false);
// //   const [isItalic, setIsItalic] = useState(false);
// //   const [isUnderline, setIsUnderline] = useState(false);
// //   const [isStrikethrough, setIsStrikethrough] = useState(false);

// //   // Update toolbar state when editor changes
// //   const updateToolbar = useCallback(() => {
// //     if (!editor) return;
    
// //     setIsBold(editor.isActive('bold'));
// //     setIsItalic(editor.isActive('italic'));
// //     setIsUnderline(editor.isActive('underline'));
// //     setIsStrikethrough(editor.isActive('strike'));
// //   }, [editor]);

// //   // Set up event listeners
// //   useEffect(() => {
// //     if (!editor) return;

// //     editor.on('transaction', updateToolbar);
// //     editor.on('selectionUpdate', updateToolbar);

// //     return () => {
// //       editor.off('transaction', updateToolbar);
// //       editor.off('selectionUpdate', updateToolbar);
// //     };
// //   }, [editor, updateToolbar]);

// //   if (!editor) {
// //     return null;
// //   }

// //   return (
// //     <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700">
// //       <button
// //         onClick={() => editor.chain().focus().toggleCode().run()}
// //         className={cn(editor.isActive('code') && 'bg-neutral-600')}
// //       >
// //         <CodesPen />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleBold().run()}
// //         className={cn(isBold && 'bg-neutral-600')}
// //       >
// //         <BoldIcon />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleItalic().run()}
// //         className={cn(isItalic && 'bg-neutral-600')}
// //       >
// //         <Italics />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleUnderline().run()}
// //         className={cn(isUnderline && 'bg-neutral-600')}
// //       >
// //         <UnderlineIcon />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleStrike().run()}
// //         className={cn(isStrikethrough && 'bg-neutral-600')}
// //       >
// //         <Strike />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <Paint2 />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <Paint />
// //       </button>

// //       {enableImage ? (
// //         <FileUploader onUploadSuccess={onImageUpload}>
// //           <ImageUpload />
// //         </FileUploader>
// //       ) : null}

// //       <button
// //         onClick={onTriggerLinkPaste}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <LinkPaste />
// //       </button>

// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <CommentPaste />
// //       </button>
// //       <button
// //         onClick={() => {}}
// //         className={cn('hover:bg-neutral-600')}
// //       >
// //         <TableCells />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('left').run()}
// //         className={cn(editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600')}
// //       >
// //         <LeftAlign />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleOrderedList().run()}
// //         className={cn(editor.isActive('orderedList') && 'bg-neutral-600')}
// //       >
// //         <NumberBullet />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().toggleBulletList().run()}
// //         className={cn(editor.isActive('bulletList') && 'bg-neutral-600')}
// //       >
// //         <Bullet />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('right').run()}
// //         className={cn(editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600')}
// //       >
// //         <RightAlign />
// //       </button>
// //       <button
// //         onClick={() => editor.chain().focus().setTextAlign('center').run()}
// //         className={cn(editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600')}
// //       >
// //         <RightAlignLight />
// //       </button>
// //     </div>
// //   );
// // }




// 'use client';
// import { Editor } from '@tiptap/react';
// import { cn } from '@/lib/utils';
// import FileUploader from '@/components/file-uploader';
// import { useCallback, useState, useEffect, useRef } from 'react';
// import { HexColorPicker } from 'react-colorful';

// // Import icons
// import Capslock from '@/assets/icons/editor/capslock.svg';
// import CodesPen from '@/assets/icons/editor/codespen.svg';
// import BoldIcon from '@/assets/icons/editor/bold.svg';
// import Italics from '@/assets/icons/editor/italics.svg';
// import UnderlineIcon from '@/assets/icons/editor/underline.svg';
// import Strike from '@/assets/icons/editor/strike.svg';
// import Paint2 from '@/assets/icons/editor/paint2.svg';
// import Paint from '@/assets/icons/editor/paint.svg';
// import ImageUpload from '@/assets/icons/editor/upload-image.svg';
// import LinkPaste from '@/assets/icons/editor/link-paste.svg';
// import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
// import TableCells from '@/assets/icons/editor/tabe-cells.svg';
// import LeftAlign from '@/assets/icons/editor/misc3-part.svg';
// import NumberBullet from '@/assets/icons/editor/misc2-part.svg';
// import Bullet from '@/assets/icons/editor/misc-parts.svg';
// import RightAlign from '@/assets/icons/editor/paragraph.svg';
// import RightAlignLight from '@/assets/icons/editor/paragraph-light.svg';

// interface ToolbarPluginProps {
//   editor: Editor | null;
//   onImageUpload: (url: string) => void;
//   enableImage?: boolean;
//   onTriggerLinkPaste?: () => void;
// }

// export default function ToolbarPlugin({
//   editor,
//   onImageUpload,
//   enableImage = true,
//   onTriggerLinkPaste,
// }: ToolbarPluginProps) {
//   // Editor state tracking
//   const [isBold, setIsBold] = useState(false);
//   const [isItalic, setIsItalic] = useState(false);
//   const [isUnderline, setIsUnderline] = useState(false);
//   const [isStrikethrough, setIsStrikethrough] = useState(false);

//   // Color picker states
//   const [showTextColorPicker, setShowTextColorPicker] = useState(false);
//   const [showBgColorPicker, setShowBgColorPicker] = useState(false);
//   const [textColor, setTextColor] = useState('#000000');
//   const [bgColor, setBgColor] = useState('#FFFFFF');

//   // Table creation states
//   const [showTableMenu, setShowTableMenu] = useState(false);
//   const [rows, setRows] = useState(3);
//   const [cols, setCols] = useState(3);

//   // Refs for click-outside detection
//   const textColorPickerRef = useRef<HTMLDivElement>(null);
//   const bgColorPickerRef = useRef<HTMLDivElement>(null);
//   const tableMenuRef = useRef<HTMLDivElement>(null);

//   // Update toolbar button states
//   const updateToolbar = useCallback(() => {
//     if (!editor) return;
    
//     setIsBold(editor.isActive('bold'));
//     setIsItalic(editor.isActive('italic'));
//     setIsUnderline(editor.isActive('underline'));
//     setIsStrikethrough(editor.isActive('strike'));
//   }, [editor]);

//   // Set up editor event listeners
//   useEffect(() => {
//     if (!editor) return;

//     editor.on('transaction', updateToolbar);
//     editor.on('selectionUpdate', updateToolbar);

//     return () => {
//       editor.off('transaction', updateToolbar);
//       editor.off('selectionUpdate', updateToolbar);
//     };
//   }, [editor, updateToolbar]);

//   // Close menus when clicking outside
//   useEffect(() => {
//     const handleClickOutside = (event: MouseEvent) => {
//       if (textColorPickerRef.current && !textColorPickerRef.current.contains(event.target as Node)) {
//         setShowTextColorPicker(false);
//       }
//       if (bgColorPickerRef.current && !bgColorPickerRef.current.contains(event.target as Node)) {
//         setShowBgColorPicker(false);
//       }
//       if (tableMenuRef.current && !tableMenuRef.current.contains(event.target as Node)) {
//         setShowTableMenu(false);
//       }
//     };

//     document.addEventListener('mousedown', handleClickOutside);
//     return () => document.removeEventListener('mousedown', handleClickOutside);
//   }, []);

//   // Apply text color
//   const applyTextColor = (color: string) => {
//     if (!editor) return;
//     editor.chain().focus().setColor(color).run();
//     setShowTextColorPicker(false);
//   };

//   // Apply background color
//   const applyBgColor = (color: string) => {
//     if (!editor) return;
//     editor.chain().focus().setHighlight({ color }).run();
//     setShowBgColorPicker(false);
//   };

//   // Insert table
//   const insertTable = () => {
//     if (!editor) return;
//     editor
//       .chain()
//       .focus()
//       .insertTable({ rows, cols, withHeaderRow: true })
//       .run();
//     setShowTableMenu(false);
//   };

//   if (!editor) {
//     return null;
//   }

//   return (
//     <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700 ">
//       {/* Text Formatting Buttons */}
//       <button
//         onClick={() => editor.chain().focus().toggleCode().run()}
//         className={cn(editor.isActive('code') && 'bg-neutral-600')}
//       >
//         <CodesPen />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleBold().run()}
//         className={cn(isBold && 'bg-neutral-600')}
//       >
//         <BoldIcon />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleItalic().run()}
//         className={cn(isItalic && 'bg-neutral-600')}
//       >
//         <Italics />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleUnderline().run()}
//         className={cn(isUnderline && 'bg-neutral-600')}
//       >
//         <UnderlineIcon />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleStrike().run()}
//         className={cn(isStrikethrough && 'bg-neutral-600')}
//       >
//         <Strike />
//       </button>

//       {/* Text Color Picker */}
//       <div className="relative" ref={textColorPickerRef}>
//         <button
//           onClick={() => {
//             setShowTextColorPicker(!showTextColorPicker);
//             setShowBgColorPicker(false);
//             setShowTableMenu(false);
//           }}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('textStyle', { color: textColor }) && 'bg-neutral-600'
//           )}
//         >
//           <Paint />
//         </button>
//         {showTextColorPicker && (
//           <div className="absolute top-0 z-50 mt-2 p-2 bg-white rounded shadow-lg">
//             <HexColorPicker color={textColor} onChange={setTextColor} />
//             <div className="flex justify-between mt-2">
//               <button
//                 onClick={() => applyTextColor(textColor)}
//                 className="px-2 py-1 bg-blue-500 text-white rounded"
//               >
//                 Apply
//               </button>
//               <button
//                 onClick={() => editor.chain().focus().unsetColor().run()}
//                 className="px-2 py-1 bg-gray-500 text-white rounded"
//               >
//                 Reset
//               </button>
//             </div>
//           </div>
//         )}
//       </div>

//       {/* Background Color Picker */}
//       <div className="relative" ref={bgColorPickerRef}>
//         <button
//           onClick={() => {
//             setShowBgColorPicker(!showBgColorPicker);
//             setShowTextColorPicker(false);
//             setShowTableMenu(false);
//           }}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('highlight', { color: bgColor }) && 'bg-neutral-600'
//           )}
//         >
//           <Paint2 />
//         </button>
//         {showBgColorPicker && (
//           <div className="absolute z-50 mt-2 p-2 bg-white rounded shadow-lg">
//             <HexColorPicker color={bgColor} onChange={setBgColor} />
//             <div className="flex justify-between mt-2">
//               <button
//                 onClick={() => applyBgColor(bgColor)}
//                 className="px-2 py-1 bg-blue-500 text-white rounded"
//               >
//                 Apply
//               </button>
//               <button
//                 onClick={() => editor.chain().focus().unsetHighlight().run()}
//                 className="px-2 py-1 bg-gray-500 text-white rounded"
//               >
//                 Reset
//               </button>
//             </div>
//           </div>
//         )}
//       </div>

//       {/* Image Upload */}
//       {enableImage && (
//         <FileUploader onUploadSuccess={onImageUpload}>
//           <ImageUpload />
//         </FileUploader>
//       )}

//       {/* Link */}
//       <button
//         onClick={onTriggerLinkPaste}
//         className={cn('hover:bg-neutral-600')}
//       >
//         <LinkPaste />
//       </button>

//       {/* Comment */}
//       <button
//         onClick={() => {}}
//         className={cn('hover:bg-neutral-600')}
//       >
//         <CommentPaste />
//       </button>

//       {/* Table Creation */}
//       <div className="relative" ref={tableMenuRef}>
//         <button
//           onClick={() => {
//             setShowTableMenu(!showTableMenu);
//             setShowTextColorPicker(false);
//             setShowBgColorPicker(false);
//           }}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('table') && 'bg-neutral-600'
//           )}
//         >
//           <TableCells />
//         </button>
//         {showTableMenu && (
//           <div className="absolute top-0 z-50 mt-2 p-4 bg-white rounded shadow-lg w-64">
//             <h4 className="font-medium mb-2">Create Table</h4>
//             <div className="grid grid-cols-2 gap-4 mb-4">
//               <div>
//                 <label className="block text-sm mb-1">Rows</label>
//                 <input
//                   type="number"
//                   min="1"
//                   max="10"
//                   value={rows}
//                   onChange={(e) => setRows(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
//                   className="w-full p-1 border rounded"
//                 />
//               </div>
//               <div>
//                 <label className="block text-sm mb-1">Columns</label>
//                 <input
//                   type="number"
//                   min="1"
//                   max="10"
//                   value={cols}
//                   onChange={(e) => setCols(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
//                   className="w-full p-1 border rounded"
//                 />
//               </div>
//             </div>
//             <button
//               onClick={insertTable}
//               className="w-full py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
//             >
//               Insert Table
//             </button>
//           </div>
//         )}
//       </div>

//       {/* Alignment Buttons */}
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('left').run()}
//         className={cn(editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600')}
//       >
//         <LeftAlign />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleOrderedList().run()}
//         className={cn(editor.isActive('orderedList') && 'bg-neutral-600')}
//       >
//         <NumberBullet />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleBulletList().run()}
//         className={cn(editor.isActive('bulletList') && 'bg-neutral-600')}
//       >
//         <Bullet />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('right').run()}
//         className={cn(editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600')}
//       >
//         <RightAlign />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('center').run()}
//         className={cn(editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600')}
//       >
//         <RightAlignLight />
//       </button>
//     </div>
//   );
// }






// 'use client';
// import { Editor } from '@tiptap/react';
// import { cn } from '@/lib/utils';
// import FileUploader from '@/components/file-uploader';
// import { useCallback, useState, useEffect, useRef } from 'react';
// import { HexColorPicker } from 'react-colorful';

// // Import icons
// import CodesPen from '@/assets/icons/editor/codespen.svg';
// import BoldIcon from '@/assets/icons/editor/bold.svg';
// import Italics from '@/assets/icons/editor/italics.svg';
// import UnderlineIcon from '@/assets/icons/editor/underline.svg';
// import Strike from '@/assets/icons/editor/strike.svg';
// import Paint2 from '@/assets/icons/editor/paint2.svg';
// import Paint from '@/assets/icons/editor/paint.svg';
// import ImageUpload from '@/assets/icons/editor/upload-image.svg';
// import LinkPaste from '@/assets/icons/editor/link-paste.svg';
// import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
// import TableCells from '@/assets/icons/editor/tabe-cells.svg';
// import LeftAlign from '@/assets/icons/editor/misc3-part.svg';
// import NumberBullet from '@/assets/icons/editor/misc2-part.svg';
// import Bullet from '@/assets/icons/editor/misc-parts.svg';
// import RightAlign from '@/assets/icons/editor/paragraph.svg';
// import RightAlignLight from '@/assets/icons/editor/paragraph-light.svg';

// interface ToolbarPluginProps {
//   editor: Editor | null;
//   onImageUpload: (url: string) => void;
//   enableImage?: boolean;
//   onTriggerLinkPaste?: () => void;
// }

// export default function ToolbarPlugin({
//   editor,
//   onImageUpload,
//   enableImage = true,
//   onTriggerLinkPaste,
// }: ToolbarPluginProps) {
//   // State for color pickers
//   const [showTextColorPicker, setShowTextColorPicker] = useState(false);
//   const [showBgColorPicker, setShowBgColorPicker] = useState(false);
//   const [textColor, setTextColor] = useState('#000000');
//   const [bgColor, setBgColor] = useState('#FFFFFF');

//   // State for table creation
//   const [showTableMenu, setShowTableMenu] = useState(false);
//   const [rows, setRows] = useState(3);
//   const [cols, setCols] = useState(3);

//   // Refs for click-outside detection
//   const textColorPickerRef = useRef<HTMLDivElement>(null);
//   const bgColorPickerRef = useRef<HTMLDivElement>(null);
//   const tableMenuRef = useRef<HTMLDivElement>(null);

//   // Apply text color
//   const applyTextColor = (color: string) => {
//     editor?.chain().focus().setColor(color).run();
//     setShowTextColorPicker(false);
//   };

//   // Apply background color
//   const applyBgColor = (color: string) => {
//     editor?.chain().focus().setHighlight({ color }).run();
//     setShowBgColorPicker(false);
//   };

//   // Insert table
//   const insertTable = () => {
//     editor?.chain().focus().insertTable({ rows, cols, withHeaderRow: true }).run();
//     setShowTableMenu(false);
//   };

//   // Close menus when clicking outside
//   useEffect(() => {
//     const handleClickOutside = (event: MouseEvent) => {
//       if (textColorPickerRef.current && !textColorPickerRef.current.contains(event.target as Node)) {
//         setShowTextColorPicker(false);
//       }
//       if (bgColorPickerRef.current && !bgColorPickerRef.current.contains(event.target as Node)) {
//         setShowBgColorPicker(false);
//       }
//       if (tableMenuRef.current && !tableMenuRef.current.contains(event.target as Node)) {
//         setShowTableMenu(false);
//       }
//     };

//     document.addEventListener('mousedown', handleClickOutside);
//     return () => document.removeEventListener('mousedown', handleClickOutside);
//   }, []);

//   if (!editor) {
//     return null;
//   }

//   return (
//     <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700 ">
//       {/* Basic formatting buttons */}
//       <button
//         onClick={() => editor.chain().focus().toggleCode().run()}
//         className={cn(editor.isActive('code') && 'bg-neutral-600')}
//         aria-label="Code"
//       >
//         <CodesPen />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleBold().run()}
//         className={cn(editor.isActive('bold') && 'bg-neutral-600')}
//         aria-label="Bold"
//       >
//         <BoldIcon />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleItalic().run()}
//         className={cn(editor.isActive('italic') && 'bg-neutral-600')}
//         aria-label="Italic"
//       >
//         <Italics />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleUnderline().run()}
//         className={cn(editor.isActive('underline') && 'bg-neutral-600')}
//         aria-label="Underline"
//       >
//         <UnderlineIcon />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().toggleStrike().run()}
//         className={cn(editor.isActive('strike') && 'bg-neutral-600')}
//         aria-label="Strikethrough"
//       >
//         <Strike />
//       </button>

//       {/* Text Color Picker */}
//       <div className="relative" ref={textColorPickerRef}>
//         <button
//           onClick={() => setShowTextColorPicker(!showTextColorPicker)}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('textStyle', { color: textColor }) && 'bg-neutral-600'
//           )}
//           aria-label="Text color"
//         >
//           <Paint />
//         </button>
//         {showTextColorPicker && (
//           <div className="absolute  z-50 mt-2 p-2 bg-white rounded shadow-lg">
//             <HexColorPicker color={textColor} onChange={setTextColor} />
//             <div className="flex justify-between mt-2">
//               <button
//                 onClick={() => applyTextColor(textColor)}
//                 className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
//               >
//                 Apply
//               </button>
//               <button
//                 onClick={() => editor.chain().focus().unsetColor().run()}
//                 className="px-2 py-1 bg-neutral-800 font-bold text-white rounded"
//               >
//                 Reset
//               </button>
//             </div>
//           </div>
//         )}
//       </div>

//       {/* Background Color Picker */}
//       <div className="relative" ref={bgColorPickerRef}>
//         <button
//           onClick={() => setShowBgColorPicker(!showBgColorPicker)}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('highlight') && 'bg-neutral-600'
//           )}
//           aria-label="Background color"
//         >
//           <Paint2 />
//         </button>
//         {showBgColorPicker && (
//           <div className="absolute z-50 mt-2 p-2 bg-white rounded shadow-lg">
//             <HexColorPicker color={bgColor} onChange={setBgColor} />
//             <div className="flex justify-between mt-2">
//               <button
//                 onClick={() => applyBgColor(bgColor)}
//                 className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
//               >
//                 Apply
//               </button>
//               <button
//                 onClick={() => editor.chain().focus().unsetHighlight().run()}
//                 className="px-2 py-1 bg-gray-500 bg-neutral-800 font-bold text-white rounded"
//               >
//                 Reset
//               </button>
//             </div>
//           </div>
//         )}
//       </div>

//       {/* Image Upload */}
//       {enableImage && (
//         <FileUploader onUploadSuccess={onImageUpload}>
//           <button type="button" aria-label="Upload image">
//             <ImageUpload />
//           </button>
//         </FileUploader>
//       )}

//       {/* Link */}
//       <button
//         onClick={onTriggerLinkPaste}
//         className={cn('hover:bg-neutral-600')}
//         aria-label="Insert link"
//       >
//         <LinkPaste />
//       </button>

//       {/* Comment */}
//       <button
//         onClick={() => {}}
//         className={cn('hover:bg-neutral-600')}
//         aria-label="Add comment"
//       >
//         <CommentPaste />
//       </button>

//       {/* Table Creation */}
//       <div className="relative" ref={tableMenuRef}>
//         <button
//           onClick={() => setShowTableMenu(!showTableMenu)}
//           className={cn(
//             'hover:bg-neutral-600',
//             editor.isActive('table') && 'bg-neutral-600'
//           )}
//           aria-label="Insert table"
//         >
//           <TableCells />
//         </button>
//         {showTableMenu && (
//           <div className="absolute z-50 mt-2 p-4 bg-white rounded shadow-lg w-64">
//             <h4 className="font-medium mb-2">Create Table</h4>
//             <div className="grid grid-cols-2 gap-4 mb-4">
//               <div>
//                 <label className="block text-sm mb-1">Rows</label>
//                 <input
//                   type="number"
//                   min="1"
//                   max="10"
//                   value={rows}
//                   onChange={(e) => setRows(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
//                   className="w-full p-1 border rounded"
//                 />
//               </div>
//               <div>
//                 <label className="block text-sm mb-1">Columns</label>
//                 <input
//                   type="number"
//                   min="1"
//                   max="10"
//                   value={cols}
//                   onChange={(e) => setCols(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
//                   className="w-full p-1 border rounded"
//                 />
//               </div>
//             </div>
//             <button
//               onClick={insertTable}
//               className="w-full py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
//             >
//               Insert Table
//             </button>
//           </div>
//         )}
//       </div>

//       {/* Bullet List - Fixed Implementation */}
//       <button
//         onClick={() => editor.chain().focus().toggleBulletList().run()}
//         className={cn(
//           'hover:bg-neutral-600',
//           editor.isActive('bulletList') && 'bg-neutral-600'
//         )}
//         disabled={!editor.can().toggleBulletList()}
//         aria-label="Bullet list"
//       >
//         <Bullet />
//       </button>

//       {/* Numbered List - Fixed Implementation */}
//       <button
//         onClick={() => editor.chain().focus().toggleOrderedList().run()}
//         className={cn(
//           'hover:bg-neutral-600',
//           editor.isActive('orderedList') && 'bg-neutral-600'
//         )}
//         disabled={!editor.can().toggleOrderedList()}
//         aria-label="Numbered list"
//       >
//         <NumberBullet />
//       </button>

//       {/* Alignment Buttons */}
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('left').run()}
//         className={cn(
//           'hover:bg-neutral-600',
//           editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600'
//         )}
//         aria-label="Align left"
//       >
//         <LeftAlign />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('right').run()}
//         className={cn(
//           'hover:bg-neutral-600',
//           editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600'
//         )}
//         aria-label="Align right"
//       >
//         <RightAlign />
//       </button>
//       <button
//         onClick={() => editor.chain().focus().setTextAlign('center').run()}
//         className={cn(
//           'hover:bg-neutral-600',
//           editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600'
//         )}
//         aria-label="Align center"
//       >
//         <RightAlignLight />
//       </button>
//     </div>
//   );
// }





'use client';
import { Editor } from '@tiptap/react';
import { cn } from '@/lib/utils';
import FileUploader from '@/components/file-uploader';
import { useCallback, useState, useEffect, useRef } from 'react';
import { HexColorPicker } from 'react-colorful';

//Icons Import
import CodesPen from '@/assets/icons/editor/codespen.svg';
import BoldIcon from '@/assets/icons/editor/bold.svg';
import Italics from '@/assets/icons/editor/italics.svg';
import UnderlineIcon from '@/assets/icons/editor/underline.svg';
import Strike from '@/assets/icons/editor/strike.svg';
import Paint2 from '@/assets/icons/editor/paint2.svg';
import Paint from '@/assets/icons/editor/paint.svg';
import ImageUpload from '@/assets/icons/editor/upload-image.svg';
import LinkPaste from '@/assets/icons/editor/link-paste.svg';
import CommentPaste from '@/assets/icons/editor/comment-paste.svg';
import TableCells from '@/assets/icons/editor/tabe-cells.svg';
import LeftAlign from '@/assets/icons/editor/misc3-part.svg';
import NumberBullet from '@/assets/icons/editor/misc2-part.svg';
import Bullet from '@/assets/icons/editor/misc-parts.svg';
import RightAlign from '@/assets/icons/editor/paragraph.svg';
import RightAlignLight from '@/assets/icons/editor/paragraph-light.svg';

interface ToolbarPluginProps {
  editor: Editor | null;
  onImageUpload: (url: string) => void;
  enableImage?: boolean;
  onTriggerLinkPaste?: () => void;
  onCommentPaste?:() => void;
}

export default function ToolbarPlugin({
  editor,
  onImageUpload,
  enableImage = true,
  onTriggerLinkPaste,
  onCommentPaste,
}: ToolbarPluginProps) {
  // State for color pickers
  const [showTextColorPicker, setShowTextColorPicker] = useState(false);
  const [showBgColorPicker, setShowBgColorPicker] = useState(false);
  const [textColor, setTextColor] = useState('#000000');
  const [bgColor, setBgColor] = useState('#FFFFFF');

  // State for table creation
  const [showTableMenu, setShowTableMenu] = useState(false);
  const [rows, setRows] = useState(3);
  const [cols, setCols] = useState(3);

  // Refs for click-outside detection
  const textColorPickerRef = useRef<HTMLDivElement>(null);
  const bgColorPickerRef = useRef<HTMLDivElement>(null);
  const tableMenuRef = useRef<HTMLDivElement>(null);

  // Apply text color
  const applyTextColor = (color: string) => {
    editor?.chain().focus().setColor(color).run();
    setShowTextColorPicker(false);
  };

  //  background color
  const applyBgColor = (color: string) => {
    editor?.chain().focus().setHighlight({ color }).run();
    setShowBgColorPicker(false);
  };

  // Insert table
  const insertTable = () => {
    editor?.chain().focus().insertTable({ rows, cols, withHeaderRow: true }).run();
    setShowTableMenu(false);
  };

  // Close menus when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (textColorPickerRef.current && !textColorPickerRef.current.contains(event.target as Node)) {
        setShowTextColorPicker(false);
      }
      if (bgColorPickerRef.current && !bgColorPickerRef.current.contains(event.target as Node)) {
        setShowBgColorPicker(false);
      }
      if (tableMenuRef.current && !tableMenuRef.current.contains(event.target as Node)) {
        setShowTableMenu(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  if (!editor) {
    return null;
  }

  return (
    <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700 ">
      {/*  formatting buttons */}
      <button
        onClick={() => editor.chain().focus().toggleCode().run()}
        className={cn(editor.isActive('code') && 'bg-neutral-600')}
        aria-label="Code"
      >
        <CodesPen />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleBold().run()}
        className={cn(editor.isActive('bold') && 'bg-neutral-600')}
        aria-label="Bold"
      >
        <BoldIcon />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleItalic().run()}
        className={cn(editor.isActive('italic') && 'bg-neutral-600')}
        aria-label="Italic"
      >
        <Italics />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleUnderline().run()}
        className={cn(editor.isActive('underline') && 'bg-neutral-600')}
        aria-label="Underline"
      >
        <UnderlineIcon />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleStrike().run()}
        className={cn(editor.isActive('strike') && 'bg-neutral-600')}
        aria-label="Strikethrough"
      >
        <Strike />
      </button>

      {/* Text Color Picker */}
      <div className="relative" ref={textColorPickerRef}>
        <button
          onClick={() => setShowTextColorPicker(!showTextColorPicker)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('textStyle', { color: textColor }) && 'bg-neutral-600'
          )}
          aria-label="Text color"
        >
          <Paint />
        </button>
        {showTextColorPicker && (
          //
          <div className="absolute bottom-full left-0 z-50 mb-2 p-2 bg-white rounded shadow-lg">
            <HexColorPicker color={textColor} onChange={setTextColor} />
            <div className="flex justify-between mt-2">
              <button
                onClick={() => applyTextColor(textColor)}
                className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
              >
                Apply
              </button>
              <button
                onClick={() => editor.chain().focus().unsetColor().run()}
                className="px-2 py-1 bg-neutral-800 font-bold text-white rounded"
              >
                Reset
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Background Color Picker */}
      <div className="relative" ref={bgColorPickerRef}>
        <button
          onClick={() => setShowBgColorPicker(!showBgColorPicker)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('highlight') && 'bg-neutral-600'
          )}
          aria-label="Background color"
        >
          <Paint2 />
        </button>
        {showBgColorPicker && (
          // COlor picker
          <div className="absolute bottom-full left-0 z-50 mb-2 p-2 bg-white rounded shadow-lg">
            <HexColorPicker color={bgColor} onChange={setBgColor} />
            <div className="flex justify-between mt-2">
              <button
                onClick={() => applyBgColor(bgColor)}
                className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
              >
                Apply
              </button>
              <button
                onClick={() => editor.chain().focus().unsetHighlight().run()}
                className="px-2 py-1 bg-gray-500 bg-neutral-800 font-bold text-white rounded"
              >
                Reset
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Image Upload - unchanged */}
      {enableImage && (
        <FileUploader onUploadSuccess={onImageUpload}>
          <button type="button" aria-label="Upload image">
            <ImageUpload />
          </button>
        </FileUploader>
      )}

      {/* Link */}
      <button
        onClick={onTriggerLinkPaste}
        className={cn('hover:bg-neutral-600')}
        aria-label="Insert link"
      >
        <LinkPaste />
      </button>

      {/* Comment*/}
      <button
        onClick={onCommentPaste}
        className={cn('hover:bg-neutral-600')}
        aria-label="Add comment"
      >
        <CommentPaste />
      </button>

      {/* Table */}
      <div className="relative" ref={tableMenuRef}>
        <button
          onClick={() => setShowTableMenu(!showTableMenu)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('table') && 'bg-neutral-600'
          )}
          aria-label="Insert table"
        >
          <TableCells />
        </button>
        {showTableMenu && (
          /* TABLE MENU*/
          <div className="absolute bottom-full left-0 z-50 mb-2 p-4 bg-white rounded shadow-lg w-64">
            <h4 className="font-medium mb-2 text-neutral-800">Create Table</h4>
            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <label className="block text-sm mb-1 text-neutral-800 font-bold">Rows</label>
                <input
                  type="number"
                  min="1"
                  max="10"
                  value={rows}
                  onChange={(e) => setRows(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
                  className="w-full p-1 border rounded text-neutral-800"
                />
              </div>
              <div>
                <label className="block text-sm mb-1 text-neutral-800 font-bold">Columns</label>
                <input
                  type="number"
                  min="1"
                  max="10"
                  value={cols}
                  onChange={(e) => setCols(Math.max(1, Math.min(10, parseInt(e.target.value) || 1)))}
                  className="w-full p-1 border rounded text-neutral-800"
                />
              </div>
            </div>
            <button
              onClick={insertTable}
              className="w-full py-1 bg-primary text-white rounded hover:bg-primary/50 text-neutral-800"
            >
              Insert Table
            </button>
          </div>
        )}
      </div>

      {/* Bullet List*/}
      <button
        onClick={() => editor.chain().focus().toggleBulletList().run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive('bulletList') && 'bg-neutral-600'
        )}
        disabled={!editor.can().toggleBulletList()}
        aria-label="Bullet list"
      >
        <Bullet />
      </button>

      {/* Numbered List */}
      <button
        onClick={() => editor.chain().focus().toggleOrderedList().run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive('orderedList') && 'bg-neutral-600'
        )}
        disabled={!editor.can().toggleOrderedList()}
        aria-label="Numbered list"
      >
        <NumberBullet />
      </button>

      {/* Alignment Buttons */}
      <button
        onClick={() => editor.chain().focus().setTextAlign('left').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600'
        )}
        aria-label="Align left"
      >
        <LeftAlign />
      </button>
      <button
        onClick={() => editor.chain().focus().setTextAlign('right').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600'
        )}
        aria-label="Align right"
      >
        <RightAlign />
      </button>
      <button
        onClick={() => editor.chain().focus().setTextAlign('center').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600'
        )}
        aria-label="Align center"
      >
        <RightAlignLight />
      </button>
    </div>
  );
}