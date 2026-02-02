import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
// import { config } from '@/config';
import { SpaceMembersEditorPage } from '@/features/spaces/members/pages/creator/space-members-editor-page';
import { SpaceMembersViewerPage } from '@/features/spaces/members/pages/viewer/space-members-viewer-page';

export default function SpaceMemberPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return (
    <>
      {space.isAdmin() ? (
        <SpaceMembersEditorPage spacePk={space.pk} />
      ) : (
        <SpaceMembersViewerPage spacePk={space.pk} />
      )}
    </>
  );
}
