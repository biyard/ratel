'use client';

import React from 'react';
import AlertCircle from '@/assets/icons/alert-circle.svg';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoaderPopup } from './loader-popup';
import { LoginPopupFooter } from './login-popup-footer';
import { EventType, loginWithGoogle } from '@/lib/service/firebase-service';
import UserSetupPopup from './user-setup-popup';
import { logger } from '@/lib/logger';
import { useEd25519KeyPair } from '@/lib/contexts/auth-context';

interface LoginFailurePopupProps {
  id?: string;
  logo: React.ReactNode;
  logoOrigin: React.ReactNode;
  title: string;
  description: string;
  msg: string;
  serviceName: string;
}

export const LoginFailurePopup = ({
  id = 'login_failure_popup',
  logo,
  logoOrigin,
  title,
  description,
  msg,
  serviceName,
}: LoginFailurePopupProps) => {
  const popup = usePopup();
  const keyPair = useEd25519KeyPair();
  const failureMsg = `Failed to connect to ${serviceName}.\nWould you like to try again?`;

  return (
    <div id={id} className="w-100 max-mobile:!w-full gap-[35px] mt-[35px]">
      <div className="flex flex-col gap-[8px] mb-[8px]">
        <div
          className="w-full flex flex-row pl-5 py-5.5 bg-black border-[1px] rounded-[10px] justify-start items-center gap-4.25 cursor-pointer border-c-p-50"
          onClick={async () => {
            logger.debug('Google login button clicked');
            const loader = popup.open(
              <LoaderPopup
                title={title}
                description={description}
                logo={logo}
                logoOrigin={logoOrigin}
                msg={msg}
                serviceName={serviceName}
              />,
            );

            try {
              const user = await loginWithGoogle(keyPair);
              loader.close();
              logger.debug('user info: ', user);

              if (user.eventType == EventType.SignUp) {
                popup
                  .open(
                    <UserSetupPopup
                      email={user.email ?? ''}
                      nickname={user.displayName ?? ''}
                      profileUrl={user.photoURL ?? ''}
                      principal={user.principal ?? ''}
                    />,
                  )
                  .withoutBackdropClose();
              } else if (user.eventType == EventType.Login) {
                popup.close();
              }
            } catch (err) {
              popup.open(
                <LoginFailurePopup
                  title={title}
                  description={description}
                  logo={logo}
                  logoOrigin={logoOrigin}
                  msg={msg}
                  serviceName={serviceName}
                />,
              );
              logger.debug('failed to google sign in with error: ', err);
            }
          }}
        >
          {logoOrigin}
          <div className="flex flex-col gap-[3px]">
            <span className="text-white text-base/4.75 font-semibold">
              {msg}
            </span>
          </div>
        </div>

        <div className="w-full flex flex-row pl-5 py-2.5 bg-c-p-50-10 rounded-[10px] justify-start items-center gap-2.5">
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
