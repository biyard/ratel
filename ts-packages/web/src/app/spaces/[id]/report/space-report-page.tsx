import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceReportEditorPage } from '@/features/spaces/reports/pages/creator/space-report-editor-page';

export function SpaceReportPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return <SpaceReportEditorPage spacePk={spacePk} />;
}
