import { useParams } from 'react-router';
import '@/features/spaces/deliberations/deliberation-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePanelEditorPage } from '@/features/spaces/panels/pages/creator/spce-panel-editor-page';
import { SpacePanelViewerPage } from '@/features/spaces/panels/pages/viewer/space-panel-viewer-page';

export default function SpacePanelPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpacePanelEditorPage spacePk={spacePk} />;
  }

  return <SpacePanelViewerPage spacePk={spacePk} />;
}
