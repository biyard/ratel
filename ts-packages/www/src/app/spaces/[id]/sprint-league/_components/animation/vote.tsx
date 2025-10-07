// @ts-nocheck
'use client';

import { useTick } from '@pixi/react';
import { Assets, Container, Ticker } from 'pixi.js';
import { useCallback, useRef } from 'react';
import Character from './character';
import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';

export default function VotePlayer({
  players = [],
  selectedPlayerId,
  scale,
  onSelect,
}: {
  players: SprintLeaguePlayer[];
  selectedPlayerId: number | null;
  scale: number;
  onSelect: (index: number) => void;
}) {
  return (
    <>
      <pixiContainer>
        {selectedPlayerId !== null && (
          <pixiText
            position={{ x: 180 * scale, y: 50 * scale }}
            anchor={{ x: 0.5, y: 0.5 }}
            style={{
              fontSize: 30 * scale,
              align: 'center',
              fontWeight: '900',
            }}
            text={players.find((p) => p.id === selectedPlayerId)?.name || ''}
          />
        )}
        <pixiContainer x={75 * scale} y={220 * scale}>
          {players.map((player, index) => (
            <Vote
              scale={scale}
              alias={player.player_images.alias}
              key={player.id}
              x={index * 105 * scale}
              selected={
                selectedPlayerId === null
                  ? null
                  : selectedPlayerId === player.id
              }
              onClick={() => onSelect(player.id)}
            />
          ))}
        </pixiContainer>

        <pixiContainer y={370 * scale}>
          {selectedPlayerId === null && (
            <pixiText
              position={{ x: 180 * scale, y: 0 }}
              anchor={{ x: 0.5, y: 0.5 }}
              style={{
                fill: '#ffffff',
                fontSize: 20 * scale,
                align: 'center',
                fontWeight: 'bold',
              }}
              text="SELECT YOUR PLAYER"
            />
          )}
          {selectedPlayerId !== null && (
            <pixiText
              position={{ x: 180 * scale, y: 0 }}
              anchor={{ x: 0.5, y: 0.5 }}
              style={{
                fill: '#ffffff',
                fontSize: 20 * scale,
                align: 'center',
                fontWeight: '900',
                wordWrap: true,
                wordWrapWidth: 300 * scale,
              }}
              text={
                players.find((p) => p.id === selectedPlayerId)?.description ||
                ''
              }
            />
          )}
        </pixiContainer>
      </pixiContainer>
    </>
  );
}

function Vote({
  alias,
  selected,
  onClick,
  x,
  y = 0,
  scale,
}: {
  alias: string;
  selected: boolean | null;
  onClick: () => void;
  x: number;
  y?: number;
  scale: number;
}) {
  const ref = useRef<Container>(null);

  const updateTicker = useCallback(
    (ticker: Ticker) => {
      const sprite = ref.current;
      if (!sprite) return;

      const targetY = selected ? y - 20 * scale : y;

      sprite.position.y +=
        (targetY - sprite.position.y) * 0.1 * ticker.deltaTime;
    },
    [scale, selected, y],
  );

  useTick((ticker) => {
    updateTicker(ticker);
  });

  const getTexture = () => {
    let textureName = 'vote-default';
    if (selected === true) {
      textureName = 'vote-selected';
    } else if (selected === false) {
      textureName = 'vote-unselected';
    }

    const texture = Assets.get(textureName);
    if (!texture) {
      console.warn(`Texture ${textureName} not found`);
      return Assets.get('vote-default') || null;
    }
    return texture;
  };

  const texture = getTexture();
  if (!texture) {
    return null;
  }

  return (
    <pixiContainer
      ref={ref}
      position={{ x, y }}
      eventMode="static"
      cursor="pointer"
      onClick={onClick}
      onMouseDown={onClick}
      onPointerTap={onClick}
    >
      <pixiSprite texture={texture} anchor={{ x: 0.5, y: 0.5 }} scale={scale} />
      <Character alias={alias} speed={0} x={-50} y={50} scale={scale} />
    </pixiContainer>
  );
}
