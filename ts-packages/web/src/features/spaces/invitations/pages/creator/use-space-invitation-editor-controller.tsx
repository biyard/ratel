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
import { useResentVerificationMutation } from '../../hooks/use-resent-resent-mutation';

export class SpaceInvitationEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public invitationMembers: InvitationMemberResponse[],
    public popup,
    public t: TFunction<'SpaceInvitationEditor', undefined>,

    public upsertInvitation: ReturnType<typeof useUpsertInvitationMutation>,
    public resentVerification: ReturnType<typeof useResentVerificationMutation>,
  ) {}

  openInviteMemberPopup = () => {
    this.popup
      .open(<InviteMemberPopup spacePk={this.spacePk} />)
      .withTitle(this.t('invite_member'));
  };

  handleResentCode = async (email: string) => {
    try {
      await this.resentVerification.mutateAsync({
        spacePk: this.spacePk,
        email: email,
      });

      showSuccessToast(this.t('success_send_code'));
    } catch {
      showErrorToast(this.t('failed_send_code'));
    } finally {
      this.popup.close();
    }
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
    }
  };
}

export function useSpaceInvitationEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: invitationMembers } = useInvitationMember(spacePk);
  const popup = usePopup();
  const { t } = useTranslation('SpaceInvitationEditor');
  const upsertInvitation = useUpsertInvitationMutation();
  const resentVerification = useResentVerificationMutation();

  return new SpaceInvitationEditorController(
    spacePk,
    space,
    invitationMembers.members,
    popup,
    t,

    upsertInvitation,
    resentVerification,
  );
}
