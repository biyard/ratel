import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceIncentiveEditorPage } from '@/features/spaces/incentive/pages/creator/space-incentive-editor-page';

export default function SpaceIncentiveSettingPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!spacePk) {
    return null;
  }

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    return <SpaceIncentiveEditorPage spacePk={spacePk} />;
  }

  return null;
}
