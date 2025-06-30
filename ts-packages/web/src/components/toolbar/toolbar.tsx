'use client';

import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useCallback, useEffect, useState } from 'react';
import {
  $getSelection,
  $isRangeSelection,
  FORMAT_TEXT_COMMAND,
  TextFormatType,
  COMMAND_PRIORITY_LOW,
  SELECTION_CHANGE_COMMAND,
} from 'lexical';
import {
  ImagePlus,
  Bold,
  Italic,
  Underline,
  Strikethrough,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import FileUploader from '@/components/file-uploader';

export default function ToolbarPlugin({
  onImageUpload,
  enableImage = true,
}: {
  onImageUpload: (url: string) => void;
  enableImage?: boolean;
}) {
  const [editor] = useLexicalComposerContext();
  const [activeEditor, setActiveEditor] = useState(editor);
  const [isBold, setIsBold] = useState(false);
  const [isItalic, setIsItalic] = useState(false);
  const [isUnderline, setIsUnderline] = useState(false);
  const [isStrikethrough, setIsStrikethrough] = useState(false);

  const updateToolbar = useCallback(() => {
    const selection = $getSelection();
    if ($isRangeSelection(selection)) {
      setIsBold(selection.hasFormat('bold'));
      setIsItalic(selection.hasFormat('italic'));
      setIsUnderline(selection.hasFormat('underline'));
      setIsStrikethrough(selection.hasFormat('strikethrough'));
    }
  }, []);

  useEffect(() => {
    return editor.registerCommand(
      SELECTION_CHANGE_COMMAND,
      (_payload, newEditor) => {
        updateToolbar();
        setActiveEditor(newEditor);
        return false;
      },
      COMMAND_PRIORITY_LOW,
    );
  }, [editor, updateToolbar]);

  useEffect(() => {
    return activeEditor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        updateToolbar();
      });
    });
  }, [activeEditor, updateToolbar]);

  const formatText = (format: TextFormatType) => {
    activeEditor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  };

  return (
    <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700">
      <button
        onClick={() => formatText('bold')}
        className={cn(isBold && 'bg-neutral-600 text-white')}
        aria-label="Format text as bold"
      >
        <Bold />
      </button>
      <button
        onClick={() => formatText('italic')}
        className={cn(isItalic && 'bg-neutral-600 text-white')}
        aria-label="Format text as italics"
      >
        <Italic />
      </button>
      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 text-white')}
        aria-label="Format text to underlined"
      >
        <Underline />
      </button>
      <button
        onClick={() => formatText('strikethrough')}
        className={cn(isStrikethrough && 'bg-neutral-600 text-white')}
        aria-label="Format text with a strikethrough"
      >
        <Strikethrough />
      </button>

      {enableImage ? (
        <FileUploader onUploadSuccess={onImageUpload}>
          <ImagePlus />
        </FileUploader>
      ) : (
        <></>
      )}
    </div>
  );
}
