// EditorToolbar.tsx
import { Button } from '@/components/ui/button';
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from '@/components/ui/popover';

import { Input } from '@/components/ui/input';
import {
  Bold,
  Italic,
  UnderlineIcon,
  Strikethrough,
  Code,
  LinkIcon,
  ImageIcon,
  Paperclip,
  Smile,
  X,
} from 'lucide-react';

import { PainIcon } from '@/components/icons';

export const EditorToolbar = ({
  editor,
  linkUrl,
  setLinkUrl,
  showLinkPopover,
  setShowLinkPopover,
  addLink,
  removeLink,
  handleLinkClick,
  setColor,
  showColorPicker,
  setShowColorPicker,
  addImage,
}: {
  editor: any;
  linkUrl: string;
  setLinkUrl: (url: string) => void;
  showLinkPopover: boolean;
  setShowLinkPopover: (v: boolean) => void;
  addLink: () => void;
  removeLink: () => void;
  handleLinkClick: () => void;
  setColor: (color: string) => void;
  showColorPicker: boolean;
  setShowColorPicker: (v: boolean) => void;
  addImage: () => void;
}) => {
  const colors = [
    '#000',
    '#f00',
    '#0f0',
    '#00f',
    '#ff0',
    '#f0f',
    '#0ff',
    '#fff',
  ];

  return (
    <div className="bg-neutral-900">
      <div className="flex items-center gap-4">
        {/* Code */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={() => editor.chain().focus().toggleCode().run()}
        >
          <Code className="h-4 w-4" />
        </button>

        {/* Bold */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={() => editor.chain().focus().toggleBold().run()}
        >
          <Bold className="h-4 w-4" />
        </button>

        {/* Italic */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={() => editor.chain().focus().toggleItalic().run()}
        >
          <Italic className="h-4 w-4" />
        </button>

        {/* Underline */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={() => editor.chain().focus().toggleUnderline().run()}
        >
          <UnderlineIcon className="h-4 w-4" />
        </button>

        {/* Strikethrough */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={() => editor.chain().focus().toggleStrike().run()}
        >
          <Strikethrough className="h-4 w-4" />
        </button>

        {/* Color Picker */}
        <Popover open={showColorPicker} onOpenChange={setShowColorPicker}>
          <PopoverTrigger asChild>
            <button className="bg-neutral-900 text-lg text-neutral-700">
              <PainIcon className="h-4 w-4" />
            </button>
          </PopoverTrigger>
          <PopoverContent className="w-48 p-2 bg-gray-800 border-gray-600">
            <div className="grid grid-cols-7 gap-1">
              {colors.map((color) => (
                <button
                  key={color}
                  onClick={() => setColor(color)}
                  className="w-6 h-6 rounded border"
                  style={{ backgroundColor: color }}
                />
              ))}
            </div>
          </PopoverContent>
        </Popover>

        {/* Link */}
        <Popover open={showLinkPopover} onOpenChange={setShowLinkPopover}>
          <PopoverTrigger asChild>
            <button
              className="bg-neutral-900 text-lg text-neutral-700"
              onClick={handleLinkClick}
            >
              <LinkIcon className="h-4 w-4" />
            </button>
          </PopoverTrigger>
          <PopoverContent className="w-80 p-3 bg-gray-800 border-gray-600">
            <div className="space-y-2">
              <div className="flex gap-2">
                <Input
                  placeholder="Enter URL"
                  value={linkUrl}
                  onChange={(e) => setLinkUrl(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') addLink();
                    if (e.key === 'Escape') setShowLinkPopover(false);
                  }}
                />
                <Button onClick={addLink} disabled={!linkUrl.trim()}>
                  {editor.isActive('link') ? 'Update' : 'Add'}
                </Button>
              </div>
              {editor.isActive('link') && (
                <Button
                  onClick={removeLink}
                  variant="outline"
                  className="text-red-400"
                >
                  Remove Link
                </Button>
              )}
            </div>
          </PopoverContent>
        </Popover>

        {/* Image */}
        <button
          className="bg-neutral-900 text-lg text-neutral-700"
          onClick={addImage}
        >
          <ImageIcon className="h-4 w-4" />
        </button>

        <button className="bg-neutral-900 text-lg text-neutral-700">
          <Paperclip className="h-4 w-4" />
        </button>

        <button className="bg-neutral-900 text-lg text-neutral-700">
          <Smile className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
};
