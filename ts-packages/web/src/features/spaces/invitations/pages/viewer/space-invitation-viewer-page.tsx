import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceInvitationViewerController } from './use-space-invitation-viewer-controller';

export function SpaceInvitationViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceInvitationViewerPage: spacePk=${spacePk}`);

  const _ctrl = useSpaceInvitationViewerController(spacePk);

  return (
    <>
      <div></div>
    </>
  );
}
