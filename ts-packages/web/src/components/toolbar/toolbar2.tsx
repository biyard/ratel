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
// import {
//   ImagePlus,
//   Bold,
//   Italic,
//   Underline,
//   Strikethrough,
// } from 'lucide-react';
import { cn } from '@/lib/utils';
import FileUploader from '@/components/file-uploader';

import Capslock from '@/assets/icons/editor/capslock.svg';
import CodesPen from '@/assets/icons/editor/codespen.svg';
import BoldIcon from '@/assets/icons/editor/bold.svg';
import Italics from '@/assets/icons/editor/italics.svg';
import Underline from '@/assets/icons/editor/underline.svg';
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
      {/* <button
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
      </button> */}

      {/* Testing editor ui.. */}
      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Capslock />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <CodesPen />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <BoldIcon />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Italics />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Underline />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Strike />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Paint2 />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Paint />
      </button>

      {enableImage ? (
        <FileUploader onUploadSuccess={onImageUpload}>
          <ImageUpload />
        </FileUploader>
      ) : (
        <></>
      )}

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <LinkPaste />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <CommentPaste />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <TableCells />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <LeftAlign />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <NumberBullet />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <Bullet />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <RightAlign />
      </button>

      <button
        onClick={() => formatText('underline')}
        className={cn(isUnderline && 'bg-neutral-600 ')}
        aria-label="Format text to underlined"
      >
        <RightAlignLight />
      </button>
    </div>
  );
}
