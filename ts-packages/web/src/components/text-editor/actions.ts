import { useCallback } from 'react';
import type { Editor } from '@tiptap/react';

export type UseEditorActionsProps = {
  editor: Editor | null;
  /* eslint-disable-next-line @typescript-eslint/no-explicit-any*/
  setUploadedImages: React.Dispatch<React.SetStateAction<any[]>>;
  linkUrl: string;
  setLinkUrl: (url: string) => void;
  setShowLinkPopover: (show: boolean) => void;
  setShowColorPicker: (show: boolean) => void;
  fileInputRef: React.RefObject<HTMLInputElement | null>;
};

export const useEditorActions = ({
  editor,
  setUploadedImages,
  linkUrl,
  setLinkUrl,
  setShowLinkPopover,
  setShowColorPicker,
  fileInputRef,
}: UseEditorActionsProps) => {
  const handleLinkClick = useCallback(() => {
    if (!editor) return;
    const previousUrl = editor.getAttributes('link').href;
    setLinkUrl(previousUrl || '');
    setShowLinkPopover(true);
  }, [editor, setLinkUrl, setShowLinkPopover]);

  const addLink = useCallback(() => {
    if (!editor || !linkUrl.trim()) return;

    // const processedUrl = linkUrl.includes('://')
    //   ? linkUrl
    //   : `https://${linkUrl}`;

    const ensureProtocol = (u: string) =>
      /^(https?:)?\/\//i.test(u) ? u : `https://${u}`;
    const candidate = ensureProtocol(linkUrl.trim());
    let processedUrl: string;
    try {
      const u = new URL(candidate);
      if (!['http:', 'https:'].includes(u.protocol)) return;
      processedUrl = u.toString();
    } catch {
      return;
    }

    if (editor.state.selection.empty === false) {
      editor
        .chain()
        .focus()
        .extendMarkRange('link')
        .setLink({ href: processedUrl })
        .run();
    } else {
      editor
        .chain()
        .focus()
        .insertContent({
          type: 'text',
          text: linkUrl,
          marks: [{ type: 'link', attrs: { href: processedUrl } }],
        })
        .run();
    }

    setShowLinkPopover(false);
    setLinkUrl('');
  }, [editor, linkUrl, setLinkUrl, setShowLinkPopover]);

  const removeLink = useCallback(() => {
    if (!editor) return;
    editor.chain().focus().unsetLink().run();
    setShowLinkPopover(false);
    setLinkUrl('');
  }, [editor, setLinkUrl, setShowLinkPopover]);

  const setColor = useCallback(
    (color: string) => {
      if (!editor) return;
      editor.chain().focus().setColor(color).run();
      // Close the color picker after applying color
      setShowColorPicker(false);
    },
    [editor, setShowColorPicker],
  );

  const addImage = useCallback(() => {
    fileInputRef.current?.click();
  }, [fileInputRef]);

  const handleImageUpload = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      if (!editor) return;
      const file = e.target.files?.[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onloadend = () => {
        const src = reader.result as string;
        const id = Date.now().toString();
        setUploadedImages((prev) => [...prev, { id, src, name: file.name }]);
        editor.chain().focus().setImage({ src }).run();
      };
      reader.readAsDataURL(file);
    },
    [editor, setUploadedImages],
  );

  const removeImage = useCallback(
    (id: string) => {
      setUploadedImages((prev) => prev.filter((img) => img.id !== id));
    },
    [setUploadedImages],
  );

  const insertImageFromPreview = useCallback(
    (src: string) => {
      if (!editor) return;
      editor.chain().focus().setImage({ src }).run();
    },
    [editor],
  );

  return {
    handleLinkClick,
    addLink,
    removeLink,
    setColor,
    addImage,
    handleImageUpload,
    removeImage,
    insertImageFromPreview,
  };
};
