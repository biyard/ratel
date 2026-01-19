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
import { Link2, Link2Off, Video, FileText } from 'lucide-react';

export const TiptapToolbar = ({
  editor,
  enabledFeatures = DEFAULT_ENABLED_FEATURES,
  className,
  variant = 'default',
  mode = 'default',
  dropdownPortalContainer,
  onHeadingDropdownOpenChange,
  onHeadingDropdownTriggerPointerDown,
  headingDropdownContentProps,
  onColorPickerOpenChange,
  onColorPickerTriggerPointerDown,
  openVideoPicker,
  onImageUpload,
  onUploadPDF,
}: TiptapToolbarProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const pdfInputRef = useRef<HTMLInputElement>(null);
  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);
  const [isInTable, setIsInTable] = useState(false);

  const features = { ...DEFAULT_ENABLED_FEATURES, ...enabledFeatures };
  const showLists = mode === 'bubble' ? true : !!features.lists;

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

  const isYouTubeUrl = (s: string) =>
    /^(https?:\/\/)?(www\.)?(youtube\.com\/watch\?v=|youtu\.be\/)/i.test(s);

  const normalizeUrl = (raw: string) => {
    const s = raw.trim();
    if (!s) return '';
    if (/^https?:\/\//i.test(s)) return s;
    if (/^[\w.+-]+@[\w.-]+\.[a-z]{2,}$/i.test(s)) return `mailto:${s}`;
    if (/^\+?\d[\d\s()-]{5,}$/.test(s)) return `tel:${s}`;
    return `https://${s}`;
  };

  const removeLink = () => {
    editor.chain().focus().extendMarkRange('link').unsetMark('link').run();
  };

  const promptAndApplyLink = () => {
    const { from, to, empty } = editor.state.selection;

    const current = editor.getAttributes('link')?.href ?? '';
    const input = window.prompt('Input Link URL', current || 'https://');
    if (input === null) return;

    const href = normalizeUrl(input);

    const chain = editor
      .chain()
      .focus()
      .setTextSelection({ from, to })
      .extendMarkRange('link');

    if (!href) {
      chain.unsetMark('link').run();
      return;
    }

    if (isYouTubeUrl(href)) {
      if (empty) {
        chain
          .insertContent([
            {
              type: 'text',
              text: href,
              marks: [
                {
                  type: 'link',
                  attrs: {
                    href,
                    target: '_blank',
                    rel: 'noopener noreferrer nofollow',
                  },
                },
              ],
            },
            { type: 'hardBreak' },
          ])
          .setTextSelection(editor.state.selection.to + href.length + 1)
          .setYoutubeVideo({ src: href })
          .run();
      } else {
        chain
          .setMark('link', {
            href,
            target: '_blank',
            rel: 'noopener noreferrer nofollow',
          })
          .setTextSelection(to)
          .insertContent([{ type: 'hardBreak' }])
          .setYoutubeVideo({ src: href })
          .run();
      }
      return;
    }

    if (empty) {
      chain
        .insertContent([
          {
            type: 'text',
            text: href,
            marks: [
              {
                type: 'link',
                attrs: {
                  href,
                  target: '_blank',
                  rel: 'noopener noreferrer nofollow',
                },
              },
            ],
          },
        ])
        .run();
    } else {
      chain
        .setMark('link', {
          href,
          target: '_blank',
          rel: 'noopener noreferrer nofollow',
        })
        .run();
    }
  };

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
    // Try to use theme-aware color if available, fallback to standard color
    if (editor.commands.setThemeAwareColor) {
      editor.chain().focus().setThemeAwareColor(color).run();
    } else {
      editor.chain().focus().setColor(color).run();
    }
    const { to } = editor.state.selection;
    editor.commands.setTextSelection(to);
  };

  const handleHighlight = (color: string) => {
    // Try to use theme-aware highlight if available, fallback to standard highlight
    if (editor.commands.setThemeAwareHighlight) {
      editor.chain().focus().setThemeAwareHighlight({ color }).run();
    } else {
      editor.chain().focus().setHighlight({ color }).run();
    }
    const { to } = editor.state.selection;
    editor.commands.setTextSelection(to);
  };

  const handleImageUpload = () => {
    fileInputRef.current?.click();
  };

  const handlePdfUpload = () => pdfInputRef.current?.click();
  const handlePdfChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) return;
    onUploadPDF?.(files);
    e.currentTarget.value = '';
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

    const reader = new FileReader();
    reader.onload = async (e) => {
      const base64 = e.target?.result as string;
      if (onImageUpload) {
        await onImageUpload(base64);
      } else {
        editor
          .chain()
          .focus()
          .setImage({ src: base64, alt: file.name, title: file.name })
          .run();
      }
      // Store filename in alt attribute for later use
    };
    reader.readAsDataURL(file);

    // Reset input
    event.target.value = '';
  };

  if (!editor) return null;

  const variantClasses = {
    default: 'bg-card',
    post: 'bg-[var(--color-post-input-bg)]',
  };

  const scrollGradientClasses = {
    default: 'from-card to-transparent hover:from-card/95',
    post: 'from-[var(--color-post-input-bg)] to-transparent hover:from-[var(--color-post-input-bg)]/95',
  };

  const borderClasses = {
    default: 'border-b border-border',
    post: 'after:content-[""] after:absolute after:bottom-0 after:left-1/2 after:-translate-x-1/2 after:w-[95%] after:h-[1px] after:bg-[var(--color-post-input-border)]/30',
  };

  return (
    <div
      className={cn(
        'relative',
        borderClasses[variant],
        variantClasses[variant],
        className,
      )}
    >
      {/* Left scroll button */}
      {canScrollLeft && (
        <button
          onClick={() => scroll('left')}
          tabIndex={-1}
          className={cn(
            'absolute left-0 top-0 bottom-0 z-10',
            'flex items-center justify-center',
            'w-8 bg-gradient-to-r',
            scrollGradientClasses[variant],
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
              portalled={!!dropdownPortalContainer || mode !== 'bubble'}
              container={dropdownPortalContainer}
              contentProps={headingDropdownContentProps}
              onOpenChange={onColorPickerOpenChange}
              onTriggerPointerDown={onColorPickerTriggerPointerDown}
            />
          )}
          {features.highlight && (
            <ColorPicker
              type="background"
              currentColor={editor.getAttributes('highlight').color}
              onColorChange={handleHighlight}
              portalled={!!dropdownPortalContainer || mode !== 'bubble'}
              container={dropdownPortalContainer}
              contentProps={headingDropdownContentProps}
              onOpenChange={onColorPickerOpenChange}
              onTriggerPointerDown={onColorPickerTriggerPointerDown}
            />
          )}
          {features.heading && (
            <HeadingDropdown
              editor={editor}
              portalled={!!dropdownPortalContainer || mode !== 'bubble'}
              container={dropdownPortalContainer}
              onOpenChange={onHeadingDropdownOpenChange}
              onTriggerPointerDown={onHeadingDropdownTriggerPointerDown}
              contentProps={headingDropdownContentProps}
            />
          )}
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
          {showLists && (
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

          {features.pdf && (
            <>
              <ToolbarButton
                icon={<FileText />}
                onClick={handlePdfUpload}
                active={false}
                disabled={!onUploadPDF}
                tooltip="Upload PDF"
                aria-label="Upload PDF"
              />
              <input
                ref={pdfInputRef}
                type="file"
                accept="application/pdf"
                multiple
                onChange={handlePdfChange}
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

          {mode !== 'bubble' && (
            <>
              <ToolbarButton
                icon={<Link2 />}
                onClick={promptAndApplyLink}
                active={editor.isActive('link')}
                tooltip="Link"
                aria-label="Link"
                data-testid="tiptap-toolbar-link"
              />

              <ToolbarButton
                icon={<Link2Off />}
                onClick={removeLink}
                disabled={!editor.isActive('link')}
                tooltip="Remove Link"
                aria-label="Remove Link"
              />

              <ToolbarButton
                icon={<Video />}
                onClick={openVideoPicker}
                active={false}
                tooltip="Upload Video"
                aria-label="Upload Video"
              />

            </>
          )}
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
            'w-8 bg-gradient-to-l',
            scrollGradientClasses[variant],
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
