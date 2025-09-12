'use client';
import { useTranslations } from 'next-intl';

export default function Footer() {
  const t = useTranslations('SignIn');
  return (
    <div className="flex flex-row w-full justify-center items-center gap-2.5 text-neutral-400 text-xs/3.5 font-medium">
      <button>{t('privacy_policy')}</button>
      <button>{t('terms_of_service')}</button>
    </div>
  );
}
