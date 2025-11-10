import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { Button } from '@/components/ui/button';
import InviteMemberTable from '../../components/invite-member-table';
import {
  SpacePublishState,
  SpaceStatus,
} from '@/features/spaces/types/space-common';
import { useSpaceMembersEditorController } from './use-space-members-editor-controller';

export function SpaceMembersEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceMembersEditorPage: spacePk=${spacePk}`);

  const ctrl = useSpaceMembersEditorController(spacePk);
  const t = ctrl.t;

  const inviteMembers = ctrl.invitationMembers ?? [];
  const isDraft = ctrl.space.publishState === SpacePublishState.Draft;

  return (
    <div className="flex flex-col w-full gap-4">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
        <div className="flex flex-col">
          <div className="text-base font-semibold">{t('invited_members')}</div>
          <div className="text-xs text-text-secondary">{t('invite_info')}</div>
        </div>
        {ctrl.space.status !== SpaceStatus.Started && (
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
        status={ctrl.space.status}
        isDraft={isDraft}
        inviteMembers={inviteMembers}
        t={t}
        handleDeleteMember={ctrl.handleDeleteMember}
        handleSendCode={ctrl.handleResentCode}
      />
    </div>
  );
}
