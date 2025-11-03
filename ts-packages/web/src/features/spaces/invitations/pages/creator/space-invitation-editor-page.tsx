import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceInvitationEditorController } from './use-space-invitation-editor-controller';
import { Button } from '@/components/ui/button';
import InviteMemberTable from '../../components/invite-member-table';
import { SpacePublishState } from '@/features/spaces/types/space-common';

export function SpaceInvitationEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceInvitationEditorPage: spacePk=${spacePk}`);

  const ctrl = useSpaceInvitationEditorController(spacePk);
  const t = ctrl.t;

  const inviteMembers = ctrl.invitationMembers ?? [];
  const isDraft = ctrl.space.publishState === SpacePublishState.Draft;

  return (
    <div className="flex flex-col w-full gap-4">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
        <div className="flex flex-col">
          <div className="text-base font-semibold">{t('invited_members')}</div>
          <div className="text-xs text-neutral-400">{t('invite_info')}</div>
        </div>
        {isDraft && (
          <Button
            variant="primary"
            className="self-start sm:self-auto w-fit"
            onClick={ctrl.openInviteMemberPopup}
          >
            {t('invite_space')}
          </Button>
        )}
      </div>

      <InviteMemberTable
        isDraft={isDraft}
        inviteMembers={inviteMembers}
        t={t}
        handleDeleteMember={ctrl.handleDeleteMember}
      />
    </div>
  );
}
