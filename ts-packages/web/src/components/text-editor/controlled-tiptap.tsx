'use client';

import { forwardRef, useImperativeHandle } from 'react';
import { EditorContent } from '@tiptap/react';
import type { Editor } from '@tiptap/core';
import { useTiptapEditor, type UseTiptapEditorOptions } from './tiptap-editor';

export interface ControlledTiptapProps
  extends Pick<
    UseTiptapEditorOptions,
    | 'content'
    | 'editable'
    | 'onUpdateHTML'
    | 'features'
    | 'editorProps'
    | 'handleImageFile'
  > {
  className?: string;
}

export const ControlledTiptap = forwardRef<
  Editor | null,
  ControlledTiptapProps
>(
  (
    {
      content = '',
      editable = true,
      onUpdateHTML,
      features,
      editorProps,
      handleImageFile,
      className = '',
    },
    ref,
  ) => {
    const editor = useTiptapEditor({
      content,
      editable,
      onUpdateHTML,
      features,
      editorProps,
      handleImageFile,
    });

    useImperativeHandle<Editor | null, Editor | null>(ref, () => editor, [
      editor,
    ]);

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

ControlledTiptap.displayName = 'ControlledTiptap';
