import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import Image from '@tiptap/extension-image';
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
      >
        {showToolbar && toolbarPosition === 'top' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            className={toolbarClassName}
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
            )}
            data-placeholder={placeholder}
          />
        </div>

        {showToolbar && toolbarPosition === 'bottom' && (
          <TiptapToolbar
            editor={editor}
            enabledFeatures={enabledFeatures}
            className={toolbarClassName}
          />
        )}
      </div>
    );
  },
);

TiptapEditor.displayName = 'TiptapEditor';
