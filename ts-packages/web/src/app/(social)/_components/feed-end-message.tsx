'use client';
import { useTranslation } from 'react-i18next';

export default function FeedEndMessage() {
  const { t } = useTranslation('Home');

  return (
    <div
      className="text-center text-gray-400 my-6"
      aria-label="End of feed message"
    >
      🎉 {t('feed_end_message')}
    </div>
  );
}
