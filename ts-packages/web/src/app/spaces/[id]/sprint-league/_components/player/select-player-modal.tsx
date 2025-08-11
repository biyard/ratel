'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import IsolatedCharacter from '../animation/isolated-character';
import { PlayerImages } from '@/lib/api/models/sprint_league';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { BasePlayerImages } from '.';

const openCharacterSelectModal = (
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
    .withTitle("Select a player's character");
};
export { openCharacterSelectModal };

export default function PlayerSelectModal({
  onSelect,
  players,
}: {
  onSelect: (id: string) => void;
  players: Record<string, string>;
}) {
  const [value, setValue] = useState<string | null>(null);

  return (
    <div className="flex flex-col gap-4 w-[50vw] max-w-200">
      <span>
        Once the Sprint league is made public, character selection can no longer
        be changed.
      </span>
      <div className="grid-cols-3 grid gap-10">
        {Object.entries(players).map(([alias]) => (
          <div
            key={alias}
            aria-selected={value === alias}
            className="aria-selected:border-primary hover:bg-black border border-transparent hover:border-primary rounded-2xl cursor-pointer aspect-square overflow-hidden"
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
