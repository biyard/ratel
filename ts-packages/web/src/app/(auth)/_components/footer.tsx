'use client';
import { useTranslation } from 'react-i18next';

export default function Footer() {
  const { t } = useTranslation('SignIn');
  return (
    <div className="flex flex-row w-full justify-center items-center gap-2.5 text-neutral-400 text-xs/3.5 font-medium">
      <button>{t('privacy_policy')}</button>
      <button>{t('terms_of_service')}</button>
    </div>
  );
}
