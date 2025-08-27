'use client';
import { Editor } from '@tiptap/react';
import { cn } from '@/lib/utils';
import FileUploader from '@/components/file-uploader';
import { useState, useEffect, useRef } from 'react';
import { HexColorPicker } from 'react-colorful';
import { ToggleIconButton } from './primitives/toggle-iconbutton';

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
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleCode().run()}
        active={editor.isActive('code')}
        aria-label="Code"
      >
        <CodesPen />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleBold().run()}
        active={editor.isActive('bold')}
        aria-label="Bold"
      >
        <BoldIcon />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleItalic().run()}
        active={editor.isActive('italic')}
        aria-label="Italic"
      >
        <Italics />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleUnderline().run()}
        active={editor.isActive('underline')}
        aria-label="Underline"
      >
        <UnderlineIcon />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleStrike().run()}
        active={editor.isActive('strike')}
        aria-label="Strikethrough"
      >
        <Strike />
      </ToggleIconButton>

      {/* Text Color Picker */}
      <div className="relative" ref={textColorPickerRef}>
        <ToggleIconButton
          onClick={() => setShowTextColorPicker(!showTextColorPicker)}
          active={editor.isActive('textStyle', { color: textColor })}
          aria-label="Text color"
        >
          <Paint />
        </ToggleIconButton>
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
        <ToggleIconButton
          onClick={() => setShowBgColorPicker(!showBgColorPicker)}
          active={editor.isActive('highlight')}
          aria-label="Background color"
        >
          <Paint2 />
        </ToggleIconButton>
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
                className="px-2 py-1 bg-gray-500 bg-neutral-800 font-bold text-white rounded"
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
          <ToggleIconButton type="button" aria-label="Upload image">
            <ImageUpload />
          </ToggleIconButton>
        </FileUploader>
      )}

      {/* Link */}
      <ToggleIconButton onClick={onTriggerLinkPaste} aria-label="Insert link">
        <LinkPaste />
      </ToggleIconButton>

      {/* Comment*/}
      <ToggleIconButton onClick={onCommentPaste} aria-label="Add comment">
        <CommentPaste />
      </ToggleIconButton>

      {/* Table */}
      <div className="relative" ref={tableMenuRef}>
        <ToggleIconButton
          onClick={() => setShowTableMenu(!showTableMenu)}
          active={editor.isActive('table')}
          aria-label="Insert table"
        >
          <TableCells />
        </ToggleIconButton>
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
              className="w-full py-1 bg-primary text-white rounded hover:bg-primary/50 text-neutral-800"
            >
              Insert Table
            </button>
          </div>
        )}
      </div>

      {/* Bullet List*/}
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleBulletList().run()}
        active={editor.isActive('bulletList')}
        disabled={!editor.can().toggleBulletList()}
        aria-label="Bullet list"
      >
        <Bullet />
      </ToggleIconButton>

      {/* Numbered List */}
      <ToggleIconButton
        onClick={() => editor.chain().focus().toggleOrderedList().run()}
        active={editor.isActive('orderedList')}
        disabled={!editor.can().toggleOrderedList()}
        aria-label="Numbered list"
      >
        <NumberBullet />
      </ToggleIconButton>

      {/* Alignment Buttons */}
      <ToggleIconButton
        onClick={() => editor.chain().focus().setTextAlign('left').run()}
        active={editor.isActive({ textAlign: 'left' })}
        aria-label="Align left"
      >
        <LeftAlign />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().setTextAlign('right').run()}
        active={editor.isActive({ textAlign: 'right' })}
        aria-label="Align right"
      >
        <RightAlign />
      </ToggleIconButton>
      <ToggleIconButton
        onClick={() => editor.chain().focus().setTextAlign('center').run()}
        active={editor.isActive({ textAlign: 'center' })}
        aria-label="Align center"
      >
        <RightAlignLight />
      </ToggleIconButton>
    </div>
  );
}
