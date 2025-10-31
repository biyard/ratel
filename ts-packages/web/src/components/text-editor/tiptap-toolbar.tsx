import { cn } from '@/lib/utils';
import { TiptapToolbarProps, DEFAULT_ENABLED_FEATURES } from './types';
import { ToolbarButton } from './tiptap-toolbar-button';
import { ColorPicker } from './tiptap-color-picker';
import { HeadingDropdown } from './tiptap-heading-dropdown';
import {
  EditorBold,
  EditorItalics,
  EditorUnderline,
  EditorStrike,
  AlignmentsAlignLeft,
  AlignmentsAlignCenter,
  AlignmentsAlignRight,
  Ordered1,
  Bullet1,
  EditorUploadImage,
  TableInsertTable,
  TableDeleteTable,
  TableAddRowBefore,
  TableAddRowAfter,
  TableDeleteRow,
  TableAddColumnBefore,
  TableAddColumnAfter,
  TableDeleteColumn,
  TableMergeCells,
  TableSplitCell,
  ChevronLeft,
  ChevronRight2,
} from '../icons';
import { useRef, useState, useEffect } from 'react';
import { Video } from 'lucide-react';

export const TiptapToolbar = ({
  editor,
  enabledFeatures = DEFAULT_ENABLED_FEATURES,
  className,
  openVideoPicker,
}: TiptapToolbarProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);
  const [isInTable, setIsInTable] = useState(false);

  if (!editor) return null;

  const features = { ...DEFAULT_ENABLED_FEATURES, ...enabledFeatures };

  // Check if cursor is inside a table
  useEffect(() => {
    if (!editor) return;

    const updateTableState = () => {
      setIsInTable(editor.isActive('table'));
    };

    // Initial check
    updateTableState();

    // Listen to editor updates
    editor.on('selectionUpdate', updateTableState);
    editor.on('update', updateTableState);

    return () => {
      editor.off('selectionUpdate', updateTableState);
      editor.off('update', updateTableState);
    };
  }, [editor]);

  // Check scroll position and update button states
  const checkScroll = () => {
    const container = scrollContainerRef.current;
    if (!container) return;

    const { scrollLeft, scrollWidth, clientWidth } = container;
    setCanScrollLeft(scrollLeft > 0);
    setCanScrollRight(scrollLeft < scrollWidth - clientWidth - 1);
  };

  // Scroll the toolbar left or right
  const scroll = (direction: 'left' | 'right') => {
    const container = scrollContainerRef.current;
    if (!container) return;

    const scrollAmount = 200; // Scroll by 200px
    const newScrollLeft =
      direction === 'left'
        ? container.scrollLeft - scrollAmount
        : container.scrollLeft + scrollAmount;

    container.scrollTo({
      left: newScrollLeft,
      behavior: 'smooth',
    });
  };

  // Check scroll on mount and when toolbar content changes
  useEffect(() => {
    checkScroll();
    const container = scrollContainerRef.current;
    if (!container) return;

    const resizeObserver = new ResizeObserver(checkScroll);
    resizeObserver.observe(container);

    container.addEventListener('scroll', checkScroll);

    return () => {
      resizeObserver.disconnect();
      container.removeEventListener('scroll', checkScroll);
    };
  }, []);

  const handleTextColor = (color: string) => {
    editor.chain().focus().setColor(color).run();
  };

  const handleHighlight = (color: string) => {
    editor.chain().focus().setHighlight({ color }).run();
  };

  const handleImageUpload = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    // Check file type
    if (!file.type.startsWith('image/')) {
      alert('Please select an image file');
      return;
    }

    // Check file size (max 5MB)
    if (file.size > 5 * 1024 * 1024) {
      alert('Image size must be less than 5MB');
      return;
    }

    // Convert to Base64
    const reader = new FileReader();
    reader.onload = (e) => {
      const base64 = e.target?.result as string;
      // Store filename in alt attribute for later use
      editor
        .chain()
        .focus()
        .setImage({ src: base64, alt: file.name, title: file.name })
        .run();
    };
    reader.readAsDataURL(file);

    // Reset input
    event.target.value = '';
  };

  return (
    <div className={cn('relative border-b border-border bg-card', className)}>
      {/* Left scroll button */}
      {canScrollLeft && (
        <button
          onClick={() => scroll('left')}
          tabIndex={-1}
          className={cn(
            'absolute left-0 top-0 bottom-0 z-10',
            'flex items-center justify-center',
            'w-8 bg-gradient-to-r from-card to-transparent',
            'hover:from-card/95',
            'transition-colors',
          )}
          aria-label="Scroll left"
        >
          <ChevronLeft className="w-5 h-5 text-foreground [&>path]:stroke-primary" />
        </button>
      )}

      {/* Scrollable toolbar content */}
      <div
        ref={scrollContainerRef}
        className={cn(
          'overflow-x-auto overflow-y-hidden',
          'scrollbar-hide', // Hide scrollbar
          '[&::-webkit-scrollbar]:hidden', // Hide scrollbar for webkit browsers
          '[-ms-overflow-style:none]', // Hide scrollbar for IE/Edge
          '[scrollbar-width:none]', // Hide scrollbar for Firefox
        )}
      >
        <div className={cn('flex items-center gap-2', 'min-w-max', 'p-1')}>
          {/* Table buttons - shown first when inside a table */}
          {isInTable && features.table && (
            <>
              <ToolbarButton
                icon={<TableAddRowBefore />}
                onClick={() => editor.chain().focus().addRowBefore().run()}
                disabled={!editor.can().addRowBefore()}
                tooltip="Add Row Before"
                aria-label="Add Row Before"
              />
              <ToolbarButton
                icon={<TableAddRowAfter />}
                onClick={() => editor.chain().focus().addRowAfter().run()}
                disabled={!editor.can().addRowAfter()}
                tooltip="Add Row After"
                aria-label="Add Row After"
              />
              <ToolbarButton
                icon={<TableDeleteRow />}
                onClick={() => editor.chain().focus().deleteRow().run()}
                disabled={!editor.can().deleteRow()}
                tooltip="Delete Row"
                aria-label="Delete Row"
              />
              <ToolbarButton
                icon={<TableAddColumnBefore />}
                onClick={() => editor.chain().focus().addColumnBefore().run()}
                disabled={!editor.can().addColumnBefore()}
                tooltip="Add Column Before"
                aria-label="Add Column Before"
              />
              <ToolbarButton
                icon={<TableAddColumnAfter />}
                onClick={() => editor.chain().focus().addColumnAfter().run()}
                disabled={!editor.can().addColumnAfter()}
                tooltip="Add Column After"
                aria-label="Add Column After"
              />
              <ToolbarButton
                icon={<TableDeleteColumn />}
                onClick={() => editor.chain().focus().deleteColumn().run()}
                disabled={!editor.can().deleteColumn()}
                tooltip="Delete Column"
                aria-label="Delete Column"
              />
              <ToolbarButton
                icon={<TableMergeCells />}
                onClick={() => editor.chain().focus().mergeCells().run()}
                disabled={!editor.can().mergeCells()}
                tooltip="Merge Cells"
                aria-label="Merge Cells"
              />
              <ToolbarButton
                icon={<TableSplitCell />}
                onClick={() => editor.chain().focus().splitCell().run()}
                disabled={!editor.can().splitCell()}
                tooltip="Split Cell"
                aria-label="Split Cell"
              />
              <ToolbarButton
                icon={<TableDeleteTable />}
                onClick={() => editor.chain().focus().deleteTable().run()}
                disabled={!editor.can().deleteTable()}
                tooltip="Delete Table"
                aria-label="Delete Table"
              />
              {/* Separator after table buttons */}
              <div className="mx-1 w-px h-6 bg-border" />
            </>
          )}

          {/* Regular toolbar buttons */}
          {features.bold && (
            <ToolbarButton
              icon={<EditorBold />}
              onClick={() => editor.chain().focus().toggleBold().run()}
              active={editor.isActive('bold')}
              tooltip="Bold (⌘B)"
              aria-label="Bold"
            />
          )}
          {features.italic && (
            <ToolbarButton
              icon={<EditorItalics />}
              onClick={() => editor.chain().focus().toggleItalic().run()}
              active={editor.isActive('italic')}
              tooltip="Italic (⌘I)"
              aria-label="Italic"
            />
          )}
          {features.underline && (
            <ToolbarButton
              icon={<EditorUnderline />}
              onClick={() => editor.chain().focus().toggleUnderline().run()}
              active={editor.isActive('underline')}
              tooltip="Underline (⌘U)"
              aria-label="Underline"
            />
          )}
          {features.strike && (
            <ToolbarButton
              icon={<EditorStrike />}
              onClick={() => editor.chain().focus().toggleStrike().run()}
              active={editor.isActive('strike')}
              tooltip="Strikethrough"
              aria-label="Strikethrough"
            />
          )}
          {features.textColor && (
            <ColorPicker
              type="text"
              currentColor={editor.getAttributes('textStyle').color}
              onColorChange={handleTextColor}
            />
          )}
          {features.highlight && (
            <ColorPicker
              type="background"
              currentColor={editor.getAttributes('highlight').color}
              onColorChange={handleHighlight}
            />
          )}
          {features.heading && <HeadingDropdown editor={editor} />}
          {features.align && (
            <ToolbarButton
              icon={<AlignmentsAlignLeft className="[svg" />}
              onClick={() => {
                if (editor.isActive({ textAlign: 'left' })) {
                  editor.chain().focus().unsetTextAlign().run();
                } else {
                  editor.chain().focus().setTextAlign('left').run();
                }
              }}
              active={editor.isActive({ textAlign: 'left' })}
              tooltip="Align Left"
              aria-label="Align Left"
            />
          )}
          {features.align && (
            <ToolbarButton
              icon={<AlignmentsAlignCenter />}
              onClick={() => {
                if (editor.isActive({ textAlign: 'center' })) {
                  editor.chain().focus().unsetTextAlign().run();
                } else {
                  editor.chain().focus().setTextAlign('center').run();
                }
              }}
              active={editor.isActive({ textAlign: 'center' })}
              tooltip="Align Center"
              aria-label="Align Center"
            />
          )}
          {features.align && (
            <ToolbarButton
              icon={<AlignmentsAlignRight />}
              onClick={() => {
                if (editor.isActive({ textAlign: 'right' })) {
                  editor.chain().focus().unsetTextAlign().run();
                } else {
                  editor.chain().focus().setTextAlign('right').run();
                }
              }}
              active={editor.isActive({ textAlign: 'right' })}
              tooltip="Align Right"
              aria-label="Align Right"
            />
          )}
          {features.lists && (
            <>
              <ToolbarButton
                icon={<Ordered1 />}
                onClick={() => editor.chain().focus().toggleOrderedList().run()}
                active={editor.isActive('orderedList')}
                tooltip="Numbered List"
                aria-label="Numbered List"
              />
              <ToolbarButton
                icon={<Bullet1 />}
                onClick={() => editor.chain().focus().toggleBulletList().run()}
                active={editor.isActive('bulletList')}
                tooltip="Bullet List"
                aria-label="Bullet List"
              />
            </>
          )}
          {features.image && (
            <>
              <ToolbarButton
                icon={<EditorUploadImage />}
                onClick={handleImageUpload}
                active={false}
                tooltip="Upload Image"
                aria-label="Upload Image"
              />
              <input
                ref={fileInputRef}
                type="file"
                accept="image/*"
                onChange={handleFileChange}
                className="hidden"
              />
            </>
          )}

          {/* Insert Table button - shown only when NOT in a table */}
          {!isInTable && features.table && (
            <ToolbarButton
              icon={<TableInsertTable />}
              onClick={() =>
                editor
                  .chain()
                  .focus()
                  .insertTable({ rows: 3, cols: 3, withHeaderRow: true })
                  .run()
              }
              active={false}
              tooltip="Insert Table"
              aria-label="Insert Table"
            />
          )}

          <button>
            <Video
              type="button"
              onClick={openVideoPicker}
              className="w-5 h-5 [&>path]:stroke-neutral-500 [&>rect]:stroke-neutral-500 ml-2"
            />
          </button>
        </div>
      </div>

      {/* Right scroll button */}
      {canScrollRight && (
        <button
          onClick={() => scroll('right')}
          tabIndex={-1}
          className={cn(
            'absolute right-0 top-0 bottom-0 z-10',
            'flex items-center justify-center',
            'w-8 bg-gradient-to-l from-card to-transparent',
            'hover:from-card/95',
            'transition-colors',
          )}
          aria-label="Scroll right"
        >
          <ChevronRight2 className="w-5 h-5 text-foreground [&>path]:stroke-primary" />
        </button>
      )}
    </div>
  );
};
