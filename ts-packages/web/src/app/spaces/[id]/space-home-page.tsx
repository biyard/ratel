import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceHomeController } from './use-space-home-controller';
import { useParams } from 'react-router';

export function SpaceHomePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk);
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
