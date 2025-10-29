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
} from '../icons';
import { useRef } from 'react';

export const TiptapToolbar = ({
  editor,
  enabledFeatures = DEFAULT_ENABLED_FEATURES,
  className,
}: TiptapToolbarProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  if (!editor) return null;

  const features = { ...DEFAULT_ENABLED_FEATURES, ...enabledFeatures };

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
    <div
      className={cn(
        'border-b border-border bg-card',
        'overflow-x-auto overflow-y-hidden',
        'scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent',
        className,
      )}
    >
      <div className={cn('flex items-center gap-2', 'min-w-max', 'p-1')}>
        {/* All buttons at the same level without grouping or separators */}
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
      </div>
    </div>
  );
};
