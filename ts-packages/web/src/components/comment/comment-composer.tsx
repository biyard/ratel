import { useRef, useState, useEffect, useCallback } from 'react';
import { EditorContent } from '@tiptap/react';
import { useEditorActions } from '@/components/text-editor/actions';
import { useTiptapEditor } from '@/components/text-editor/useTiptapEditor';
import { EditorToolbarComment as EditorToolbar } from '@/components/toolbar/editor-toolbar-comment';
import { CommentIcon } from '../icons';
import { Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { ChevronDoubleDownIcon } from '@heroicons/react/20/solid';

interface RichTextEditorProps {
  /*eslint-disable-next-line @typescript-eslint/no-explicit-any*/
  onSubmit: (content: string) => Promise<any>;
  onClose?: () => void;
  validateString?: (content: string) => boolean;
  className?: string;
}

export default function CommentComposer({
  onSubmit,
  onClose,
  className,
  validateString = (c) => c.trim().length > 0,
}: RichTextEditorProps) {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [linkUrl, setLinkUrl] = useState('');
  const [showLinkPopover, setShowLinkPopover] = useState(false);
  const [showColorPicker, setShowColorPicker] = useState(false);
  /*eslint-disable-next-line @typescript-eslint/no-unused-vars, unused-imports/no-unused-vars*/
  const [uploadedImages, setUploadedImages] = useState<
    { id: string; src: string; name: string }[]
  >([]);
  const [isLoading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(true);

  const handleImageFile = useCallback((file: File) => {
    const reader = new FileReader();
    reader.onloadend = () => {
      const src = reader.result as string;
      const id = Date.now().toString();
      setUploadedImages((prev) => [...prev, { id, src, name: file.name }]);
    };
    reader.readAsDataURL(file);
  }, []);

  const editor = useTiptapEditor({
    handleImageFile,
    features: {
      underline: true,
      color: true,
      link: true,
      image: true,
    },
  });

  const {
    handleLinkClick,
    addLink,
    removeLink,
    setColor,
    addImage,
    handleImageUpload,
    // removeImage,
    // insertImageFromPreview,
  } = useEditorActions({
    editor,
    setUploadedImages,
    linkUrl,
    setLinkUrl,
    setShowLinkPopover,
    fileInputRef,
  });

  useEffect(() => {
    if (!editor) return;
    const interval = setInterval(() => {
      const content = editor.getHTML();
      setDisabled(!validateString(content));
    }, 200);
    return () => clearInterval(interval);
  }, [editor, validateString]);

  const handleSubmit = async () => {
    if (!editor) return;
    const content = editor.getHTML();

    if (!isLoading && content.trim() !== '' && validateString(content)) {
      setLoading(true);
      try {
        await onSubmit(content);
        editor.commands.clearContent();
        setLinkUrl('');
        setShowLinkPopover(false);
        setUploadedImages([]);
        onClose?.();
      } catch (error) {
        console.error('Submit error:', error);
      } finally {
        setLoading(false);
      }
    }
  };

  if (!editor) return null;

  return (
    <div className={cn('w-full max-w-desktop mx-auto space-y-2', className)}>
      <div className="relative bg-neutral-900 border-2 border-primary rounded-lg overflow-hidden">
        <button
          className="absolute top-2 right-2 p-1 flex flex-row z-20"
          onClick={onClose}
          type="button"
        >
          <ChevronDoubleDownIcon width={24} height={24} />
        </button>

        <div className="min-h-[140px] pt-4">
          <EditorContent editor={editor} />
          {editor.isEmpty && (
            <div className="absolute top-8 left-4 text-neutral-600 pointer-events-none">
              Type here. Use Markdown, BB code, or HTML to format. Drag or paste
              images.
            </div>
          )}
        </div>

        <div className="flex items-center justify-between bg-neutral-900 p-2">
          <EditorToolbar
            editor={editor}
            linkUrl={linkUrl}
            setLinkUrl={setLinkUrl}
            showLinkPopover={showLinkPopover}
            setShowLinkPopover={setShowLinkPopover}
            addLink={addLink}
            removeLink={removeLink}
            handleLinkClick={handleLinkClick}
            setColor={setColor}
            showColorPicker={showColorPicker}
            setShowColorPicker={setShowColorPicker}
            addImage={addImage}
          />

          <button
            onClick={handleSubmit}
            disabled={disabled || isLoading}
            className={cn(
              'flex items-center gap-2 p-2 rounded-full font-medium text-sm transition-all',
              !disabled && !isLoading
                ? 'bg-primary text-black hover:bg-primary/50'
                : 'bg-neutral-700 text-neutral-500 cursor-not-allowed',
            )}
          >
            {isLoading ? (
              <Loader2 className="animate-spin size-6" />
            ) : (
              <CommentIcon
                width={24}
                height={24}
                className="[&>path]:stroke-white [&>line]:stroke-white"
              />
            )}
          </button>
        </div>

        <input
          ref={fileInputRef}
          type="file"
          accept="image/*"
          onChange={handleImageUpload}
          className="hidden"
        />
      </div>
    </div>
  );
}
