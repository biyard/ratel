import { route } from '@/route';
import { useTranslation } from 'react-i18next';
import Link from 'next/link';

export default function NotFound() {
  const { t } = useTranslation('Threads');
  return (
    <div className="flex flex-col items-center w-full">
      <h1 className="text-4xl font-bold mb-4">{t('not_found_title')}</h1>
      <p className="text-gray-600 mb-8">{t('not_found_description')}</p>
      <div className="flex gap-4">
        <Link href={route.home()} className="text-primary hover:underline">
          {t('go_home')}
        </Link>
      </div>
    </div>
  );
}
