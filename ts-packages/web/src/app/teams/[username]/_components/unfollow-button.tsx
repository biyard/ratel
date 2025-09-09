import { useTranslations } from 'next-intl';
import React, { useState } from 'react';

export default function UnFollowButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('Team');
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-transparent border border-neutral-700 hover:border-[#ff4d4f] hover:bg-[#ffe3e3] rounded-[50px]"
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div
        className={`font-bold  ${isHovered ? 'text-[#ff4d4f]' : 'text-neutral-700'} text-xs`}
      >
        {isHovered ? t('unfollow') : t('following')}
      </div>
    </div>
  );
}
