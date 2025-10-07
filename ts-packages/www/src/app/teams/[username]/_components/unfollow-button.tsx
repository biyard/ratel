import { useTranslation } from 'react-i18next';
import { useState } from 'react';

export default function UnFollowButton({ onClick }: { onClick: () => void }) {
  const { t } = useTranslation('Team');
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] border border-unfollow-button-border hover:border-[#ff4d4f] hover:bg-unfollow-button-bg/80 rounded-[50px] bg-unfollow-button-bg"
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div
        className={`font-bold  ${isHovered ? 'text-[#ff4d4f]' : 'text-unfollow-button-text'} text-xs`}
      >
        {isHovered ? t('unfollow') : t('following')}
      </div>
    </div>
  );
}
