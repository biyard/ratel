import { useTranslations } from 'next-intl';
import React from 'react';

export default function FollowButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('Team');

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-follow-button-bg-secondary hover:bg-follow-button-bg-secondary/80 rounded-[50px]"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-follow-button-text-secondary text-xs">
        {t('follow')}
      </div>
    </div>
  );
}
