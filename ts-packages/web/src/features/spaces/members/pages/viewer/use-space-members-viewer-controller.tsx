import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useVerifySpaceCodeMutation } from '../../hooks/use-verify-space-code-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useEffect, useMemo, useRef } from 'react';
import { useLocation, useNavigate } from 'react-router';

export class SpaceMembersViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public t: TFunction<'SpaceMemberViewer', undefined>,
    public verifySpaceCode: ReturnType<typeof useVerifySpaceCodeMutation>,
  ) {}

  handleVerify = async (code: string) => {
    await this.verifySpaceCode.mutateAsync({ spacePk: this.spacePk, code });
    showSuccessToast(this.t('success_verify_user'));
  };
}

export function useSpaceMembersViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceMemberViewer');
  const verifySpaceCode = useVerifySpaceCodeMutation();
  const ctrl = useMemo(
    () => new SpaceMembersViewerController(spacePk, space, t, verifySpaceCode),
    [spacePk, space, t, verifySpaceCode],
  );

  const location = useLocation();
  const navigate = useNavigate();

  const { code, cleanedPath } = useMemo(() => {
    const sp = new URLSearchParams(location.search);
    const c = (sp.get('code') || '').trim();
    sp.delete('code');
    const clean =
      location.pathname + (sp.toString() ? `?${sp.toString()}` : '');
    return { code: c, cleanedPath: clean };
  }, [location.pathname, location.search]);

  const inFlightRef = useRef(false);

  useEffect(() => {
    if (!code) return;

    const key = `redeem:${spacePk}:${code.toUpperCase()}`;
    if (sessionStorage.getItem(key)) {
      navigate(cleanedPath, { replace: true });
      return;
    }
    if (inFlightRef.current || ctrl.verifySpaceCode.isPending) return;

    inFlightRef.current = true;
    sessionStorage.setItem(key, '1');

    (async () => {
      try {
        await ctrl.handleVerify(code);
      } catch {
        showErrorToast(t('failed_verify_user'));
      } finally {
        sessionStorage.removeItem(key);
        navigate(cleanedPath, { replace: true });
        inFlightRef.current = false;
      }
    })();
  }, [code, cleanedPath, ctrl, navigate, spacePk, t]);

  return ctrl;
}
