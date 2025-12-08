import { useParams } from 'react-router';
import '@/features/spaces/deliberations/deliberation-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { FilesPage } from '@/features/spaces/files';

export default function SpaceFilePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return <FilesPage spacePk={spacePk} isAdmin={space.isAdmin()} />;
}
