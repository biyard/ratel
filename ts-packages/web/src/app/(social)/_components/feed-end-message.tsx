'use client';
import { useTranslations } from 'next-intl';

export default function FeedEndMessage() {
  const t = useTranslations('Home');

  return (
    <div
      className="text-center text-gray-400 my-6"
      aria-label="End of feed message"
    >
      🎉 {t('feed_end_message')}
    </div>
  );
}
