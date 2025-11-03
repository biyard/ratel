import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { usePopup } from '@/lib/contexts/popup-service';
import InviteMemberPopup from '../../components/modals/invite_member';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import useInvitationMember from '../../hooks/use-invitation';
import { InvitationMemberResponse } from '../../types/invitation-member-response';
import { useUpsertInvitationMutation } from '../../hooks/use-upsert-invitation-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export class SpaceInvitationEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public invitationMembers: InvitationMemberResponse[],
    public popup,
    public t: TFunction<'SpaceInvitationEditor', undefined>,

    public upsertInvitation: ReturnType<typeof useUpsertInvitationMutation>,
  ) {}

  openInviteMemberPopup = () => {
    this.popup
      .open(<InviteMemberPopup spacePk={this.spacePk} />)
      .withTitle(this.t('invite_member'));
  };

  handleDeleteMember = async (index: number) => {
    const userPks: string[] = this.invitationMembers
      .filter((_, i) => i !== index)
      .map((u) => u.user_pk)
      .filter((v): v is string => typeof v === 'string' && v.length > 0);

    try {
      await this.upsertInvitation.mutateAsync({
        spacePk: this.spacePk,
        user_pks: userPks,
      });

      showSuccessToast(this.t('success_invitation_users'));
    } catch {
      showErrorToast(this.t('failed_invitation_users'));
    } finally {
      this.popup.close();
    }
  };
}

export function useSpaceInvitationEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: invitationMembers } = useInvitationMember(spacePk);
  const popup = usePopup();
  const { t } = useTranslation('SpaceInvitationEditor');
  const upsertInvitation = useUpsertInvitationMutation();

  return new SpaceInvitationEditorController(
    spacePk,
    space,
    invitationMembers.members,
    popup,
    t,

    upsertInvitation,
  );
}
