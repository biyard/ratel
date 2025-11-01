import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { usePopup } from '@/lib/contexts/popup-service';
import InviteMemberPopup from '../../components/modals/invite_member';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class SpaceInvitationEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public popup,
    public t: TFunction<'SpaceInvitationEditor', undefined>,
  ) {}

  openInviteMemberPopup = () => {
    this.popup
      .open(<InviteMemberPopup spacePk={this.spacePk} />)
      .withTitle(this.t('invite_member'));
  };
}

export function useSpaceInvitationEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const popup = usePopup();
  const { t } = useTranslation('SpaceInvitationEditor');
  return new SpaceInvitationEditorController(spacePk, space, popup, t);
}
