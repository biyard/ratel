import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceLayoutContext } from './use-space-layout-context';
import { SpaceInvitationEditorPage } from '@/features/spaces/invitations/pages/creator/space-invitation-editor-page';
import { SpaceInvitationViewerPage } from '@/features/spaces/invitations/pages/viewer/space-invitation-viewer-page';

export function SpaceHomePage() {
  const ctrl = useSpaceLayoutContext();

  return (
    <>
      <div className="flex flex-col w-full gap-10">
        <SpaceHTMLContentEditor
          htmlContent={ctrl.space.content}
          canEdit={ctrl.isAdmin}
          onContentChange={ctrl.handleChange}
        />

        {ctrl.space.isAdmin() ? (
          <SpaceInvitationEditorPage spacePk={ctrl.space.pk} />
        ) : (
          <SpaceInvitationViewerPage spacePk={ctrl.space.pk} />
        )}
      </div>
    </>
  );
}
