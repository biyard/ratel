import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Image from '@tiptap/extension-image';
import Table from '@tiptap/extension-table';
import TableRow from '@tiptap/extension-table-row';
import TableCell from '@tiptap/extension-table-cell';
import TableHeader from '@tiptap/extension-table-header';
import { forwardRef, useEffect, useImperativeHandle } from 'react';
import { Editor } from '@tiptap/core';
import { cn } from '@/lib/utils';
import { TiptapEditorProps, DEFAULT_ENABLED_FEATURES } from './types';
import { TiptapToolbar } from './tiptap-toolbar';

export const TiptapEditor = forwardRef<Editor | null, TiptapEditorProps>(
  (
    {
      content = '',
      onUpdate,
      editable = true,
      placeholder = 'Type your script',
      showToolbar = true,
      toolbarPosition = 'top',
      enabledFeatures = DEFAULT_ENABLED_FEATURES,
      className,
      toolbarClassName,
      editorClassName,
      minHeight = '200px',
      maxHeight,
      onFocus,
      onBlur,
      onImageUpload,
      'data-pw': dataPw,
    },
    ref,
  ) => {
    const editor = useEditor(
      {
        extensions: [
          StarterKit.configure({
            heading: {
              levels: [1, 2, 3],
            },
            bulletList: {
              HTMLAttributes: { class: 'list-disc pl-4' },
            },
            orderedList: {
              HTMLAttributes: { class: 'list-decimal pl-4' },
            },
          }),
          TextStyle,
          Color,
          Highlight.configure({ multicolor: true }),
          TextAlign.configure({
            types: ['heading', 'paragraph'],
            alignments: ['left', 'center', 'right'],
          }),
          Underline,
          Image.configure({
            inline: true,
            allowBase64: true,
            HTMLAttributes: {
              class: 'rounded-lg max-w-full h-auto my-4 mx-auto block',
            },
          }),
          Table.configure({
            resizable: true,
            HTMLAttributes: {
              class: 'border-collapse table-auto w-full my-4',
            },
          }),
          TableRow,
          TableHeader.configure({
            HTMLAttributes: {
              class: 'bg-muted font-semibold',
            },
          }),
          TableCell.configure({
            HTMLAttributes: {
              class: 'border border-border p-2 min-w-[100px]',
            },
          }),
        ],
        content,
        editable,
        onUpdate: ({ editor }: { editor: Editor }) => {
          const html = editor.getHTML();
          onUpdate?.(html);
        },
        onFocus: () => {
          onFocus?.();
        },
        onBlur: () => {
          onBlur?.();
        },
      },
      [editable],
    ) as Editor | null;

    useImperativeHandle<Editor | null, Editor | null>(ref, () => editor, [
      editor,
    ]);

    useEffect(() => {
      if (editor && !editor.isDestroyed && content !== editor.getHTML()) {
        editor.commands.setContent(content);
      }
    }, [content, editor]);

    useEffect(() => {
      return () => {
        if (editor) {
          editor.destroy();
        }
      };
    }, [editor]);

    if (!editor) {
      return <></>;
    }

    return (
      <div
        className={cn(
          'flex flex-col w-full rounded-lg border transition-colors p-1',
          'bg-card text-foreground border-transparent',
          'focus-within:border-primary',
          className,
        )}
        data-pw={dataPw}
      >
        {showToolbar && toolbarPosition === 'top' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            className={toolbarClassName}
            onImageUpload={onImageUpload}
          />
        )}

        <div
          className={cn('flex-1 overflow-y-auto', 'px-5 py-3', editorClassName)}
          style={{
            minHeight: showToolbar ? `calc(${minHeight} - 48px)` : minHeight,
            maxHeight,
          }}
        >
          <EditorContent
            editor={editor}
            className={cn(
              'tiptap-editor',
              'text-foreground text-[15px]',
              '[&_.ProseMirror]:outline-none',
              '[&_.ProseMirror]:min-h-full',
              '[&_.ProseMirror]:h-full',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:content-[attr(data-placeholder)]',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:text-foreground-muted',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:float-left',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:pointer-events-none',
              '[&_.ProseMirror_p.is-editor-empty:first-child::before]:h-0',
              '[&_.ProseMirror_h1]:text-2xl [&_.ProseMirror_h1]:font-bold [&_.ProseMirror_h1]:mt-6 [&_.ProseMirror_h1]:mb-4',
              '[&_.ProseMirror_h2]:text-xl [&_.ProseMirror_h2]:font-bold [&_.ProseMirror_h2]:mt-5 [&_.ProseMirror_h2]:mb-3',
              '[&_.ProseMirror_h3]:text-lg [&_.ProseMirror_h3]:font-semibold [&_.ProseMirror_h3]:mt-4 [&_.ProseMirror_h3]:mb-2',
              '[&_.ProseMirror_ul]:list-disc [&_.ProseMirror_ul]:pl-6 [&_.ProseMirror_ul]:my-2',
              '[&_.ProseMirror_ol]:list-decimal [&_.ProseMirror_ol]:pl-6 [&_.ProseMirror_ol]:my-2',
              '[&_.ProseMirror_li]:my-1',
              '[&_.ProseMirror_p]:my-2',
              '[&_.ProseMirror_mark]:bg-yellow-200 [&_.ProseMirror_mark]:px-0.5',
              // Table styles
              '[&_.ProseMirror_table]:border-collapse [&_.ProseMirror_table]:table-auto [&_.ProseMirror_table]:w-full [&_.ProseMirror_table]:my-4',
              '[&_.ProseMirror_td]:border [&_.ProseMirror_td]:border-border [&_.ProseMirror_td]:p-2 [&_.ProseMirror_td]:min-w-[100px] [&_.ProseMirror_td]:relative',
              '[&_.ProseMirror_th]:border [&_.ProseMirror_th]:border-border [&_.ProseMirror_th]:p-2 [&_.ProseMirror_th]:min-w-[100px] [&_.ProseMirror_th]:bg-muted [&_.ProseMirror_th]:font-semibold [&_.ProseMirror_th]:relative',
              // Selected cells highlighting for merging
              '[&_.ProseMirror_.selectedCell]:bg-primary/20',
              '[&_.ProseMirror_.selectedCell]:border-primary',
              '[&_.ProseMirror_.selectedCell]:border-2',
              '[&_.ProseMirror_.selectedCell]:outline',
              '[&_.ProseMirror_.selectedCell]:outline-2',
              '[&_.ProseMirror_.selectedCell]:outline-primary/40',
              '[&_.ProseMirror_.selectedCell]:outline-offset-[-1px]',
              // Column resize handle
              '[&_.ProseMirror_.column-resize-handle]:absolute [&_.ProseMirror_.column-resize-handle]:right-[-2px] [&_.ProseMirror_.column-resize-handle]:top-0 [&_.ProseMirror_.column-resize-handle]:bottom-0 [&_.ProseMirror_.column-resize-handle]:w-[4px] [&_.ProseMirror_.column-resize-handle]:bg-primary [&_.ProseMirror_.column-resize-handle]:pointer-events-none',
            )}
            data-placeholder={placeholder}
          />
        </div>

        {showToolbar && toolbarPosition === 'bottom' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            className={toolbarClassName}
            onImageUpload={onImageUpload}
          />
        )}
      </div>
    );
  },
);

TiptapEditor.displayName = 'TiptapEditor';
