import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useVerifySpaceCodeMutation } from '../../hooks/use-verify-space-code-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export class SpaceInvitationViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public t: TFunction<'SpaceInvitationViewer', undefined>,

    public verifySpaceCode: ReturnType<typeof useVerifySpaceCodeMutation>,
  ) {}

  handleVerify = async (code: string) => {
    try {
      await this.verifySpaceCode.mutateAsync({
        spacePk: this.spacePk,
        code,
      });

      showSuccessToast(this.t('success_verify_user'));
    } catch {
      showErrorToast(this.t('failed_verify_user'));
    }
  };
}

export function useSpaceInvitationViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceInvitationViewer');
  const verifySpaceCode = useVerifySpaceCodeMutation();
  return new SpaceInvitationViewerController(
    spacePk,
    space,
    t,
    verifySpaceCode,
  );
}
