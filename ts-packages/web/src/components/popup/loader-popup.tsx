'use client';

import React from 'react';
import { LoginPopupFooter } from './login-popup-footer';

interface LoaderPopupProps {
  id?: string;
  title: string;
  description: string;
  logo: React.ReactNode;
  logoOrigin: React.ReactNode;
  msg: string;
  serviceName: string;
}

export const LoaderPopup = ({
  id = 'loader_popup',
  description,
  logoOrigin,
}: LoaderPopupProps) => {
  return (
    <div
      id={id}
      className="flex flex-col w-100 max-w-100 mx-4.25  max-mobile:!w-full max-mobile:!max-w-full  gap-8.75 mt-8.75"
    >
      <div className="flex flex-col w-full justify-center items-center gap-35">
        {/* Spinner */}
        <div className="border-6 border-t-6 w-20.5 h-20.5 border-primary border-t-background rounded-full animate-spin" />
        <div className="absolute flex-row w-16 h-16 bg-white rounded-full justify-center items-center flex">
          <div className="flex flex-row w-6 h-6 justify-center items-center">
            {logoOrigin}
          </div>
        </div>
      </div>

      <div className="justify-center text-center text-white font-bold text-base/6">
        {description}
      </div>

      <LoginPopupFooter />
    </div>
  );
};
