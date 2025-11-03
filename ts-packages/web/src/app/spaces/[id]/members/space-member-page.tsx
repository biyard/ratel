import { useParams } from 'react-router';
import '@/features/spaces/deliberations/deliberation-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
// import { config } from '@/config';
import { SpaceInvitationEditorPage } from '@/features/spaces/invitations/pages/creator/space-invitation-editor-page';
import { SpaceInvitationViewerPage } from '@/features/spaces/invitations/pages/viewer/space-invitation-viewer-page';

export default function SpaceMemberPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return (
    <>
      {space.isAdmin() ? (
        <SpaceInvitationEditorPage spacePk={space.pk} />
      ) : (
        <SpaceInvitationViewerPage spacePk={space.pk} />
      )}
    </>
  );
}
