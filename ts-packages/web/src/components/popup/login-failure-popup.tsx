'use client';

import React from 'react';
import AlertCircle from '@/assets/icons/alert-circle.svg';
import { LoginPopupFooter } from './login-popup-footer';
import { useTranslations } from 'next-intl';

interface LoginFailurePopupProps {
  id?: string;
  logo: React.ReactNode;
  logoOrigin: React.ReactNode;
  title: string;
  description: string;
  msg: string;
  serviceName: string;
  onRetry: () => Promise<void>;
}

export const LoginFailurePopup = ({
  id = 'login_failure_popup',
  logoOrigin,
  msg,
  serviceName,
  onRetry,
}: LoginFailurePopupProps) => {
  // const keyPair = useEd25519KeyPair();
  const t = useTranslations('Signup');
  const failureMsg = t('failureMsg', { serviceName });

  return (
    <div id={id} className="w-100 max-mobile:!w-full gap-[35px] mt-[35px]">
      <div className="flex flex-col gap-[8px] mb-[8px]">
        <div
          className="w-full flex flex-row pl-5 py-5.5 bg-component-bg border-[1px] rounded-[10px] justify-start items-center gap-4.25 cursor-pointer border-c-p-50"
          onClick={onRetry}
        >
          {logoOrigin}
          <div className="flex flex-col gap-[3px]">
            <span className="text-foreground text-base/4.75 font-semibold">
              {msg}
            </span>
          </div>
        </div>

        <div className="w-full flex flex-row pl-5 py-2.5 bg-c-p-50-10 light:bg-white border border-transparent light:border-neutral-300 rounded-[10px] justify-start items-center gap-2.5">
          <AlertCircle color="#DB2780" />
          <div className="flex flex-col gap-[3px]">
            <span className="text-c-p-50 text-[15px]/6 font-semibold tracking-wide whitespace-pre-line">
              {failureMsg}
            </span>
          </div>
        </div>
      </div>

      <LoginPopupFooter />
    </div>
  );
};
