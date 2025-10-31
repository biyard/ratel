import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceInvitationEditorController } from './use-space-invitation-editor-controller';

export function SpaceInvitationEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceInvitationEditorPage: spacePk=${spacePk}`);

  const _ctrl = useSpaceInvitationEditorController(spacePk);

  return (
    <>
      <div></div>
    </>
  );
}
