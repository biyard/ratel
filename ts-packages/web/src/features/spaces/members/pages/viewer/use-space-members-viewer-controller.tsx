import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { showSuccessToast } from '@/lib/toast';
import { useMemo } from 'react';

export class SpaceMembersViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public t: TFunction<'SpaceMemberViewer', undefined>,
  ) {}

  handleVerify = async () => {
    showSuccessToast(this.t('success_verify_user'));
  };
}

export function useSpaceMembersViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceMemberViewer');
  const ctrl = useMemo(
    () => new SpaceMembersViewerController(spacePk, space, t),
    [spacePk, space, t],
  );

  return ctrl;
}
