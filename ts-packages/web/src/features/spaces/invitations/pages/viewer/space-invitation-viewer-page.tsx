import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceInvitationViewerController } from './use-space-invitation-viewer-controller';
import InviteCodeBox from '../../components/invite-code-box';

export function SpaceInvitationViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceInvitationViewerPage: spacePk=${spacePk}`);

  const ctrl = useSpaceInvitationViewerController(spacePk);

  return (
    <>
      {!ctrl.space.verified && (
        <InviteCodeBox
          t={ctrl.t}
          verify={async (code: string) => {
            await ctrl.handleVerify(code);
          }}
        />
      )}
    </>
  );
}
