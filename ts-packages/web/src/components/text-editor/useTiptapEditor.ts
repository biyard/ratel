import { useEditor, EditorOptions } from '@tiptap/react';
import type { Extension as TiptapExtension } from '@tiptap/core';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';
import Highlight from '@tiptap/extension-highlight';
import TextAlign from '@tiptap/extension-text-align';
import Table from '@tiptap/extension-table';
import TableRow from '@tiptap/extension-table-row';
import TableHeader from '@tiptap/extension-table-header';
import TableCell from '@tiptap/extension-table-cell';

export type EditorFeatures = {
  underline?: boolean;
  color?: boolean;
  link?: boolean;
  image?: boolean;
  highlight?: boolean;
  align?: boolean;
  table?: boolean;
  lists?: boolean; // uses StarterKit list configs
};

export interface UseTiptapEditorOptions {
  content?: string;
  editable?: boolean;
  onUpdateHTML?: (html: string) => void;
  features?: EditorFeatures;
  editorProps?: EditorOptions['editorProps'];
  handleImageFile?: (file: File) => void;
}

export const useTiptapEditor = (options?: UseTiptapEditorOptions) => {
  const {
    content = '',
    editable = true,
    onUpdateHTML,
    features = {},
    editorProps,
    handleImageFile,
  } = options ?? {};

  const exts: TiptapExtension[] = [
    StarterKit.configure({
      bulletList: { HTMLAttributes: { class: 'list-disc pl-4' } },
      orderedList: { HTMLAttributes: { class: 'list-decimal pl-4' } },
    }) as unknown as TiptapExtension,
  ];

  if (features.underline) exts.push(Underline as unknown as TiptapExtension);
  // TextStyle is required for Color extension
  if (features.color) {
    exts.push(TextStyle as unknown as TiptapExtension);
    exts.push(Color as unknown as TiptapExtension);
  }
  if (features.link) {
    exts.push(
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'text-blue-400 underline cursor-pointer hover:text-blue-300',
        },
        // Loosen validation – callers can sanitize if needed
        /*eslint-disable-next-line @typescript-eslint/no-unused-vars*/
        validate: (_href) => true,
      }) as unknown as TiptapExtension,
    );
  }
  if (features.image) {
    exts.push(
      Image.configure({
        HTMLAttributes: {
          class:
            'inline-block max-w-[120px] max-h-[80px] object-cover rounded border border-gray-600 mx-1 my-1',
        },
        inline: true,
      }) as unknown as TiptapExtension,
    );
  }
  if (features.highlight) {
    exts.push(Highlight.configure({ multicolor: true }) as unknown as TiptapExtension);
  }
  if (features.align) {
    exts.push(
      TextAlign.configure({
        types: ['heading', 'paragraph'],
      }) as unknown as TiptapExtension,
    );
  }
  if (features.table) {
    exts.push(
      Table.configure({
        resizable: true,
        HTMLAttributes: {
          class: 'tiptap-table',
          style: 'border: 1px solid #e0e0e0;',
        },
      }) as unknown as TiptapExtension,
      TableRow.configure({
        HTMLAttributes: { class: 'tiptap-table-row' },
      }) as unknown as TiptapExtension,
      TableHeader.configure({
        HTMLAttributes: {
          class: 'tiptap-table-header',
          style: 'background-color: #f5f5f5;',
        },
      }) as unknown as TiptapExtension,
      TableCell.configure({
        HTMLAttributes: {
          class: 'tiptap-table-cell',
          style:
            'background-color: #fcb300; color: #333; border: 1px solid #e0e0e0;',
        },
      }) as unknown as TiptapExtension,
    );
  }

  return useEditor({
    extensions: exts,
    content,
    editable,
    onUpdate: ({ editor }) => {
      const html = editor.getHTML();
      onUpdateHTML?.(html);
    },
    editorProps: {
      attributes: {
        class:
          'prose prose-invert max-w-none focus:outline-none min-h-[120px] p-4 text-gray-300',
        placeholder:
          'Type here. Use Markdown, BB code, or HTML to format. Drag or paste images.',
      },
      // Support drag-drop image
      handleDrop:
        handleImageFile
          ? (view, event) => {
              const file = (event as DragEvent).dataTransfer?.files?.[0];
              if (file?.type?.startsWith('image/')) {
                event.preventDefault();
                handleImageFile(file);
                return true;
              }
              return false;
            }
          : undefined,
      // Support paste image
      handlePaste:
        handleImageFile
          ? (view, event) => {
              const items = (event as ClipboardEvent).clipboardData?.items;
              if (!items) return false;
              for (let i = 0; i < items.length; i++) {
                const f = items[i]?.getAsFile?.();
                if (f?.type?.startsWith('image/')) {
                  event.preventDefault();
                  handleImageFile(f);
                  return true;
                }
              }
              return false;
            }
          : undefined,
      ...(editorProps ?? {}),
    },
  });
};
