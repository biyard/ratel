'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import { WelcomeHeader } from './welcome-header';
import { PrimaryButton } from '../button/primary-button';
import { LoginPopupFooter } from './login-popup-footer';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface ConfirmPopupProps {}

export const ConfirmPopup = ({}: ConfirmPopupProps) => {
  const popup = usePopup();

  const handleClose = () => {
    popup.close();
  };

  return (
    <div className="max-w-100 w-full mx-1.25 max-mobile:!max-w-full mt-[35px]">
      <div className="w-full flex flex-col gap-[35px] mb-6">
        <WelcomeHeader
          title="Welcome to Ratel!"
          description="Policy is shaped by civic engagement—when we speak up, policymakers listen. Ratel gives you a platform to take action and shape crypto policy. Your voice matters, so make it heard and help secure a bright future for crypto."
        />

        <PrimaryButton onClick={handleClose} disabled={false}>
          {'Start'}
        </PrimaryButton>
      </div>

      <LoginPopupFooter />
    </div>
  );
};
