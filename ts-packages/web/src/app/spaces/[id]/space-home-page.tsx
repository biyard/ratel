import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceLayoutContext } from './use-space-layout-context';

export function SpaceHomePage() {
  const ctrl = useSpaceLayoutContext();

  return (
    <div className="flex flex-col w-full gap-2.5">
      <SpaceHTMLContentEditor
        htmlContent={ctrl.space.content}
        canEdit={ctrl.isAdmin}
        onContentChange={ctrl.handleChange}
        url={ctrl.image.get()}
        files={ctrl.files.get()}
        disabledFileUpload={false}
        onRemovePdf={ctrl.handleRemovePdf}
        onUploadPDF={ctrl.handlePdfUpload}
        onImageUpload={ctrl.handleImageUpload}
        onRemoveImage={ctrl.handleRemoveImage}
      />
    </div>
  );
}
