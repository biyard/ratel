'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import { WelcomeHeader } from './welcome-header';
import { PrimaryButton } from '../button/primary-button';
import { LoginPopupFooter } from './login-popup-footer';
import { useTranslation } from 'react-i18next';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface ConfirmPopupProps {}

export const ConfirmPopup = ({}: ConfirmPopupProps) => {
  const { t } = useTranslation('Signup');
  const popup = usePopup();

  const handleClose = () => {
    popup.close();
  };

  return (
    <div className="max-w-100 w-full mx-1.25 max-mobile:!max-w-full mt-[35px]">
      <div className="w-full flex flex-col gap-[35px] mb-6">
        <WelcomeHeader
          title={t('welcome_title')}
          description={t('welcome_description')}
        />

        <PrimaryButton onClick={handleClose} disabled={false}>
          {t('start')}
        </PrimaryButton>
      </div>

      <LoginPopupFooter />
    </div>
  );
};
