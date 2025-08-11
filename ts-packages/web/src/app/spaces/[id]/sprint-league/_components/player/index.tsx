'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import Image from 'next/image';
import unknown from '@/assets/images/unknown.png';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Input } from '@/components/ui/input';
import { useSprintLeagueStore } from '../../sprint-league-store';
import { openCharacterSelectModal } from './select-player-modal';
import { usePopup } from '@/lib/contexts/popup-service';
import { PlayerImages } from '@/lib/api/models/sprint_league';
import IsolatedCharacter from '../animation/isolated-character';

import { pixiAssetManager } from '../animation/assets';
import { useEffect } from 'react';

export const BasePlayerImages: PlayerImages[] = [
  {
    alias: 'lee_jun',
    run: {
      json: 'https://metadata.ratel.foundation/assets/lee_jun_run.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jun_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/lee_jun_win.png',
    lose: 'https://metadata.ratel.foundation/assets/lee_jun_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/lee_jun_selected.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jun_selected.webp',
    },
  },
  {
    alias: 'kim_moon',
    run: {
      json: 'https://metadata.ratel.foundation/assets/kim_moon_run.json',
      image: 'https://metadata.ratel.foundation/assets/kim_moon_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/kim_moon_win.png',
    lose: 'https://metadata.ratel.foundation/assets/kim_moon_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/kim_moon_selected.json',
      image: 'https://metadata.ratel.foundation/assets/kim_moon_selected.webp',
    },
  },
  {
    alias: 'lee_jae',
    run: {
      json: 'https://metadata.ratel.foundation/assets/lee_jae_run.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jae_run.webp',
    },
    win: 'https://metadata.ratel.foundation/assets/lee_jae_win.png',
    lose: 'https://metadata.ratel.foundation/assets/lee_jae_lose.png',
    select: {
      json: 'https://metadata.ratel.foundation/assets/lee_jae_selected.json',
      image: 'https://metadata.ratel.foundation/assets/lee_jae_selected.webp',
    },
  },
];

export default function PlayerEdit({ isEdit }: { isEdit: boolean }) {
  const updatePlayer = useSprintLeagueStore((s) => s.updatePlayer);
  const storePlayers = useSprintLeagueStore((s) => s.players);
  const popup = usePopup();
  useEffect(() => {
    const preloadAssets = async () => {
      const loadPromises = BasePlayerImages.flatMap((v) => [
        pixiAssetManager.loadSpritesheet(`${v.alias}_run`, v.run.json),
        pixiAssetManager.loadSpritesheet(`${v.alias}_selected`, v.select.json),
        pixiAssetManager.loadSpritesheet(`${v.alias}_win`, v.win),
        pixiAssetManager.loadSpritesheet(`${v.alias}_lose`, v.lose),
      ]);

      await Promise.allSettled(loadPromises);
    };

    preloadAssets();
  }, []);

  useEffect(() => {
    const loadAssets = async () => {
      if (!storePlayers) {
        return;
      }

      const loadPromises = Object.values(storePlayers).flatMap((v) => [
        pixiAssetManager.loadSpritesheet(
          `${v.player_images.alias}_run`,
          v.player_images.run.json,
        ),
        pixiAssetManager.loadSpritesheet(
          `${v.player_images.alias}_selected`,
          v.player_images.select.json,
        ),
        pixiAssetManager.loadSpritesheet(
          `${v.player_images.alias}_win`,
          v.player_images.win,
        ),
        pixiAssetManager.loadSpritesheet(
          `${v.player_images.alias}_lose`,
          v.player_images.lose,
        ),
      ]);

      await Promise.allSettled(loadPromises);
    };
    loadAssets();
  }, [storePlayers]);

  return (
    <BlackBox>
      <div className="w-full flex flex-col gap-8">
        <div className="font-bold text-white text-[15px]/[20px]">
          Sprint Players
        </div>
        <div className="flex flex-col gap-10 w-full">
          {Object.values(storePlayers).map((player) => (
            <div key={player.id} className="flex flex-row min-h-50 gap-5">
              <div className="aspect-square">
                <PlayerSelector
                  id={player.id}
                  isEdit={isEdit}
                  handleSelect={() => {
                    openCharacterSelectModal(popup, (images: PlayerImages) => {
                      updatePlayer(player.id, {
                        ...player,
                        player_images: images,
                      });
                    });
                  }}
                  alias={`${player.player_images.alias}`}
                />
              </div>
              <div className="flex flex-col flex-1 gap-2.5">
                <Input
                  disabled={!isEdit}
                  contentEditable={isEdit}
                  className="w-full text-[15px]/[23px] px-4 py-6"
                  value={player.name}
                  onChange={(e) => {
                    updatePlayer(player.id, {
                      ...player,
                      name: e.currentTarget.value || '',
                    });
                  }}
                />
                <Textarea
                  disabled={!isEdit}
                  contentEditable={isEdit}
                  className="flex-1"
                  value={player.description || ''}
                  onChange={(e) =>
                    updatePlayer(player.id, {
                      ...player,
                      description: e.currentTarget.value || '',
                    })
                  }
                />
              </div>
            </div>
          ))}
        </div>
      </div>
    </BlackBox>
  );
}
function PlayerSelector({
  isEdit,
  alias,
  handleSelect,
}: {
  id: number;
  isEdit: boolean;
  alias: string;
  handleSelect: () => void;
}) {
  const isEmpty = alias.startsWith('_');
  return (
    <div className="relative bg-neutral-800 rounded-lg">
      {isEdit && (
        <Button
          variant="default"
          size="sm"
          className="z-1 absolute -translate-x-1/2 -translate-y-3/4 top-3/4 left-1/2"
          onClick={handleSelect}
        >
          Select Character
        </Button>
      )}
      <div className="aspect-square size-75 rounded-lg overflow-hidden relative">
        <div className="absolute inset-0 left-0 top-0">
          <IsolatedCharacter alias={alias} />
        </div>
        {isEmpty && <Image src={unknown} alt="Unknown Player" />}
      </div>
    </div>
  );
}
