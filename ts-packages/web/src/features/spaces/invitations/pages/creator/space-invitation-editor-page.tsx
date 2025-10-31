import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceInvitationEditorController } from './use-space-invitation-editor-controller';
import { Button } from '@/components/ui/button';

export function SpaceInvitationEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceInvitationEditorPage: spacePk=${spacePk}`);

  const ctrl = useSpaceInvitationEditorController(spacePk);

  return (
    <>
      <div className="flex flex-col w-full">
        <div className="flex flex-row w-full justify-end">
          <Button variant="primary" onClick={ctrl.openInviteMemberPopup}>
            {ctrl.t('invite_space')}
          </Button>
        </div>
      </div>
    </>
  );
}
