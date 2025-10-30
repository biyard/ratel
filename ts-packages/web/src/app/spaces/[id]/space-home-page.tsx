import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceLayoutContext } from './use-space-layout-context';

export function SpaceHomePage() {
  const ctrl = useSpaceLayoutContext();

  return (
    <>
      <SpaceHTMLContentEditor
        htmlContent={ctrl.space.content}
        canEdit={ctrl.isAdmin}
        onContentChange={ctrl.handleChange}
        url={ctrl.image.get()}
        onImageUpload={ctrl.handleImageUpload}
        onRemoveImage={ctrl.handleRemoveImage}
      />
    </>
  );
}
