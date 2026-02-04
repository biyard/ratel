import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceBoardsCreatePage } from '@/features/spaces/boards/pages/creator/create/space-boards-create-page';

export default function SpaceBoardCreatePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceBoardsCreatePage spacePk={spacePk} />;
  }

  return <></>;
}
