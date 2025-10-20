// import { usePopup } from '@/lib/contexts/popup-service';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PlayerImage, SpriteSheet } from '../types/sprint-league-player';
import { BasePlayerImages } from './game/constants';
// import { TFunction } from 'i18next';

// export const BasePlayerImages: Record<string, PlayerImage> = {
//   lee_jun: {
//     run: {
//       json: 'https://metadata.ratel.foundation/assets/lee_jun_run.json',
//       image: 'https://metadata.ratel.foundation/assets/lee_jun_run.webp',
//     },
//     win: 'https://metadata.ratel.foundation/assets/lee_jun_win.png',
//     lose: 'https://metadata.ratel.foundation/assets/lee_jun_lose.png',
//     select: {
//       json: 'https://metadata.ratel.foundation/assets/lee_jun_selected.json',
//       image: 'https://metadata.ratel.foundation/assets/lee_jun_selected.webp',
//     },
//   },
//   kim_moon: {
//     run: {
//       json: 'https://metadata.ratel.foundation/assets/kim_moon_run.json',
//       image: 'https://metadata.ratel.foundation/assets/kim_moon_run.webp',
//     },
//     win: 'https://metadata.ratel.foundation/assets/kim_moon_win.png',
//     lose: 'https://metadata.ratel.foundation/assets/kim_moon_lose.png',
//     select: {
//       json: 'https://metadata.ratel.foundation/assets/kim_moon_selected.json',
//       image: 'https://metadata.ratel.foundation/assets/kim_moon_selected.webp',
//     },
//   },
//   lee_jae: {
//     run: {
//       json: 'https://metadata.ratel.foundation/assets/lee_jae_run.json',
//       image: 'https://metadata.ratel.foundation/assets/lee_jae_run.webp',
//     },
//     win: 'https://metadata.ratel.foundation/assets/lee_jae_win.png',
//     lose: 'https://metadata.ratel.foundation/assets/lee_jae_lose.png',
//     select: {
//       json: 'https://metadata.ratel.foundation/assets/lee_jae_selected.json',
//       image: 'https://metadata.ratel.foundation/assets/lee_jae_selected.webp',
//     },
//   },
// };

// const openCharacterSelectModal = (
//   t: TFunction<'SprintLeague', undefined>,
//   popup: ReturnType<typeof usePopup>,
//   handleSelect: (images: PlayerImage) => void,
// ) => {
//   popup
//     .open(
//       <PlayerSelectModal
//         players={BasePlayerImages}
//         onSelect={(id) => {
//           const selectedPlayer = BasePlayerImages[id];

//           if (!selectedPlayer) {
//             console.error(`Player with id ${id} not found`);
//             return;
//           }

//           handleSelect(selectedPlayer);

//           popup.close();
//         }}
//       />,
//     )
//     .withTitle(t('select_player_modal_title'));
// };
// export { openCharacterSelectModal };

export default function PlayerSelectModal({
  onSelect,
  playerImages = BasePlayerImages,
}: {
  onSelect: (spriteSheet: PlayerImage) => void;
  playerImages?: PlayerImage[];
}) {
  const { t } = useTranslation('SprintSpace');
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  return (
    <div className="flex flex-col gap-4 w-[50vw] max-w-200">
      <span>{t('select_player_modal_desc')}</span>
      <div className="grid-cols-3 grid gap-10">
        {playerImages.map((playerImage, index) => (
          <div
            key={playerImage.win}
            aria-selected={selectedIndex === index}
            className="aria-selected:border-primary hover:bg-black border border-transparent hover:border-primary light:hover:bg-primary/10 rounded-2xl cursor-pointer aspect-square overflow-hidden"
            onClick={() => {
              setSelectedIndex(index);
            }}
          >
            {/* <CharacterPreview images={playerImage} /> */}
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
        Select
      </Button>
    </div>
  );
}
