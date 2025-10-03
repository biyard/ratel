'use client';
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Link from '@tiptap/extension-link';
import Table from '@tiptap/extension-table';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import TableRow from '@tiptap/extension-table-row';
import TableHeader from '@tiptap/extension-table-header';
import TableCell from '@tiptap/extension-table-cell';
import { forwardRef, useEffect, useImperativeHandle } from 'react';
import { CaseToggle } from '@/lib/tiptap/extensions/case-toggle';
import { Editor } from '@tiptap/core';

interface TiptapEditorProps {
  content?: string;
  onUpdate?: (content: string) => void;
  editable?: boolean;
  className?: string;
  onCreate?: ((editor: Editor) => void) | (() => void);
}

export const TiptapEditor = forwardRef<Editor | null, TiptapEditorProps>(
  (
    { content = '', onUpdate, editable = true, className = '', onCreate },
    ref,
  ) => {
    const editor = useEditor({
      extensions: [
        StarterKit.configure({
          bulletList: {
            HTMLAttributes: { class: 'list-disc pl-4' },
          },
          orderedList: {
            HTMLAttributes: { class: 'list-decimal pl-4' },
          },
        }),
        TextStyle,
        CaseToggle,
        Color,
        Highlight.configure({ multicolor: true }),
        TextAlign.configure({
          types: ['heading', 'paragraph'],
        }),
        Table.configure({
          resizable: true,
          HTMLAttributes: {
            class: 'tiptap-table',
          },
        }),
        TableRow.configure({
          HTMLAttributes: {
            class: 'tiptap-table-row',
          },
        }),
        TableHeader.configure({
          HTMLAttributes: {
            class: 'tiptap-table-header',
          },
        }),
        TableCell.configure({
          HTMLAttributes: {
            class: 'tiptap-table-cell',
          },
        }),
        Link.configure({
          openOnClick: false,
          autolink: true,
          linkOnPaste: true,
        }),
        Underline,
      ],
      content,
      editable,
      onUpdate: ({ editor }: { editor: Editor }) => {
        const html = editor.getHTML();
        onUpdate?.(html);
      },

      onCreate: onCreate
        ? ({ editor }) => {
            if (onCreate.length > 0) {
              (onCreate as (editor: Editor) => void)(editor);
            } else {
              (onCreate as () => void)();
            }
          }
        : undefined,
    }) as Editor | null;

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
      return <div className={className}>Loading editor...</div>;
    }

    return (
      <div className={`flex flex-col ${className}`}>
        <EditorContent
          editor={editor}
          className="outline-none min-h-[100px] text-desc-text text-sm [&_.ProseMirror]:outline-none [&_.ProseMirror]:border-0 [&_.ProseMirror]:ring-0 [&_.ProseMirror]:focus:outline-none [&_.ProseMirror]:focus:border-0 [&_.ProseMirror]:focus:ring-0 [&_.ProseMirror]:focus:shadow-none [&_.ProseMirror]:focus-within:outline-none [&_.ProseMirror]:focus-within:ring-0 [&_.ProseMirror]:focus-within:border-0 [&_.ProseMirror]:focus-within:shadow-none"
        />
      </div>
    );
  },
);

TiptapEditor.displayName = 'TiptapEditor';
