// import { usePopup } from '@/lib/contexts/popup-service';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { PlayerImage } from '../types/sprint-league-player';
import { BasePlayerImages } from './game/constants';
import { TFunction } from 'i18next';

export default function PlayerSelectModal({
  t,
  onSelect,
  playerImages = BasePlayerImages,
}: {
  t: TFunction<'SpaceSprintLeague', undefined>;
  onSelect: (spriteSheet: PlayerImage) => void;
  playerImages?: PlayerImage[];
}) {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  return (
    <div className="flex flex-col gap-4 w-[50vw] max-w-200">
      <span>{t('select_player_modal_desc')}</span>
      <div className="grid-cols-3 grid gap-10">
        {playerImages.map((playerImage, index) => (
          <div
            key={playerImage.win}
            aria-selected={selectedIndex === index}
            className="aria-selected:border-primary hover:bg-black border border-transparent hover:border-primary light:hover:bg-primary/10 rounded-2xl cursor-pointer aspect-square overflow-hidden flex items-center justify-center"
            onClick={() => {
              setSelectedIndex(index);
            }}
          >
            <img
              src={playerImage.win}
              alt=""
              className="w-full h-full object-contain"
            />
          </div>
        ))}
      </div>
      <Button
        variant="default"
        className="bg-primary"
        disabled={selectedIndex === null}
        onClick={() => {
          if (selectedIndex !== null) {
            onSelect(playerImages[selectedIndex]);
          }
        }}
      >
        {t('select_player_modal_confirm_button')}
      </Button>
    </div>
  );
}
