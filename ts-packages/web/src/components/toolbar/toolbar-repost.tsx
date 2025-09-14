'use client';
import { Editor } from '@tiptap/react';
import { cn } from '@/lib/utils';
import FileUploader from '@/components/file-uploader';
import { useState, useEffect, useRef } from 'react';
import { HexColorPicker } from 'react-colorful';

//Icons Import
import CodesPen from '@/assets/icons/editor/codespen.svg';
import BoldIcon from '@/assets/icons/editor/bold.svg';
import Italics from '@/assets/icons/editor/italics.svg';
import UnderlineIcon from '@/assets/icons/editor/underline.svg';
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

interface ToolbarPluginProps {
  editor: Editor | null;
  onImageUpload?: (url: string) => void;
  enableImage?: boolean;
  onTriggerLinkPaste?: () => void;
  onCommentPaste?: () => void;
}

export default function ToolbarPlugin({
  editor,
  onImageUpload,
  enableImage = true,
  onTriggerLinkPaste,
  onCommentPaste,
}: ToolbarPluginProps) {
  // State for color pickers
  const [showTextColorPicker, setShowTextColorPicker] = useState(false);
  const [showBgColorPicker, setShowBgColorPicker] = useState(false);
  const [textColor, setTextColor] = useState('#000000');
  const [bgColor, setBgColor] = useState('#FFFFFF');

  // State for table creation
  const [showTableMenu, setShowTableMenu] = useState(false);
  const [rows, setRows] = useState(3);
  const [cols, setCols] = useState(3);

  // Refs for click-outside detection
  const textColorPickerRef = useRef<HTMLDivElement>(null);
  const bgColorPickerRef = useRef<HTMLDivElement>(null);
  const tableMenuRef = useRef<HTMLDivElement>(null);

  // Apply text color
  const applyTextColor = (color: string) => {
    editor?.chain().focus().setColor(color).run();
    setShowTextColorPicker(false);
  };

  //  background color
  const applyBgColor = (color: string) => {
    editor?.chain().focus().setHighlight({ color }).run();
    setShowBgColorPicker(false);
  };

  // Insert table
  const insertTable = () => {
    editor
      ?.chain()
      .focus()
      .insertTable({ rows, cols, withHeaderRow: true })
      .run();
    setShowTableMenu(false);
  };

  // Close menus when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        textColorPickerRef.current &&
        !textColorPickerRef.current.contains(event.target as Node)
      ) {
        setShowTextColorPicker(false);
      }
      if (
        bgColorPickerRef.current &&
        !bgColorPickerRef.current.contains(event.target as Node)
      ) {
        setShowBgColorPicker(false);
      }
      if (
        tableMenuRef.current &&
        !tableMenuRef.current.contains(event.target as Node)
      ) {
        setShowTableMenu(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  if (!editor) {
    return null;
  }

  return (
    <div className="flex items-center gap-4 [&>button]:size-6 [&>button]:rounded [&>button]:hover:bg-neutral-700 ">
      {/*  formatting buttons */}
      <button
        onClick={() => editor.chain().focus().toggleCode().run()}
        className={cn(editor.isActive('code') && 'bg-neutral-600')}
        aria-label="Code"
      >
        <CodesPen />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleBold().run()}
        className={cn(editor.isActive('bold') && 'bg-neutral-600')}
        aria-label="Bold"
      >
        <BoldIcon />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleItalic().run()}
        className={cn(editor.isActive('italic') && 'bg-neutral-600')}
        aria-label="Italic"
      >
        <Italics />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleUnderline().run()}
        className={cn(editor.isActive('underline') && 'bg-neutral-600')}
        aria-label="Underline"
      >
        <UnderlineIcon />
      </button>
      <button
        onClick={() => editor.chain().focus().toggleStrike().run()}
        className={cn(editor.isActive('strike') && 'bg-neutral-600')}
        aria-label="Strikethrough"
      >
        <Strike />
      </button>

      {/* Text Color Picker */}
      <div className="relative" ref={textColorPickerRef}>
        <button
          onClick={() => setShowTextColorPicker(!showTextColorPicker)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('textStyle', { color: textColor }) &&
              'bg-neutral-600',
          )}
          aria-label="Text color"
        >
          <Paint />
        </button>
        {showTextColorPicker && (
          //
          <div className="absolute bottom-full left-0 z-50 mb-2 p-2 bg-white rounded shadow-lg">
            <HexColorPicker color={textColor} onChange={setTextColor} />
            <div className="flex justify-between mt-2">
              <button
                onClick={() => applyTextColor(textColor)}
                className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
              >
                Apply
              </button>
              <button
                onClick={() => editor.chain().focus().unsetColor().run()}
                className="px-2 py-1 bg-neutral-800 font-bold text-white rounded"
              >
                Reset
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Background Color Picker */}
      <div className="relative" ref={bgColorPickerRef}>
        <button
          onClick={() => setShowBgColorPicker(!showBgColorPicker)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('highlight') && 'bg-neutral-600',
          )}
          aria-label="Background color"
        >
          <Paint2 />
        </button>
        {showBgColorPicker && (
          // COlor picker
          <div className="absolute bottom-full left-0 z-50 mb-2 p-2 bg-white rounded shadow-lg">
            <HexColorPicker color={bgColor} onChange={setBgColor} />
            <div className="flex justify-between mt-2">
              <button
                onClick={() => applyBgColor(bgColor)}
                className="px-2 py-1 bg-primary font-bold text-neutral-800 rounded"
              >
                Apply
              </button>
              <button
                onClick={() => editor.chain().focus().unsetHighlight().run()}
                className="px-2 py-1  bg-neutral-800 font-bold text-white rounded"
              >
                Reset
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Image Upload - unchanged */}
      {enableImage && (
        <FileUploader onUploadSuccess={onImageUpload}>
          <button type="button" aria-label="Upload image">
            <ImageUpload />
          </button>
        </FileUploader>
      )}

      {/* Link */}
      <button
        onClick={onTriggerLinkPaste}
        className={cn('hover:bg-neutral-600')}
        aria-label="Insert link"
      >
        <LinkPaste />
      </button>

      {/* Comment*/}
      <button
        onClick={onCommentPaste}
        className={cn('hover:bg-neutral-600')}
        aria-label="Add comment"
      >
        <CommentPaste />
      </button>

      {/* Table */}
      <div className="relative" ref={tableMenuRef}>
        <button
          onClick={() => setShowTableMenu(!showTableMenu)}
          className={cn(
            'hover:bg-neutral-600',
            editor.isActive('table') && 'bg-neutral-600',
          )}
          aria-label="Insert table"
        >
          <TableCells />
        </button>
        {showTableMenu && (
          /* TABLE MENU*/
          <div className="absolute bottom-full left-0 z-50 mb-2 p-4 bg-white rounded shadow-lg w-64">
            <h4 className="font-medium mb-2 text-neutral-800">Create Table</h4>
            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <label className="block text-sm mb-1 text-neutral-800 font-bold">
                  Rows
                </label>
                <input
                  type="number"
                  min="1"
                  max="10"
                  value={rows}
                  onChange={(e) =>
                    setRows(
                      Math.max(1, Math.min(10, parseInt(e.target.value) || 1)),
                    )
                  }
                  className="w-full p-1 border rounded text-neutral-800"
                />
              </div>
              <div>
                <label className="block text-sm mb-1 text-neutral-800 font-bold">
                  Columns
                </label>
                <input
                  type="number"
                  min="1"
                  max="10"
                  value={cols}
                  onChange={(e) =>
                    setCols(
                      Math.max(1, Math.min(10, parseInt(e.target.value) || 1)),
                    )
                  }
                  className="w-full p-1 border rounded text-neutral-800"
                />
              </div>
            </div>
            <button
              onClick={insertTable}
              className="w-full py-1 bg-primary rounded hover:bg-primary/50 text-neutral-800"
            >
              Insert Table
            </button>
          </div>
        )}
      </div>

      {/* Bullet List*/}
      <button
        onClick={() => editor.chain().focus().toggleBulletList().run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive('bulletList') && 'bg-neutral-600',
        )}
        disabled={!editor.can().toggleBulletList()}
        aria-label="Bullet list"
      >
        <Bullet />
      </button>

      {/* Numbered List */}
      <button
        onClick={() => editor.chain().focus().toggleOrderedList().run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive('orderedList') && 'bg-neutral-600',
        )}
        disabled={!editor.can().toggleOrderedList()}
        aria-label="Numbered list"
      >
        <NumberBullet />
      </button>

      {/* Alignment Buttons */}
      <button
        onClick={() => editor.chain().focus().setTextAlign('left').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'left' }) && 'bg-neutral-600',
        )}
        aria-label="Align left"
      >
        <LeftAlign />
      </button>
      <button
        onClick={() => editor.chain().focus().setTextAlign('right').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'right' }) && 'bg-neutral-600',
        )}
        aria-label="Align right"
      >
        <RightAlign />
      </button>
      <button
        onClick={() => editor.chain().focus().setTextAlign('center').run()}
        className={cn(
          'hover:bg-neutral-600',
          editor.isActive({ textAlign: 'center' }) && 'bg-neutral-600',
        )}
        aria-label="Align center"
      >
        <RightAlignLight />
      </button>
    </div>
  );
}
