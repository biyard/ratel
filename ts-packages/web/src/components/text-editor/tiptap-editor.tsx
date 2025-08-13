'use client';
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Link from '@tiptap/extension-link';
import { Table } from '@tiptap/extension-table';
import Highlight from '@tiptap/extension-highlight';
import TextStyle from '@tiptap/extension-text-style';
import Color from '@tiptap/extension-color';
import TableRow from '@tiptap/extension-table-row';
import TableHeader from '@tiptap/extension-table-header';
import TableCell from '@tiptap/extension-table-cell';
import { forwardRef, useEffect, useImperativeHandle } from 'react';
import { Editor } from '@tiptap/core';

interface TiptapEditorProps {
  content?: string;
  onUpdate?: (content: string) => void;
  editable?: boolean;
  className?: string;
}

export const TiptapEditor = forwardRef<Editor | null, TiptapEditorProps>(
  ({ content = '', onUpdate, editable = true, className = '' }, ref) => {
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
        Color,
        Highlight.configure({ multicolor: true }),
        TextAlign.configure({
          types: ['heading', 'paragraph'],
        }),
        Table.configure({
          resizable: true,
          HTMLAttributes: {
            class: 'tiptap-table',
            style: 'border: 1px solid #e0e0e0;',
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
            style: 'background-color: #f5f5f5;',
          },
        }),
        TableCell.configure({
          HTMLAttributes: {
            class: 'tiptap-table-cell',
            style:
              'background-color: #fcb300; color: #333; border: 1px solid #e0e0e0;',
          },
        }),
        Link.configure({
          openOnClick: false,
        }),
        Underline,
      ],
      content,
      editable,
      onUpdate: ({ editor }) => {
        const html = editor.getHTML();
        onUpdate?.(html);
      },
    });

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
          className="outline-none min-h-[100px] text-neutral-300"
        />
      </div>
    );
  },
);

TiptapEditor.displayName = 'TiptapEditor';
