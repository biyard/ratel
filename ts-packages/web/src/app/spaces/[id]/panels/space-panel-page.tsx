import { useParams } from 'react-router';
import { SpacePanelEditorPage } from '@/features/spaces/panels/pages/creator/spce-panel-editor-page';
import { useRedirectUser } from '@/features/spaces/hooks/use-redirect-user';

export default function SpacePanelPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  useRedirectUser();

  return <SpacePanelEditorPage spacePk={spacePk} />;
}
