'use client';

import { useTranslations } from 'next-intl';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface LoginPopupFooterProps {}

export const LoginPopupFooter = ({}: LoginPopupFooterProps) => {
  const t = useTranslations('SignIn');
  return (
    <div className="flex flex-row w-full justify-center items-center gap-2.5">
      <div className="cursor-pointer text-neutral-400 text-xs/3.5 font-medium">
        {t('privacy_policy')}
      </div>
      <div className="cursor-pointer text-neutral-400 text-xs/3.5 font-medium">
        {t('terms_of_service')}
      </div>
    </div>
  );
};
