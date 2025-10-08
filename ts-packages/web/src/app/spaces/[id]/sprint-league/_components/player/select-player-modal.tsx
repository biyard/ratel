'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import IsolatedCharacter from '../animation/isolated-character';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PlayerImages, BasePlayerImages } from '.';

const openCharacterSelectModal = (
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  t: (key: string, values?: Record<string, any>) => string,
  popup: ReturnType<typeof usePopup>,
  handleSelect: (images: PlayerImages) => void,
) => {
  const players: Record<string, string> = BasePlayerImages.reduce(
    (acc, player) => {
      acc[`${player.alias}`] = player.run.json;
      return acc;
    },
    {} as Record<string, string>,
  );

  popup
    .open(
      <PlayerSelectModal
        players={players}
        onSelect={(alias) => {
          const selectedPlayer = BasePlayerImages.find(
            (player) => `${player.alias}` === alias,
          );

          if (!selectedPlayer) {
            console.error(`Player with alias ${alias} not found`);
            return;
          }

          handleSelect(selectedPlayer);

          popup.close();
        }}
      />,
    )
    .withTitle(t('select_player_modal_title'));
};
export { openCharacterSelectModal };

export default function PlayerSelectModal({
  onSelect,
  players,
}: {
  onSelect: (id: string) => void;
  players: Record<string, string>;
}) {
  const { t } = useTranslation('SprintSpace');
  const [value, setValue] = useState<string | null>(null);

  return (
    <div className="flex flex-col gap-4 w-[50vw] max-w-200">
      <span>{t('select_player_modal_desc')}</span>
      <div className="grid-cols-3 grid gap-10">
        {Object.entries(players).map(([alias]) => (
          <div
            key={alias}
            aria-selected={value === alias}
            className="aria-selected:border-primary hover:bg-black border border-transparent hover:border-primary light:hover:bg-primary/10 rounded-2xl cursor-pointer aspect-square overflow-hidden"
            onClick={() => {
              setValue(alias);
            }}
          >
            <IsolatedCharacter alias={alias} />
          </div>
        ))}
      </div>
      <Button
        variant="default"
        className="bg-primary"
        disabled={!value}
        onClick={() => {
          if (value) {
            onSelect(value);
          }
        }}
      >
        Select
      </Button>
    </div>
  );
}
