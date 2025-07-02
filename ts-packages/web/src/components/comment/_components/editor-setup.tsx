// // EditorSetup.tsx
// import { useEditor } from "@tiptap/react"
// import StarterKit from "@tiptap/starter-kit"
// import Underline from "@tiptap/extension-underline"
// import TextStyle from "@tiptap/extension-text-style"
// import Color from "@tiptap/extension-color"
// import Link from "@tiptap/extension-link"
// import Image from "@tiptap/extension-image"

// export const useRichTextEditor = (handleImageFile: (file: File) => void) => {
//   return useEditor({
//     extensions: [
//       StarterKit,
//       Underline,
//       TextStyle,
//       Color,
//       Link.configure({
//         openOnClick: false,
//         HTMLAttributes: {
//           class: "text-blue-400 underline cursor-pointer hover:text-blue-300",
//         },
//         validate: (href) => /^https?:\/\//.test(href),
//       }),
//       Image.configure({
//         HTMLAttributes: {
//           class:
//             "inline-block max-w-[120px] max-h-[80px] object-cover rounded border border-gray-600 mx-1 my-1",
//         },
//         inline: true,
//       }),
//     ],
//     content: "",
//     editorProps: {
//       attributes: {
//         class:
//           "prose prose-invert max-w-none focus:outline-none min-h-[120px] p-4 text-gray-300",
//         placeholder:
//           "Type here. Use Markdown, BB code, or HTML to format. Drag or paste images.",
//       },
//       handleDrop: (view, event, slice, moved) => {
//         const file = event.dataTransfer?.files?.[0]
//         if (!moved && file?.type?.startsWith("image/")) {
//           event.preventDefault()
//           handleImageFile(file)
//           return true
//         }
//         return false
//       },
//       handlePaste: (view, event, slice) => {
//         const items = event.clipboardData?.items
//         for (let i = 0; i < (items?.length || 0); i++) {
//           const file = items?.[i]?.getAsFile?.()
//           if (file?.type.startsWith("image/")) {
//             event.preventDefault()
//             handleImageFile(file)
//             return true
//           }
//         }
//         return false
//       },
//     },
//   })
// }

// components/editor/EditorSetup.tsx
import { useEditor } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';

export const useRichTextEditor = (handleImageFile: (file: File) => void) =>
  useEditor({
    extensions: [
      StarterKit,
      Underline,
      TextStyle,
      Color,
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'text-blue-400 underline cursor-pointer hover:text-blue-300',
        },
        validate: (href) => /^https?:\/\//.test(href),
      }),
      Image.configure({
        HTMLAttributes: {
          class:
            'inline-block max-w-[120px] max-h-[80px] object-cover rounded border border-gray-600 mx-1 my-1',
        },
        inline: true,
      }),
    ],
    content: '',
    editorProps: {
      attributes: {
        class:
          'prose prose-invert max-w-none focus:outline-none min-h-[120px] p-4 text-gray-300',
        placeholder:
          'Type here. Use Markdown, BB code, or HTML to format. Drag or paste images.',
      },
      handleDrop(view, event) {
        const file = event.dataTransfer?.files?.[0];
        if (file?.type?.startsWith('image/')) {
          event.preventDefault();
          handleImageFile(file);
          return true;
        }
        return false;
      },
      handlePaste(view, event) {
        // const items = event.clipboardData?.items

        // for (let i = 0; i < (items?.length || 0); i++) {
        //   const file = items[i]?.getAsFile?.()
        //   if (file?.type.startsWith("image/")) {
        //     event.preventDefault()
        //     handleImageFile(file)
        //     return true
        //   }
        // }
        const items = event.clipboardData?.items;
        if (!items) return false;
        for (let i = 0; i < items.length; i++) {
          const file = items[i]?.getAsFile?.();
          if (file?.type.startsWith('image/')) {
            event.preventDefault();
            handleImageFile(file);
            return true;
          }
        }
        return false;
        
      },
    },
  });
