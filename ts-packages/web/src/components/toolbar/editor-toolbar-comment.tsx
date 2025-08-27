import { Button } from '@/components/ui/button';
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from '@radix-ui/react-popover';
import { Input } from '@/components/ui/input';
import {
  Bold,
  Italic,
  UnderlineIcon,
  Strikethrough,
  Code,
  LinkIcon,
  ImageIcon,
} from 'lucide-react';

import PaintIcon from '@/assets/icons/editor/paint.svg';

interface EditorToolbarProps {
  /* eslint-disable-next-line @typescript-eslint/no-explicit-any*/
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
}

export const EditorToolbarComment = ({
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
}: EditorToolbarProps) => {
  const colors = ['#000', '#f00', '#0f0', '#00f', '#ff0', '#f0f', '#0ff', '#fff'];

  return (
    <div className="bg-neutral-900">
      <div className="flex items-center gap-4">
        {/* Code */}
        <button
          className={`bg-neutral-900 text-lg ${
            editor.isActive('code') ? 'text-white' : 'text-neutral-600'
          }`}
          onClick={() => editor.chain().focus().toggleCode().run()}
        >
          <Code className="h-6 w-6" />
        </button>

        {/* Bold */}
        <button
          className={`bg-neutral-900 text-lg ${
            editor.isActive('bold') ? 'text-white' : 'text-neutral-600'
          }`}
          onClick={() => editor.chain().focus().toggleBold().run()}
        >
          <Bold className="h-6 w-6" />
        </button>

        {/* Italic */}
        <button
          className={`bg-neutral-900 text-lg ${
            editor.isActive('italic') ? 'text-white' : 'text-neutral-600'
          }`}
          onClick={() => editor.chain().focus().toggleItalic().run()}
        >
          <Italic className="h-6 w-6" />
        </button>

        {/* Underline */}
        <button
          className={`bg-neutral-900 text-lg ${
            editor.isActive('underline') ? 'text-white' : 'text-neutral-600'
          }`}
          onClick={() => editor.chain().focus().toggleUnderline().run()}
        >
          <UnderlineIcon className="h-6 w-6" />
        </button>

        {/* Strikethrough */}
        <button
          className={`bg-neutral-900 text-lg ${
            editor.isActive('strike') ? 'text-white' : 'text-neutral-600'
          }`}
          onClick={() => editor.chain().focus().toggleStrike().run()}
        >
          <Strikethrough className="h-6 w-6" />
        </button>

        {/* Color Picker */}
        <Popover open={showColorPicker} onOpenChange={setShowColorPicker}>
          <PopoverTrigger asChild>
            <button className="bg-neutral-900 text-lg text-neutral-600">
              <PaintIcon className="h-6 w-6" />
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
              className={`bg-neutral-900 text-lg ${
                editor.isActive('link') ? 'text-white' : 'text-neutral-600'
              }`}
              onClick={handleLinkClick}
            >
              <LinkIcon className="h-6 w-6" />
            </button>
          </PopoverTrigger>
          <PopoverContent className="w-80 p-3 bg-gray-800 border-gray-600">
            <div className="space-y-2">
              <div className="flex gap-2">
                <Input
                  placeholder="Enter URL (e.g., google.com or https://example.com)"
                  value={linkUrl}
                  onChange={(e) => setLinkUrl(e.target.value)}
                  className="bg-gray-700 border-gray-600 text-white placeholder:text-gray-400"
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      e.preventDefault();
                      addLink();
                    } else if (e.key === 'Escape') {
                      setShowLinkPopover(false);
                    }
                  }}
                  autoFocus
                />
                <Button
                  onClick={addLink}
                  size="sm"
                  className="bg-yellow-500 hover:bg-yellow-600 text-black"
                  disabled={!linkUrl.trim()}
                >
                  {editor.isActive('link') ? 'Update' : 'Add'}
                </Button>
              </div>
              {editor.isActive('link') && (
                <div className="flex gap-2">
                  <Button
                    onClick={removeLink}
                    size="sm"
                    variant="outline"
                    className="text-red-400 border-red-400 hover:bg-red-400 hover:text-white bg-transparent"
                  >
                    Remove Link
                  </Button>
                </div>
              )}
              <p className="text-xs text-gray-400">
                Tip: Select text first, then add a link, or just enter a URL to
                insert it directly.
              </p>
            </div>
          </PopoverContent>
        </Popover>

        {/* Image */}
        <button className="bg-neutral-900 text-lg text-neutral-600" onClick={addImage}>
          <ImageIcon className="h-6 w-6" />
        </button>
      </div>
    </div>
  );
};
