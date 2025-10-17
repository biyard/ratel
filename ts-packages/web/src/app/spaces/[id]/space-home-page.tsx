import { useParams } from 'react-router';
import { useSpaceHomeController } from './use-space-home-controller';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';

export function SpaceHomePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpaceHomeController(spacePk);

  return (
    <>
      <SpaceHTMLContentEditor
        htmlContent={ctrl.space.content}
        canEdit={ctrl.isAdmin}
        onContentChange={ctrl.handleChange}
      />
    </>
  );
}
