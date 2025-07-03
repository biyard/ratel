// components/editor/useEditorActions.ts
import { useCallback } from 'react';

type UseEditorActionsProps = {
  /* eslint-disable-next-line @typescript-eslint/no-explicit-any*/
  editor: any;
  /* eslint-disable-next-line @typescript-eslint/no-explicit-any*/
  setUploadedImages: React.Dispatch<React.SetStateAction<any[]>>;
  setLinkUrl: (url: string) => void;
  setShowLinkPopover: (show: boolean) => void;
  fileInputRef: React.RefObject<HTMLInputElement | null>;
};
export const useEditorActions = ({
  editor,
  setUploadedImages,
  setLinkUrl,
  setShowLinkPopover,
  fileInputRef,
}: UseEditorActionsProps) => {
  const handleLinkClick = useCallback(() => {
    const previousUrl = editor.getAttributes('link').href;
    setLinkUrl(previousUrl || '');
    setShowLinkPopover(true);
  }, [editor]);

  const addLink = useCallback(() => {
    const url = editor.getAttributes('link').href;
    if (url) {
      editor
        .chain()
        .focus()
        .extendMarkRange('link')
        .setLink({ href: url })
        .run();
    } else {
      editor.chain().focus().unsetLink().run();
    }
    setShowLinkPopover(false);
    setLinkUrl('');
  }, [editor]);

  const removeLink = useCallback(() => {
    editor.chain().focus().unsetLink().run();
    setShowLinkPopover(false);
    setLinkUrl('');
  }, [editor]);

  const setColor = useCallback(
    (color: string) => {
      editor.chain().focus().setColor(color).run();
      setShowLinkPopover(false);
    },
    [editor],
  );

  const addImage = () => {
    fileInputRef.current?.click();
  };

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
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
  };

  const removeImage = (id: string) => {
    setUploadedImages((prev) => prev.filter((img) => img.id !== id));
  };

  const insertImageFromPreview = (src: string) => {
    editor.chain().focus().setImage({ src }).run();
  };

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
