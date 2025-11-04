import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceMembersViewerController } from './use-space-members-viewer-controller';
// import InviteCodeBox from '../../components/invite-code-box';

export function SpaceMembersViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceMembersViewerPage: spacePk=${spacePk}`);

  const _ctrl = useSpaceMembersViewerController(spacePk);

  return (
    <>
      {/* {!ctrl.space.verified && (
        <InviteCodeBox
          t={ctrl.t}
          verify={async (code: string) => {
            await ctrl.handleVerify(code);
          }}
        />
      )} */}
    </>
  );
}
