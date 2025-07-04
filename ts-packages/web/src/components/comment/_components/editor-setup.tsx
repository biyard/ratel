import { useEditor } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';

interface UseRichTextEditorOptions {
  handleImageFile?: (file: File) => void;
}

export const useRichTextEditor = (options?: UseRichTextEditorOptions) => {
  const handleImageFile = options?.handleImageFile;

  return useEditor({
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
        validate: (href) => true,
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
      handleDrop: handleImageFile
        ? (view, event) => {
            const file = event.dataTransfer?.files?.[0];
            if (file?.type?.startsWith('image/')) {
              event.preventDefault();
              handleImageFile(file);
              return true;
            }
            return false;
          }
        : undefined,
      handlePaste: handleImageFile
        ? (view, event) => {
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
          }
        : undefined,
    },
  });
};
