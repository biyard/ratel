import { useParams } from 'react-router';
import '@/features/spaces/deliberations/deliberation-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePanelEditorPage } from '@/features/spaces/panels/pages/creator/spce-panel-editor-page';

export default function SpacePanelPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return <SpacePanelEditorPage spacePk={spacePk} />;
}
