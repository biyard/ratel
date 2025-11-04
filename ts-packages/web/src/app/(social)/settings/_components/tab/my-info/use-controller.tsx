import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import { useIdentityVerification } from '@/features/did/hooks/use-identity-verification';

export class Controller {
  constructor(
    public userInfo: ReturnType<typeof useSuspenseUserInfo>,
    public navigate: ReturnType<typeof useNavigate>,
    public t: TFunction<'Settings', undefined>,
    public identityVerification: ReturnType<typeof useIdentityVerification>,
  ) {}

  get user() {
    return this.userInfo.data;
  }

  handleIdentityVerification = async () => {
    await this.identityVerification.mutateAsync();
  };
}

export function useController() {
  const userInfo = useSuspenseUserInfo();
  const navigate = useNavigate();
  const { t } = useTranslation('Settings');
  const identityVerification = useIdentityVerification();

  return new Controller(userInfo, navigate, t, identityVerification);
}
