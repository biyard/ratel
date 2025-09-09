import { useTranslations } from 'next-intl';
import React from 'react';

export default function FollowButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('Team');

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-white light:bg-primary hover:bg-gray-300 rounded-[50px]"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-xs">{t('follow')}</div>
    </div>
  );
}
